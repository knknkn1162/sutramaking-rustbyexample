#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

pub fn test() {
    let rect1 = Rectangle { length: 50, width: 30};

    println!("{}", rect1.area());
}