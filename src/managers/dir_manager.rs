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
        let mut locker = self.locker.clone();

        locker.push(path);

        if let Err(err) = fs::read_dir(&locker) {
            if err.kind() == ErrorKind::NotFound {
                fs::create_dir_all(&locker)?;
            }
        }

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
               DirManager::pb_to_str(&dir.path())
            );
        }

        self.locker.pop();

        Ok(entries)
    } 
}

#[cfg(test)]
mod test {
    use super::*;
    
    use std::env::current_dir;
    use std::fs::remove_dir_all;
    use crate::tests::setup::Setup;

    fn after_each(this: &mut Setup) {
        for path in this.paths.iter() {
            let exists = Path::new(&path).exists();

            if exists {
                remove_dir_all(path)
                    .expect("Could not remove file in test");
            } 
        } 
    }

    #[test]
    fn new() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                DirManager::new(config, locker);
            },
        }; 
    } 

    #[test]
    fn create() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (mut config, locker) = this.as_path_buf();

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
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();

                let mut dm = DirManager::new(config.clone(), locker);
                let path = config.as_path().to_str().unwrap().to_owned();
                let res = dm.read(&path).unwrap();

                assert_eq!(res.len(), 0);
            },
        };
    }

    #[test]
    fn remove() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();

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
