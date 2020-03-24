use std::collections::HashMap;

use serde_yaml::Value;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Index(String, HashMap<String, Value>);

impl Index {

    /* Initialisers */

    fn from_yaml(yaml: &str) -> Result<Index, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    /* Methods */

    fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self) 
    }

    pub fn get(&self, path: String) -> String {
       String::new() 
    }
}
