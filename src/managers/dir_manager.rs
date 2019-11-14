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
    
    use std::env;
    use std::fs::remove_dir_all;
    use crate::tests::setup::Setup;

    fn after_each(this: &Setup) {
        let locker_path = format!("dump/{}_{}_{}", this.name, this.test_type, this.count.0);
        let config_path = format!("dump/{}_{}_{}", this.name, this.test_type, this.count.1);

        remove_dir_all(locker_path)
            .expect("Could not remove file in test");
        remove_dir_all(config_path)
            .expect("Could not remove file in test");
    }

    #[test]
    fn new() {
        Setup { 
            name: "dir_manager", 
            test_type: "new",
            count: (1, 2),
            after_each: &after_each,
            process: &|this| {
                let (config, locker) = this.paths();
                DirManager::new(config, locker);
            },
        }; 
    } 

    #[test]
    fn create() {
        Setup { 
            name: "dir_manager", 
            test_type: "create",
            count: (3, 4),
            after_each: &after_each,
            process: &|this| {
                let (mut config, locker) = this.paths();

                let mut dm = DirManager::new(config.clone(), locker);
                
                config.push("hello");

                let hello_path = config.as_path().to_str().unwrap().to_owned();

                dm.create(&hello_path);

                assert_eq!(Path::new(&hello_path).exists(), true);
            },
        }; 
    }

    #[test]
    fn read() {
        Setup { 
            name: "dir_manager", 
            test_type: "read",
            count: (5, 6),
            after_each: &after_each,
            process: &|this| {
                let (config, locker) = this.paths();

                let mut dm = DirManager::new(config.clone(), locker);
                let path = config.as_path().to_str().unwrap().to_owned();
                let res = dm.read(&path).unwrap();

                assert_eq!(res.len(), 0);
            },
        };
    }

    #[test]
    fn remove() {
        let mut setup = Setup { 
            name: "dir_manager", 
            test_type: "remove",
            count: (7, 8),
            after_each: &|this| {},
            process: &|this| {
                let (config, locker) = this.paths();

                let mut dm = DirManager::new(config.clone(), locker.clone());
                
                let path = config.as_path().to_str().unwrap().to_owned();
                dm.remove(&path).unwrap();

                assert_eq!(config.as_path().exists(), false);

                let path = locker.as_path().to_str().unwrap().to_owned();
                dm.remove(&path).unwrap();

                assert_eq!(locker.as_path().exists(), false);
            },
        }; 
    }

    // #[test]
    fn pb_to_str() {
        let current_dir = env::current_dir().unwrap();

        let current_dir_str = DirManager::pb_to_str(&current_dir);

        let current_dir = current_dir.as_path().to_str().unwrap().to_owned(); 

        assert_eq!(current_dir_str, current_dir);
    }
}
