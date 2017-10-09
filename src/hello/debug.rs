#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

pub fn test01() {
    println!("{:?} months in a year", 12);

    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Chriistian",
        actor = "actors",
    );

    println!("Now {:?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

pub fn test02() {
    let name = "Peter";
    let age = 27;
    let peter = Person {name, age };

    println!("{:#?}", peter);
}