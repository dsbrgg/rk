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
                (_, _) => {}
            }
        }

        pub fn handle_add(&mut self, args: &ArgMatches) {
            match args.subcommand() {
                ("entity", Some(arg)) => {
                    let entity = arg.value_of("name").unwrap();

                    self.keeper.add(
                        Some(entity),
                        None,
                        None
                    );
                },
                ("account", Some(arg)) => {
                    let account = arg.value_of("name").unwrap();
                    let entity = arg.value_of("entity").unwrap();

                    self.keeper.add(
                        Some(entity),
                        Some(account),
                        None
                    );
                },
                ("password", Some(arg)) => {
                    let password = arg.value_of("pwd").unwrap();
                    let account = arg.value_of("account").unwrap();
                    let entity = arg.value_of("entity").unwrap();

                    self.keeper.add(
                        Some(entity),
                        Some(account),
                        Some(password) 
                    );
                },
                (_, _) => {}
            } 
        }

        pub fn handle_find(&self, args: &ArgMatches) {}
        pub fn handle_remove(&self, args: &ArgMatches) {}
    }
}

// #[cfg(test)]
mod tests {
    use super::handler::CLI;

    // #[test]
    fn operation() {
        let locker = dirs::home_dir().unwrap();
        let config = dirs::home_dir().unwrap();

        let cli = CLI::new(config, locker);

        // TODO: find a way to construct ArgMatches to test this
        // let m = App::new("prog")
        // .arg(Arg::with_name("config")
        //     .short("c"))
        // .get_matches_from(vec![
        //     "prog", "-c"
        // ]);
    }
}
