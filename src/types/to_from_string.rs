use std::string::ToString;

struct Circle {
    radius: i32,
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius : {:?}", self.radius)
    }
}

pub fn test() {
    let circle = Circle {radius : 6};
    println!("{}", circle.to_string());

    let parsed = "5".parse::<i32>().unwrap();
    println!("{}", parsed);
}