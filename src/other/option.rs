pub fn plus_one(x: Option<i32>)-> Option<i32> {
    x.map(|s| s+1)
}

pub fn do_twice(f: fn(i32)->i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}