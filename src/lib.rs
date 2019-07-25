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
    path: Option<&'a str>,
}

impl<'a> Locker<'a> {

    pub fn new() -> Locker<'a> {
        Locker { 
            path: Some("t.txt"), 
        }
    }

    fn open(&self, action: LockerAction) -> File {
        let unwrap = &self.path.unwrap();
        let path = Path::new(&unwrap);
        
        let file = match action {
            LockerAction::Read => File::open(&path).expect("Unable to open locker to read!"),
            LockerAction::Write => File::create(&path).expect("Unable to open locker to write!"),
        };

        file
    }

    pub fn read(&self) {
        let mut file = &self.open(LockerAction::Read);
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect("Unable to read opened locker!");
    }

    pub fn write(&self) {
        let mut file = &self.open(LockerAction::Write);
        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        match file.write_all(input.as_bytes()) {
            Err(why) => panic!("Couldn't write to locker: {}", why),
            Ok(_) => println!("Success writing to locker!")
        } 
    }
}

