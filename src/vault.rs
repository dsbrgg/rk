
/* Dependencies */

use std::io;
use std::ffi::OsStr;
use std::path::{PathBuf};
use std::collections::HashMap;

use crate::managers::{DirManager, FileManager};
use crate::locker::{Locker, Encrypted, Distinguished};

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

    pub fn new(index: &PathBuf, config: &PathBuf, locker: &PathBuf) {
        let mut dm = DirManager::new(config, locker);
        let mut fm = FileManager::new(index, config, locker);
        let mut entities = Structure::new();
        let structure = dm.read_locker("").unwrap();

        for entity in structure.iter() {
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
                entities
                    .entry(encrypted_entity.clone())
                    .and_modify(|a| { a.insert(account.to_owned(), password.to_owned()); })
                    .or_insert_with(|| { 
                        let mut new = Account::new();
                        new.insert(account.to_owned(), password.to_owned());
                        new
                    });
            }
        }

        println!("{:?}", entities);

        panic!(":)");
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

    #[test]
    fn new() {
        Setup { 
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf();
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

                Vault::new(&index, &config, &locker);
            }
        }; 
    }
}
