use std::collections::HashMap;

use serde_yaml:: Value;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Config {
    paths: HashMap<String, Value>
}

impl Config {

    /* Initialisers */

    fn from_yaml(yaml: &str) -> Result<Config, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    /* Methods */

    fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self) 
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    use std::env;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn from_yaml() {
        let mut yaml = String::new();
        let mut current_dir = env::current_dir().unwrap();

        current_dir.push("settings.yml");
        
        let path = current_dir.as_path();
        let mut config_file = File::open(path).unwrap();

        config_file.read_to_string(&mut yaml).unwrap();

        let deserialized = Config::from_yaml(&yaml).unwrap();
        let mut default_paths: HashMap<String, Value> = HashMap::new();

        let index = String::from("index");
        let config = String::from("config");
        let locker = String::from("locker");

        let index_value =  Value::String("".to_string());
        let config_value = Value::String("".to_string());
        let locker_value = Value::String("".to_string());
        
        default_paths.insert(index, index_value);
        default_paths.insert(config, config_value);
        default_paths.insert(locker, locker_value);

        let default_config = Config {
            paths: default_paths
        };

        assert_eq!(deserialized, default_config);
    }
}
