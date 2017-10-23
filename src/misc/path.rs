use std::path::Path;

pub fn test() {
    let path = Path::new(".");

    let _display = path.display();

    let new_path = path.join("a").join("b");

    let s = new_path.to_str().expect("new path is not a valid UTF-8 sequence");
    println!("new path is {}", s);
}