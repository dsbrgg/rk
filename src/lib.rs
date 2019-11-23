mod setup;
mod locker;
mod managers;

use std::io;
use std::path::{PathBuf};

use locker::Locker;

use managers::traits::Manager;
use managers::dir_manager::DirManager;
use managers::file_manager::FileManager;

#[derive(Debug)]
pub enum Resolve {
    Add,
    Find(Vec<String>),
    Remove
}

impl Resolve {
    pub fn to_vec(self) -> Vec<String> {
        if let Resolve::Find(vec) = self { return vec; }
        panic!("to_vec should be called on a Resolve::Find only");
    }
}

pub struct Keeper {
    // lock: Locker,
    files: FileManager,
    directories: DirManager,
}

impl Keeper {
    pub fn new(config: PathBuf, locker: PathBuf) -> Keeper {
        // NOTE: just for future reference
        // let mut config_path = dirs::home_dir().unwrap();
        // let mut locker_path = dirs::home_dir().unwrap();

        // let lock = Locker::new();
        let directories = DirManager::new(&config, &locker);
        let files = FileManager::new(&config, &locker);

        Keeper { files, directories }
    }

    // TODO: this stil has to handle when an entity/account
    // is already created. so it doesn't erases previous
    // registers. 
    //
    // It also needs to hash/encrypt values appropriately.
    //
    // It also needs to create an index with the 
    // entities and accounts hashes for further searching
    // operations
    pub fn add(
        &mut self, 
        entity: Option<&str>, 
        account: Option<&str>, 
        password: Option<&str>
    ) -> io::Result<Resolve> {
        let mut paths = Vec::new();

        if entity.is_none() && account.is_some() {
            return Err(io::Error::new(
                io::ErrorKind::Other, "Entity must be provided when an account is set."
            ));
        }

        if let Some(e) = entity { 
            self.directories.create(e)?; 
        }

        if let Some(a) = account {
            paths.push(a);

            let e = entity.unwrap();

            self.directories.create(
                &DirManager::append_paths(e, &paths)
            )?; 
        }

        if let Some(p) = password {
            paths.push(p);

            let e = entity.unwrap();

            self.files.create(
                &FileManager::append_paths(e, &paths)
            )?; 
        }

        Ok(Resolve::Add)
    }

    pub fn find(
        &mut self, 
        entity: Option<&str>, 
        account: Option<&str> 
    ) -> io::Result<Resolve> {
        if entity.is_none() && account.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::Other, "Neither entity or account provided."
            ));
        }

        let e = entity.unwrap_or("");
        let a = account.unwrap_or("");
        let ap = DirManager::append_path(&e, &a);

        let registers = self.directories.read(&ap)?;

        Ok(Resolve::Find(registers))
    }

    pub fn remove(
        &mut self,
        entity: Option<&str>,
        account: Option<&str>
    ) -> io::Result<Resolve> {
        if entity.is_none() && account.is_some() {
            return Err(io::Error::new(
                io::ErrorKind::Other, "Entity must be provided when an account is set."
            ));
        }

        let e = entity.unwrap_or("");
        let a = account.unwrap_or("");

        let path = DirManager::append_path(&e, &a);         
        
        self.directories.remove(&path)?;

        Ok(Resolve::Remove)
    }

    // pub fn new() -> Keeper<'l, 'd, 'f> {
    //     let mut config_path = dirs::home_dir().unwrap();
    //     let mut locker_path = dirs::home_dir().unwrap();

    //     locker_path.push(".rk");
    //     config_path.push(".config/rk");

    //     Keeper { 
    //         lock: Locker::new(),
    //         files: FileManager::new(config_path, locker_path),
    //         directories: DirManager::new(config_path, locker_path),
    //     }
    // }

    // // TODO: read file + decrypt and find account
    // fn find(&self) {
    //     let mut file = self.read_locker();
    // }

    // fn handle_input() -> String {
    //     let mut input = String::new();

    //     stdin()
    //         .read_line(&mut input)
    //         .expect("Failed to read user input");

    //     input
    // }

    // pub fn read_locker(&self) -> String {
    //     let mut contents = String::new();
    //     
    //     let locker_path = self.files.get_locker_path();
    //     let mut file = FileManager::open(locker_path, None, FileAction::Read);

    //     match file.read_to_string(&mut contents) {
    //         Err(why) => panic!("Couldn't open file to read: {}", why),
    //         Ok(_) => println!("\n::: Success reading locker :::\n"),
    //     };
    //    
    //     contents
    // }

    // pub fn write_on(&self, contents: &mut String) {
    //     let mut input = Keeper::handle_input();
    //     let encrypted = self.lock.input_encryption(&mut input);

    //     // TODO: decryption working, have to define a way
    //     // to maintain key and iv to decrypt further values
    //     self.lock.input_decryption(&encrypted);


    //     let locker_path = self.files.get_locker_path();
    //     let mut file = FileManager::open(locker_path, None, FileAction::Write);
    //     
    //     let new_register = format!("{}", encrypted.trim());

    //     contents.push_str(
    //         new_register.as_str()
    //     );

    //     match file.write_all(contents.as_bytes()) {
    //         Err(why) => panic!("Couldn't write to locker: {}", why),
    //         Ok(_) => println!("\n::: Success writing to locker :::\n")
    //     } 
    // }

    // pub fn add_account(&mut self) {
    //     let input = Keeper::handle_input();
    //     let hash = self.lock.hash(input);

    //     self.directories.create(&hash[..]);
    // }
}

#[cfg(test)]
mod tests_keeper {
    use super::*;

    use setup::setup::Setup;

    use std::path::Path;
    use std::fs::remove_dir_all;
    use std::panic::{AssertUnwindSafe, catch_unwind};

    fn after_each(this: &mut Setup) {
        for path in this.paths.iter() {
            let exists = Path::new(path).exists();

            if exists {
                remove_dir_all(path)
                    .expect("Could not remove file in `lib.rs` test");
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
                Keeper::new(config, locker);
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
                
                let mut keeper = Keeper::new(config, locker);
      
                dump.push("add_entity");
                let e = this.add_to_paths(&dump);

                let entity = Some(e.as_str());
                let account = None;
                let password = None;

                keeper.add(entity, account, password);

                assert!(dump.as_path().exists());
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
                
                let mut keeper = Keeper::new(config, locker);

                let result = catch_unwind(AssertUnwindSafe(|| {
                    keeper.add(
                        None, 
                        Some("account"),
                        None
                    ).unwrap();
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
                let (config, locker) = this.as_path_buf();
                
                let mut keeper = Keeper::new(config, locker);

                dump.push("add_account_1");
                let e = this.add_to_paths(&dump);

                dump.push("add_account_2");
                let a = this.add_to_paths(&dump);

                let entity = Some(e.as_str());
                let account = Some(a.as_str());
                let password = None;

                keeper.add(entity, account, password);

                assert!(dump.as_path().exists());
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
                
                let mut keeper = Keeper::new(config, locker);

                dump.push("add_password_1");
                let e = this.add_to_paths(&dump);

                dump.push("add_password_2");
                let a = this.add_to_paths(&dump);

                let entity = Some(e.as_str());
                let account = Some(a.as_str());
    
                keeper.add(entity, account, Some("password"));

                dump.push("password");

                assert!(dump.as_path().exists());
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
                
                let mut keeper = Keeper::new(config, locker);

                dump.push("find_entity_1");
                let e = this.add_to_paths(&dump);
                let entity = Some(e.as_str());
    
                keeper.add(entity, None, None);

                let result = keeper.find(entity, None).unwrap();
                
                assert_eq!(result.to_vec().len(), 0);
            }
        };
    }

    #[test]
    fn find_entity_accounts() {
        Setup {
            paths: Vec::new(), 
            after_each: &after_each,
            test: &|this| {
                let mut dump = this.dump_path();
                let (config, locker) = this.as_path_buf();
                
                let mut keeper = Keeper::new(config, locker);

                dump.push("find_entity_account_1");
                let e = this.add_to_paths(&dump);
                let entity = Some(e.as_str());
   
                dump.push("find_entity_account_2");
                let a = this.add_to_paths(&dump);
                let account = Some(a.as_str());

                keeper.add(entity, account, None);

                let result = keeper.find(entity, None).unwrap();
    
                assert_eq!(result.to_vec().len(), 1);
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
                
                let mut keeper = Keeper::new(config, locker);
                
                let operation = keeper.find(None, None);
                
                assert!(operation.is_err());
                assert_eq!(
                    operation.unwrap_err().to_string(), 
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
                let mut keeper = Keeper::new(config, locker);
                
                let entity = Some("entity");
                let account = Some("account");

                keeper.add(
                    entity, 
                    account, 
                    None
                );

                keeper.remove(
                    entity, 
                    account
                ); 
               
                let result = catch_unwind(AssertUnwindSafe(|| {
                    keeper.find(entity, account).unwrap();
                }));

                assert_eq!(result.is_err(), true);
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
                let mut keeper = Keeper::new(config, locker);
                
                let entity = Some("entity");

                keeper.add(
                    entity, 
                    None,
                    None
                );

                keeper.remove(
                    entity,
                    None
                ); 
               
                let result = catch_unwind(AssertUnwindSafe(|| {
                    keeper.find(entity, None).unwrap();
                }));

                assert_eq!(result.is_err(), true);
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
                let mut keeper = Keeper::new(config, locker);

                let result = catch_unwind(AssertUnwindSafe(|| {
                    keeper.remove(None, Some("account")).unwrap();
                }));

                assert_eq!(result.is_err(), true);
            }
        };
    }
}
