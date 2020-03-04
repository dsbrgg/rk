use std::default::Default;
use std::collections::HashMap;

use serde_yaml:: Value;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Settings {
    paths: HashMap<String, Value>
}

impl Default for Settings {
    fn default() -> Self {
        let mut paths: HashMap<String, Value> = HashMap::new();

        let index = String::from("index");
        let config = String::from("config");
        let locker = String::from("locker");

        let index_value =  Value::String("$HOME/.rk/index".to_string());
        let config_value = Value::String("$HOME/.config/rk/settings.yml".to_string());
        let locker_value = Value::String("$HOME/.rk".to_string());

        paths.insert(index, index_value);
        paths.insert(config, config_value);
        paths.insert(locker, locker_value);

        Settings {
            paths
        }
    }
}

impl Settings {

    /* Initialisers */

    fn from_yaml(yaml: &str) -> Result<Settings, serde_yaml::Error> {
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
    fn default() {
        let mut paths: HashMap<String, Value> = HashMap::new();

        let index = String::from("index");
        let config = String::from("config");
        let locker = String::from("locker");

        let index_value =  Value::String("$HOME/.rk/index".to_string());
        let config_value = Value::String("$HOME/.config/rk/settings.yml".to_string());
        let locker_value = Value::String("$HOME/.rk".to_string());
        
        paths.insert(index, index_value);
        paths.insert(config, config_value);
        paths.insert(locker, locker_value);

        let settings = Settings { paths };
        let default_settings: Settings = Default::default();

        assert_eq!(settings, default_settings);
    }

    #[test]
    fn from_yaml() {
        let mut yaml = String::new();
        let mut current_dir = env::current_dir().unwrap();

        current_dir.push("settings.yml");
        
        let path = current_dir.as_path();
        let mut config_file = File::open(path).unwrap();

        config_file.read_to_string(&mut yaml).unwrap();

        let deserialized = Settings::from_yaml(&yaml).unwrap();
        let mut default_paths: HashMap<String, Value> = HashMap::new();

        let index = String::from("index");
        let config = String::from("config");
        let locker = String::from("locker");

        let index_value =  Value::String("$HOME/.rk/index".to_string());
        let config_value = Value::String("$HOME/.config/rk/settings.yml".to_string());
        let locker_value = Value::String("$HOME/.rk".to_string());
        
        default_paths.insert(index, index_value);
        default_paths.insert(config, config_value);
        default_paths.insert(locker, locker_value);

        let default_config = Settings {
            paths: default_paths
        };

        assert_eq!(deserialized, default_config);
    }
}
