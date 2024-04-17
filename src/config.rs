use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use crate::{history, shared::Browser, Error};

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
struct Config {
    path: PathBuf,
    inner: InnerConfig,
}

#[derive(Deserialize, Serialize, Default)]
struct InnerConfig {
    browsers: Vec<Browser>,
}

impl Config {
    fn new() -> Self {
        Config::default()
    }

    fn init<P: AsRef<Path>>(&mut self, base: P) {
        let mut path = PathBuf::new();
        path.push(base);
        path.push("plugins/history/config.json");
        self.path = path.clone();
        if !path.exists() {
            self.create()
        }
    }

    fn load(&mut self) {
        let config: Result<InnerConfig, Error> = read_file(&self.path);
        if let Ok(config) = config {
            self.inner = config
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
}


mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let mut config = Config::new();
        config.init(std::env::current_dir().unwrap());
    }
}
