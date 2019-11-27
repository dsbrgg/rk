mod app;
mod cli;
mod locker;
mod managers;
mod mocks;

// TODO:
// extract cmd -> encrypt/zip all passwords and send it to a specific path
// set config cmd -> allow for custom config settings (eg. locker location, config location, cypto algo, etc...)

fn main() {
    use cli::cli::CLI;

    let args = app::execute();

    let mut config = dirs::home_dir().unwrap();
    let mut locker = dirs::home_dir().unwrap();
    
    config.push(".config");
    config.push("rk");
    locker.push(".rk");

    CLI::start(config, locker)
        .operation(args)
        .expect("Error on app operation");
}
