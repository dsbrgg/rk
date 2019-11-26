use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

pub fn add_entity(v: Vec<&str>) -> ArgMatches {
    App::new("test")
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
        .get_matches_from(v)
}

pub fn add_account(v: Vec<&str>) -> ArgMatches {
    App::new("test")
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
        .get_matches_from(v)
}

pub fn add_password(v: Vec<&str>) -> ArgMatches {
    App::new("test")
        .subcommand(
            SubCommand::with_name("add")
                .setting(AppSettings::SubcommandRequired)
                .subcommand(
                    SubCommand::with_name("password")
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
        .get_matches_from(v)
}

pub fn find_entity(v: Vec<&str>) -> ArgMatches {
    App::new("test")
        .subcommand(
            SubCommand::with_name("find")
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
        .get_matches_from(v)
}
