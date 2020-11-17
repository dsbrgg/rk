mod args;
mod locker;
mod managers;
mod mocks;
mod vault;
mod settings;
mod tables;

use std::path::PathBuf;

pub use tables::*;
pub use args::Args;
pub use vault::{Vault, VaultResult, VaultError};
pub use locker::{Locker, Distinguished, Encrypted};

#[derive(Debug, PartialEq)]
pub enum Resolve {
    Done,
    Failure,
    Read(String),
    Find(Vec<(String, String)>),
    List(Vec<String>)
}

impl Resolve {
    pub fn to_vec(self) -> Vec<(String, String)> {
        if let Resolve::Find(vec) = self { return vec; }
        panic!("to_vec should be called on a Resolve::Find only");
    }
    
    pub fn to_list(self) -> Vec<String> {
        if let Resolve::List(vec) = self { return vec; }
        panic!("to_list should be called on a Resolve::List only");
    }

    pub fn to_string(self) -> String {
        if let Resolve::Read(string) = self { return string; }
        panic!("to_string should be called on a Resolve::Read only");
    }
}

pub struct Keeper { vault: Vault }

impl Keeper {
    pub fn new(config: PathBuf, locker: PathBuf) -> VaultResult<Keeper> {
        let vault = Vault::new(&config, &locker)?;
        let keeper = Keeper { vault };

        Ok(keeper)
    }

    pub fn add(&mut self, args: Args) -> VaultResult<Resolve> {
        let Args {
            entity,
            account,
            password
        } = args;

        self.vault.set(&entity, &account, &password)?;

        Ok(Resolve::Done)
    }

    pub fn find(&mut self, args: Args) -> VaultResult<Resolve> {
        let Args {
            entity,
            account,
            ..
        } = args;

        if entity.is_empty() && account.is_empty() {
            let err = VaultError::Error("Neither entity or account provided.".to_string());

            return Err(err);
        }

        if entity.is_empty() && !account.is_empty() {
            let err = VaultError::Error("Entity must be provided when an account is set.".to_string());

            return Err(err);
        }

        if !account.is_empty() {
            let keeper_account = self.vault.get_account(&entity, &account)?;
            let locker = Locker::from_encrypted(&keeper_account);

            return Ok(Resolve::Read(locker.decrypt()));
        }

        let entity = self.vault.get_entity(&entity)?;
        let accounts: Vec<(String, String)> = entity.iter()
            .map(|(acc, pass)| {
                let account = Locker::from_encrypted(&acc);
                let password = Locker::from_encrypted(&pass);

                (account.decrypt(), password.decrypt())
            })
            .collect();

        Ok(Resolve::Find(accounts))
    }

    pub fn list(&mut self) -> VaultResult<Resolve> {
        let entities = self.vault.list()?;
        let list: Vec<String> = entities.iter()
            .map(|entity| {
                let locker = Locker::from_encrypted(&entity);

                locker.decrypt()
            })
            .collect();

        Ok(Resolve::List(list))
    }

    pub fn read(&mut self, args: Args) -> VaultResult<Resolve> {
        let Args {
            entity,
            account,
            ..
        } = args;

        if entity.is_empty() && !account.is_empty() {
            let err = VaultError::Error("Entity and account must be provided".to_string());

            return Err(err);
        }

        let keeper_account = self.vault.get_account(&entity, &account)?;
        let Distinguished { iv, key, dat, .. } = keeper_account.distinguish();
        let locker = Locker::from(iv, key, dat);
        let decrypted = locker.decrypt();

        Ok(Resolve::Read(decrypted))
    }

    pub fn remove(&mut self, args: Args ) -> VaultResult<Resolve> {
        let Args {
            entity,
            account,
            ..
        } = args;

        if entity.is_empty() && !account.is_empty() {
            let err = VaultError::Error("Entity must be provided when an account is set".to_string());

            return Err(err);
        }

        if !entity.is_empty() && account.is_empty() {
            self.vault.remove_entity(&entity)?;

            return Ok(Resolve::Done);
        }

        self.vault.remove_account(&entity, &account)?;

        Ok(Resolve::Done)
    }
}

#[cfg(test)]
mod keeper {
    use super::*;

    use mocks::Setup;

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
            let msg = format!("Could not remove {} {:?} in `lib.rs` test", remove, path);

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
                let (config, locker) = this.as_path_buf();
                let keeper = Keeper::new(config, locker);

                assert!(keeper.is_ok());
            },
        }; 
    }

    #[test]
    fn add_entity() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let mut dump = this.dump_path();
                let (config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(config, locker.clone()).unwrap();

                let entity = Some("add_entity_1");
                let account = None;
                let password = None;

                let args = Args::new(
                    entity,
                    account,
                    password
                );

                let entity_path = args.entity.path();

                dump.push(locker);
                dump.push(entity_path);

                let add = keeper.add(args);

                assert!(add.is_ok());
                assert!(dump.exists());
            }
        };
    }

    #[test]
    fn add_account_without_entity() {
        Setup {
            paths: Vec::new(), 
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(config, locker).unwrap();

                let args = Args::new(
                    None,
                    Some("account"),
                    None
                );

                let result = keeper.add(args);

                assert_eq!(result.is_err(), true);
            }
        };
    }

    #[test]
    fn add_account() {
        Setup {
            paths: Vec::new(), 
            after_each: &after_each,
            test: &|this| {
                let mut dump = this.dump_path();
                let (config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(config, locker.clone()).unwrap();

                let entity = Some("add_account_1");
                let account = Some("add_account_2");
                let password = None;

                let args = Args::new(
                    entity,
                    account,
                    password
                );

                let entity_path = args.entity.path();
                let account_path = args.account.path();

                dump.push(locker);
                dump.push(entity_path);
                dump.push(account_path);

                let add = keeper.add(args);
                
                assert!(add.is_ok());
                assert!(dump.exists());
            }
        };
    }

    #[test]
    fn add_password() {
        Setup {
            paths: Vec::new(), 
            after_each: &after_each,
            test: &|this| {
                let mut dump = this.dump_path();
                let (config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(config, locker.clone()).unwrap();
   
                let args = Args::new(
                    Some("add_password_1"),
                    Some("add_password_2"),
                    Some("password") 
                );

                let entity_path = args.entity.path();
                let account_path = args.account.path();
                let password_path = args.password.path();

                dump.push(locker);
                dump.push(entity_path);
                dump.push(account_path);

                let add = keeper.add(args);

                assert!(add.is_ok());
                assert!(dump.exists());
                assert!(dump.is_dir());

                dump.push(password_path);

                assert!(dump.exists());
                assert!(dump.is_file());
            }
        };
    }

    #[test]
    fn list() {
        Setup {
            paths: Vec::new(), 
            after_each: &after_each,
            test: &|this| {
                let mut dump = this.dump_path();
                let (config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(config, locker.clone()).unwrap();

                let args = Args::new(
                    Some("find_entity_1"),
                    None,
                    None 
                );

                let entity = args.entity.path();

                dump.push(locker);
                dump.push(entity);

                let add = keeper.add(args.clone());

                assert!(add.is_ok());

                let result = keeper.list().unwrap();
                let locker = Locker::from_encrypted(&args.entity);
               
                assert!(dump.exists());
                assert_eq!(result.to_list(), vec![locker.decrypt()]);
            }
        };
    }

    #[test]
    fn find_entity() {
        Setup {
            paths: Vec::new(), 
            after_each: &after_each,
            test: &|this| {
                let mut dump = this.dump_path();
                let (config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(config, locker.clone()).unwrap();

                let args = Args::new(
                    Some("find_entity_1"),
                    None,
                    None 
                );

                let entity = args.entity.path();

                dump.push(locker);
                dump.push(entity);

                let add = keeper.add(args.clone());

                assert!(add.is_ok());

                let result = keeper.find(args).unwrap();
               
                assert!(dump.exists());
                assert_eq!(result.to_vec().len(), 0);
            }
        };
    }

    #[test]
    fn find_entity_with_accounts() {
        Setup {
            paths: Vec::new(), 
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(config, locker).unwrap();

                let args_add = Args::new(
                    Some("find_entity_account_1"),
                    Some("find_entity_account_2"),
                    None 
                );

                let add = keeper.add(args_add.clone());

                assert!(add.is_ok());

                let args_find = Args::new(
                    Some("find_entity_account_1"),
                    None,
                    None 
                );

                let result = keeper.find(args_find).unwrap();
                let found = result.to_vec();

                assert_eq!(found.len(), 1);
                assert_eq!(found[0], (String::from("find_entity_account_2"), String::new()));
            }
        };
    }

    #[test]
    fn read_account_password() {
        Setup {
            paths: Vec::new(), 
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let mut dump = this.dump_path();
                let mut keeper = Keeper::new(config, locker.clone()).unwrap();

                let entity = Some("read_account_password");
                let account = Some("read_account_password");
                let password = Some("read_account_password");

                let args = Args::new(
                    entity,
                    account,
                    password
                );

                let entity_path = args.entity.path();
                let account_path = args.account.path();
                let password_path = args.password.path();

                dump.push(locker);
                dump.push(entity_path);
                dump.push(account_path);
                dump.push(password_path);
                
                let add = keeper.add(args.clone()); 

                assert!(add.is_ok());

                let result = keeper.read(args).unwrap().to_string();

                assert_eq!(result, "read_account_password");
            }
        };
    }
    
    #[test]
    fn find_without_params_returns_error() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(config, locker).unwrap();
               
                let args = Args::new(
                    None,
                    None,
                    None
                );

                let operation = keeper.find(args);
                
                assert!(operation.is_err());
                assert_eq!(
                    operation.unwrap_err().to_str(), 
                    "Neither entity or account provided."
                );
            }
        };
    }

    #[test]
    fn remove() {
        Setup {
            paths: Vec::new(), 
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(config, locker).unwrap();
                
                let entity = Some("entity");
                let account = Some("account");

                let args = Args::new(
                    entity,
                    account,
                    None 
                );

                let add = keeper.add(args.clone());

                assert!(add.is_ok());

                let remove = keeper.remove(args.clone());

                assert!(remove.is_ok());
              
                let result = keeper.find(args);

                assert!(result.is_err());
            }
        };
    }

    #[test]
    fn remove_only_with_entity() {
        Setup {
            paths: Vec::new(), 
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(config, locker).unwrap();
                
                let entity = Some("entity");
                let args_add = Args::new(
                    entity,
                    None,
                    None 
                );

                let add = keeper.add(args_add);

                assert!(add.is_ok());
                
                let entity = Some("entity");
                let args_remove = Args::new(
                    entity,
                    None,
                    None
                );

                let remove = keeper.remove(args_remove.clone());

                assert!(remove.is_ok());
               
                let result = keeper.find(args_remove);

                assert!(result.is_err());
            }
        };
    }

    #[test]
    fn remove_only_with_account() {
        Setup {
            paths: Vec::new(), 
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf(); 
                let mut keeper = Keeper::new(config, locker).unwrap();

                let result = catch_unwind(AssertUnwindSafe(|| {
                    let args = Args::new(
                        None,
                        Some("account"),
                        None
                    );

                    keeper.remove(args).unwrap();
                }));

                assert_eq!(result.is_err(), true);
            }
        };
    }
}
