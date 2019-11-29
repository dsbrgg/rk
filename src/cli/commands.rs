use clap::{
    App, 
    AppSettings, 
    Arg, 
    ArgMatches, 
    SubCommand
};

pub enum Commands {
    AddEntity,
    AddAccount,
    AddPassword,
    FindEntity,
    FindAccount,
    RemoveEntity,
    RemoveAccount
}

pub fn command(cmd: Commands, args: Vec<&str>) -> ArgMatches<'static> {
   let app = App::new("test");

    match cmd {
        Commands::AddEntity => app.subcommand(add_entity()).get_matches_from(args),
        Commands::AddAccount => app.subcommand(add_account()).get_matches_from(args),
        Commands::AddPassword => app.subcommand(add_password()).get_matches_from(args),
        Commands::FindEntity => app.subcommand(find_entity()).get_matches_from(args),
        Commands::FindAccount => app.subcommand(find_account()).get_matches_from(args),
        Commands::RemoveEntity => app.subcommand(remove_entity()).get_matches_from(args),
        Commands::RemoveAccount => app.subcommand(remove_account()).get_matches_from(args),
    }
}

fn add_entity() -> App<'static, 'static> {
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
}

fn add_account() -> App<'static, 'static> {
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
}

fn add_password() -> App<'static, 'static> {
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
}

fn find_entity() -> App<'static, 'static> {
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
}

fn find_account() -> App<'static, 'static> {
    SubCommand::with_name("find")
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
}

fn remove_entity() -> App<'static, 'static> {
    SubCommand::with_name("remove")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("entity")
                .arg(
                    Arg::with_name("entity")
                        .takes_value(true)
                        .required(true)
                )
        )
}

fn remove_account() -> App<'static, 'static> {
    SubCommand::with_name("remove")
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
}
