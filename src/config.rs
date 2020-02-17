use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config<'c> {
    path: &'c str 
}

impl<'c> Config<'c> {
    fn init() {
        let mut yaml = String::new();
        let mut current_dir = env::current_dir().unwrap();
        
        current_dir.push("settings.yml");

        let path = current_dir.as_path();
        let mut config_file = File::open(path).unwrap();

        config_file.read_to_string(&mut yaml).unwrap();

        println!("{:?}", yaml);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init() {
        Config::init();
        panic!(":)");
    }
}
