mod args;
mod locker;
mod managers;
mod mocks;
mod vault;
mod yaml;

use std::path::PathBuf;

pub use args::Args;
pub use vault::{Vault, VaultError};
pub use locker::{Locker, Encrypted};

#[derive(Debug, PartialEq)]
pub enum Resolve {
    Add,
    Read(String),
    Find(Vec<Encrypted>),
    Remove
}

impl Resolve {
    pub fn to_vec(self) -> Vec<Encrypted> {
        if let Resolve::Find(vec) = self { return vec; }
        panic!("to_vec should be called on a Resolve::Find only");
    }

    pub fn to_string(self) -> String {
        if let Resolve::Read(string) = self { return string; }
        panic!("to_string should be called on a Resolve::Read only");
    }
}

pub struct Keeper { vault: Vault }

impl Keeper {
    pub fn new(index: PathBuf, config: PathBuf, locker: PathBuf) -> Result<Keeper, VaultError> {
        let vault = Vault::new(&index, &config, &locker)?;
        let keeper = Keeper { vault };

        Ok(keeper)
    }

    pub fn add(&mut self, args: Args) -> Result<Resolve, VaultError> {
        if args.has_account() && !args.has_entity() {
            let err = VaultError::Error("Entity must be provided when an account is set.".to_string());

            return Err(err);
        }
       
        if args.has_entity() { self.vault.set_entity(&args.entity)?; }
        if args.has_account() { self.vault.set_account(&args.entity, &args.account)?; }
        if args.has_password() { self.vault.set_password(&args.entity, &args.account, &args.password)?; }

        Ok(Resolve::Add)
    }

    pub fn find(&mut self, args: Args) -> Result<Resolve, VaultError> {
        if !args.has_entity() && !args.has_account() {
            let err = VaultError::Error("Neither entity or account provided.".to_string());

            return Err(err);
        }

        if !args.has_entity() && args.has_account() {
            let err = VaultError::Error("Entity must be provided when an account is set.".to_string());

            return Err(err);
        }

        if args.has_account() {
            let account = self.vault.get_account(&args.entity, &args.account)?;

            return Ok(Resolve::Read(account.path()));
        }

        let entity = self.vault.get_entity(&args.entity)?;
        let accounts = entity.keys().cloned().collect();

        Ok(Resolve::Find(accounts))
    }

    pub fn read(&mut self, path: PathBuf) -> Result<Resolve, VaultError> {
        // let dat = path.file_name()
        //     .unwrap()
        //     .to_str()
        //     .unwrap()
        //     .to_string();

        // let locker = Locker::from(iv, key, dat);
        // let decrypted = locker.decrypt();
        
        Ok(Resolve::Add)
    }

    pub fn remove(&mut self, args: Args ) -> Result<Resolve, VaultError> {
        let Args {
            entity,
            account,
            ..
        } = args;

        if entity.is_empty() && !account.is_empty() {
            let err = VaultError::Error("Entity must be provided when an account is set.".to_string());

            return Err(err);
        }

        Ok(Resolve::Remove)
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
                let (index, config, locker) = this.as_path_buf();
                let keeper = Keeper::new(index, config, locker);

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
                let (index, config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(index, config, locker.clone()).unwrap();

                let entity = Some("add_entity_1");
                let account = None;
                let password = None;

                let args = Args::new(
                    entity,
                    account,
                    password
                );

                println!("{:?}", args);

                let entity_path = args.entity.path();

                dump.push(locker);
                dump.push(entity_path);

                keeper.add(args);

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
                let (index, config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(index, config, locker).unwrap();

                let args = Args::new(
                    None,
                    Some("account"),
                    None
                );

                let result = catch_unwind(AssertUnwindSafe(|| {
                    keeper.add(args).unwrap();
                }));

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
                let (index, config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(index, config, locker.clone()).unwrap();

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

                keeper.add(args);
                
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
                let (index, config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(index, config, locker.clone()).unwrap();
   
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

                keeper.add(args);

                assert!(dump.exists());
                assert!(dump.is_dir());

                dump.push(password_path);

                assert!(dump.exists());
                assert!(dump.is_file());
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
                let (index, config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(index, config, locker.clone()).unwrap();

                let args = Args::new(
                    Some("find_entity_1"),
                    None,
                    None 
                );

                let entity = args.entity.path();

                dump.push(locker);
                dump.push(entity);

                keeper.add(args.clone());

                let result = keeper.find(args).unwrap();
               
                assert!(dump.exists());
                assert_eq!(result.to_vec().len(), 1);
            }
        };
    }

    #[test]
    fn find_entity_with_accounts() {
        Setup {
            paths: Vec::new(), 
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(index, config, locker).unwrap();

                let args_add = Args::new(
                    Some("find_entity_account_1"),
                    Some("find_entity_account_2"),
                    None 
                );

                keeper.add(args_add.clone());

                let args_find = Args::new(
                    Some("find_entity_account_1"),
                    None,
                    None 
                );

                let result = keeper.find(args_find).unwrap();
                let found = result.to_vec();

                assert_eq!(found.len(), 1);
                assert_eq!(found[0], args_add.account);
            }
        };
    }

    #[test]
    fn read_account_password() {
        Setup {
            paths: Vec::new(), 
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf();
                let mut dump = this.dump_path();
                let mut keeper = Keeper::new(index, config, locker.clone()).unwrap();

                let entity = Some("read_account_password");
                let account = Some("read_account_password");
                let password = Some("read_account_password");

                let args = Args::new(
                    entity,
                    account,
                    password
                );

                let entity_path= args.entity.path();
                let account_path= args.account.path();
                let password_path= args.password.path();

                dump.push(locker);
                dump.push(entity_path);
                dump.push(account_path);
                dump.push(password_path);
                
                keeper.add(args); 

                let result = keeper.read(dump).unwrap().to_string();

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
                let (index, config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(index, config, locker).unwrap();
               
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
                let (index, config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(index, config, locker).unwrap();
                
                let entity = Some("entity");
                let account = Some("account");

                let args = Args::new(
                    entity,
                    account,
                    None 
                );

                keeper.add(args.clone());
                keeper.remove(args.clone());
              
                let result = keeper.find(args).unwrap();

                assert_eq!(result.to_vec().len(), 0);
            }
        };
    }

    #[test]
    fn remove_only_with_entity() {
        Setup {
            paths: Vec::new(), 
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(index, config, locker).unwrap();
                
                let entity = Some("entity");
                let args_add = Args::new(
                    entity,
                    None,
                    None 
                );

                keeper.add(args_add);
                
                let entity = Some("entity");
                let args_remove = Args::new(
                    entity,
                    None,
                    None
                );

                keeper.remove(args_remove.clone());
               
                let result = keeper.find(args_remove).unwrap();

                assert_eq!(result.to_vec().len(), 0);
            }
        };
    }

    #[test]
    fn remove_only_with_account() {
        Setup {
            paths: Vec::new(), 
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf(); 
                let mut keeper = Keeper::new(index, config, locker).unwrap();

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
