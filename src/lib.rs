mod args;
mod locker;
mod managers;
mod mocks;
mod yaml;

use std::io;
use std::path::{PathBuf};

pub use args::{Args, Arg, Mode};
use managers::Manager;
use managers::DirManager;
use managers::FileManager;
use locker::{Locker, Distinguished, Bytes};
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
    // NOTE: this is readlly bad
    pub index: Index,
    files: FileManager,
    directories: DirManager,
}

impl Keeper {
    pub fn new(index: PathBuf, config: PathBuf, locker: PathBuf) -> Keeper {
        let mut directories = DirManager::new(&config, &locker);
        let mut files = FileManager::new(&index, &config, &locker);
        
        let index_path = files.read_index().unwrap();
        let mut index = Index::from_yaml(&index_path).unwrap();

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
       
        for arg in args {
            let Arg {
                mut path,
                keys,
                hash,
                parent_hash,
                is_dir
            } = arg;

            let Distinguished {
                iv,
                key,
                dat
            } = keys;

            if let Some(parent) = parent_hash {
                self.index.add(parent, Some(hash));
            } else {
                self.index.add(hash, None);
            }

            let pa = format!("{}{}", iv, key);
            let yaml = self.index.to_yaml().unwrap();

            if is_dir {
                self.directories
                    .create_locker(path.to_str().unwrap())
                    .expect("Unable to create locker directory"); 
 
                // TODO: need another way to persist data to decrypt dirs

                path.push("meta");

                self.files
                    .create_locker(path.to_str().unwrap())
                    .expect("Unable to create meta file");

                self.files
                    .write_locker(path.to_str().unwrap(), &pa)
                    .expect("Unable to write to meta file");

                self.files
                    .write_index(&yaml)
                    .expect("Unable to write to index file");
            } else {
                self.files
                    .create_locker(path.to_str().unwrap())
                    .expect("Unable to create locker file");

                self.files
                    .write_locker(path.to_str().unwrap(), &pa)
                    .expect("Unable to write to locker file");
            }
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
       
        let ehash = if entity.is_empty() != true { Some(entity.get_hash()) } else { None };
        let ahash = if account.is_empty() != true { Some(account.get_hash()) } else { None };

        let index = self.index.find(ehash, ahash);

        if index.is_some() {
            let path = DirManager::append_path(
                &entity.get_encrypted(), 
                &account.get_encrypted()
            );
            println!("{:?}", path);

            let registers = self.directories.read_locker(&path)?;
         
            return Ok(Resolve::Find(registers));
        } 

        Ok(Resolve::Find(vec![]))
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

        let path = DirManager::append_path(
            &entity.get_encrypted(), 
            &account.get_encrypted()
        ); 
        
        self.directories.remove_locker(&path)?;

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

                let entity = Some("add_entity_1");
                let account = None;
                let password = None;

                let args = Args::new(
                    Mode::Add,
                    entity,
                    account,
                    password
                );

                dump.push(locker);
                dump.push(args.entity.get_encrypted().clone());

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
                    Mode::Add,
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
                let mut keeper = Keeper::new(index, config, locker.clone());

                let entity = Some("add_account_1");
                let account = Some("add_account_2");
                let password = None;

                let args = Args::new(
                    Mode::Add,
                    entity,
                    account,
                    password
                );

                dump.push(locker);
                dump.push(args.entity.get_encrypted().clone());
                dump.push(args.account.get_encrypted().clone());

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
                let mut keeper = Keeper::new(index, config, locker.clone());
   
                let args = Args::new(
                    Mode::Add,
                    Some("add_password_1"),
                    Some("add_password_2"),
                    Some("password") 
                );

                let password = args.password.get_encrypted().clone();

                dump.push(locker);
                dump.push(args.entity.get_encrypted().clone());
                dump.push(args.account.get_encrypted().clone());

                keeper.add(args);

                assert!(dump.exists());
                assert!(dump.is_dir());

                dump.push(password);

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
                let mut keeper = Keeper::new(index, config, locker.clone());

                let args = Args::new(
                    Mode::Add,
                    Some("find_entity_1"),
                    None,
                    None 
                );

                dump.push(locker);
                dump.push(args.entity.get_encrypted().clone());

                keeper.add(args.clone());

                let result = keeper.find(args).unwrap();
               
                assert!(dump.exists());
                assert_eq!(result.to_vec().len(), 1);
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

                let args_add = Args::new(
                    Mode::Add,
                    Some("find_entity_account_1"),
                    Some("find_entity_account_2"),
                    None 
                );

                keeper.add(args_add);

                let args_find = Args::new(
                    Mode::Find,
                    Some("find_entity_account_1"),
                    None,
                    None 
                );

                let result = keeper.find(args_find).unwrap();

                assert_eq!(result.to_vec().len(), 2);
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
                    Mode::Add,
                    entity,
                    account,
                    password
                );

                let password_encrypted = args.password.get_encrypted().clone();
                let Distinguished { dat, .. } = Locker::distinguish(&password_encrypted);

                dump.push(locker);
                dump.push(entity_hash);
                dump.push(account_hash);
                dump.push(dat);
                
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
                    Mode::Find,
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
                    Mode::Add,
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
                let args_add = Args::new(
                    Mode::Add,
                    entity,
                    None,
                    None 
                );

                keeper.add(args_add);
                
                let entity = Some("entity");
                let args_remove = Args::new(
                    Mode::Remove,
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
                let mut keeper = Keeper::new(index, config, locker);

                let result = catch_unwind(AssertUnwindSafe(|| {
                    let args = Args::new(
                        Mode::Remove,
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
