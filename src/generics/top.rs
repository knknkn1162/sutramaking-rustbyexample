#[derive(Debug)]
struct A;

#[derive(Debug)]
struct Single(A);

#[derive(Debug)]
struct SingleGen<T>(T);

pub fn test() {
    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');

    println!("{:?}", _t);
}