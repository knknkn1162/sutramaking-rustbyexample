use std::num::ParseIntError;

type Res<T> = Result<T, ParseIntError>;

fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        // lambda always success
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn print(result: Res<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn test() {
    let twenty = multiply("10", "2");
    print(twenty);

    let tt = multiply("t", "2");
    print(tt);
}