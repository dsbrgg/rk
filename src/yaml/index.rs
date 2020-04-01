use std::io::Read;
use std::fs::File;
use std::error::Error;
use std::path::{PathBuf};
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
        serde_yaml::from_str(yaml)
    }

    pub fn from_pathbuf(yaml: PathBuf) -> Result<Index, serde_yaml::Error> {
        if !yaml.as_path().exists() { 
            let msg = format!("Unable to create index file at {:?}", yaml);
            
            File::create(&yaml).expect(&msg); 
        }

        let mut contents = String::new();
        let mut file = File::open(yaml).expect("Unable to open index file");

        file.read_to_string(&mut contents)
            .expect("Unable to read index file into string");

        if contents.is_empty() {
            let empty_index = Index {
                accounts: HashMap::new()
            };

            return Ok(empty_index);
        }

        serde_yaml::from_str(&contents)
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

    pub fn get(&mut self, entity: String, account: String) -> String {
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
    fn get() {
        let yaml = "---\naccounts:\n  entity_hash:\n    - account_hash";
        let mut index = Index::from_yaml(yaml).unwrap();
        index.add("entity_hash".to_string(), Some("new_account".to_string()));

        let entity = String::from("entity_hash");
        let account = String::from("new_account");

        assert_eq!(
            index.get(entity, account), 
            String::from("new_account")
        );
    }
}
