static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

pub fn test() {
    let n = 16;
    println!("This is {}", LANGUAGE);

    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) {"big"} else {"small"});
}
