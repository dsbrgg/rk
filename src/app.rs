use clap::{
    App, 
    AppSettings, 
    Arg, 
    ArgMatches, 
    SubCommand
};

pub fn execute() -> ArgMatches<'static> {
    App::new("rk")
        .version("1.0")
        .author("Diego Braga <dsbrgg@gmail.com>")
        .about("Local password manager")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("list")
                .about("List entities or entity accounts")
                .arg(
                    Arg::with_name("entity")
                        .short("e")
                        .takes_value(true)
                        .required(false)
                )
        )
        .subcommand(
            SubCommand::with_name("find")
                .about("Find an entity or account")
                .arg(
                    Arg::with_name("entity")
                        .short("e")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("account")
                        .short("a")
                        .takes_value(true)
                        .required(false)
                )
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Add an entity, account or password")
                .arg(
                    Arg::with_name("entity")
                        .short("e")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("account")
                        .short("a")
                        .takes_value(true)
                        .required(false)
                )
                .arg(
                    Arg::with_name("password")
                        .short("p")
                        .takes_value(true)
                        .required(false)
                )
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("Remove an entity, account or password")
                .arg(
                    Arg::with_name("entity")
                        .short("e")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("account")
                        .short("a")
                        .takes_value(true)
                        .required(false)
                )
        )
        .get_matches()
} 
