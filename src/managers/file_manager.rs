use serde_yaml::{Mapping, Value};

use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::collections::{HashMap, BTreeMap};
use std::io::{self, Write, ErrorKind};

use crate::managers::dir_manager::{DirManager};

pub enum FileAction {
    Read,
    Write,
}

const DEFAULT_PATHS: [&'static str; 2] = [ 
    "locker::.rk",
    "config::.config/rk/rk.yml" 
];

pub struct FileManager {
    paths: HashMap<String, String> 
}

impl FileManager {
    pub fn new() -> FileManager {
        if !FileManager::yaml_exists() {
            FileManager::init_default_yaml()
                .expect("Failed initiating default yaml file");
        }

        FileManager {
            paths: FileManager::read_paths()
        }
    } 

    // TODO: this is pretty ugly, def a better way to do it
    fn init_default_yaml() -> serde_yaml::Result<()> {
        type YAML = BTreeMap<String, BTreeMap<String, String>>;
        
        let mut map: YAML = BTreeMap::new();
        map.insert(String::from("paths"), BTreeMap::new());

        let config_path = FileManager::get_config_file_path();

        for path in DEFAULT_PATHS.iter() {
            let data: Vec<&str> = path.split("::").collect();
            
            let key = data[0];
            let value = data[1];

            let mut yaml_path = dirs::home_dir().unwrap();
            yaml_path.push(value);

            let new_path = yaml_path.as_path();

            let paths = map.get_mut("paths").unwrap();
            let path_string = String::from(new_path.to_str().unwrap());

            paths.insert(String::from(key), path_string); 
        }
        
        FileManager::write(
            config_path.to_str().unwrap(),
            serde_yaml::to_string(&map)?
        );

        Ok(()) 
    }

    fn yaml_exists() -> bool {
        let mut config_dir = dirs::home_dir().unwrap();
        config_dir.push(".config/rk/rk.yml");
        
        config_dir.as_path().exists()
    }

    fn get_config_file_path() -> PathBuf {
        let mut config_path = dirs::home_dir().unwrap();
        config_path.push(".config/rk/rk.yml");

        config_path
    }

    fn read_paths() -> HashMap<String, String> {
        let mut path_map: HashMap<String, String> = HashMap::new();

        let config_path = FileManager::get_config_file_path()
            .to_str()
            .unwrap()
            .to_owned();

        let config_file = FileManager::open(&config_path[..], None, FileAction::Read);
        let yml_map: Mapping = serde_yaml::from_reader(config_file).unwrap();

        let paths = FileManager::key_mapping(&yml_map, "paths");
        
        for (key_raw, path_raw) in paths.iter() {
            path_map.insert(
                key_raw.as_str().unwrap().to_string(),
                path_raw.as_str().unwrap().to_string()
            );
        }

        path_map
    }

    fn build_value_string(string: &str) -> Value {
        Value::String(
            String::from(string)
        )
    }

    fn key_mapping(map: &Mapping, string: &str) -> Mapping {
       let value = FileManager::build_value_string(string); 

        map.get(&value)
            .unwrap()
            .to_owned()
            .as_mapping()
            .unwrap()
            .to_owned()
    }

    fn key_values(map: &Mapping, string: &str) -> Value {
       let value = FileManager::build_value_string(string); 

        map.get(&value)
            .unwrap()
            .to_owned()
    }

    pub fn open(path: &str, append: Option<String>, action: FileAction) -> File {
        let buf = match append {
            Some(string) => Path::new(path).join(string.as_str()),
            None => PathBuf::from(path),
        };

        match action {
            FileAction::Read => FileManager::try_open(buf.as_path()),
            FileAction::Write => File::create(buf.as_path()).expect("Unable to open path to write!"),
        }
    }

    fn try_open(path: &Path) -> File {
        // TODO: Handle when file does not exist on read (eg. running command in "wrong" dir)
        match File::open(&path) {
            Err(_) => File::create(&path).expect("Unable to create path file!"),
            Ok(file) => file,
        }
    }

    fn write(path: &str, contents: String) {
        let mut file = FileManager::open(path, None, FileAction::Write);

        match file.write_all(contents.as_bytes()) {
            Err(why) => panic!("Couldn't write to {}: {}", path, why),
            Ok(_) => println!("\n::: Success writing to {} :::\n", path),
        } 
    }

    pub fn get_locker_path(&self) -> &String {
        self.paths.get("locker").unwrap()
    }
}
