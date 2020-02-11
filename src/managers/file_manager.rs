use serde_yaml::{Mapping, Value};

use std::fmt::Write;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::collections::{HashMap, BTreeMap};
use std::io::{self, Read, ErrorKind};

use crate::managers::{Manager, ManagerOption};

use ManagerOption::*;

pub struct FileManager {
    config: PathBuf,
    locker: PathBuf
}

impl FileManager {
    pub fn new(config: &PathBuf, locker: &PathBuf) -> FileManager {
        let config = config.clone();
        let locker = locker.clone();

        let mut fm = FileManager { config, locker };

        fm.init().expect("Could not initialize FileManager");

        fm
    } 

    pub fn create_locker(&mut self, path: &str) -> io::Result<()> {
        self.create(
            &self.gen_path(Locker, path)
        )
    }

    pub fn read_locker(&mut self, path: &str) -> io::Result<String> {
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

    pub fn read_config(&mut self, path: &str) -> io::Result<String> {
        self.read(
            &self.gen_path(Config, path)
        )
    }

    pub fn remove_config(&mut self, path: &str) -> io::Result<()> {
        self.remove(
            &self.gen_path(Config, path)
        )
    }

    pub fn write_locker(&mut self, path: &str, content: &str) -> io::Result<()> {
        self.write(
            &self.gen_path(Locker, path),
            content
        )
    }

    fn gen_path(&self, for_path: ManagerOption, path: &str) -> String {
        let mut location = PathBuf::new();

        match for_path {
            Config => { location.push(self.config.clone()); },
            Locker => { location.push(self.locker.clone()); },
        };

        location.push(path);

        Self::pb_to_str(&location)
    } 
}

impl Manager for FileManager {
    type Output = String; 

    // NOTE: maybe this is not needed
    // both initial process are required
    // only for DirManager, only moving 
    // the default settings.yml file is
    // required on the config_path
    fn init(&mut self) -> io::Result<()> {
        let config_path = self.config.as_path().to_owned();
        
        if !config_path.exists() { 
            self.create(
                config_path
                    .to_str()
                    .unwrap()
            )?;
        }

        Ok(())
    }

    fn create(&mut self, path: &str) -> io::Result<()> {
        let p = Path::new(path);

        if !p.exists() { 
            File::create(p)
                .expect(&format!("Unable to create/write file {:?}", p)); 
        }

        Ok(()) 
    }

    fn read(&mut self, path: &str) -> io::Result<Self::Output> {
        let p = Path::new(path);
        
        let not_found_msg = format!("Trying to open file {:?} that does not exist", path);
        let not_a_file_msg = format!("{:?} is a directory. FileManager can't open it", path);

        if !p.exists() { panic!(not_found_msg); }
        if !p.is_file() { panic!(not_a_file_msg); }

        let mut file = File::open(p)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    fn remove(&mut self, path: &str) -> io::Result<()> {
        if Path::new(path).exists() { fs::remove_file(path)?; }
        
        Ok(()) 
    }

    fn write(&mut self, path: &str, content: &str) -> io::Result<()> {
        fs::write(&path, &content)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::path::PathBuf;
    use std::env::current_dir;
    use std::fs::remove_file;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    
    use crate::mocks::Setup;

    fn after_each(this: &mut Setup) {
        for path in this.paths.iter() {
            let exists = Path::new(&path).exists();
            let msg = format!("Could not remove {} in `file_manager.rs` test", path);

            if exists { remove_file(path).expect(&msg); }
        }
    }

    #[test] 
    fn new() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                FileManager::new(&config, &locker);
            }
        }; 
    }

    #[test] 
    fn create_locker() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, mut locker) = this.as_path_buf();
                
                let mut fm = FileManager::new(&config, &locker);
                let locker_path = FileManager::pb_to_str(&locker); 

                locker.push(&locker_path);
                fm.create_locker(&locker_path);

                assert_eq!(locker.exists(), true);
            }
        };
    }

    #[test] 
    fn create_config() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (mut config, locker) = this.as_path_buf();
                
                let mut fm = FileManager::new(&config, &locker);
                let config_path = FileManager::pb_to_str(&config);

                config.push(&config_path);
                fm.create_config(&config_path);

                assert_eq!(config.exists(), true);
            }
        };
    }

    #[test] 
    fn read_locker() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, mut locker) = this.as_path_buf();
                
                let mut fm = FileManager::new(&config, &locker);
                let locker_path = FileManager::pb_to_str(&locker); 

                fm.create_locker(&locker_path);
                
                let file = fm.read_locker(&locker_path).unwrap();

                assert_eq!(file, String::from(""));
            }
        };
    } 

    #[test] 
    fn read_config() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                
                let config_path = FileManager::pb_to_str(&config);
                let mut fm = FileManager::new(&config, &locker);
                
                let file = fm.read_config(&config_path).unwrap();

                assert_eq!(file, String::from(""));
            }
        };
    }

    #[test]
    fn read_locker_panic() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let mut fm = FileManager::new(&config, &locker);

                // https://doc.rust-lang.org/std/panic/struct.AssertUnwindSafe.html
                let result = catch_unwind(AssertUnwindSafe(|| {
                    fm.read_locker("dump/unknown");
                }));

                assert!(result.is_err());
            }
        };
    }

    #[test]
    fn read_config_panic() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let mut fm = FileManager::new(&config, &locker);

                // https://doc.rust-lang.org/std/panic/struct.AssertUnwindSafe.html
                let result = catch_unwind(AssertUnwindSafe(|| {
                    fm.read_config("dump/unknown");
                }));

                assert!(result.is_err());
            }
        };
    }

    #[test]
    fn remove_locker() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let mut fm = FileManager::new(&config, &locker);
                let path_to_remove = FileManager::pb_to_str(&locker);
                
                fm.remove_locker(&path_to_remove);

                let path = Path::new(&path_to_remove);
                assert_eq!(path.exists(), false);
            }
        }; 
    }

    #[test]
    fn remove_config() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let mut fm = FileManager::new(&config, &locker);
                let path_to_remove = FileManager::pb_to_str(&config);
                
                fm.remove_config(&path_to_remove);

                let path = Path::new(&path_to_remove);
                assert_eq!(path.exists(), false);
            }
        }; 
    }

    #[test]
    fn write_locker() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, mut locker) = this.as_path_buf();
                
                let mut fm = FileManager::new(&config, &locker);
                let locker_path = FileManager::pb_to_str(&locker); 

                fm.create_locker(&locker_path);
                fm.write_locker(&locker_path, "test");

                let file = fm.read_locker(&locker_path).unwrap();

                assert_eq!(file, "test");
            }
        };
    }

    #[test]
    fn pb_to_str() {
        let dir = current_dir().unwrap();
        let manager_str = FileManager::pb_to_str(&dir);
        let current_str = dir.as_path().to_str().unwrap().to_owned(); 

        assert_eq!(manager_str, current_str);
    }

    #[test]
    fn append_paths() {
        let mut dir = current_dir().unwrap();
        let current_str = FileManager::pb_to_str(&dir);
        let appended = FileManager::append_paths(&current_str, &vec!["src"]);
        
        dir.push("src");

        assert_eq!(appended.as_str(), FileManager::pb_to_str(&dir));
    }

    #[test]
    fn append_path() {
        let mut dir = current_dir().unwrap();
        let current_str = FileManager::pb_to_str(&dir);
        let appended = FileManager::append_path(&current_str, "src");
        
        dir.push("src");

        assert_eq!(appended.as_str(), FileManager::pb_to_str(&dir));
    }
}
