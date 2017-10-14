type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

pub fn test() {
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // aliases are not new types
    println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds+inches);
}