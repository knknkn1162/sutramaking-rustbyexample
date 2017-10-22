use std::error;
use std::fmt;
use std::num::ParseIntError;
use std::result;

type Result<T> = result::Result<T, Box<error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

fn double_first_v1(vec: &Vec<&str>)-> Result<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into())
        .and_then(|s| s.parse::<i32>().map_err(|e| e.into()).map(|i| 2*i))
}

fn double_first_v2(vec: &Vec<&str>)-> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?; // may raise "invalid first item to double"
    let parsed = first.parse::<i32>()?; // Error : invalid digit found in string
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error : {}", e),
    }
}

pub fn test() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first_v1(&numbers));
    print(double_first_v1(&empty));
    print(double_first_v1(&strings));

    print(double_first_v2(&numbers));
    print(double_first_v2(&empty));
    print(double_first_v2(&strings));
}