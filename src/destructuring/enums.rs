#[allow(dead_code)]
#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

pub fn test() {
    //let color = Color::RGB(122, 17, 40);
    let color = Color::CMY(123, 23, 45);
    println!("what color is it?");

    match color {
        Color::Red => println!("the color is red!"),
        Color::RGB(r, g, b) => println!("Red : {}, green : {}, blue : {}!", r, g, b),
        _  => println!("{:?}", color),
    }
}