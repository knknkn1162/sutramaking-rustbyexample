use std::fs::File;
use std::io;
use std::io::Read;

pub fn test()->Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}