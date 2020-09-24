
/* Dependencies */

use std::io;
use std::path::{PathBuf};
use std::collections::HashMap;

use crate::locker::{Locker, Distinguished};
use crate::managers::{DirManager, FileManager};

/* Custom types */

type Account = HashMap<Locker, Locker>;
type Entity = HashMap<Locker, Vec<Account>>;

/* Vault struct definition */

pub struct Vault {
   structure: Entity,
   files: FileManager,
   directories: DirManager,
}

/* Vault struct behavior */

impl Vault {

    /* Intialisers */

    pub fn new(index: &PathBuf, config: &PathBuf, locker: &PathBuf) {
        let mut dm = DirManager::new(config, locker);
        let mut fm = FileManager::new(index, config, locker);

        dm.create_locker("testing");
        dm.create_locker("testing2");

        let main_locker = dm.read_locker("").unwrap();

        for entity in main_locker.iter() {
            println!("{:?}", entity.file_name());

            let entity_locker = entity.file_name().unwrap().to_str();
            let account_locker = dm.read_locker(entity_locker.unwrap()).unwrap();

            for account in account_locker.iter() {
                println!("{:?}", account.file_name());
            }
        }

        panic!(":)");
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
                Vault::new(&index, &config, &locker);
            }
        }; 
    }
}
