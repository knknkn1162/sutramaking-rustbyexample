#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

pub fn test() {
    use self::Status::{Poor, Rich};

    use self::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("the rich have lots of money"),
        Poor => println!("The poor have no money"),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Solders fight"),
    }
}