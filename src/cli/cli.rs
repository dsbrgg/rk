use clap::{ArgMatches};

use std::io;
use std::path::{PathBuf};

use rk::Args;
use rk::{Resolve, Keeper};
use crate::cli::select;

struct Params<'p> { 
    entity: Option<&'p str>,
    account: Option<&'p str>,
    password: Option<&'p str>
}

pub struct CLI { keeper: Keeper }

impl<'p> CLI {
    pub fn start(config: PathBuf, locker: PathBuf) -> CLI {
        CLI {
            keeper: Keeper::new(config, locker)
        }
    }

    pub fn operation(&mut self, args: ArgMatches) -> io::Result<Resolve> {
        match args.subcommand() {
            ("add", Some(add)) => { self.handle_add(add) },
            ("find", Some(find)) => { self.handle_find(find) },
            ("remove", Some(remove)) => { self.handle_remove(remove) }
            (_, _) => { panic!("Unknown operation in CLI"); }
        }
    }

    fn extract_values(args: &'p ArgMatches) -> Params<'p> {
        let (_, arg) = args.subcommand();
        let options = arg.unwrap();

        let password = options.value_of("pwd");
        let account = options.value_of("account");
        let entity = options.value_of("entity");
        
        Params {
            entity,
            account,
            password
        }
    }

    fn handle_add(&mut self, args: &'p ArgMatches) -> io::Result<Resolve> {
        let Params { 
            entity, 
            account, 
            password 
        } = CLI::extract_values(args);

        let args = Args::new(
            entity,
            account,
            password
        );

        self.keeper.add(args)
    }

    fn handle_find(&mut self, args: &'p ArgMatches) -> io::Result<Resolve> {
        let Params { entity, account, .. } = CLI::extract_values(args);

        let args = Args::new(
            entity,
            account,
            None 
        );

        let to_read = args.has_all();

        let found = self.keeper
            .find(args)?
            .to_vec();

        let selected = select(found);

        if let Some(option) = selected {
            if to_read {
                let read = self.keeper.read(option);
                // TODO: copy value to clipboard
            } 
        }

        Ok(Resolve::Found)
    }
    
    fn handle_remove(&mut self, args: &'p ArgMatches) -> io::Result<Resolve> {
        let Params { entity, account, .. } = CLI::extract_values(args);
       
        let args = Args::new(
            entity,
            account,
            None 
        );

        self.keeper.remove(args)
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::fs::{remove_dir_all, remove_file};

    use super::CLI;
    use super::{Resolve};
    
    use crate::mocks::Setup; 
    use crate::locker::Locker;
    use crate::cli::commands::{command, Commands};

    use Commands::*;

    fn after_each(this: &mut Setup) {
        for path in this.paths.iter() {
            let p = Path::new(path);

            let exists = &p.exists();
            let is_dir = &p.is_dir();
            let is_file = &p.is_file();

            let remove = if *is_dir { "dir" } else { "file" };
            let msg = format!("Could not remove {} {:?} in `handler.rs` test", remove, path);

            if *exists { 
                if *is_file { remove_file(path).expect(&msg); }
                if *is_dir { remove_dir_all(path).expect(&msg); }
            } 
        }
    }

    #[test]
    fn operation_add_entity() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let locker_instance = Locker::new();

                let mut l = locker.clone();
                let entity_hash = locker_instance.hash("add_entity");
                l.push(entity_hash);

                let mut cli = CLI::start(config, locker);
               
                let args = vec![ "test", "add", "entity", "add_entity" ];
                let results = command(AddEntity, args);
                let add = cli.operation(results).unwrap();

                assert_eq!(add, Resolve::Add);
                assert_eq!(l.as_path().exists(), true);
            }
        };
    }

    #[test]
    fn operation_add_account() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let locker_instance = Locker::new();

                let mut l = locker.clone();
                let entity_hash = locker_instance.hash("add_account_entity");
                let account_hash = locker_instance.hash("add_account");

                l.push(entity_hash);
                l.push(account_hash);

                let mut cli = CLI::start(config, locker);
                
                let args = vec![ "test", "add", "account", "add_account", "-e", "add_account_entity" ];
                let results = command(AddAccount, args);
                let add = cli.operation(results).unwrap();

                assert_eq!(add, Resolve::Add);
                assert_eq!(l.as_path().exists(), true);
            }
        };
    }

    #[test]
    fn operation_add_password() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let locker_instance = Locker::new();

                let mut l = locker.clone();
                let entity_hash = locker_instance.hash("entity_for_password");
                let account_hash = locker_instance.hash("account_for_password");
                let password_hash = "very_good_password_1"; 

                l.push(entity_hash);
                l.push(account_hash);

                let mut cli = CLI::start(config, locker);

                let args = vec![ 
                    "test", "add", "password", "very_good_password_1", 
                    "-a", "account_for_password", "-e", "entity_for_password" 
                ];

                let results = command(AddPassword, args);
                let add = cli.operation(results).unwrap();

                assert_eq!(add, Resolve::Add);

                let account_dir = l.read_dir().expect("Account dir not created!");

                for entry in account_dir {
                    if let Ok(entry) = entry {
                        assert!(entry.path().exists());
                    }
                }
            }
        };
    }

    #[test]
    fn operation_find_entity() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let mut cli = CLI::start(config, locker);
               
                let add_args = vec![ "test", "add", "entity", "operation_find_entity" ];
                let add_results = command(AddEntity, add_args);
                let add = cli.operation(add_results).unwrap();

                assert_eq!(add, Resolve::Add);

                let find_args = vec![ "test", "find", "entity", "operation_find_entity" ]; 
                let find_results = command(FindEntity, find_args);
                let found = cli.operation(find_results).unwrap();

                assert_eq!(found, Resolve::Found);
            }
        };
    }

    #[test]
    fn operation_find_account() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let mut cli = CLI::start(config, locker);
               
                let add_args = vec![ "test", "add", "account", "account", "-e", "operation_find_account" ];
                let add_results = command(AddAccount, add_args);
                cli.operation(add_results);

                let find_args = vec![ "test", "find", "account", "account", "-e", "operation_find_account" ];
                let find_results = command(FindAccount, find_args);
                let found = cli.operation(find_results).unwrap();

                assert_eq!(found, Resolve::Found);

                let find_args = vec![ "test", "find", "entity", "operation_find_account" ];
                let find_results = command(FindEntity, find_args);
                let found = cli.operation(find_results).unwrap();

                assert_eq!(found, Resolve::Found);
            }
        };
    }

    #[test]
    fn operation_remove_entity() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let mut cli = CLI::start(config, locker);
               
                let add_args = vec![ "test", "add", "entity", "entity" ];
                let add_results = command(AddEntity, add_args);
                let add = cli.operation(add_results).unwrap();

                assert_eq!(add, Resolve::Add);

                let remove_args = vec![ "test", "remove", "entity", "entity" ];
                let remove_results = command(RemoveEntity, remove_args);
                let removed = cli.operation(remove_results).unwrap();

                assert_eq!(removed, Resolve::Remove);
            }
        };
    }

    #[test]
    fn operation_remove_account() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
                let mut cli = CLI::start(config, locker);
               
                let add_args = vec![ "test", "add", "account", "new_account", "-e", "new_entity" ];
                let add_results = command(AddAccount, add_args);
                let add = cli.operation(add_results).unwrap();

                assert_eq!(add, Resolve::Add);

                let remove_args = vec![ "test", "remove", "account", "new_account", "-e", "new_entity" ];
                let remove_results = command(RemoveAccount, remove_args);
                let removed = cli.operation(remove_results).unwrap();

                assert_eq!(removed, Resolve::Remove);
            }
        };
    }
}
