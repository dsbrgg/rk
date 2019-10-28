use clap::{App, Arg, SubCommand};
use rk::Keeper;

fn main() {
    let matches = App::new("MyApp")
        .version("1.0")
        .author("Diego Braga")
        .about("Local password manager")
        .subcommand(
            SubCommand::with_name("add")
                .about("Add and entity, account or password")
                .subcommand(
                    SubCommand::with_name("entity")
                        .about("Add an entity")
                        .arg(
                            Arg::with_name("name")
                                .takes_value(true)
                                .required(true)
                        )
                )
                .subcommand(
                    SubCommand::with_name("account")
                        .about("Add an account")
                        .arg(
                            Arg::with_name("name")
                                .takes_value(true)
                                .required(true)
                        )
                )
                .subcommand(
                    SubCommand::with_name("password")
                        .about("Add a password")
                        .arg(
                            Arg::with_name("pwd")
                                .takes_value(true)
                                .required(true)
                        )
                )
        )
        .get_matches();

    let keeper = Keeper::new();
    keeper.add_account();
}
