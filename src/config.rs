use parking_lot::RwLock;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::{Path, PathBuf}
};

use crate::{history::{self, get_database_list, BrowserDatabase}, shared::Browser, Error};

fn read_file<P, T>(path: P) -> Result<T, Error>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let file = File::open(path)?;
    Ok(serde_json::from_reader(&file)?)
}

fn create_file<P, T>(path: P, data: &T) -> Result<(), Error>
where
    P: AsRef<Path>,
    T: Serialize,
{
    if let Some(parent_dir) = path.as_ref().parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }
    }
    let mut file = File::create(path)?;
    serde_json::to_writer(&mut file, data)?;
    Ok(())
}

#[derive(Default)]
pub struct Config {
    path: PathBuf,
    inner: RwLock<InnerConfig>,
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct InnerConfig {
    browsers: Vec<Browser>,
}

impl Config {
    pub fn new<P: AsRef<Path>>(base: P) -> Self {
        let mut config = Config::default();
        let path = base.as_ref();
        config.path = path.join("config.json");
        config
    }

    pub fn init(&mut self) {
        if !self.path.exists() {
            self.create()
        }
        self.load();
    }

    fn load(&mut self) {
        let config: Result<InnerConfig, Error> = read_file(&self.path);
        if let Ok(mut config) = config {
            let list = get_database_list();
            for BrowserDatabase { name, .. } in list {
                let browser = config.browsers.iter().position(|b| b.name == name);
                if browser.is_none() {
                    config.browsers.push(Browser { name: name.to_string(), last_sync: 0 })
                }
            }            
            self.set(config);
        }
    }

    fn create(&self) {
        let browsers: Vec<Browser> = history::get_database_list()
            .into_iter()
            .map(|d| Browser {
                name: d.name.to_string(),
                last_sync: 0,
            })
            .collect();
        let config = InnerConfig { browsers };
        let _ = create_file(&self.path, &config);
    }

    pub fn get(&self) -> InnerConfig {
        self.inner.read().clone()
    }

    pub fn set(&self, config: InnerConfig) {
        let mut c = self.inner.write();
        *c = config.clone();
        self.save(config);
    }

    fn save(&self, config: InnerConfig) {
        let _ = create_file(&self.path, &config);
    }
}


mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let mut config = Config::new(std::env::current_dir().unwrap());
        config.init();
    }

    #[test]
    fn test_set_config() {
        let mut config = Config::new(std::env::current_dir().unwrap());
        config.init();
        config.set(InnerConfig { browsers: vec![Browser { name: "123".to_string(), last_sync: 0 }] });
    }
}
