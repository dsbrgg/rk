use serde_yaml::{Mapping, Value};

use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write, ErrorKind};

use crate::managers::traits::Manager;

pub struct DirManager<'d> {
    name: &'d str,
    config: PathBuf,
    locker: PathBuf,
}

impl<'d> DirManager<'d> {
    pub fn new(config: PathBuf, locker: PathBuf) -> DirManager<'d> {
        let mut dm = DirManager { 
            name: "directories", 
            config, 
            locker 
        };

        dm.init().expect("Could not initialize DirManager");

        dm
    }
}

impl<'d> Manager for DirManager<'d> { 
    fn init(&mut self) -> io::Result<()> {
        let config_path = self.config.as_path().to_owned();
        let locker_path = self.locker.as_path().to_owned();

        if !config_path.exists() { 
            self.create(
                config_path
                    .to_str()
                    .unwrap()
            );
        }

        if !locker_path.exists() { 
            self.create(
                locker_path
                    .to_str()
                    .unwrap()
            );
        }

        Ok(())
    }

    fn create(&mut self, path: &str) -> io::Result<()> {
        self.locker.push(path);

        if let Err(err) = fs::read_dir(&self.locker) {
            if err.kind() == ErrorKind::NotFound {
                fs::create_dir_all(&self.locker)?;
            }
        }

        self.locker.pop();

        Ok(())
    }

    // TODO: implement this
    fn remove(&mut self, path: &str) -> io::Result<()> { Ok(()) }

    fn read(&mut self, dir: &str) -> io::Result<Vec<String>> {
        self.locker.push(dir);

        let mut entries = Vec::new();

        for entry in fs::read_dir(&self.locker)? {
            let dir = entry?;
            
            entries.push(
                dir
                    .path()
                    .as_path()
                    .to_str()
                    .unwrap()
                    .to_owned()
            );
        }

        self.locker.pop();

        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
   use super::*;

    #[test]
    fn creates_new_dir_manager() {
        let mut locker_path = dirs::home_dir().unwrap();
        let mut config_path = dirs::home_dir().unwrap();

        locker_path.push(".rk");
        config_path.push(".config/rk");

        DirManager::new(config_path, locker_path);
    } 
}
