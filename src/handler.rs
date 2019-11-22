use clap::{ArgMatches};
use std::path::{PathBuf};

pub mod handler { 
    use super::*;

    use std::io;
    use rk::Keeper;

    pub struct CLI { keeper: Keeper }

    impl CLI {
        pub fn new(config: PathBuf, locker: PathBuf) -> CLI {
            CLI {
                keeper: Keeper::new(config, locker)
            }
        }

        pub fn operation(&mut self, args: ArgMatches) {
            match args.subcommand() {
                ("add", Some(add)) => { self.handle_add(add); },
                ("find", Some(find)) => { self.handle_find(find); },
                ("remove", Some(remove)) => { self.handle_remove(remove); }
                (_, _) => { println!("Unknown operation"); }
            }
        }

        pub fn handle_add(&mut self, args: &ArgMatches) {
            let (_, arg) = args.subcommand();
            let options = arg.unwrap();

            let password = options.value_of("pwd");
            let account = options.value_of("account");
            let entity = options.value_of("entity");
            
            self.keeper.add(
                entity,
                account,
                password
            );
        }

        pub fn handle_find(&self, args: &ArgMatches) {}
        pub fn handle_remove(&self, args: &ArgMatches) {}
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
    fn operation_add() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|this| {
                let (config, locker) = this.as_path_buf();
               
                let mut l = locker.clone();
                l.push("an_entity");

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
                        vec![ "test", "add", "entity", "an_entity" ]
                    ); 

                cli.operation(results);

                assert_eq!(l.as_path().exists(), true);
            }
        
        };
    }
}
