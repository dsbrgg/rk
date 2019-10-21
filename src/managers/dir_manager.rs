use serde_yaml::{Mapping, Value};

use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write, ErrorKind};

const DEFAULT_DIRS: [&'static str; 2] = [ 
    ".rk",
    ".config/rk" 
];

pub enum DirAction {
    Read,
    Write,
}

pub struct DirManager {}

impl DirManager {
    pub fn new() -> DirManager {
        if !DirManager::config_exists() {
            DirManager::init_default()
                .expect("Failed initiating default diretories");
        }

        DirManager {}
    }

    fn init_default() -> io::Result<()> {
        for dir in DEFAULT_DIRS.iter() {
            let mut new_dir = dirs::home_dir().unwrap();

            new_dir.push(dir);

            let dir_path = new_dir.as_path();

            if !dir_path.exists() {
                DirManager::create_dir(&dir_path.to_str().unwrap())?;
            }
        }

        Ok(())
    }

    fn config_exists() -> bool {
        let mut config_dir = dirs::home_dir().unwrap();
        config_dir.push(".config/rk");
        
        config_dir.as_path().exists()
    }

    fn read_dir(dir: &str) -> io::Result<Vec<String>> {
        let mut entries = Vec::new();

        for entry in fs::read_dir(dir)? {
            let dir = entry?;
            
            entries.push(
                dir.path().as_path().to_str().unwrap().to_owned()
            );
        }

        Ok(entries)
    }

    pub fn read_account_dir(dir: &str) -> io::Result<Vec<String>> {
        let mut acc_dir = dirs::home_dir().unwrap();
        
        acc_dir.push(".rk");
        acc_dir.push(dir);

        DirManager::read_dir(
            acc_dir
                .as_path()
                .to_str()
                .unwrap()
        )
    }

    fn create_dir(path: &str) -> io::Result<()> {
        match fs::read_dir(&path) {
            Ok(_) => Ok(()),
            Err(err) => {
                if err.kind() == ErrorKind::NotFound { fs::create_dir_all(&path)?; }
                
                Ok(())
            },
        }
    }

    pub fn create_account_dir(path: &str) -> io::Result<()> {
        let mut acc_dir = dirs::home_dir().unwrap();
        
        acc_dir.push(".rk");
        acc_dir.push(path);

        DirManager::create_dir(
            acc_dir
                .as_path()
                .to_str()
                .unwrap()
        )
    }
}
