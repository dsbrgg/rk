use std::path::PathBuf;
use std::default::Default;
use std::collections::HashMap;

use serde_yaml:: Value;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Settings {
    paths: HashMap<String, Value>
}

pub enum SettingsOpts {
    Locker,
    Config,
}

impl SettingsOpts {
    fn to_str<'a>(self) -> &'a str {
        match self {
            SettingsOpts::Locker => "locker",
            SettingsOpts::Config => "config",
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

    pub fn get(&self, path: SettingsOpts) -> PathBuf {
        let option = path.to_str();

        let path = self.paths
            .get(option)
            .unwrap()
            .as_str()
            .unwrap();

        PathBuf::from(path)
    }
}

impl Default for Settings {
    fn default() -> Self {
        let mut paths: HashMap<String, Value> = HashMap::new();

        let locker = String::from("locker");
        let config = String::from("config");

        let mut locker_dir = dirs::home_dir().unwrap();
        let mut config_dir = dirs::config_dir().unwrap();

        locker_dir.push(".rk");
        
        config_dir.push("rk");
        config_dir.push("settings.yml");

        let locker_location = locker_dir.to_str().unwrap().to_string();
        let config_location = config_dir.to_str().unwrap().to_string();

        let config_value = Value::String(config_location);
        let locker_value = Value::String(locker_location);

        paths.insert(config, config_value);
        paths.insert(locker, locker_value);

        Settings {
            paths
        }
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

        let locker = String::from("locker");
        let config = String::from("config");

        let mut locker_dir = dirs::home_dir().unwrap();
        let mut config_dir = dirs::config_dir().unwrap();

        locker_dir.push(".rk");
        
        config_dir.push("rk");
        config_dir.push("settings.yml");

        let locker_location = locker_dir.to_str().unwrap().to_string();
        let config_location = config_dir.to_str().unwrap().to_string();

        let config_value = Value::String(config_location);
        let locker_value = Value::String(locker_location);

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

        let config = String::from("config");
        let locker = String::from("locker");

        let config_value = Value::String("$HOME/.config/rk/settings.yml".to_string());
        let locker_value = Value::String("$HOME/.rk".to_string());
        
        default_paths.insert(config, config_value);
        default_paths.insert(locker, locker_value);

        let default_config = Settings {
            paths: default_paths
        };

        assert_eq!(deserialized, default_config);
    }

    #[test]
    fn get() {
        let mut paths = HashMap::new();
        
        let locker = String::from("locker");
        let locker_value = Value::String("locker".to_string());

        let config = String::from("config");
        let config_value = Value::String("config".to_string());
        
        paths.insert(locker, locker_value);
        paths.insert(config, config_value);

        let settings = Settings { paths };
        let get_locker = settings.get(SettingsOpts::Locker);
        let get_config = settings.get(SettingsOpts::Config);

        assert_eq!(get_locker, PathBuf::from("locker"));
        assert_eq!(get_config, PathBuf::from("config"));
    }

    #[test]
    fn settings_opts_to_str() {
        let locker_option = SettingsOpts::Locker;
        let config_option = SettingsOpts::Config;

        assert_eq!(locker_option.to_str(), "locker");
        assert_eq!(config_option.to_str(), "config");
    }
}
