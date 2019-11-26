use clap::{ArgMatches};

use std::io;
use std::path::{PathBuf};

use rk::{Resolve, Keeper};

pub mod handler { 
    use super::*;    

    struct Params<'p> { 
        entity: Option<&'p str>,
        account: Option<&'p str>,
        password: Option<&'p str>
    }

    pub struct CLI { keeper: Keeper }

    impl<'p> CLI {
        pub fn new(config: PathBuf, locker: PathBuf) -> CLI {
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
}

#[cfg(test)]
mod tests {
    use clap::{App, AppSettings, Arg, SubCommand};

    use std::path::Path;
    use std::fs::remove_dir_all;

    use super::{Resolve};
    use super::handler::CLI;
    use crate::cli_operations;
    use crate::setup::setup::Setup;

    fn after_each(this: &mut Setup) {
        for path in this.paths.iter() {
            let exists = Path::new(path).exists();

            if exists {
                remove_dir_all(path)
                    .expect("Could not remove file in `handlers.rs` test");
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

                let mut cli = CLI::new(config, locker);
               
                let args = vec![ "test", "add", "entity", "add_entity" ];
                let results = cli_operations::add_entity(args);

                cli.operation(results);

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

                let mut cli = CLI::new(config, locker);
                
                let args = vec![ "test", "add", "account", "add_account", "-e", "add_account_entity" ];
                let results = cli_operations::add_account(args);

                cli.operation(results);

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

                let mut cli = CLI::new(config, locker);
               
                let args = vec![ "test", "add","password", "very_good_password_1", "-a", "account_for_password", "-e", "entity_for_password" ];
                let results =  cli_operations::add_password(args);

                cli.operation(results);

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
               
                let mut cli = CLI::new(config, locker);
               
                let add_args = vec![ "test", "add", "entity", "operation_find_entity" ];
                let find_args = vec![ "test", "find", "entity", "operation_find_entity" ]; 

                let add_results = cli_operations::add_entity(add_args);
                cli.operation(add_results);

                let find_results = cli_operations::find_entity(find_args);
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
               
                let mut cli = CLI::new(config, locker);
               
                let add_args = vec![ "test", "add", "account", "account", "-e", "operation_find_account" ];
                let find_args = vec![ "test", "find", "account", "account", "-e", "operation_find_account" ];

                let add_results = cli_operations::add_account(add_args);
                cli.operation(add_results);

                let find_results = cli_operations::find_account(find_args);
                let found = cli.operation(find_results).unwrap();

                let should_equal_to: Vec<String> = vec![];

                assert_eq!(found.to_vec(), should_equal_to);

                let find_args = vec![ "test", "find", "entity", "operation_find_account" ];
                let find_results = cli_operations::find_entity(find_args);
                let found = cli.operation(find_results).unwrap();

                let should_equal_to: Vec<String> = vec![String::from("account")];

                assert_eq!(found.to_vec().len(), 1);
            }
        };
    }
}
