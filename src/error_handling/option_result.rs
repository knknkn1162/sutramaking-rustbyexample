use std::num::ParseIntError;
// delay ParseIntError
fn double_first_v1(vec: &Vec<&str>)-> Option<Result<i32, ParseIntError>> {
    // vec.first(&self) -> Option<&T>
    vec.first().map(|first| first.parse::<i32>().map(
        |n| 2 * n)
    )
}

// raise ParseIntError or Ok(None)
fn double_first_v2(vec: &Vec<&str>)-> Result<Option<i32>, ParseIntError> {
    // type => Option<Result<i32>, ParseIntError>
    let opt = vec.first().map(
        |first| first.parse::<i32>().map(|n| 2*n)
    );
    // fn map_or<U, F>(self, default: U, f: F) -> U where U = Result<i32, ParseIntError>
    // F: FnOnce(T) -> U
    let opt = opt.map_or(
        Ok(None),
        |r| r.map(Some))?;
    Ok(opt)

}

pub fn test() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    // The first doubled is Some(Ok(84))
    println!("The first doubled is {:?}", double_first_v1(&numbers));
    // The first doubled is None
    println!("The first doubled is {:?}", double_first_v1(&empty));

    // The first doubled is Some(Err(ParseIntError { kind: InvalidDigit }))
    println!("The first doubled is {:?}", double_first_v1(&strings));

    //The first doubled is Ok(Some(84))
    println!("The first doubled is {:?}", double_first_v2(&numbers));

    // The first doubled is Ok(None)
    println!("The first doubled is {:?}", double_first_v2(&empty));
    // The first doubled is Err(ParseIntError { kind: InvalidDigit })
    println!("The first doubled is {:?}", double_first_v2(&strings));
}