extern crate serde_yaml;

use serde_yaml::{Mapping, Value};

use std::io::Write;
use std::io::Read;

use std::fs::File;
use std::path::Path;

use std::collections::HashMap;

enum Action {
    Read,
    Write,
}

pub struct Files<'f> {
    paths: HashMap<&'f str, &'f str> 
}

impl<'f> Files<'f> {
    pub fn test() {
        let f = Files::open("test.yaml", Action::Read);
        let mut d: Mapping = serde_yaml::from_reader(f).unwrap();
       
        Files::test_consume_yaml(&mut d);

        // let e = d.get(&Value::String("test1".to_string()));
        // printlna("Read YAML string: {:?}", e);
    }

    fn test_consume_yaml(d: &mut Mapping) {
        let paths = Files::key_mapping(&d, "paths");
        let locker_path = Files::key_values(&paths, "locker");

        println!("{:?}", locker_path);
    }

    fn build_value_string(string: &str) -> Value {
        Value::String(
            String::from(string)
        )
    }

    fn key_mapping(map: &Mapping, string: &str) -> Mapping {
       let value = Files::build_value_string(string); 

        map.get(&value)
            .unwrap()
            .to_owned()
            .as_mapping()
            .unwrap()
            .to_owned()
    }

    fn key_values(map: &Mapping, string: &str) -> Value {
       let value = Files::build_value_string(string); 

        map.get(&value)
            .unwrap()
            .to_owned()
    }

    pub fn new() -> Files<'f> {
        Files {
            paths: HashMap::new()
        }
    }

    fn serialize(from: String) {

    }

    fn open(path: &str, action: Action) -> File {
        let path = Path::new(path);
        
        let file = match action {
            Read => Files::try_open(path),
            Write => File::create(path).expect("Unable to open locker to write!"),
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
