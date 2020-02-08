use std::path::PathBuf;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn select(found: Vec<PathBuf>) -> PathBuf {
    if found.len() == 0 { return PathBuf::new(); }
    if found.len() == 1 { return PathBuf::from(found[0].clone()); }
    
    let options: Vec<&str> = found.iter()
        .map(|pb| pb.file_name().unwrap().to_str().unwrap())
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick account")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    found[selection].clone()
}
