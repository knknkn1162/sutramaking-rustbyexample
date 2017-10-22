use std::num::ParseIntError;

fn double_first_v1(vec: &Vec<&str>)-> Option<Result<i32, ParseIntError>> {
    // vec.first(&self) -> Option<&T>
    vec.first().map(|first| first.parse::<i32>().map(
        |n| 2 * n)
    )
}

fn double_first_v2(vec: &Vec<&str>)-> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(
        |first| first.parse::<i32>().map(|n| 2*n)
    ); // type => Option<Result<i32>, ParseIntError>
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

    println!("The first doubled is {:?}", double_first_v1(&numbers));
    println!("The first doubled is {:?}", double_first_v1(&empty));
    println!("The first doubled is {:?}", double_first_v1(&strings));

    println!("The first doubled is {:?}", double_first_v2(&numbers));
    println!("The first doubled is {:?}", double_first_v2(&empty));
    println!("The first doubled is {:?}", double_first_v2(&strings));
}