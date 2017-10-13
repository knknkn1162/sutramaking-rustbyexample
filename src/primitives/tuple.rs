fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

pub fn test() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64, 0.1f32, 0.2f64, 'a', true);

    println!("Long tuple first value : {}", long_tuple.0);
    println!("Long tuple second value : {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    println!("tuple of tuples {:?}", tuple_of_tuples);


    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;

    println!("{}, {}, {}, {}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2,2.1, 2.2);
    println!("{:?}", matrix);


}