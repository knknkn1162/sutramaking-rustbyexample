pub fn test() {
    let reference = &4;

    match reference {
        &val => println!("{:?}", val),
    }

    match *reference {
        val => println!("Got a value {:?}", val),
    }

    let _not_a_ref = 3;
    let ref _is_a_ref = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        // if `& r` instead of `ref r`, error: expected integral variable, found reference
        ref r => println!("got a ref to a value : {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10, mut_value : {:?}", m);
        },
    }
}