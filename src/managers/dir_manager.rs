use serde_yaml::{Mapping, Value};

use std::fs;
use std::env;
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
        let mut dm = DirManager { name: "directories", config, locker };

        dm.init().expect("Could not initialize DirManager");

        dm
    }
}

// TODO: having to self.locker.push and .pop all the time seems really bad

impl<'d> Manager for DirManager<'d> {
    type Output = Vec<String>;

    fn init(&mut self) -> io::Result<()> {
        let config_path = self.config.as_path().to_owned();
        let locker_path = self.locker.as_path().to_owned();

        if !config_path.exists() { 
            self.create(
                config_path
                    .to_str()
                    .unwrap()
            )?;
        }

        if !locker_path.exists() { 
            self.create(
                locker_path
                    .to_str()
                    .unwrap()
            )?;
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

    fn remove(&mut self, path: &str) -> io::Result<()> { 
        self.locker.push(path);

        fs::remove_dir(&self.locker)?;

        self.locker.pop();

        Ok(()) 
    }

    fn read(&mut self, dir: &str) -> io::Result<Self::Output> {
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
mod test {
    use super::*;

    #[test]
    fn new() {
        let mut locker_path = env::current_dir().unwrap();
        let mut config_path = env::current_dir().unwrap();

        locker_path.push("dump/dir_manager_new_1");
        config_path.push("dump/dir_manager_new_2");

        DirManager::new(config_path, locker_path);
    } 

    #[test]
    fn create() {
        let mut locker_path = env::current_dir().unwrap();
        let mut config_path = env::current_dir().unwrap();

        locker_path.push("dump/dir_manager_create_1");
        config_path.push("dump/dir_manager_create_2");

        let mut dm = DirManager::new(config_path.clone(), locker_path);
        
        config_path.push("hello");

        let hello_path = config_path.as_path().to_str().unwrap().to_owned();

        dm.create(&hello_path);

        assert_eq!(Path::new(&hello_path).exists(), true); 
    }

    #[test]
    fn read() {
        let mut locker_path = env::current_dir().unwrap();
        let mut config_path = env::current_dir().unwrap();

        locker_path.push("dump/dir_manager_read_1");
        config_path.push("dump/dir_manager_read_2");

        let mut dm = DirManager::new(config_path.clone(), locker_path);
        
        let path = config_path.as_path().to_str().unwrap().to_owned();

        let res = dm.read(&path).unwrap();

        assert_eq!(res.len(), 0); 
    }

    #[test]
    fn remove() {
        let mut locker_path = env::current_dir().unwrap();
        let mut config_path = env::current_dir().unwrap();

        locker_path.push("dump/dir_manager_remove_1");
        config_path.push("dump/dir_manager_remove_2");

        let mut dm = DirManager::new(config_path.clone(), locker_path.clone());
        
        let path = config_path.as_path().to_str().unwrap().to_owned();
        dm.remove(&path).unwrap();

        assert_eq!(config_path.as_path().exists(), false);

        let path = locker_path.as_path().to_str().unwrap().to_owned();
        dm.remove(&path).unwrap();

        assert_eq!(locker_path.as_path().exists(), false);
    }

    #[test]
    fn pb_to_str() {
        let current_dir = env::current_dir().unwrap();

        let current_dir_str = DirManager::pb_to_str(&current_dir);

        let current_dir = current_dir.as_path().to_str().unwrap().to_owned(); 

        assert_eq!(current_dir_str, current_dir);
    }
}
