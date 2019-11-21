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
