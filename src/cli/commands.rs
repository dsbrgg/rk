use clap::{
    App, 
    AppSettings, 
    Arg, 
    ArgMatches, 
    SubCommand
};

pub enum Commands {
    Add,
    Find,
    Remove
}

pub fn command(cmd: Commands, args: Vec<&str>) -> ArgMatches<'static> {
   let app = App::new("test");

    match cmd {
        Commands::Add => app.subcommand(add()).get_matches_from(args),
        Commands::Find => app.subcommand(find()).get_matches_from(args),
        Commands::Remove => app.subcommand(remove()).get_matches_from(args),
    }
}

fn add() -> App<'static, 'static> {
    SubCommand::with_name("add")
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
}

fn find() -> App<'static, 'static> {
    SubCommand::with_name("find")
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
}

fn remove() -> App<'static, 'static> {
    SubCommand::with_name("remove")
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
}
