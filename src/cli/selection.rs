use std::path::{PathBuf};
use dialoguer::{theme::ColorfulTheme, Select};

pub fn select(found: Vec<PathBuf>) -> PathBuf {
    if found.len() > 1 {
        let options: Vec<&str> = found.iter()
            .map(|pb| pb.file_name().unwrap().to_str().unwrap())
            .collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick account")
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();

        return found[selection].clone();
    }

    PathBuf::new()
}
