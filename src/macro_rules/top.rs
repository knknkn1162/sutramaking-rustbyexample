macro_rules! say_hello {
() => (
    println!("Hello!");
)
}

pub fn test() {
    say_hello!()
}