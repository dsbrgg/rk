use serde_yaml::{Mapping, Value};

use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write, ErrorKind};

use crate::managers::{Manager, ManagerOption};

use ManagerOption::*;

pub struct DirManager {
    config: PathBuf,
    locker: PathBuf,
}

impl DirManager {
    pub fn new(config: &PathBuf, locker: &PathBuf) -> DirManager {
        let config = config.clone();
        let locker = locker.clone();

        let mut dm = DirManager { config, locker };

        dm.init().expect("Could not initialize DirManager");

        dm
    }

    pub fn create_locker(&mut self, path: &str) -> io::Result<()> {
        self.create(
            &self.gen_path(Locker, path)
        )
    }

    pub fn read_locker(&mut self, path: &str) -> io::Result<Vec<String>> {
        self.read(
            &self.gen_path(Locker, path)
        )
    }

    pub fn remove_locker(&mut self, path: &str) -> io::Result<()> {
        self.remove(
            &self.gen_path(Locker, path)
        )
    }

    pub fn create_config(&mut self, path: &str) -> io::Result<()> {
        self.create(
            &self.gen_path(Config, path)
        )
    }

    pub fn read_config(&mut self, path: &str) -> io::Result<Vec<String>> {
        self.read(
            &self.gen_path(Config, path)
        )
    }

    pub fn remove_config(&mut self, path: &str) -> io::Result<()> {
        self.remove(
            &self.gen_path(Config, path)
        )
    }

    // NOTE: this can be moved to the Manager Trait
    // if field traits are implemented in the future
    fn gen_path(&self, for_path: ManagerOption, path: &str) -> String {
        let mut location = PathBuf::new();

        match for_path {
            Locker => { location.push(self.locker.clone()); },
            Config => { location.push(self.config.clone()); },
            _ => panic!("Unsupported location {:?}", location.as_path().to_str())
        };

        
        location.push(path);

        Self::pb_to_str(&location)
    }
}

impl Manager for DirManager {
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
        if let Err(err) = fs::read_dir(path) {
            if err.kind() == ErrorKind::NotFound {
                fs::create_dir_all(path)?;
            }
        }

        Ok(())
    }

    fn read(&mut self, dir: &str) -> io::Result<Self::Output> {
        let mut entries = Vec::new();

        for entry in fs::read_dir(&dir)? {
            let dir = entry?;
            
            entries.push(
               DirManager::pb_to_str(&dir.path())
            );
        }

        Ok(entries)
    }

    fn remove(&mut self, path: &str) -> io::Result<()> { 
        fs::remove_dir_all(&path)?;

        Ok(()) 
    } 
}

#[cfg(test)]
mod test {
    use super::*;
    
    use std::env::current_dir;
    use std::fs::remove_dir_all;

    use crate::mocks::Setup;

    fn after_each(this: &mut Setup) {
        for path in this.paths.iter() {
            let exists = Path::new(&path).exists();
            let msg = format!("Could not remove {} in `dir_manager.rs` test", path);

            if exists { remove_dir_all(path).expect(&msg); } 
        } 
    }

    #[test]
    fn new() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (.., config, locker) = this.as_path_buf();
                DirManager::new(&config, &locker);
            },
        }; 
    } 

    #[test]
    fn create_locker() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, mut locker) = this.as_path_buf();

                let mut dm = DirManager::new(&config, &locker);
                
                locker.push("hello");

                let hello_path = DirManager::pb_to_str(&locker);

                dm.create_locker(&hello_path);

                assert_eq!(Path::new(&hello_path).exists(), true);
            },
        }; 
    }

    #[test]
    fn create_config() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (mut config, locker) = this.as_path_buf();

                let mut dm = DirManager::new(&config, &locker);
                
                config.push("hello");

                let hello_path = DirManager::pb_to_str(&config);

                dm.create_config(&hello_path);

                assert_eq!(Path::new(&hello_path).exists(), true);
            },
        }; 
    }

    #[test]
    fn read_locker() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();

                let mut dm = DirManager::new(&config, &locker);
                let path = DirManager::pb_to_str(&locker);
                let res = dm.read_locker(&path).unwrap();

                assert_eq!(res.len(), 0);
            },
        };
    }

    #[test]
    fn read_config() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();

                let mut dm = DirManager::new(&config, &locker);
                let path = DirManager::pb_to_str(&config);
                let res = dm.read_config(&path).unwrap();

                assert_eq!(res.len(), 0);
            },
        };
    }

    #[test]
    fn remove_locker() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let mut dm = DirManager::new(&config, &locker);
                let path = DirManager::pb_to_str(&locker);
                
                dm.remove_locker(&path).unwrap();

                assert_eq!(locker.exists(), false);
            },
        }; 
    }

    #[test]
    fn remove_config() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let mut dm = DirManager::new(&config, &locker);
                let path = DirManager::pb_to_str(&config);
                
                dm.remove_config(&path).unwrap();

                assert_eq!(config.exists(), false);
            },
        }; 
    }

    #[test]
    fn pb_to_str() {
        let dir = current_dir().unwrap();
        let manager_str = DirManager::pb_to_str(&dir);
        let current_str = dir.as_path().to_str().unwrap().to_owned(); 

        assert_eq!(manager_str, current_str);
    }

    #[test]
    fn append_paths() {
        let mut dir = current_dir().unwrap();
        let current_str = DirManager::pb_to_str(&dir);
        let appended = DirManager::append_paths(&current_str, &vec!["src"]);
        
        dir.push("src");

        assert_eq!(appended.as_str(), DirManager::pb_to_str(&dir));
    }

    #[test]
    fn append_path() {
        let mut dir = current_dir().unwrap();
        let current_str = DirManager::pb_to_str(&dir);
        let appended = DirManager::append_path(&current_str, "src");
        
        dir.push("src");

        assert_eq!(appended.as_str(), DirManager::pb_to_str(&dir));
    }
}
