use serde_yaml::{Mapping, Value};

use std::fmt::Write;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::collections::{HashMap, BTreeMap};
use std::io::{self, Read, ErrorKind};

use crate::managers::manager::Manager;

enum ManagerOption {
    Config,
    Locker,
}

use ManagerOption::*;

pub struct FileManager {
    config: PathBuf,
    locker: PathBuf,
    index: PathBuf
}

impl FileManager {
    pub fn new(index: &PathBuf, config: &PathBuf, locker: &PathBuf) -> FileManager {
        let index = index.clone();
        let config = config.clone();
        let locker = locker.clone();

        let mut fm = FileManager { config, locker, index };

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

    pub fn read_index(&mut self) -> io::Result<String> {
        self.read(
           &Self::pb_to_str(&self.index) 
        )
    }

    pub fn write_index(&mut self, new_index: &str) -> io::Result<()> {
        let index_path = Self::pb_to_str(&self.index.clone());
        let previous_index = self.read(&index_path)?;

        let append_index = format!("{}{}\n", previous_index, new_index);
        let error_msg = format!("Unable to write {} to index at path {}", new_index, index_path);

        fs::write(&index_path, append_index).expect(&error_msg);

        Ok(())
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

    // TODO: this is pretty ugly, def a better way to do it
    // fn init_default_yaml() -> serde_yaml::Result<()> {
    //     type YAML = BTreeMap<String, BTreeMap<String, String>>;
    //     
    //     let mut map: YAML = BTreeMap::new();
    //     map.insert(String::from("paths"), BTreeMap::new());

    //     let config_path = FileManager::get_config_file_path();

    //     for path in DEFAULT_PATHS.iter() {
    //         let data: Vec<&str> = path.split("::").collect();
    //         
    //         let key = data[0];
    //         let value = data[1];

    //         let mut yaml_path = dirs::home_dir().unwrap();
    //         yaml_path.push(value);

    //         let new_path = yaml_path.as_path();

    //         let paths = map.get_mut("paths").unwrap();
    //         let path_string = String::from(new_path.to_str().unwrap());

    //         paths.insert(String::from(key), path_string); 
    //     }
    //     
    //     FileManager::write(
    //         config_path.to_str().unwrap(),
    //         serde_yaml::to_string(&map)?
    //     );

    //     Ok(()) 
    // }

    // fn yaml_exists() -> bool {
    //     let mut config_dir = dirs::home_dir().unwrap();
    //     config_dir.push(".config/rk/settings.yml");
    //     
    //     config_dir.exists()
    // }

    // fn get_config_file_path() -> PathBuf {
    //     let mut config_path = dirs::home_dir().unwrap();
    //     config_path.push(".config/rk/settings.yml");

    //     config_path
    // }

    // fn read_paths() -> HashMap<String, String> {
    //     let mut path_map: HashMap<String, String> = HashMap::new();

    //     let config_path = FileManager::get_config_file_path()
    //         .to_str()
    //         .unwrap()
    //         .to_owned();

    //     let config_file = FileManager::open(&config_path[..], None, FileAction::Read);
    //     let yml_map: Mapping = serde_yaml::from_reader(config_file).unwrap();

    //     let paths = FileManager::key_mapping(&yml_map, "paths");
    //     
    //     for (key_raw, path_raw) in paths.iter() {
    //         path_map.insert(
    //             key_raw.as_str().unwrap().to_string(),
    //             path_raw.as_str().unwrap().to_string()
    //         );
    //     }

    //     path_map
    // }

    // fn build_value_string(string: &str) -> Value {
    //     Value::String(
    //         String::from(string)
    //     )
    // }

    // fn key_mapping(map: &Mapping, string: &str) -> Mapping {
    //    let value = FileManager::build_value_string(string); 

    //     map.get(&value)
    //         .unwrap()
    //         .to_owned()
    //         .as_mapping()
    //         .unwrap()
    //         .to_owned()
    // }

    // fn key_values(map: &Mapping, string: &str) -> Value {
    //    let value = FileManager::build_value_string(string); 

    //     map.get(&value)
    //         .unwrap()
    //         .to_owned()
    // }

    // pub fn open(path: &str, append: Option<String>, action: FileAction) -> File {
    //     let buf = match append {
    //         Some(string) => Path::new(path).join(string.as_str()),
    //         None => PathBuf::from(path),
    //     };

    //     match action {
    //         FileAction::Read => File::open(buf.as_path()).unwrap(),
    //         FileAction::Write => File::create(buf.as_path()).expect("Unable to open path to write!"),
    //     }
    // }

    // fn write(path: &str, contents: String) {
    //     let mut file = FileManager::open(path, None, FileAction::Write);

    //     match file.write_all(contents.as_bytes()) {
    //         Err(why) => panic!("Couldn't write to {}: {}", path, why),
    //         Ok(_) => println!("\n::: Success writing to {} :::\n", path),
    //     } 
    // }

    // pub fn get_locker_path(&self) -> &String {
    //     self.paths.get("locker").unwrap()
    // }
}

impl Manager for FileManager {
    type Output = String; 

    // NOTE: maybe this is not needed
    // both initial process are required
    // only for DirManager, only moving 
    // the default settings.yml file is
    // required on the config_path
    fn init(&mut self) -> io::Result<()> {
        let index_path = self.index.as_path().to_owned();
        let config_path = self.config.as_path().to_owned();
        
        if !index_path.exists() {
            self.create(
                index_path
                    .to_str()
                    .unwrap()
            )?;
        }

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

        if !p.exists() {
            panic!(
                format!("Trying to open file {:?} that does not exist", path)
            ); 
        }

        let mut file = File::open(p)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    fn remove(&mut self, path: &str) -> io::Result<()> {
        let p = Path::new(path);

        if p.exists() { fs::remove_file(path)?; }
        
        Ok(()) 
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::env::current_dir;
    use std::fs::remove_file;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    
    use crate::mocks::setup::Setup;

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
                let (index, config, locker) = this.as_path_buf();
                FileManager::new(&index, &config, &locker);
            }
        }; 
    }

    #[test] 
    fn create_locker() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (index, config, mut locker) = this.as_path_buf();
                
                let mut fm = FileManager::new(&index, &config, &locker);
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
                let (index, mut config, locker) = this.as_path_buf();
                
                let mut fm = FileManager::new(&index, &config, &locker);
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
                let (index, config, mut locker) = this.as_path_buf();
                
                let mut fm = FileManager::new(&index, &config, &locker);
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
                let (index, config, locker) = this.as_path_buf();
                
                let config_path = FileManager::pb_to_str(&config);
                let mut fm = FileManager::new(&index, &config, &locker);
                
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
                let (index, config, locker) = this.as_path_buf();
                let mut fm = FileManager::new(&index, &config, &locker);

                // https://doc.rust-lang.org/std/panic/struct.AssertUnwindSafe.html
                let result = catch_unwind(AssertUnwindSafe(|| {
                    // TODO: use PathBuf here
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
                let (index, config, locker) = this.as_path_buf();
                let mut fm = FileManager::new(&index, &config, &locker);

                // https://doc.rust-lang.org/std/panic/struct.AssertUnwindSafe.html
                let result = catch_unwind(AssertUnwindSafe(|| {
                    // TODO: use PathBuf here
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
                let (index, config, locker) = this.as_path_buf();
                let mut fm = FileManager::new(&index, &config, &locker);
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
                let (index, config, locker) = this.as_path_buf();
                let mut fm = FileManager::new(&index, &config, &locker);
                let path_to_remove = FileManager::pb_to_str(&config);
                
                fm.remove_config(&path_to_remove);

                let path = Path::new(&path_to_remove);
                assert_eq!(path.exists(), false);
            }
        }; 
    }

    #[test]
    fn write_index() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf();
                let mut fm = FileManager::new(&index, &config, &locker);
                
                fm.write_index("new_index");
                fm.write_index("another_index");

                assert_eq!(fm.read_index().unwrap(), "new_index\nanother_index\n");
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
