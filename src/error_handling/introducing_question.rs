use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str)-> Result<i32, ParseIntError> {
    // ? is almost exactly equivalent to an unwrap which returns instead of panics on Errs.
    let first_number: i32 = first_number_str.parse::<i32>()?;

    let second_number: i32 = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error : {}", e),
    }
}

pub fn test() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}

