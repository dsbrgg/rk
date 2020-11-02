use std::io;
use std::path::PathBuf;

use clap::ArgMatches;
use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;

use rk::{
    Args, 
    Encrypted,
    Resolve, 
    Keeper,
    VaultError
};

use crate::cli::select;

struct Params<'p> { 
    entity: Option<&'p str>,
    account: Option<&'p str>,
    password: Option<&'p str>
}

pub struct CLI { keeper: Keeper }

impl<'p> CLI {
    pub fn start(config: PathBuf, locker: PathBuf) -> CLI {
        let keeper = Keeper::new(config, locker).unwrap();

        CLI {
            keeper
        }
    }

    pub fn operation(&mut self, args: ArgMatches) -> Result<Resolve, VaultError> {
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

    fn handle_add(&mut self, args: &'p ArgMatches) -> Result<Resolve, VaultError> {
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

    fn handle_find(&mut self, args: &'p ArgMatches) -> Result<Resolve, VaultError> {
        let Params { 
            entity, 
            account, 
            .. 
        } = CLI::extract_values(args);

        let args = Args::new(
            entity,
            account,
            None 
        );

        let found = self.keeper.find(args)?;
        
        if let Resolve::Find(vec) = &found {
            let selected = select(vec.to_owned());

            if let Some(option) = selected {
                // NOTE: having issues on linux to copy/paste on clipboard:
                // https://github.com/alacritty/alacritty/issues/2795
                // let mut ctx = ClipboardContext::new().unwrap();
                // ctx.set_contents(read).unwrap();
                // let read = self.keeper.read(option)?.to_string();
                
                return Ok(Resolve::Read(option));
            }
        }

        if let Resolve::Read(password) = &found {
            println!("{:?}", password);
        }

        Ok(found)
    }
    
    fn handle_remove(&mut self, args: &'p ArgMatches) -> Result<Resolve, VaultError> {
        let Params { 
            entity, 
            account, 
            .. 
        } = CLI::extract_values(args);
       
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
    use super::*;

    use std::path::{Path, PathBuf};
    use std::fs::{remove_dir_all, remove_file};
 
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
                let mut cli = CLI::start(config, locker);
                let args = vec![ "test", "add", "entity", "add_entity" ];
                let results = command(AddEntity, args);
                let add = cli.operation(results).unwrap();

                assert_eq!(add, Resolve::Add);
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
                let mut cli = CLI::start(config, locker);
                let args = vec![ "test", "add", "account", "add_account", "-e", "add_account_entity" ];
                let results = command(AddAccount, args);
                let add = cli.operation(results).unwrap();

                assert_eq!(add, Resolve::Add);
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
                let mut cli = CLI::start(config, locker);

                let args = vec![ 
                    "test", "add", "password", "very_good_password_1", 
                    "-a", "account_for_password", "-e", "entity_for_password" 
                ];

                let results = command(AddPassword, args);
                let add = cli.operation(results).unwrap();

                assert_eq!(add, Resolve::Add);
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

                assert_eq!(found, Resolve::Find(vec![]));
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
               
                let add_args = vec![ "test", "add", "account", "account", "-e", "entity" ];
                let add_results = command(AddAccount, add_args);
                cli.operation(add_results);

                let find_args = vec![ "test", "find", "account", "account", "-e", "entity" ];
                let find_results = command(FindAccount, find_args);
                let found = cli.operation(find_results).unwrap();

                assert_eq!(found, Resolve::Read("".to_string()));
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
