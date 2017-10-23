use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


pub fn test() {
    let path = Path::new("hello.txt");
    let display  = path.display();

    let mut file = File::open(&path).unwrap();

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}:{}", display, why.description()),
        Ok(_) => println!("{} contains: \n{}", display, s),
    }
}