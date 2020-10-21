
/* Dependencies */

use std::io;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::locker::{Locker, Encrypted};
use crate::managers::{Manager, DirManager, FileManager};

/* Custom types */

type Account = HashMap<Encrypted, Encrypted>;
type Structure = HashMap<Encrypted, Account>;

/* VaultError enum */

#[derive(Debug)]
pub enum VaultError {
    Error(String),
    Io(io::Error)
}

impl VaultError {
    pub fn to_str(self) -> String {
        match self {
            VaultError::Error(s) => s,
            VaultError::Io(e) => format!("{:?}", e)
        }
    }
}

/* VaultError From implementations */

impl From<io::Error> for VaultError {
    fn from(err: io::Error) -> VaultError {
        VaultError::Io(err)
    }
}

impl From<String> for VaultError {
    fn from(err: String) -> VaultError {
        VaultError::Error(err)
    }
}

impl From<&str> for VaultError {
    fn from(err: &str) -> VaultError {
        VaultError::Error(err.to_string())
    }
}

/* Vault struct definition */

#[derive(Debug)]
pub struct Vault {
   structure: Structure,
   files: FileManager,
   directories: DirManager,
}

/* Vault struct behavior */

impl Vault {

    /* Intialisers */

    pub fn new(index: &PathBuf, config: &PathBuf, locker: &PathBuf) -> Result<Vault, VaultError> {
        let mut dm = DirManager::new(config, locker);
        let mut fm = FileManager::new(index, config, locker);
        let mut structure = Structure::new();
        let entities = dm.read_locker("")?;

        for entity in entities.iter() {
            let mut accounts = Vec::new();
            let entity_name = Self::get_name(&entity);
            let encrypted_entity = Encrypted::from(&entity_name)?;
            let entity_dir = dm.read_locker(&entity_name)?;

            for account in entity_dir.iter() {
                let account_name = Self::get_name(&account);
                let encrypted_account = Encrypted::from(&account_name)?;
                let path = DirManager::append_path(&entity_name, &account_name);
                let account_dir = dm.read_locker(&path)?;

                if account_dir.len() == 1 {
                    let password_file = &account_dir[0];
                    let password_name = Self::get_name(&password_file);
                    let encrypted_password = Encrypted::from(&password_name)?;
                    
                    accounts.push((encrypted_account, encrypted_password));
                } else {
                    accounts.push((encrypted_account, Encrypted::empty()));
                }
            }

            if accounts.len() == 0 {
                structure
                    .entry(encrypted_entity.clone())
                    .or_insert(Account::new());
            
                continue;
            }

            for (account, password) in accounts.iter() {
                structure
                    .entry(encrypted_entity.clone())
                    .and_modify(|a| { a.insert(account.to_owned(), password.to_owned()); })
                    .or_insert_with(|| { 
                        let mut new = Account::new();
                        new.insert(account.to_owned(), password.to_owned());
                        new
                    });
            }
        }

        Ok(Vault {
            structure,
            files: fm,
            directories: dm
        })
    }

    /* Methods */

    fn get_entity_key(&self, entity: &Encrypted) -> Result<Encrypted, VaultError> {
        let error = VaultError::Error("Entity not found".to_string());
        let (vault_entity, _) = self.structure.get_key_value(&entity).ok_or(error)?;

        Ok(vault_entity.to_owned())
    }

    fn get_account_key(&self, entity: &Encrypted, account: &Encrypted) -> Result<Encrypted, VaultError> {
        let error = VaultError::Error("Account not found".to_string());
        let entity = self.get_entity(entity)?;
        let (vault_account, _) = entity.get_key_value(&account).ok_or(error)?;

        Ok(vault_account.to_owned())
    }

    fn has_entity(&self, entity: &Encrypted) -> bool {
        self.structure.contains_key(entity)
    }

    fn has_account(&self, entity: &Encrypted, account: &Encrypted) -> Result<bool, VaultError> {
        let entity = self.get_entity(entity)?;

        Ok(entity.contains_key(account))
    }

    fn has_password(&self, entity: &Encrypted, account: &Encrypted) -> Result<bool, VaultError> {
        let password = self.get_account(entity, account)?;

        Ok(!password.is_empty())
    }

    pub fn get_entity(&self, entity: &Encrypted) -> Result<&Account, VaultError> {
        let error = VaultError::Error("Entity not found".to_string());

        self.structure
            .get(entity)
            .ok_or(error)
    }

    pub fn get_account(&self, entity: &Encrypted, account: &Encrypted) -> Result<&Encrypted, VaultError> {
        let error = VaultError::Error("Account not found".to_string());
        let structure = self.get_entity(entity)?;
        
        structure
            .get(account)
            .ok_or(error)
    }

    pub fn set_entity(&mut self, entity: &Encrypted) -> Result<(), VaultError> {
        if self.has_entity(entity) {
            return Ok(());
        }

        let path = entity.path();

        self.directories.create_locker(&path)?;
        self.structure.insert(entity.to_owned(), Account::new());

        Ok(())
    }

    pub fn set_account(&mut self, entity: &Encrypted, account: &Encrypted) -> Result<(), VaultError> {
        if self.has_account(entity, account)? {
            return Ok(());
        }

        let error = VaultError::Error(entity.path());
        let vault_entity = self.get_entity_key(entity)?;
        let entity_path = vault_entity.path();
        let account_path = account.path();
        let path = DirManager::append_path(&entity_path, &account_path);
        let structure_entity = self.structure
            .get_mut(&vault_entity)
            .ok_or(error)?;

        self.directories.create_locker(&path)?;

        structure_entity.insert(
            account.to_owned(), 
            Encrypted::empty()
        );

        Ok(())
    }

    pub fn set_password(&mut self, entity: &Encrypted, account: &Encrypted, password: &Encrypted) -> Result<(), VaultError> {
        let mut path = PathBuf::new();
        let error = VaultError::Error(entity.path());

        let vault_entity = self.get_entity_key(entity)?;
        let vault_account = self.get_account_key(entity, account)?;

        let entity_path = vault_entity.path();
        let account_path = vault_account.path();
        let password_path = password.path();

        path.push(entity_path);
        path.push(account_path);

        if self.has_password(entity, account)? {
            let old_password = self.get_account(entity, account)?;

            path.push(old_password.path());

            let old_password_path = DirManager::pb_to_str(&path);
            self.files.remove_locker(&old_password_path)?;

            path.pop();
        }

        path.push(password_path);
        let password_locker = DirManager::pb_to_str(&path);
        self.files.create_locker(&password_locker)?;

        let structure_entity = self.structure
            .get_mut(&vault_entity)
            .ok_or(error)?;

        structure_entity.insert(
            account.to_owned(), 
            password.to_owned()
        );

        Ok(())
    }

    pub fn remove_entity(&mut self, entity: &Encrypted) -> Result<(), VaultError> {
        let directory = self.get_entity_key(entity)?;
        let locker = directory.path();

        self.structure.remove(entity);
        self.directories.remove_locker(&locker)?;

        Ok(())
    }

    pub fn remove_account(&mut self, entity: &Encrypted, account: &Encrypted) -> Result<(), VaultError> {
        let ent = self.get_entity_key(entity)?;
        let acc = self.get_account_key(entity, account)?;
        let path = DirManager::append_path(&ent.path(), &acc.path());
        // TODO: abstract this so unwrap does not need to be called
        let structure_entity = self.structure.get_mut(entity).unwrap();

        structure_entity.remove(account);
        self.directories.remove_locker(&path)?;

        Ok(())
    }

    /* Associated functions */

    fn get_name(path_string: &PathBuf) -> String {
        path_string.file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
}

/* Vault tests */

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks::Setup;

    use std::path::Path;
    use std::fs::{remove_dir_all, remove_file};

    fn after_each(this: &mut Setup) {
        for path in this.paths.iter() {
            let p = Path::new(path);

            let exists = &p.exists();
            let is_dir = &p.is_dir();
            let is_file = &p.is_file();

            let remove = if *is_dir { "dir" } else { "file" };
            let msg = format!("Could not remove {} {:?} in `vault.rs` test", remove, path);

            if *exists { 
                if *is_file { remove_file(path).expect(&msg); }
                if *is_dir { remove_dir_all(path).expect(&msg); }
            } 
        }
    }

    fn fill_locker(index: &PathBuf, config: &PathBuf, locker: &PathBuf) {
        let mut dm = DirManager::new(&config, &locker);
        let mut fm = FileManager::new(&index, &config, &locker);
        let mut path = PathBuf::new();

        let entity = Encrypted::from("foo$bar$biz$fred").unwrap();
        let account = Encrypted::from("quux$foo$bar$biz").unwrap();
        let password = Encrypted::from("biz$fred$bar$corge").unwrap();

        path.push(&entity.path());
        path.push(&account.path());

        let entity_locker_path = path.to_str().unwrap();
        dm.create_locker(entity_locker_path)
            .expect("Unable to create locker directories");

        path.push(&password.path());

        let password_locker_path = path.to_str().unwrap();
        fm.create_locker(password_locker_path)
            .expect("Unable to create locker file");
    }

    #[test]
    fn new() {
        Setup { 
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf();

                fill_locker(&index, &config, &locker);
                
                let vault = Vault::new(&index, &config, &locker);

                assert!(vault.is_ok());
            }
        }; 
    }

    #[test]
    fn get_entity() {
        Setup { 
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf();
                
                fill_locker(&index, &config, &locker);

                let vault = Vault::new(&index, &config, &locker).unwrap();
                let entity = Encrypted::from("foo$bar$biz$fred").unwrap();
                let accounts = vault.get_entity(&entity);

                assert!(accounts.is_ok());

                for (account, password) in accounts.unwrap() {
                    let encrypted_account = Encrypted::from("quux$foo$bar$biz").unwrap();
                    let encrypted_password = Encrypted::from("biz$fred$bar$corge").unwrap();

                    assert_eq!(*account, encrypted_account);
                    assert_eq!(*password, encrypted_password);
                }
            }
        }; 
    }

    #[test]
    fn get_account() {
        Setup { 
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf();

                fill_locker(&index, &config, &locker);

                let vault = Vault::new(&index, &config, &locker).unwrap();
                let entity = Encrypted::from("foo$bar$biz$fred").unwrap();
                let account = Encrypted::from("quux$foo$bar$biz").unwrap();
                let pass = Encrypted::from("biz$fred$bar$corge").unwrap();
                let acc = vault.get_account(&entity, &account).unwrap();

                assert_eq!(*acc, pass);
            }
        }; 
    }

    #[test]
    fn set_entity() {
        Setup { 
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf();

                fill_locker(&index, &config, &locker);

                let mut dm = DirManager::new(&config, &locker);
                let mut vault = Vault::new(&index, &config, &locker).unwrap();
                let entity = Encrypted::from("foo$foo$foo$foo").unwrap();

                assert!(vault.set_entity(&entity).is_ok());

                let structure_entity = vault.get_entity(&entity);
                let dm_entity = dm.read_locker(&entity.path()).unwrap();
                let dm_expected: Vec<PathBuf> = Vec::new();

                assert!(structure_entity.is_ok());
                assert_eq!(dm_entity, dm_expected);

                if let Ok(e) = structure_entity {
                    assert_eq!(*e, Account::new());
                }
            }
        }; 
    }

    #[test]
    fn set_entity_no_duplicates() {
        Setup { 
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf();
                let mut dm = DirManager::new(&config, &locker);
                let mut vault = Vault::new(&index, &config, &locker).unwrap();

                let mut locker_instance = Locker::new();
                let entity = locker_instance.encrypt("foo");
                assert!(vault.set_entity(&entity).is_ok());

                let mut locker_instance = Locker::new();
                let entity = locker_instance.encrypt("foo");
                assert!(vault.set_entity(&entity).is_ok());

                let dm_entity = dm.read_locker("").unwrap();

                assert_eq!(dm_entity.len(), 1);
            }
        }; 
    }

    #[test]
    fn set_account() {
        Setup { 
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf();

                fill_locker(&index, &config, &locker);

                let mut dm = DirManager::new(&config, &locker);
                let mut vault = Vault::new(&index, &config, &locker).unwrap();
                let ent = Encrypted::from("foo$foo$foo$foo").unwrap();
                let acc = Encrypted::from("bar$bar$bar$bar").unwrap();
                let path = DirManager::append_path(&ent.path(), &acc.path());
                
                assert!(vault.set_entity(&ent).is_ok());
                assert!(vault.set_account(&ent, &acc).is_ok());

                let account = vault.get_account(&ent, &acc);

                assert!(account.is_ok());

                if let Ok(a) = account {
                    assert_eq!(*a, Encrypted::empty());
                }
            }
        }; 
    }

    #[test]
    fn set_account_no_duplicates() {
        Setup { 
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf();
                let mut dm = DirManager::new(&config, &locker);
                let mut vault = Vault::new(&index, &config, &locker).unwrap();

                let mut locker_instance = Locker::new();
                let entity = locker_instance.encrypt("foo");
                let account = locker_instance.encrypt("bar");
                let same_account = locker_instance.encrypt("bar");
                let path = DirManager::append_path(&entity.path(), "");
                
                assert!(vault.set_entity(&entity).is_ok());
                assert!(vault.set_account(&entity, &account).is_ok());
                assert!(vault.set_account(&entity, &same_account).is_ok());

                let dm_entity = dm.read_locker(&path).unwrap();

                assert_eq!(dm_entity.len(), 1);
            }
        }; 
    }

    #[test]
    fn set_password() {
        Setup { 
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf();
                let mut dm = DirManager::new(&config, &locker);
                let mut vault = Vault::new(&index, &config, &locker).unwrap();
                let ent = Encrypted::from("foo$foo$foo$foo").unwrap();
                let acc = Encrypted::from("bar$bar$bar$bar").unwrap();
                let pass = Encrypted::from("biz$biz$biz$biz").unwrap();
                let path = DirManager::append_path(&ent.path(), &acc.path());
                
                assert!(vault.set_entity(&ent).is_ok());
                assert!(vault.set_account(&ent, &acc).is_ok());
                assert!(vault.set_password(&ent, &acc, &pass).is_ok());

                let account = vault.get_account(&ent, &acc);
                let password_location = dm.read_locker(&path).unwrap();

                assert!(account.is_ok());
                assert_eq!(*account.unwrap(), pass);
                assert_eq!(password_location.len(), 1);
            }
        }; 
    }

    #[test]
    fn set_password_one_file_limit() {
        Setup { 
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf();
                let mut dm = DirManager::new(&config, &locker);
                let mut vault = Vault::new(&index, &config, &locker).unwrap();
                let ent = Encrypted::from("foo$foo$foo$foo").unwrap();
                let acc = Encrypted::from("bar$bar$bar$bar").unwrap();
                let pass = Encrypted::from("biz$biz$biz$biz").unwrap();
                let other_pass = Encrypted::from("baz$baz$baz$baz").unwrap();
                let path = DirManager::append_path(&ent.path(), &acc.path());
                
                assert!(vault.set_entity(&ent).is_ok());
                assert!(vault.set_account(&ent, &acc).is_ok());
                assert!(vault.set_password(&ent, &acc, &pass).is_ok());
                assert!(vault.set_password(&ent, &acc, &other_pass).is_ok());

                let account = vault.get_account(&ent, &acc);
                let password_location = dm.read_locker(&path).unwrap();

                assert!(account.is_ok());
                assert_eq!(*account.unwrap(), pass);
                assert_eq!(password_location.len(), 1);
            }
        }; 
    }

    #[test]
    fn remove_entity() {
        Setup { 
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf();

                fill_locker(&index, &config, &locker);

                let mut dm = DirManager::new(&config, &locker);
                let mut vault = Vault::new(&index, &config, &locker).unwrap();
                let entity = Encrypted::from("foo$foo$foo$foo").unwrap();

                assert!(vault.set_entity(&entity).is_ok());

                let structure_entity = vault.get_entity(&entity);
                let dm_entity = dm.read_locker(&entity.path()).unwrap();
                let dm_expected: Vec<PathBuf> = Vec::new();

                assert!(structure_entity.is_ok());
                assert_eq!(dm_entity, dm_expected);
                assert!(vault.remove_entity(&entity).is_ok());

                let dm_entity = dm.read_locker(&entity.path()).unwrap();

                assert_eq!(dm_entity.len(), 0);
                assert!(vault.get_entity(&entity).is_err());
            }
        }; 
    }

    #[test]
    fn remove_account() {
        Setup { 
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf();

                fill_locker(&index, &config, &locker);

                let mut path = PathBuf::new();
                let mut dm = DirManager::new(&config, &locker);
                let mut vault = Vault::new(&index, &config, &locker).unwrap();
                let ent = Encrypted::from("foo$foo$foo$foo").unwrap();
                let acc = Encrypted::from("bar$bar$bar$bar").unwrap();
                let other_acc = Encrypted::from("fred$fred$fred$fred").unwrap();
                
                assert!(vault.set_entity(&ent).is_ok());
                assert!(vault.set_account(&ent, &acc).is_ok());
                assert!(vault.set_account(&ent, &other_acc).is_ok());

                path.push(ent.path());
                path.push(acc.path());

                let entity = vault.get_entity(&ent).unwrap();

                assert_eq!(entity.keys().len(), 2);
                assert!(vault.remove_account(&ent, &acc).is_ok());

                let entity = vault.get_entity(&ent).unwrap();
                let dm_entity = dm.read_locker(&ent.path()).unwrap();

                assert_eq!(dm_entity.len(), 1);
                assert_eq!(entity.keys().len(), 1);
                assert_eq!(entity.get(&acc), None);
                assert!(entity.get(&other_acc).is_some());
            }
        }; 
    }
}
