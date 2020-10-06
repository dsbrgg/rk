use std::path::PathBuf;

use rk::Encrypted;

use dialoguer::{theme::ColorfulTheme, Select};

pub fn select(found: Vec<Encrypted>) -> Option<Encrypted> {
    if found.len() == 0 { return None; }
    if found.len() == 1 { return Some(found[0].clone()); }
    
    let options: Vec<String> = found.iter()
        .map(|enc| enc.path())
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick account")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    Some(found[selection].clone())
}
