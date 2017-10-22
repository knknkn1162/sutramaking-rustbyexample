fn division(dividend: i32, divisor: i32)-> i32 {
    // if true, call destructor of all its objects
    if divisor == 0 { panic!("division by zero");} else {dividend/divisor}
}

pub fn test() {
    let _x = Box::new(0_i32);
    division(3,0);
    println!("this point won't be reached");
}