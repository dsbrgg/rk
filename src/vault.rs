
/* Dependencies */

use std::io;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::locker::Encrypted;
use crate::managers::{DirManager, FileManager};

/* Custom types */

type Account = HashMap<Encrypted, Encrypted>;
type Structure = HashMap<Encrypted, Account>;

/* Vault struct definition */

pub struct Vault {
   structure: Structure,
   files: FileManager,
   directories: DirManager,
}

/* Vault struct behavior */

impl Vault {

    /* Intialisers */

    pub fn new(index: &PathBuf, config: &PathBuf, locker: &PathBuf) -> Vault {
        let mut dm = DirManager::new(config, locker);
        let mut fm = FileManager::new(index, config, locker);
        let mut structure = Structure::new();
        let entities = dm.read_locker("").unwrap();

        for entity in entities.iter() {
            let mut accounts = Vec::new();
            let entity_name = Self::to_string(&entity);
            let encrypted_entity = Encrypted::from(&entity_name).unwrap();
            let entity_dir = dm.read_locker(&entity_name).unwrap();

            for account in entity_dir.iter() {
                let mut account_path = PathBuf::new();
                let account_name = Self::to_string(&account);
                let encrypted_account = Encrypted::from(&account_name).unwrap();

                account_path.push(&entity_name);
                account_path.push(&account_name);

                let path = account_path.to_str().unwrap();
                let account_dir = dm.read_locker(path).unwrap();
                let password_file = &account_dir[0];
                let password_name = Self::to_string(&password_file);
                let encrypted_password = Encrypted::from(&password_name).unwrap();
                
                accounts.push((encrypted_account, encrypted_password));
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

        Vault {
            structure,
            files: fm,
            directories: dm
        }
    }

    /* Methods */

    pub fn get_entity(&self, entity: &Encrypted) -> Option<&Account> {
        let structure = self.structure.get(entity);

        if structure.is_none() {
            return None;
        }

        structure
    }

    pub fn get_account(&self, entity: &Encrypted, account: &Encrypted) -> Option<&Encrypted> {
        let structure = self.get_entity(entity).unwrap();
        
        structure.get(account)
    }

    pub fn set_entity(&mut self, entity: &Encrypted) {
        let path = entity.path();
        let msg = format!("Unable to create locker at: {}", &path);

        self.directories.create_locker(&path).expect(&msg);
        self.structure.insert(entity.to_owned(), Account::new());
    }

    pub fn set_account(&mut self, entity: &Encrypted, account: &Encrypted, password: &Encrypted) {
        let mut path = PathBuf::new();
        let structure_entity = self.structure.get_mut(entity).unwrap();
        let entity_path = entity.path();
        let account_path = account.path();
        let password_path = password.path();

        path.push(entity_path);
        path.push(account_path);

        let account_locker = path.to_str().unwrap();
        let msg = format!("Unable to create account locker at: {}", &account_locker);
        self.directories.create_locker(account_locker).expect(&msg);

        path.push(password_path);

        let password_locker = path.to_str().unwrap();
        let msg = format!("Unable to create password locker at: {}", &password_locker);
        self.files.create_locker(password_locker).expect(&msg);

        structure_entity.insert(
            account.to_owned(), 
            password.to_owned()
        );
    }

    pub fn remove_entity(&mut self, entity: &Encrypted) {
        let path = entity.path();
        let msg = format!("Unable to remove locker at: {}", &path);

        self.directories.remove_locker(&path).expect(&msg);
        self.structure.remove(entity);
    }

    pub fn remove_account(&mut self, entity: &Encrypted, account: &Encrypted) {
        let mut path = PathBuf::new();
        let structure_entity = self.structure.get_mut(entity).unwrap();
        let entity_path = entity.path();
        let account_path = account.path();

        path.push(entity_path);
        path.push(account_path);

        let account_locker = path.to_str().unwrap();
        let msg = format!("Unable to remove account locker at: {}", &account_locker);
        self.directories.remove_locker(account_locker).expect(&msg);
        
        structure_entity.remove(account);
    }

    /* Associated functions */

    fn to_string(path_string: &PathBuf) -> String {
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
    use std::panic::{AssertUnwindSafe, catch_unwind};

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

        path.push("foo$bar$biz$fred");
        path.push("quux$foo$bar$biz");

        let entity_locker_path = path.to_str().unwrap();
        dm.create_locker(entity_locker_path)
            .expect("Unable to create locker directories");

        path.push("biz$fred$bar$corge");

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
                
                Vault::new(&index, &config, &locker);
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

                let vault = Vault::new(&index, &config, &locker);
                let entity = Encrypted::from("foo$bar$biz$fred").unwrap();
                let accounts = vault.get_entity(&entity);

                assert!(accounts.is_some());

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

                let vault = Vault::new(&index, &config, &locker);
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
                let mut vault = Vault::new(&index, &config, &locker);
                let entity = Encrypted::from("foo$foo$foo$foo").unwrap();

                vault.set_entity(&entity);

                let structure_entity = vault.get_entity(&entity);
                let dm_entity = dm.read_locker(&entity.path()).unwrap();
                let dm_expected: Vec<PathBuf> = Vec::new();

                assert!(structure_entity.is_some());
                assert_eq!(dm_entity, dm_expected);

                if let Some(e) = structure_entity {
                    assert_eq!(*e, Account::new());
                }
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

                let mut path = PathBuf::new();
                let mut dm = DirManager::new(&config, &locker);
                let mut vault = Vault::new(&index, &config, &locker);
                let ent = Encrypted::from("foo$foo$foo$foo").unwrap();
                let acc = Encrypted::from("bar$bar$bar$bar").unwrap();
                let pass = Encrypted::from("biz$biz$biz$biz").unwrap();
                
                vault.set_entity(&ent);
                vault.set_account(&ent, &acc, &pass);

                path.push(ent.path());
                path.push(acc.path());

                let account = vault.get_account(&ent, &acc);
                let dm_account = dm.read_locker(path.to_str().unwrap()).unwrap();

                assert!(account.is_some());
                assert!(dm_account[0].ends_with(pass.path()));

                if let Some(a) = account {
                    assert_eq!(*a, pass);
                }
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
                let mut vault = Vault::new(&index, &config, &locker);
                let entity = Encrypted::from("foo$foo$foo$foo").unwrap();

                vault.set_entity(&entity);

                let structure_entity = vault.get_entity(&entity);
                let dm_entity = dm.read_locker(&entity.path()).unwrap();
                let dm_expected: Vec<PathBuf> = Vec::new();

                assert!(structure_entity.is_some());
                assert_eq!(dm_entity, dm_expected);

                vault.remove_entity(&entity);
                let dm_entity = dm.read_locker(&entity.path()).unwrap();

                assert_eq!(dm_entity.len(), 0);
                assert_eq!(vault.get_entity(&entity), None);
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
                let mut vault = Vault::new(&index, &config, &locker);
                let ent = Encrypted::from("foo$foo$foo$foo").unwrap();
                let acc = Encrypted::from("bar$bar$bar$bar").unwrap();
                let pass = Encrypted::from("biz$biz$biz$biz").unwrap();
                let other_acc = Encrypted::from("fred$fred$fred$fred").unwrap();
                let other_pass = Encrypted::from("bar$bar$bar$bar").unwrap();
                
                vault.set_entity(&ent);
                vault.set_account(&ent, &acc, &pass);
                vault.set_account(&ent, &other_acc, &other_pass);

                path.push(ent.path());
                path.push(acc.path());

                let entity = vault.get_entity(&ent).unwrap();

                assert_eq!(entity.keys().len(), 2);

                vault.remove_account(&ent, &acc);

                let entity = vault.get_entity(&ent).unwrap();
                let dm_entity = dm.read_locker(&ent.path()).unwrap();

                assert_eq!(entity.keys().len(), 1);
                assert_eq!(dm_entity.len(), 1);
            }
        }; 
    }
}
