use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let home = env::var("HOME").unwrap();
    let mut settings = String::new();
    let mut file = File::open("settings.yml").unwrap();

    file.read_to_string(&mut settings)
        .expect("Unable to read settings.yml");

    let replaced = settings.replace("$HOME", &home);
    let mut tmp = File::create("settings_tmp.yml").unwrap();
    tmp.write_all(replaced.as_bytes());
}
