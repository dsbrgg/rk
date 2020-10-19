use std::path::PathBuf;

use rk::Encrypted;

use dialoguer::{theme::ColorfulTheme, Select};

pub fn select(found: Vec<String>) -> Option<String> {
    if found.len() == 0 { return None; }
    if found.len() == 1 { return Some(found[0].clone()); }
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick account")
        .default(0)
        .items(&found[..])
        .interact()
        .unwrap();

    Some(found[selection].clone())
}
