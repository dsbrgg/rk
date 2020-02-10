use std::path::PathBuf;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn select(found: Vec<PathBuf>) -> Option<PathBuf> {
    if found.len() == 0 { return None; }
    if found.len() == 1 { return Some(PathBuf::from(found[0].clone())); }
    
    let options: Vec<&str> = found.iter()
        .map(|pb| pb.file_name().unwrap().to_str().unwrap())
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick account")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    Some(found[selection].clone())
}
