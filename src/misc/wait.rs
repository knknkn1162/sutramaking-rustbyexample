use std::process::Command;

pub fn test() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();

    let _result = child.wait().unwrap();

    println!("{:?}", _result);
}