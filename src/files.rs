extern crate dirs;
extern crate serde_yaml;

use serde_yaml::{Mapping, Value};

use std::io;
use std::io::Write;
use std::io::Read;
use std::io::ErrorKind;

use std::fs;
use std::fs::File;

use std::path::Path;
use std::path::PathBuf;

use std::collections::HashMap;

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

    fn init_default_dirs() {
        let mut config_dir = dirs::home_dir().unwrap();
        let mut locker_dir = dirs::home_dir().unwrap();

        locker_dir.push(".rk");
        config_dir.push(".config/rk");
 
        match LockerFiles::create_dir(&config_dir) {
            Ok(_) => (),
            Err(err) => panic!("Unable to init config dir: {}", err),
        };

        match LockerFiles::create_dir(&locker_dir) {
            Ok(_) => (),
            Err(err) => panic!("Unable to init config dir: {}", err),
        };
    }

    pub fn test() {
        LockerFiles::init_default_dirs();
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
            Action::Write => File::create(path).expect("Unable to open locker to write!"),
        };

        file
    }

    fn try_open(path: &Path) -> File {
        // TODO: Handle when file does not exist on read (eg. running command in "wrong" dir)
        match File::open(&path) {
            Err(_) => File::create(&path).expect("Unable to create locker file!"),
            Ok(file) => file,
        }
    }
}
