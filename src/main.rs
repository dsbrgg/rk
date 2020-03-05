#![allow(warnings)]

mod app;
mod cli;
mod args;
mod locker;
mod managers;
mod mocks;
mod settings;

// TODO:
// extract cmd -> encrypt/zip all passwords and send it to a specific path
// set config cmd -> allow for custom config settings (eg. locker location, config location, cypto algo, etc...)

fn main() {
    use cli::CLI;
    use settings::Settings;

    let args = app::execute();
    let Settings { paths } = Settings::default();

    let config = paths.get("config").unwrap();
    let locker = paths.get("locker").unwrap();

    // TODO: impl PathBuf for Values
    CLI::start(config, locker)
        .operation(args)
        .expect("Error on app operation");
}
