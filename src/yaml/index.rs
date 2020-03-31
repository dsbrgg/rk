use std::error::Error;
use std::collections::HashMap;

use serde_yaml::Value;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Index {
    accounts: HashMap<String, Vec<Value>>
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

    pub fn add(&mut self, entity: String, account: Option<String>) -> Result<(), Box<dyn Error>> {
        let entry = self.accounts.entry(entity).or_insert(Vec::new());
       
        if let Some(acc) = account {
            entry.push(Value::String(acc));
        }

        Ok(())
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
        let mut accounts: HashMap<String, Vec<Value>> = HashMap::new();
        
        accounts.insert(
            "entity_hash".to_string(), 
            vec![Value::String("account_hash".to_string())]
        );

        let yaml = "---\naccounts:\n  entity_hash:\n    - account_hash";
        let index = Index::from_yaml(yaml).unwrap();
        let compare = Index { accounts };

        assert_eq!(index, compare);
    }

    #[test]
    fn to_yaml() {
        let mut accounts: HashMap<String, Vec<Value>> = HashMap::new();
        accounts.insert(
            "entity_hash".to_string(), 
            vec![Value::String("account_hash".to_string())]
        );
        
        let yaml = "---\naccounts:\n  entity_hash:\n    - account_hash";
        let index = Index { accounts };

        assert_eq!(index.to_yaml().unwrap(), yaml);
    }

    #[test]
    fn add_entity_without_account() {
        let yaml = "---\naccounts:\n  entity_hash:\n    - account_hash";
        let mut accounts: HashMap<String, Vec<Value>> = HashMap::new();
        accounts.insert(
            "entity_hash".to_string(), 
            vec![Value::String("account_hash".to_string())]
        );

        accounts.insert(
            "new_entity".to_string(), 
            vec![]
        );

        let mut index = Index::from_yaml(yaml).unwrap();
        index.add("new_entity".to_string(), None);

        assert_eq!(index.accounts, accounts);
    }

    #[test]
    fn add_entity_with_account() {
        let yaml = "---\naccounts:\n  entity_hash:\n    - account_hash";
        let mut accounts: HashMap<String, Vec<Value>> = HashMap::new();
        accounts.insert(
            "entity_hash".to_string(), 
            vec![Value::String("account_hash".to_string())]
        );

        accounts.insert(
            "new_entity".to_string(), 
            vec![Value::String("new_account".to_string())]
        );

        let mut index = Index::from_yaml(yaml).unwrap();
        index.add("new_entity".to_string(), Some("new_account".to_string()));

        assert_eq!(index.accounts, accounts);
    }

    #[test]
    fn add_account_to_existing_entity() {
        let yaml = "---\naccounts:\n  entity_hash:\n    - account_hash";
        let mut accounts: HashMap<String, Vec<Value>> = HashMap::new();
        accounts.insert(
            "entity_hash".to_string(), 
            vec![Value::String("account_hash".to_string())]
        );

        let existing_entity = accounts.get_mut("entity_hash").unwrap();
        existing_entity.push(Value::String("new_account".to_string()));

        let mut index = Index::from_yaml(yaml).unwrap();
        index.add("entity_hash".to_string(), Some("new_account".to_string()));

        assert_eq!(index.accounts, accounts);
    }
}
