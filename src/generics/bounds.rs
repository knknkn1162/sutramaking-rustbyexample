use std::fmt::Debug;

trait HasArea {
    fn area(&self)-> f64;
}

impl HasArea for Rectangle {
    fn area(&self)->f64 {self.length * self.height}
}

#[derive(Debug)]
struct Rectangle {length: f64, height: f64}

#[allow(dead_code)]
struct Triangle {length: f64, height: f64}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T)-> f64 {
    t.area()
}

pub fn test() {
    let rectangle = Rectangle {length: 3.0, height: 4.0};
    let _triangle = Triangle {length: 3.0, height: 4.0};

    print_debug(&rectangle);

    println!("Area: {}", area(&rectangle));
    // the trait bound `generics::bounds::Triangle: generics::bounds::HasArea` is not satisfied
    // println!("Area: {}", area(&_triangle));
}