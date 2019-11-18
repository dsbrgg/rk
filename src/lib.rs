mod tests;
mod locker;
mod managers;

use std::path::{Path, PathBuf};
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
        let mut paths = Vec::new();

        if let Some(e) = entity { self.directories.create(e)?; }

        if let Some(a) = account {
            paths.push(a);

            let e = entity.unwrap();

            self.directories.create(
                &DirManager::append_path(e, &paths)
            )?; 
        }

        if let Some(p) = password {
            paths.push(p);

            let e = entity.unwrap();

            self.files.create(
                &FileManager::append_path(e, &paths)
            )?; 
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

    use tests::setup::Setup;
    use std::fs::remove_dir_all;

    fn after_each(this: &mut Setup) {
        for path in this.paths.iter() {
            let exists = Path::new(path).exists();

            if exists {
                remove_dir_all(path)
                    .expect("Could not remove file in test");
            } 
        }
    }

    #[test]
    fn new() {
        Setup { 
            paths: Vec::new(),
            after_each: &after_each,
            test: &|mut this| {
                let (config, locker) = this.as_path_buf();
                Keeper::new(config, locker);
            },
        }; 
    }

    #[test]
    fn add_entity() {
        Setup {
            paths: Vec::new(),
            after_each: &after_each,
            test: &|mut this| {
                let (config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(config, locker);
       
                let e = this.add_to_paths("add_entity".to_string());

                let entity = Some(e.as_str());
                let account = None;
                let password = None;

                keeper.add(entity, account, password);
            }
        };
    }

    #[test]
    fn add_account() {
        Setup {
            paths: Vec::new(), 
            after_each: &after_each,
            test: &|mut this| {
                let (config, locker) = this.as_path_buf();
                let mut keeper = Keeper::new(config, locker);
       
                let e = this.add_to_paths("add_account_1".to_string());
                let a = this.add_to_paths("add_account_2".to_string());

                let entity = Some(e.as_str());
                let account = Some(a.as_str());
                let password = None;

                keeper.add(entity, account, password);
            }
        };
    }
}
