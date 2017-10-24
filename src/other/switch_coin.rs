#[derive(Debug)] // So we can inspect the state in a minute
pub enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


pub fn add(coin: self::Coin) {
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("OK: {:?}", state);
    } else { count += 1; }
}