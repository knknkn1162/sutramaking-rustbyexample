fn age() -> u32 { 15 }

pub fn test() {
    println!("tell me type of person you are");

    match age() {
        0 => println!("I'm not born yet I gusss"),
        n@1...12 => println!("child : {}", n),
        n@13...19 => println!("teenage : {}", n),
        n => println!("old person of age {}", n),
    }
}