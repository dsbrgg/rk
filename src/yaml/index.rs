use std::collections::HashMap;

use serde_yaml::Value;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Index {
    entities: Vec<Value>,
    accounts: HashMap<String, Value>
}

impl Index {

    /* Initialisers */

    fn from_yaml(yaml: &str) -> Result<Index, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    /* Methods */

    fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self) 
    }

    pub fn add(&self, entity: String, account: String) {
        
    }

    pub fn get(&self, entity: String, account: Option<String>) -> String {
       String::new() 
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_yaml() {
        let mut accounts: HashMap<String, Value> = HashMap::new();
        let entities = vec![Value::String("entity_hash".to_string())]; 
        accounts.insert("account_hash".to_string(), Value::String("entity_hash".to_string()));

        let yaml = "---\nentities:\n  - entity_hash\naccounts:\n  account_hash: entity_hash";
        let index = Index::from_yaml(yaml).unwrap();

        let compare = Index {
            entities,
            accounts
        };

        assert_eq!(index, compare);
    }

    #[test]
    fn to_yaml() {
        let mut accounts: HashMap<String, Value> = HashMap::new();
        let entities = vec![Value::String("entity_hash".to_string())]; 
        accounts.insert("account_hash".to_string(), Value::String("entity_hash".to_string()));
        
        let yaml = "---\nentities:\n  - entity_hash\naccounts:\n  account_hash: entity_hash";
        let index = Index {
            entities,
            accounts
        };

        assert_eq!(index.to_yaml().unwrap(), yaml);
    }
}
