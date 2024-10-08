use std::{
    fmt::Debug,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tempfile::Builder;

use crate::Result;

pub struct Database {
    temp_dir: PathBuf,
}

impl Database {
    pub fn new<P: AsRef<Path>>(base: P) -> Self {
        let path = base.as_ref();
        let temp_dir = path.join("temp");
        Database { temp_dir }
    }

    pub fn clean_temp(&self) {
        let _ = fs::remove_dir_all(&self.temp_dir);
        let _ = fs::create_dir_all(&self.temp_dir);
    }

    pub fn read<P: AsRef<Path>>(
        &self,
        name: &str,
        path: P,
        start: u64,
        end: u64,
    ) -> Result<Vec<Record>> {
        let mut temp_file = Builder::new().tempfile_in(&self.temp_dir)?;
        let buffer = fs::read(path)?;
        temp_file.write_all(&buffer)?;
        let temp_path = temp_file.path();
        let connection = Connection::open(&temp_path)?;
        let browser = get_browser(name);
        let valid = browser.check(&connection)?;
        let result = if valid {
            browser.read(connection, start, end)?
        } else {
            vec![]
        };
        Ok(result)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    title: String,
    url: String,
    last_visited: u64,
}

struct Table {
    name: String,
}

fn get_browser(name: &str) -> Box<dyn Browse> {
    match name {
        "Google Chrome" | "Microsoft Edge" | "Arc" => Box::new(Chromium),
        "Firefox" => Box::new(Firefox),
        _ => Box::new(UnknownBrowser),
    }
}

trait Browse {
    fn read(&self, connection: Connection, start: u64, end: u64) -> rusqlite::Result<Vec<Record>>;
    fn check(&self, connection: &Connection) -> rusqlite::Result<bool>;
}

struct Chromium;

impl Browse for Chromium {
    fn read(&self, connection: Connection, start: u64, end: u64) -> rusqlite::Result<Vec<Record>> {
        let start = (start / 1000 + 11644473600) * 1000000;
        let end = (end / 1000 + 11644473600) * 1000000;
        let mut stmt = connection.prepare(
            "SELECT title,
                    url,
                    CAST((last_visit_time / 1000000.0 - 11644473600) * 1000 AS INTEGER) AS last_visited
                FROM urls
                WHERE last_visit_time > ? AND last_visit_time < ?;",
        )?;
        let rows = stmt.query_map([start, end], |row| {
            Ok(Record {
                title: row.get(0)?,
                url: row.get(1)?,
                last_visited: row.get(2)?,
            })
        })?;
        rows.collect()
    }

    fn check(&self, connection: &Connection) -> rusqlite::Result<bool> {
        let mut stmt = connection.prepare(
            "SELECT *
                FROM sqlite_master
                WHERE type = 'table' AND 
                    name = 'urls';
                ",
        )?;
        let rows = stmt
            .query_map([], |row| Ok(Table { name: row.get(0)? }))?
            .collect::<Vec<_>>();
        Ok(rows.len() == 1)
    }
}

struct Firefox;

impl Browse for Firefox {
    fn read(&self, connection: Connection, start: u64, end: u64) -> rusqlite::Result<Vec<Record>> {
        let start = start * 1000;
        let end = end * 1000;
        let mut stmt = connection.prepare(
            "SELECT ifnull(p.title, '') AS title,
                    p.url,
                    h.visit_date / 1000 AS last_visited
                FROM moz_places p,
                    moz_historyvisits h
                WHERE p.id = h.place_id AND 
                    visit_date > ? AND 
                    visit_date < ?;
                ",
        )?;
        let rows = stmt.query_map([start, end], |row| {
            Ok(Record {
                title: row.get(0)?,
                url: row.get(1)?,
                last_visited: row.get(2)?,
            })
        })?;
        rows.collect()
    }

    fn check(&self, connection: &Connection) -> rusqlite::Result<bool> {
        let mut stmt = connection.prepare(
            "SELECT name
                FROM sqlite_master
                WHERE type = 'table' AND 
                    name = 'moz_places' OR 
                    name = 'moz_historyvisits';
                ",
        )?;
        let rows = stmt
            .query_map([], |row| Ok(Table { name: row.get(0)? }))?
            .collect::<Vec<_>>();
        Ok(rows.len() == 2)
    }
}

struct UnknownBrowser;

impl Browse for UnknownBrowser {
    fn read(
        &self,
        _connection: Connection,
        _start: u64,
        _end: u64,
    ) -> rusqlite::Result<Vec<Record>> {
        Ok(vec![])
    }

    fn check(&self, _connection: &Connection) -> rusqlite::Result<bool> {
        Ok(false)
    }
}

mod tests {
    use std::env::current_dir;

    use home::home_dir;

    use super::*;

    #[test]
    fn test_get_edge_records() {
        let dir = home_dir().unwrap();
        let path = dir.join("AppData/Local/Microsoft/Edge/User Data/Default/History");
        let base = current_dir().unwrap().join("plugins/history");
        let databse = Database::new(base);
        let result = databse.read("Microsoft Edge", path, 1713283200000, 1713330000000);
        println!("{:#?}", result);
    }

    #[test]
    fn test_get_firefox_records() {
        let dir = home_dir().unwrap();
        let path = dir.join(
            "AppData/Roaming/Mozilla/Firefox/Profiles/g0ogiynj.default-release/places.sqlite",
        );
        let base = current_dir().unwrap().join("plugins/history");
        let databse = Database::new(base);
        let result = databse.read("Firefox", path, 1713744000000, 1713836157169);
        println!("{:#?}", result);
    }

    #[test]
    fn test_get_arc_records() {
        let dir = home_dir().unwrap();
        let path = dir.join(
            "AppData/Local/Packages/TheBrowserCompany.Arc_ttt1ap7aakyb4/LocalCache/Local/Arc/User Data/Default/History",
        );
        let base = current_dir().unwrap().join("plugins/history");
        let databse = Database::new(base);
        let result = databse.read("Arc", path, 1725194649777, 1725281019344);
        println!("{:#?}", result);
    }
}
