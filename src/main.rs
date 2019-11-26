mod setup;
mod locker;
mod handler;
mod managers;
mod cli_operations;

use handler::handler::CLI;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

// TODO:
// extract cmd -> encrypt/zip all passwords and send it to a specific path
// set config cmd -> allow for custom config settings (eg. locker location, config location, cypto algo, etc...)

fn main() {
    let mut config = dirs::home_dir().unwrap();
    let mut locker = dirs::home_dir().unwrap();
    
    config.push(".config");
    config.push("rk");
    locker.push(".rk");

    let mut cli = CLI::new(config, locker);
    
    cli.operation(
        execute()
    );
}

fn execute() -> ArgMatches<'static> {
    App::new("Rusty Keeper")
        .version("1.0")
        .author("Diego Braga <dsbrgg@gmail.com>")
        .about("Local password manager")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("list")
                .about("List entities or accounts")
                .setting(AppSettings::SubcommandRequired)
                .subcommand(
                    SubCommand::with_name("entity")
                        .about("List entities")
                )
                .subcommand(
                    SubCommand::with_name("account")
                        .about("List accounts of an entity")
                        .arg(
                            Arg::with_name("entity")
                                .takes_value(true)
                                .required(true)
                        )
                )
        )
        .subcommand(
            SubCommand::with_name("find")
                .about("Find an entity or account")
                .setting(AppSettings::SubcommandRequired)
                .subcommand(
                    SubCommand::with_name("entity")
                        .about("Find an entity")
                        .arg(
                            Arg::with_name("name")
                                .takes_value(true)
                                .required(true)
                        )
                )
                .subcommand(
                    SubCommand::with_name("account")
                        .about("Find an account")
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
        .subcommand(
            SubCommand::with_name("add")
                .about("Add an entity, account or password")
                .setting(AppSettings::SubcommandRequired)
                .subcommand(
                    SubCommand::with_name("entity")
                        .about("Add an entity")
                        .arg(
                            Arg::with_name("entity")
                                .takes_value(true)
                                .required(true)
                        )
                )
                .subcommand(
                    SubCommand::with_name("account")
                        .about("Add an account")
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
                .subcommand(
                    SubCommand::with_name("password")
                        .about("Add a password")
                        .arg(
                            Arg::with_name("pwd")
                                .takes_value(true)
                                .required(true)
                        )
                        .arg(
                            Arg::with_name("account")
                                .short("a")
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
        .subcommand(
            SubCommand::with_name("remove")
                .about("Remove an entity, account or password")
                .setting(AppSettings::SubcommandRequired)
                .subcommand(
                    SubCommand::with_name("entity")
                        .about("Remove an entity")
                        .arg(
                            Arg::with_name("entity")
                                .takes_value(true)
                                .required(true)
                        )
                )
                .subcommand(
                    SubCommand::with_name("account")
                        .about("Remove an account")
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
        .get_matches()
}
