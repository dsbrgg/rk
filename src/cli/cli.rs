use clap::{ArgMatches};

use std::io;
use std::path::{PathBuf};

use rk::{Resolve, Keeper};

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

        self.keeper.add(
            entity,
            account,
            password
        )
    }

    fn handle_find(&mut self, args: &'p ArgMatches) -> io::Result<Resolve> {
        let Params { entity, account, .. } = CLI::extract_values(args);
        
        self.keeper.find(
            entity,
            account
        )
    }
    
    fn handle_remove(&mut self, args: &'p ArgMatches) -> io::Result<Resolve> {
        let Params { entity, account, .. } = CLI::extract_values(args);
        
        self.keeper.remove(
            entity,
            account
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::fs::remove_dir_all;

    use super::CLI;
    use super::{Resolve};
    
    use crate::mocks::setup::Setup;
    use crate::cli::commands::{command, Commands};

    use Commands::*;

    fn after_each(this: &mut Setup) {
        for path in this.paths.iter() {
            let exists = Path::new(path).exists();

            if exists {
                let msg = format!("Could not remove {} in `handlers.rs` test", path);
                
                remove_dir_all(path).expect(&msg);
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
               
                let mut l = locker.clone();
                l.push("add_entity");

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

                let mut l = locker.clone();
                l.push("add_account_entity");
                l.push("add_account");

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
 
                let mut l = locker.clone();
                l.push("entity_for_password");
                l.push("account_for_password");
                l.push("very_good_password_1");

                let mut cli = CLI::start(config, locker);

                let args = vec![ 
                    "test", "add", "password", "very_good_password_1", 
                    "-a", "account_for_password", "-e", "entity_for_password" 
                ];

                let results = command(AddPassword, args);
                let add = cli.operation(results).unwrap();

                assert_eq!(add, Resolve::Add);
                assert_eq!(l.as_path().exists(), true);
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

                let should_equal_to: Vec<String> = vec![];
                assert_eq!(found.to_vec(), should_equal_to);
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

                let should_equal_to: Vec<String> = vec![];
                assert_eq!(found.to_vec(), should_equal_to);

                let find_args = vec![ "test", "find", "entity", "operation_find_account" ];
                let find_results = command(FindEntity, find_args);
                let found = cli.operation(find_results).unwrap();

                assert_eq!(found.to_vec().len(), 1);
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
}
