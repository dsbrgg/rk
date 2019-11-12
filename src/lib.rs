mod locker;
mod managers;

use std::path::PathBuf;
use std::io::{self, Write, Read};

use locker::Locker;

use managers::traits::Manager;
use managers::dir_manager::DirManager;
use managers::file_manager::FileManager;

pub struct Keeper<'l> {
    lock: Locker<'l>,
    files: FileManager<'l>,
    directories: DirManager<'l>,
}

impl<'l> Keeper<'l> {
    pub fn new(config: PathBuf, locker: PathBuf) -> Keeper<'l> {
        // TODO: just for future reference
        // let mut config_path = dirs::home_dir().unwrap();
        // let mut locker_path = dirs::home_dir().unwrap();

        let lock = Locker::new();
        let directories = DirManager::new(config.clone(), locker.clone());
        let files = FileManager::new(config, locker);

        Keeper { lock, files, directories }
    }

    pub fn add(
        &mut self, 
        entity: Option<&str>, 
        account: Option<&str>, 
        password: Option<&str>
    ) -> io::Result<()> {
        self.add_entity(entity)?;

        Ok(())
    }
    
    fn add_entity(&mut self, entity: Option<&str>) -> io::Result<()> {
        if let Some(value) = entity {
            self.directories.create(value)?; 
        }

        Ok(())
    }

    // pub fn new() -> Keeper<'l, 'd, 'f> {
    //     let mut config_path = dirs::home_dir().unwrap();
    //     let mut locker_path = dirs::home_dir().unwrap();

    //     locker_path.push(".rk");
    //     config_path.push(".config/rk");

    //     Keeper { 
    //         lock: Locker::new(),
    //         files: FileManager::new(config_path, locker_path),
    //         directories: DirManager::new(config_path, locker_path),
    //     }
    // }

    // // TODO: read file + decrypt and find account
    // fn find(&self) {
    //     let mut file = self.read_locker();
    // }

    // fn handle_input() -> String {
    //     let mut input = String::new();

    //     stdin()
    //         .read_line(&mut input)
    //         .expect("Failed to read user input");

    //     input
    // }

    // pub fn read_locker(&self) -> String {
    //     let mut contents = String::new();
    //     
    //     let locker_path = self.files.get_locker_path();
    //     let mut file = FileManager::open(locker_path, None, FileAction::Read);

    //     match file.read_to_string(&mut contents) {
    //         Err(why) => panic!("Couldn't open file to read: {}", why),
    //         Ok(_) => println!("\n::: Success reading locker :::\n"),
    //     };
    //    
    //     contents
    // }

    // pub fn write_on(&self, contents: &mut String) {
    //     let mut input = Keeper::handle_input();
    //     let encrypted = self.lock.input_encryption(&mut input);

    //     // TODO: decryption working, have to define a way
    //     // to maintain key and iv to decrypt further values
    //     self.lock.input_decryption(&encrypted);


    //     let locker_path = self.files.get_locker_path();
    //     let mut file = FileManager::open(locker_path, None, FileAction::Write);
    //     
    //     let new_register = format!("{}", encrypted.trim());

    //     contents.push_str(
    //         new_register.as_str()
    //     );

    //     match file.write_all(contents.as_bytes()) {
    //         Err(why) => panic!("Couldn't write to locker: {}", why),
    //         Ok(_) => println!("\n::: Success writing to locker :::\n")
    //     } 
    // }

    // pub fn add_account(&mut self) {
    //     let input = Keeper::handle_input();
    //     let hash = self.lock.hash(input);

    //     self.directories.create(&hash[..]);
    // }
}

#[cfg(test)]
mod tests_keeper {
    use super::*;
    use std::env; 

    // TODO: implement struct that will init and drop files/folders
    // https://stackoverflow.com/questions/38253321/what-is-a-good-way-of-cleaning-up-after-a-unit-test-in-rust
    fn setup_paths(test: &str, count: u8) -> (PathBuf, PathBuf) {
        let mut config_path = env::current_dir().unwrap();
        let mut locker_path = env::current_dir().unwrap();
        
        locker_path.push(format!("dump/keeper_{}_{}", test, count));
        config_path.push(format!("dump/keeper_{}_{}", test, count+1));

        (config_path, locker_path)
    }

    #[test]
    fn new() {
        let paths = setup_paths("new", 1); 
        Keeper::new(paths.0, paths.1);
    }

    #[test]
    fn add_entity() {
        let paths = setup_paths("add", 1);
        
        let mut keeper = Keeper::new(paths.0, paths.1);
        
        let entity = Some("entity");
        let account = None;
        let password = None;

        keeper.add(entity, account, password);
    }
}
