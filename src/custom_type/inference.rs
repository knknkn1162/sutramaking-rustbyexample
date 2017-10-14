pub fn test() {
    let elem = 5u8;
    // cannot infer type for T. consider giving `vec` a type
    let mut vec = Vec::new();

    vec.push(elem);

    println!("{:?}", vec);
}