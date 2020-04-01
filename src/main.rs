#![allow(warnings)]

mod app;
mod cli;
mod args;
mod locker;
mod managers;
mod mocks;
mod yaml;

// TODO:
// extract cmd -> encrypt/zip all passwords and send it to a specific path
// set config cmd -> allow for custom config settings (eg. locker location, config location, cypto algo, etc...)

fn main() {
    use cli::CLI;
    use yaml::{
        Settings, 
        SettingsOpts::*
    };

    let args = app::execute();
    let settings = Settings::default();
    
    let index = settings.get(Index);
    let config = settings.get(Config);
    let locker = settings.get(Locker);

    CLI::start(index, config, locker)
        .operation(args)
        .expect("Error on app operation");
}
