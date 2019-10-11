mod locker;
mod file_manager;

use std::io::stdin;
use std::io::Write;
use std::io::Read;

use std::fs::File;
use std::path::Path;

use locker::Locker;
use file_manager::FileManager;

enum KeeperAction {
    Read,
    Write,
}

pub struct Keeper<'a> {
    path: &'a str,
    lock: Locker<'a>,
}

impl<'a> Keeper<'a> {
    pub fn new() -> Keeper<'a> {
        FileManager::init();

        Keeper { 
            path: "t",
            lock: Locker::new(),
        }
    }

    fn open(&self, action: KeeperAction) -> File {
        let path = Path::new(&self.path);
        
        let file = match action {
            KeeperAction::Read => Keeper::try_open(&path),
            KeeperAction::Write => File::create(&path).expect("Unable to open locker to write!"),
        };

        file
    }

    fn try_open(path: &Path) -> File {
        // TODO: Handle when file does not exist on read (eg. running command in "wrong" dir)
        match File::open(&path) {
            Err(_) => File::create(&path).expect("Unable to create locker file!"),
            Ok(file) => file,
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
        let mut file = self.open(KeeperAction::Read);

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


        let mut file = self.open(KeeperAction::Write);
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

