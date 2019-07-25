use rk;

use std::io::stdin;
use std::io::Write;
use std::io::Read;

use std::fs::File;
use std::path::Path;

fn main() {
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let path = Path::new("t.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {} : {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(input.as_bytes()) {
        Err(why) => panic!("couldn't write to {} : {}", display, why),
        Ok(_) => println!("success {}", display),
    };

    let mut read_file = File::open(&path).expect("Unable to open file");
    let mut contents = String::new();

    read_file.read_to_string(&mut contents).expect("Unable to read file");

    println!("reding: {}", contents);
}
