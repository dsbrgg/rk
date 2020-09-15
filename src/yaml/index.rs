use std::io::Read;
use std::fs::File;
use std::error::Error;
use std::path::PathBuf;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use serde_yaml::Value;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Index {
    accounts: HashMap<String, Vec<Value>>
}

#[derive(Debug)]
pub enum Type {
    Entity(Vec<Value>),
    Account(String)
}

impl Type {
    pub fn is_entity(&self) -> bool {
        if let Type::Entity(_) = self {
            true
        } else {
            false
        }
    }

    pub fn to_entities(self) -> Vec<Value> {
        if let Type::Entity(vec) = self { return vec; }
        panic!("to_entities should be called on a Type::Entity only");
    }

    pub fn is_account(&self) -> bool {
        if let Type::Account(_) = self {
            true
        } else {
            false
        }
    }

    pub fn to_account(self) -> String {
        if let Type::Account(string) = self { return string; }
        panic!("to_account should be called on a Type::Account only");
    }
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

    pub fn find(&mut self, entity: Option<String>, account: Option<String>) -> Option<Type> {
        if entity.is_some() && account.is_some() {
            let e = entity.unwrap();
            let a = account.unwrap();
                
            if let Some(register) = self.get_account(e, a) {
                println!("{:?}", register);
                return Some(Type::Account(register));
            };

            return None;
        }

        if entity.is_some() && account.is_none() { 
            let e = entity.unwrap();

            if let Some(register) = self.get_entity(e) {
                return Some(Type::Entity(register));
            }
        }

        None
    }

    pub fn get_all(&mut self) -> HashMap<String, Vec<Value>> {
        self.accounts.to_owned()
    }

    pub fn get_entity(&mut self, entity: String) -> Option<Vec<Value>> {
        match self.accounts.entry(entity) {
            Entry::Occupied(mut e) => Some(e.get_mut().to_owned()),
            Entry::Vacant(v) => None,
        }
    }

    pub fn get_account(&mut self, entity: String, account: String) -> Option<String> {
        let entry = self.get_entity(entity);
        println!("{:?}", entry);
        
        if let Some(ent) = entry {
            println!("{:?}", ent);
            let filter = |acc: &&Value| -> bool { **acc == Value::String(account.to_owned()) };
            let mut filtered = ent.iter().filter(filter);
            
            if let Some(acc) = filtered.next() {
                return Some(acc.as_str().unwrap().to_string());
            }
        } 

        None
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
            index.get_entity(entity).unwrap(),
            vec![Value::String("account_hash".to_string())]
        );
    }

    #[test]
    fn get_entity_missing() {
        let entity = String::from("entity_hash");
        let yaml = "---\naccounts:\n  different_hash:\n    - account_hash";
        let mut index = Index::from_yaml(yaml).unwrap();

        assert_eq!(index.get_entity(entity), None);
    }

    #[test]
    fn get_account() {
        let yaml = "---\naccounts:\n  entity_hash:\n    - account_hash";
        let mut index = Index::from_yaml(yaml).unwrap();
        index.add("entity_hash".to_string(), Some("new_account".to_string()));

        let entity = String::from("entity_hash");
        let account = String::from("new_account");

        assert_eq!(
            index.get_account(entity, account).unwrap(), 
            String::from("new_account")
        );
    }

    #[test]
    fn get_account_missing() {
        let yaml = "---\naccounts:\n  entity_hash:\n    - different_hash";
        let mut index = Index::from_yaml(yaml).unwrap();
        
        let entity = String::from("entity_hash");
        let account = String::from("new_account");

        assert_eq!(index.get_account(entity, account), None);
    }

    #[test]
    fn find_some_account() {
        let yaml = "---\naccounts:\n  entity_hash:\n    - new_account";
        let mut index = Index::from_yaml(yaml).unwrap();
        
        let entity = String::from("entity_hash");
        let account = String::from("new_account");

        let e = Some(entity);
        let a = Some(account);
        let result = index.find(e, a)
            .unwrap()
            .to_account();

        assert_eq!(result, String::from("new_account"));
    }

    #[test]
    fn find_none_account() {
        let yaml = "---\naccounts:\n  entity_hash:\n    - different_hash";
        let mut index = Index::from_yaml(yaml).unwrap();
        
        let entity = String::from("entity_hash");
        let account = String::from("new_account");

        let e = Some(entity);
        let a = Some(account);
        let result = index.find(e, a); 

        assert_eq!(result.is_none(), true);
    }
}
