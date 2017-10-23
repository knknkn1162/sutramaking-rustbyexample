use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::os::unix;
use std::path::Path;

fn cat(path: &Path)->io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    let res = f.read_to_string(&mut s)?;
    Ok(s)
}

fn echo(s: &str, path: &Path)-> io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())
}

fn touch(path: &Path) -> io::Result<()> {
    let res = OpenOptions::new().create(true).write(true).open(path)?;
    Ok(())
}