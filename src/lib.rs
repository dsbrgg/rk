mod args;
mod locker;
mod managers;
mod mocks;
mod yaml;

use std::io;
use std::path::{PathBuf};

pub use args::Args;
use managers::Manager;
use managers::DirManager;
use managers::FileManager;
use locker::{Locker, Bytes};
use yaml::Index;

#[derive(Debug, PartialEq)]
pub enum Resolve {
    Add,
    Read(String),
    Find(Vec<PathBuf>),
    Remove
}

impl Resolve {
    pub fn to_vec(self) -> Vec<PathBuf> {
        if let Resolve::Find(vec) = self { return vec; }
        panic!("to_vec should be called on a Resolve::Find only");
    }

    pub fn to_string(self) -> String {
        if let Resolve::Read(string) = self { return string; }
        panic!("to_string should be called on a Resolve::Read only");
    }
}

pub struct Keeper {
    index: Index,
    files: FileManager,
    directories: DirManager,
}

impl Keeper {
    pub fn new(index: PathBuf, config: PathBuf, locker: PathBuf) -> Keeper {
        let index = Index::from_pathbuf(index).unwrap();
        let files = FileManager::new(&config, &locker);
        let directories = DirManager::new(&config, &locker);

        Keeper { 
            index, 
            files, 
            directories 
        }
    }

    // TODO: this stil has to handle when an entity/account
    // is already created. so it doesn't erases previous
    // registers. 
    
    pub fn add(&mut self, args: Args) -> io::Result<Resolve> {
        if args.has_account() && !args.has_entity() {
            return Err(io::Error::new(
                io::ErrorKind::Other, "Entity must be provided when an account is set."
            ));
        }
       
        let has_pass = args.has_all();

        let Args {
            entity,
            account,
            password
        } = args; 

        let mut p = String::new();
        let mut pa = String::new();
        let mut path = PathBuf::new(); 

        path.push(&entity);
        path.push(&account); 
        
        if has_pass {
            let (i, k, d) = Locker::distinguish(&password);
            p = d; pa = format!("{}{}", i, k);
            path.push(&p);
        } 

        let mut total_components = 1;
        let mut full_path = PathBuf::new();

        for component in path.iter() {
            let path_string = component
                .to_str()
                .unwrap();
            
            full_path.push(path_string);

            let path_string = full_path
                .to_str()
                .unwrap();

            if total_components <= 2 {
                self.directories
                    .create_locker(path_string)
                    .expect("Unable to create locker directory");
            } else {
                self.files
                    .create_locker(&path_string)
                    .expect("Unable to create locker file");

                self.files
                    .write_locker(&path_string, &pa)
                    .expect("Unable to write to locker file");
            }

            total_components += 1;
        }

        Ok(Resolve::Add)
    }

    pub fn find(&mut self, args: Args) -> io::Result<Resolve> {
        if !args.has_entity() && !args.has_account() {
            return Err(io::Error::new(
                io::ErrorKind::Other, "Neither entity or account provided."
            ));
        }
        
        let Args {
            entity,
            account,
            ..
        } = args; 
        
        let path = DirManager::append_path(&entity, &account);
        let registers = self.directories.read_locker(&path)?;
     
        Ok(Resolve::Find(registers))
    }

    pub fn read(&mut self, path: PathBuf) -> io::Result<Resolve> {
        // TODO: will have to rethink directories
        // hash -> encrypt
        
        let path_to_str = FileManager::pb_to_str(&path);
        let content = self.files.read_locker(&path_to_str)?;
      
        // TODO: this will have to be disinguished
        // when iv and key can be longer than 16 bytes
        let iv = format!("0x{}", &content[..32]);
        let key = format!("0x{}", &content[32..]);
        
        let dat = path.file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let locker = Locker::from(iv, key, dat);
        let decrypted = locker.decrypt();
        
        Ok(Resolve::Read(decrypted))
    }

    pub fn remove(&mut self, args: Args ) -> io::Result<Resolve> {
        let Args {
            entity,
            account,
            ..
        } = args;

        if entity.is_empty() && account.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other, "Entity must be provided when an account is set."
            ));
        }

        let path = DirManager::append_path(&entity, &account); 
        
        self.directories.remove_locker(&path)?;

        Ok(Resolve::Remove)
    }
}

#[cfg(test)]
mod keeper {
    use super::*;

    use mocks::Setup;
    use locker::Locker;

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
                Keeper::new(index, config, locker);
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
                
                let mut keeper = Keeper::new(index, config, locker.clone());
                let mut locker_instance = Locker::new();

                let entity = Some("add_entity_1");
                let account = None;
                let password = None;

                let args = Args::new(
                    entity,
                    account,
                    password
                );

                dump.push(locker);
                dump.push(
                    locker_instance.hash("add_entity_1")
                );

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
                let mut keeper = Keeper::new(index, config, locker);

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
                let (index, config, locker) = this.as_path_buf();
                
                let mut dump = this.dump_path();
                let mut keeper = Keeper::new(index, config, locker.clone());
                let mut locker_instance = Locker::new();

                let entity = Some("add_account_1");
                let account = Some("add_account_2");
                let password = None;

                let args = Args::new(
                    entity,
                    account,
                    password
                );

                let entity_hash = locker_instance.hash("add_account_1");
                let account_hash = locker_instance.hash("add_account_2");

                dump.push(locker);
                dump.push(entity_hash);
                dump.push(account_hash);

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
                let (index, config, locker) = this.as_path_buf();

                let mut dump = this.dump_path();
                let mut keeper = Keeper::new(index, config, locker.clone());
                let mut locker_instance = Locker::new();

                let entity = Some("add_password_1");
                let account = Some("add_password_2");
   
                let args = Args::new(
                    entity,
                    account,
                    Some("password") 
                );

                let entity_hash = locker_instance.hash("add_password_1");
                let account_hash = locker_instance.hash("add_password_2");

                dump.push(locker);
                dump.push(entity_hash);
                dump.push(account_hash);

                keeper.add(args);

                assert!(dump.exists());
                assert!(dump.is_dir());

                let account_dir = dump.read_dir().expect("Account directory not created!");

                for entry in account_dir {
                    if let Ok(entry) = entry {
                        assert!(entry.path().exists());
                    }
                } 
            }
        };
    }

    #[test]
    fn find_entity() {
        Setup {
            paths: Vec::new(), 
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf();

                let mut dump = this.dump_path();
                let mut keeper = Keeper::new(index, config, locker.clone());
                let mut locker_instance = Locker::new();

                let entity = Some("find_entity_1");
                let entity_hash = locker_instance.hash("find_entity_1");
  
                let args = Args::new(
                    entity,
                    None,
                    None 
                );

                dump.push(locker);
                dump.push(entity_hash);

                keeper.add(args.clone());

                let result = keeper.find(args).unwrap();
               
                assert!(dump.exists());
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
                let (index, config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(index, config, locker);

                let entity = Some("find_entity_account_1");
                let account = Some("find_entity_account_2");

                let args_add = Args::new(
                    entity,
                    account,
                    None 
                );

                keeper.add(args_add);

                let args_find = Args::new(
                    entity,
                    None,
                    None 
                );

                let result = keeper.find(args_find).unwrap();
    
                assert_eq!(result.to_vec().len(), 1);
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
                let mut locker_instance = Locker::new();
                let mut keeper = Keeper::new(index, config, locker.clone());

                let entity_hash = locker_instance.hash("read_account_password");
                let account_hash = locker_instance.hash("read_account_password");

                let entity = Some("read_account_password");
                let account = Some("read_account_password");
                let password = Some("read_account_password");

                let args = Args::new(
                    entity,
                    account,
                    password
                );

                let password_encrypted = args.password.clone();
                let values = Locker::distinguish(&password_encrypted);

                dump.push(locker);
                dump.push(entity_hash);
                dump.push(account_hash);
                dump.push(values.2);
                
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
                let mut keeper = Keeper::new(index, config, locker);
               
                let args = Args::new(
                    None,
                    None,
                    None
                );

                let operation = keeper.find(args);
                
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
                let (index, config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(index, config, locker);
                
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
                let mut keeper = Keeper::new(index, config, locker);
                
                let entity = Some("entity");

                let args = Args::new(
                    entity,
                    None,
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
    fn remove_only_with_account() {
        Setup {
            paths: Vec::new(), 
            after_each: &after_each,
            test: &|this| {
                let (index, config, locker) = this.as_path_buf(); 
                let mut keeper = Keeper::new(index, config, locker);

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
