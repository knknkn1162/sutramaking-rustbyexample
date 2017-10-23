use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str =
    "the quick brown fox jumped over the lazy dog\n";

pub fn test() {
    let process = Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("couldn't spawn wc");

    process.stdin.unwrap().write_all(PANGRAM.as_bytes()).expect("couldn't write to wc stdin");

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why.description()),
        Ok(_) => println!("wc responded with : \n{}", s)
    }
}