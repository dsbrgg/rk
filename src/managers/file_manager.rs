use serde_yaml::{Mapping, Value};

use std::env;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::collections::{HashMap, BTreeMap};
use std::io::{self, Read, Write, ErrorKind};

use crate::managers::traits::Manager;

pub struct FileManager<'f> {
    name: &'f str,
    config: PathBuf,
    locker: PathBuf,
}

impl<'f> FileManager<'f> {
    pub fn new(config: PathBuf, locker: PathBuf) -> FileManager<'f> {
        let mut fm = FileManager { name: "files", config, locker };

        fm.init().expect("Could not initialize FileManager");

        fm
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
    //     config_dir.as_path().exists()
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

impl<'f> Manager for FileManager<'f> {
    type Output = String; 

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
        let p = Path::new(path);

        if !p.exists() { File::create(p).expect("Unable to create file"); }

        Ok(()) 
    }

    fn read(&mut self, path: &str) -> io::Result<Self::Output> {
        let p = Path::new(path);

        if !p.exists() { panic!("Trying to open file that does not exist"); }

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
    use crate::tests::setup::Setup;

   #[test] 
    fn new() {
        let mut locker_path = env::current_dir().unwrap();
        let mut config_path = env::current_dir().unwrap();

        locker_path.push("dump/file_manager_new_1");
        config_path.push("dump/file_manager_new_2");

        FileManager::new(config_path, locker_path);
    }

    #[test] 
    fn create() {
        let mut locker_path = env::current_dir().unwrap();
        let mut config_path = env::current_dir().unwrap();

        locker_path.push("dump/file_manager_create_1");
        config_path.push("dump/file_manager_create_2");

        let mut fm = FileManager::new(config_path.clone(), locker_path);
        let hello_path = config_path.as_path().to_str().unwrap().to_owned();

        fm.create(&hello_path);

        assert_eq!(Path::new(&hello_path).exists(), true);
    }

    #[test] 
    fn read() {
        let mut locker_path = env::current_dir().unwrap();
        let mut config_path = env::current_dir().unwrap();

        locker_path.push("dump/file_manager_read_1");
        config_path.push("dump/file_manager_read_2");

        let mut fm = FileManager::new(config_path, locker_path);

        let file = fm.read("dump/file_manager_read_1").unwrap();

        assert_eq!(file, String::from(""));
    }

    #[test]
    #[should_panic]
    fn read_panic() {
        let mut locker_path = env::current_dir().unwrap();
        let mut config_path = env::current_dir().unwrap();

        locker_path.push("dump/file_manager_read_1");
        config_path.push("dump/file_manager_read_2");

        let mut fm = FileManager::new(config_path, locker_path);

        fm.read("dump/unknown").unwrap();
    }

    #[test]
    fn remove() {
        let mut locker_path = env::current_dir().unwrap();
        let mut config_path = env::current_dir().unwrap();

        locker_path.push("dump/file_manager_read_1");
        config_path.push("dump/file_manager_read_2");

        let mut fm = FileManager::new(config_path.clone(), locker_path);
        let path_to_remove = config_path.as_path().to_str().unwrap().to_owned();

        fm.remove(&path_to_remove);

        let path = Path::new("dump/file_manager_read_2");

        assert_eq!(path.exists(), false);
    }
}
