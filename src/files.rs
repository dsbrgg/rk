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

pub struct LockerFiles {
    paths: HashMap<String, String> 
}

impl LockerFiles {
    pub fn new() -> LockerFiles {
        // TODO: handle if user is a first-timer
        // to load paths appropriately
        LockerFiles {
            paths: HashMap::new()
        }
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

    fn init_default_dirs() -> io::Result<()> {
        let mut config_dir = dirs::home_dir().unwrap();
        let mut locker_dir = dirs::home_dir().unwrap();

        locker_dir.push(".rk");
        config_dir.push(".config/rk");

        let config_dir_path = config_dir.as_path();
        let locker_dir_path = locker_dir.as_path();

        if !locker_dir_path.exists() {
            match LockerFiles::create_dir(&locker_dir) {
                Ok(_) => (),
                Err(err) => panic!("Unable to initialize locker dir: {}", err),
            };
        }

        if !config_dir_path.exists() {
            match LockerFiles::create_dir(&config_dir) {
                Ok(_) => (),
                Err(err) => panic!("Unable to initialize config dir: {}", err),
            };
        } 

        Ok(())
    }

    fn init_default_yaml() -> serde_yaml::Result<()> {
        type YAML = BTreeMap<String, BTreeMap<String, String>>;

        let mut rk_yml = dirs::home_dir().unwrap();
        let mut locker = dirs::home_dir().unwrap();

        locker.push(".rk");
        rk_yml.push(".config/rk/rk.yml");

        let config_path = rk_yml.as_path();
        let locker_path = locker.as_path();

        if !config_path.exists() {
            let mut map: YAML = BTreeMap::new();
            map.insert(String::from("paths"), BTreeMap::new());
            
            let paths = map.get_mut("paths").unwrap();
            let config_path_string = String::from(config_path.to_str().unwrap());
            let locker_path_string = String::from(locker_path.to_str().unwrap());

            paths.insert(String::from("config"), config_path_string);
            paths.insert(String::from("locker"), locker_path_string);

            LockerFiles::write(
                config_path.to_str().unwrap(),
                serde_yaml::to_string(&map)?
            );
        } 

        Ok(()) 
    }

    pub fn test() {
        LockerFiles::init_default_dirs();
        LockerFiles::init_default_yaml();

        let f = LockerFiles::open("../rk.yml", Action::Read);
        let mut d: Mapping = serde_yaml::from_reader(f).unwrap();
       
        LockerFiles::test_consume_yaml(&mut d);
    }

    fn test_consume_yaml(d: &mut Mapping) {
        let paths = LockerFiles::key_mapping(&d, "paths");
        let locker_path = LockerFiles::key_values(&paths, "locker");

        println!("{:?}", locker_path);
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
