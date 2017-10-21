static NUM: i32 = 18;

//lifetime is coerced
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

pub fn test() {
    {
        // the data remains in the binary for the last
        let static_string = "I'm in read_only memory";

        println!("static_string: {}", static_string);
    }

    {
        let lifetime_num = 9;

        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {:?}", coerced_static);
        println!("NUM : {}", NUM);
    }

    println!("NUM : {} stays accessible", NUM);
}