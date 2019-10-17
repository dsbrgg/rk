mod locker;
mod managers;

use std::io::{stdin, Write, Read};

use locker::Locker;
use managers::dir_manager::{DirAction, DirManager};
use managers::file_manager::{FileAction, FileManager};

pub struct Keeper<'a> {
    lock: Locker<'a>,
    files: FileManager,
    directories: DirManager,
}

impl<'a> Keeper<'a> {
    pub fn new() -> Keeper<'a> {
        Keeper { 
            lock: Locker::new(),
            directories: DirManager::new(),
            files: FileManager::new(),
        }
    }

    // TODO: read file + decrypt and find account
    fn find(&self) {
        let mut file = self.read_locker();
    }

    fn handle_input() -> String {
        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        input
    }

    pub fn read_locker(&self) -> String {
        let mut contents = String::new();
        
        let locker_path = self.files.get_locker_path();
        let mut file = FileManager::open(locker_path, None, FileAction::Read);

        match file.read_to_string(&mut contents) {
            Err(why) => panic!("Couldn't open file to read: {}", why),
            Ok(_) => println!("\n::: Success reading locker :::\n"),
        };
       
        contents
    }

    pub fn write_on(&self, contents: &mut String) {
        let mut input = Keeper::handle_input();
        let encrypted = self.lock.input_encryption(&mut input);

        // TODO: decryption working, have to define a way
        // to maintain key and iv to decrypt further values
        self.lock.input_decryption(&encrypted);


        let locker_path = self.files.get_locker_path();
        let mut file = FileManager::open(locker_path, None, FileAction::Write);
        
        let new_register = format!("{}", encrypted.trim());

        contents.push_str(
            new_register.as_str()
        );

        match file.write_all(contents.as_bytes()) {
            Err(why) => panic!("Couldn't write to locker: {}", why),
            Ok(_) => println!("\n::: Success writing to locker :::\n")
        } 
    }

    pub fn add_account(&self) {
        let input = Keeper::handle_input();
        let hash = self.lock.hash(input);
        let path = self.files.get_locker_path();

       FileManager::open(path, Some(hash), FileAction::Write);
    }

    // TODO: deprecate this, no way to append out of nothing
    pub fn append(&self) {
        let previous_data = &mut self.read_locker();
        self.write_on(previous_data);
    }
}

