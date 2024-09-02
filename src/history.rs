use std::{collections::HashMap, path::Path};

use home::home_dir;
use lazy_static::lazy_static;

use crate::{
    config::{Config, InnerConfig},
    database::{Database, Record},
    Result,
};

#[derive(PartialEq, Clone)]
enum Platform {
    Windows,
    Mac,
    Linux,
    Unknown,
}

#[derive(Clone)]
pub struct BrowserDatabase {
    platform: Platform,
    pub name: &'static str,
    pattern: String,
}

lazy_static! {
    static ref DATABASE_LIST: Vec::<BrowserDatabase> = {
        let dir = home_dir();
        if let Some(dir) = dir {
            let list = vec![
                BrowserDatabase {
                    platform: Platform::Windows,
                    name: "Google Chrome",
                    pattern: "AppData/Local/Google/Chrome/User Data/*/History".to_string(),
                },
                BrowserDatabase {
                    platform: Platform::Windows,
                    name: "Microsoft Edge",
                    pattern: "AppData/Local/Microsoft/Edge/User Data/*/History".to_string(),
                },
                BrowserDatabase {
                    platform: Platform::Windows,
                    name: "Firefox",
                    pattern: "AppData/Roaming/Mozilla/Firefox/Profiles/*/places.sqlite".to_string(),
                },
                BrowserDatabase {
                    platform: Platform::Windows,
                    name: "Arc",
                    pattern: "AppData/Local/Packages/TheBrowserCompany.Arc_ttt1ap7aakyb4/LocalCache/Local/Arc/User Data/*/History".to_string(),
                },
            ]
            .into_iter()
            .map(|d| BrowserDatabase {
                pattern: dir.join(d.pattern).to_str().unwrap().to_string(),
                ..d
            })
            .filter(|d| get_platform() == d.platform)
            .collect();
            list
        } else {
            vec![]
        }
    };
}

fn get_platform() -> Platform {
    match std::env::consts::OS {
        "windows" => Platform::Windows,
        "macos" => Platform::Mac,
        "linux" => Platform::Linux,
        _ => Platform::Unknown,
    }
}

pub fn get_database_list() -> Vec<BrowserDatabase> {
    DATABASE_LIST.to_vec()
}

pub fn get_database_map() -> HashMap<&'static str, Vec<String>> {
    let mut map = HashMap::new();
    for BrowserDatabase { pattern, name, .. } in DATABASE_LIST.iter() {
        if let Ok(entries) = glob::glob(pattern) {
            for e in entries {
                if let Ok(path) = e {
                    let mut list = map.get(name).unwrap_or(&vec![]).clone();
                    let path = path.into_os_string().into_string().unwrap();
                    list.push(path);
                    map.insert(*name, list);
                };
            }
        }
    }
    map
}

pub struct History {
    config: Config,
    database: Database,
}

impl History {
    pub fn new<P: AsRef<Path>>(base: P) -> Self {
        let base = base.as_ref();
        let base = base.join("plugins/history");
        let mut config = Config::new(&base);
        config.init();
        let database = Database::new(base);
        Self { config, database }
    }

    pub fn get_config(&self) -> InnerConfig {
        self.config.get()
    }

    pub fn read<P: AsRef<Path>>(
        &self,
        name: &str,
        path: P,
        start: u64,
        end: u64,
    ) -> Result<Vec<Record>> {
        self.database.read(name, path, start, end)
    }

    pub fn set_config(&self, config: InnerConfig) {
        self.config.set(config);
    }

    pub fn clean_temp(&self) {
        self.database.clean_temp();
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_get_list() {
        let map = get_database_map();
        println!("{:#?}", map);
    }
}
