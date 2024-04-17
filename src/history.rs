use std::path::Path;

use home::home_dir;
use lazy_static::lazy_static;

use crate::config::{Config, InnerConfig};

#[derive(PartialEq, Clone)]
enum Platform {
    Windows,
    Mac,
    Linux,
    Unknown,
}

#[derive(Clone)]
pub struct Database {
    platform: Platform,
    pub name: &'static str,
    pattern: String,
}

lazy_static! {
    static ref DATABASE_LIST: Vec::<Database> = {
        let dir = home_dir();
        if let Some(dir) = dir {
            let list = vec![
                Database {
                    platform: Platform::Windows,
                    name: "Google Chrome",
                    pattern: "AppData/Local/Google/Chrome/User Data/*/History".to_string(),
                },
                Database {
                    platform: Platform::Windows,
                    name: "Microsoft Edge",
                    pattern: "AppData/Local/Microsoft/Edge/User Data/*/History".to_string(),
                },
            ]
            .into_iter()
            .map(|d| Database {
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

pub fn get_database_list() -> Vec<Database> {
    DATABASE_LIST.to_vec()
}

fn get_list() -> Vec<String> {
    let mut list = Vec::new();
    for Database { pattern, .. } in DATABASE_LIST.iter() {
        if let Ok(entries) = glob::glob(pattern) {
            for e in entries {
                if let Ok(path) = e {
                    list.push(path.into_os_string().into_string().unwrap())
                };
            }
        }
    }
    list
}

pub struct History {
    config: Config
}

impl History {
    pub fn new<P: AsRef<Path>>(base: P) -> Self {
        let config = Config::new(base);
        Self { config }
    }

    pub fn get_config(&self) -> InnerConfig  {
        self.config.list()
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_get_list() {
        let list = get_list();
        println!("{:#?}", list);
    }
}
