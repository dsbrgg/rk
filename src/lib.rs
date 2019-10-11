mod locker;
mod file_manager;

use std::io::stdin;
use std::io::Write;
use std::io::Read;

use std::fs::File;
use std::path::Path;

use locker::Locker;
use file_manager::{FileAction, FileManager};

enum KeeperAction {
    Read,
    Write,
}

pub struct Keeper<'a> {
    path: &'a str,
    lock: Locker<'a>,
    manager: FileManager
}

impl<'a> Keeper<'a> {
    pub fn new() -> Keeper<'a> {
        Keeper { 
            path: "t",
            lock: Locker::new(),
            manager: FileManager::new()
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
        
        let locker_path = self.manager.get_locker_path();
        let mut file = FileManager::open(locker_path, FileAction::Read);

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


        let mut file = FileManager::open(KeeperAction::Write);
        let new_register = format!("{}", encrypted.trim());

        contents.push_str(
            new_register.as_str()
        );

        match file.write_all(contents.as_bytes()) {
            Err(why) => panic!("Couldn't write to locker: {}", why),
            Ok(_) => println!("\n::: Success writing to locker :::\n")
        } 
    }

    pub fn append(&self) {
        let previous_data = &mut self.read_locker();
        self.write_on(previous_data);
    }
}

