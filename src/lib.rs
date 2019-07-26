use std::io::stdin;
use std::io::Write;
use std::io::Read;

use std::fs::File;
use std::path::Path;

enum LockerAction {
    Read,
    Write
}

#[derive(Debug)]
pub struct Locker<'a> {
    path: &'a str,
}

impl<'a> Locker<'a> {

    pub fn new() -> Locker<'a> {
        Locker { 
            path: "t.txt", 
        }
    }

    fn open(&self, action: LockerAction) -> File {
        let path = Path::new(&self.path);
        
        let file = match action {
            LockerAction::Read => File::open(&path).expect("Unable to open locker to read!"),
            LockerAction::Write => File::create(&path).expect("Unable to open locker to write!"),
        };

        file
    }

    pub fn read(&self) -> String {
        let mut contents = String::new();
        let mut file = self.open(LockerAction::Read);

        match file.read_to_string(&mut contents) {
            Err(why) => panic!("Couldn't open file to read: {}", why),
            Ok(_) => println!("\n::: Success reading locker :::\n"),
        };
       
        contents
    }

    pub fn write(&self, contents: &mut String) {
        let mut input = String::new();
        let mut file = self.open(LockerAction::Write);

        stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        contents.push_str(input.as_str());

        match file.write_all(contents.as_bytes()) {
            Err(why) => panic!("Couldn't write to locker: {}", why),
            Ok(_) => println!("\n::: Success writing to locker :::\n")
        } 
    }

    pub fn append(&self) {
        self.write(
            &mut self.read()
        );
    }
}

