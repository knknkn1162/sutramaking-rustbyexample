use std::error;
use std::fmt;
//use std::num::ParseIntError;
use std;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for DoubleError {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&error::Error> {
        // generic error, underlying cause isn't tracked.
        None
    }
}

fn double_first(vec: Vec<&str>)-> Result<i32> {
    // Vec<&str> -> Option<Vec<&str>> ->Result<Vec<&str>, DoubleError>
    vec.first()
        //Option<Vec<&str>>
        .ok_or(DoubleError)
        // Result<Vec<&str>, DoubleError>
        .and_then(
            |s| s.parse::<i32>().map_err(|_| DoubleError).map(|i| 2*i)
        )
        // Result<i32, DoubleError>

}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("the first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn test() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}