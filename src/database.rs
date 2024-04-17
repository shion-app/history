use std::{fmt::Debug, fs, path::{Path, PathBuf}};

use rusqlite::{Connection};
use serde::{Deserialize, Serialize};


use crate::Result;

pub struct Database {
    base: PathBuf,
}

impl Database {
    pub fn new<P: AsRef<Path>>(base: P) -> Self {
        let mut path = PathBuf::new();
        path.push(base);
        Database { base: path }
    }

    pub fn read<P: AsRef<Path>>(&self, name: &str, path: P, start: u64, end: u64) -> Result<Vec<Record>> {
        let temp_path = self.base.join("plugins/history/temp/data.db");
        if let Some(parent_dir) = temp_path.parent() {
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir)?;
            }
        }
        fs::copy(path, &temp_path)?;
        let connection = Connection::open(&temp_path)?;
        let browser = get_browser(name);
        let result = browser.read(connection, start, end)?;
        fs::remove_file(temp_path)?;
        Ok(result)
    }

}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    title: String,
    url: String,
    last_visited: u64,
}

fn get_browser(name: &str) -> Box<dyn Browse> {
    match name {
        "Google Chrome" | "Microsoft Edge" => Box::new(Chromium),
        _ => Box::new(UnknownBrowser)
    }
}

trait Browse {
    fn read(&self, connection: Connection, start: u64, end: u64) -> rusqlite::Result<Vec<Record>>;
}

struct Chromium;

impl Browse for Chromium {
    fn read(&self, connection: Connection, start: u64, end: u64) -> rusqlite::Result<Vec<Record>> {
        let start = (start / 1000 + 11644473600) * 1000000;
        let end: u64 = (end / 1000 + 11644473600) * 1000000;
        let mut stmt = connection.prepare(
            "SELECT title,
                    url,
                    (last_visit_time / 1000000 - 11644473600) * 1000 AS last_visited
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
}

struct UnknownBrowser;

impl Browse for UnknownBrowser {
    fn read(&self, _connection: Connection, _start: u64, _end: u64) -> rusqlite::Result<Vec<Record>> {
        Ok(vec![])
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
        let databse = Database::new(current_dir().unwrap());
        let _ = databse.read("Microsoft Edge", path, 1713283200000, 1713330000000);
    }
}
