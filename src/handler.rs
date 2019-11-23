use clap::{ArgMatches};
use std::path::{PathBuf};

pub mod handler { 
    use super::*;

    use std::io;
    use rk::{Resolve, Keeper};

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
    use super::handler::CLI;
    use crate::setup::setup::Setup;

    use std::path::Path;
    use std::fs::remove_dir_all;
    use clap::{App, AppSettings, Arg, SubCommand};

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
                
                let results = App::new("test")
                    .subcommand(
                        SubCommand::with_name("add")
                            .setting(AppSettings::SubcommandRequired)
                            .subcommand(
                                SubCommand::with_name("entity")
                                    .arg(
                                        Arg::with_name("entity")
                                            .takes_value(true)
                                            .required(true)
                                    )
                            )
                    ) 
                    .get_matches_from(
                        vec![ "test", "add", "entity", "add_entity" ]
                    ); 

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
                
                let results = App::new("test")
                    .subcommand(
                        SubCommand::with_name("add")
                            .setting(AppSettings::SubcommandRequired)
                            .subcommand(
                                SubCommand::with_name("account")
                                    .arg(
                                        Arg::with_name("account")
                                            .takes_value(true)
                                            .required(true)
                                    )
                                    .arg(
                                        Arg::with_name("entity")
                                            .short("e")
                                            .takes_value(true)
                                            .required(true)
                                    )
                            )
                    ) 
                    .get_matches_from(
                        vec![ 
                            "test", 
                            "add", 
                            "account", 
                            "add_account", 
                            "-e", 
                            "add_account_entity" 
                        ]
                    ); 

                cli.operation(results);

                assert_eq!(l.as_path().exists(), true);
            }
        };
    }

}
