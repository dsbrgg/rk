extern crate dirs;
extern crate serde_yaml;

use serde_yaml::{Mapping, Value};

use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::collections::{HashMap, BTreeMap};
use std::io::{self, Write, Read, ErrorKind};

enum Action {
    Read,
    Write,
}

const default_dirs: [&'static str; 2] = [ 
    ".rk",
    ".config/rk" 
];

const default_paths: [&'static str; 2] = [ 
    "locker::.rk",
    "config::.config/rk/rk.yml" 
];

pub struct LockerFiles {
    paths: HashMap<String, String> 
}

impl LockerFiles {
    pub fn new() -> LockerFiles {
        if !LockerFiles::config_exists() {
            LockerFiles::init_default_dirs().expect("Failed initiating default diretories");
            LockerFiles::init_default_yaml().expect("Failed initiating default yaml file");
        }

        LockerFiles {
            paths: LockerFiles::read_paths()
        }
    } 

    fn init_default_dirs() -> io::Result<()> {
        for dir in default_dirs.iter() {
            let mut new_dir = dirs::home_dir().unwrap();

            new_dir.push(dir);

            let dir_path = new_dir.as_path();

            if !dir_path.exists() {
                LockerFiles::create_dir(&new_dir)?;
            }
        } 

        Ok(())
    }

    // TODO: this is pretty ugly, def a better way to do it
    fn init_default_yaml() -> serde_yaml::Result<()> {
        type YAML = BTreeMap<String, BTreeMap<String, String>>;
        
        let mut map: YAML = BTreeMap::new();
        map.insert(String::from("paths"), BTreeMap::new());

        let config_path = LockerFiles::default_config_dir();

        for path in default_paths.iter() {
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
        
        LockerFiles::write(
            config_path.to_str().unwrap(),
            serde_yaml::to_string(&map)?
        );

        Ok(()) 
    }

    fn config_exists() -> bool {
        let mut config_dir = dirs::home_dir().unwrap();
        config_dir.push(".config/rk");
        
        config_dir.as_path().exists()
    }

    fn default_config_dir() -> PathBuf {
        let mut config_path = dirs::home_dir().unwrap();
        config_path.push(".config/rk/rk.yml");

        config_path
    }

    fn create_dir(path: &PathBuf) -> io::Result<()> {
        match fs::read_dir(&path) {
            Ok(_) => Ok(()),
            Err(err) => {
                if err.kind() == ErrorKind::NotFound { fs::create_dir_all(&path)?; }
                
                Ok(())
            },
        }
    } 

    fn read_paths() -> HashMap<String, String> {
        let mut path_map: HashMap<String, String> = HashMap::new();

        let config_path = LockerFiles::default_config_dir()
            .to_str()
            .unwrap()
            .to_owned();

        let config_file = LockerFiles::open(&config_path[..], Action::Read);
        let yml_map: Mapping = serde_yaml::from_reader(config_file).unwrap();

        let paths = LockerFiles::key_mapping(&yml_map, "paths");
        
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
       let value = LockerFiles::build_value_string(string); 

        map.get(&value)
            .unwrap()
            .to_owned()
            .as_mapping()
            .unwrap()
            .to_owned()
    }

    fn key_values(map: &Mapping, string: &str) -> Value {
       let value = LockerFiles::build_value_string(string); 

        map.get(&value)
            .unwrap()
            .to_owned()
    } 

    fn open(path: &str, action: Action) -> File {
        let path = Path::new(path);
        
        let file = match action {
            Action::Read => LockerFiles::try_open(path),
            Action::Write => File::create(path).expect("Unable to open path to write!"),
        };

        file
    }

    fn try_open(path: &Path) -> File {
        // TODO: Handle when file does not exist on read (eg. running command in "wrong" dir)
        match File::open(&path) {
            Err(_) => File::create(&path).expect("Unable to create path file!"),
            Ok(file) => file,
        }
    }

    fn write(path: &str, contents: String) {
        let mut file = LockerFiles::open(path, Action::Write);

        match file.write_all(contents.as_bytes()) {
            Err(why) => panic!("Couldn't write to {}: {}", path, why),
            Ok(_) => println!("\n::: Success writing to {} :::\n", path),
        } 
    }
}
