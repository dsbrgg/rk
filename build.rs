use std::env;

fn main() {
    // TODO: use serde to deserialize settings.yml
    // and apply changes to settings.yml and let FileManager
    // and DirManager handle file creation
    println!("cargo:warning=building...");
    for (key, value) in env::vars() {
        println!("cargo:warning={}: {}", key, value);
    }
}
