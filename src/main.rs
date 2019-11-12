use rk::Keeper;
use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let matches = App::new("Rusty Keeper")
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
                            Arg::with_name("name")
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
                        .arg(
                            Arg::with_name("entity")
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
                            Arg::with_name("name")
                                .takes_value(true)
                                .required(true)
                        )
                )
                .subcommand(
                    SubCommand::with_name("account")
                        .about("Remove an account")
                        .arg(
                            Arg::with_name("name")
                                .takes_value(true)
                                .required(true)
                        )
                )
                .subcommand(
                    SubCommand::with_name("password")
                        .about("Remove a password")
                        .arg(
                            Arg::with_name("pwd")
                                .takes_value(true)
                                .required(true)
                        )
                )
        )
        .get_matches();

    
    if let Some(sub) = matches.subcommand_matches("add") {
        if let Some(cmd) = sub.subcommand_matches("entity") {
            println!("{:#?}", cmd.value_of("name").unwrap());
        }
    }

    // let mut keeper = Keeper::new();
    // keeper.add_account();
}
