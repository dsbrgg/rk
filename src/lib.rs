use std::io::Write;
use std::io::Read;

use std::fs::File;
use std::path::Path;

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

    pub fn read(&self) {
        let path = if let Some(path) = &self.path {
            Path::new(path)
        } else {
            panic!("Locker can't be read if not instantianed by new method!");
        };
        
        let mut contents = String::new();
        let mut open = File::open(&path)
            .expect("Unable to open locker!");

        open.read_to_string(&mut contents)
            .expect("Unable to read opened locker!");
    }
}

