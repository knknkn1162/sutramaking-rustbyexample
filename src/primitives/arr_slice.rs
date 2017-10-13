use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice : {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

pub fn test() {
    let xs: [i32; 5] = [1,2,3,4,5];

    let ys: [i32; 500] = [0;500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("array size: {}", xs.len());
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("borrow the whole array as a slice");
    analyze_slice(&ys[1..4]);

    //panic!! out-of-bound
    // println!("{}", xs[5]);
    // `[i32]` does not have a constant size known at compile-time
    //println!("{:?}", ys[1..4]);
    println!("{:?}", &ys[1..4]);
}