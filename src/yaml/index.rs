use std::io::Read;
use std::fs::File;
use std::error::Error;
use std::path::PathBuf;
use std::collections::HashMap;

use serde_yaml::Value;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Index {
    accounts: HashMap<String, Vec<Value>>
}

impl Index {

    /* Initialisers */

    pub fn from_yaml(yaml: &str) -> Result<Index, serde_yaml::Error> {
        if yaml.is_empty() {
            let accounts: HashMap<String, Vec<Value>> = HashMap::new();
            let index = Index { accounts };

            return Ok(index);
        }

        serde_yaml::from_str(yaml)
    }

    /* Methods */

    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self) 
    }

    pub fn add(&mut self, entity: String, account: Option<String>) -> Result<(), Box<dyn Error>> {
        let entry = self.accounts.entry(entity).or_insert(Vec::new());
       
        if let Some(acc) = account {
            entry.push(Value::String(acc));
        }

        Ok(())
    }

    pub fn get_all(&mut self) -> HashMap<String, Vec<Value>> {
        self.accounts.to_owned()
    }

    pub fn get_entity(&mut self, entity: String) -> Vec<Value> {
        self.accounts
            .entry(entity)
            .or_default()
            .to_owned()
    }

    pub fn get_account(&mut self, entity: String, account: String) -> String {
        let entry = self.accounts.entry(entity).or_default();
        let filter =  |acc: &&Value| -> bool { **acc == Value::String(account.to_owned()) };

        entry.iter()
            .filter(filter)
            .next()
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
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
    fn from_yaml_default() {
        let yaml = "---\naccounts: {}";
        let index = Index::from_yaml("").unwrap();
        let index_str = index.to_yaml().unwrap();

        assert_eq!(&index_str, yaml);
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

    #[test]
    fn get_all() {
        let yaml = "---\naccounts:\n  entity_hash:\n    - account_hash";
        let mut index = Index::from_yaml(yaml).unwrap();

        let mut all: HashMap<String, Vec<Value>> = HashMap::new();
        all.insert(
            "entity_hash".to_string(), 
            vec![Value::String("account_hash".to_string())]
        );
        
        assert_eq!(index.get_all(), all);
    }

    #[test]
    fn get_entity() {
        let entity = String::from("entity_hash");
        let yaml = "---\naccounts:\n  entity_hash:\n    - account_hash";
        let mut index = Index::from_yaml(yaml).unwrap();

        assert_eq!(
            index.get_entity(entity), 
            vec![Value::String("account_hash".to_string())]
        );
    }

    #[test]
    fn get_account() {
        let yaml = "---\naccounts:\n  entity_hash:\n    - account_hash";
        let mut index = Index::from_yaml(yaml).unwrap();
        index.add("entity_hash".to_string(), Some("new_account".to_string()));

        let entity = String::from("entity_hash");
        let account = String::from("new_account");

        assert_eq!(
            index.get_account(entity, account), 
            String::from("new_account")
        );
    }
}
