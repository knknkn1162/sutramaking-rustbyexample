pub fn test() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("otherwise"),
    }

    let y = Some(10);

    match y {
        Some(50) => println!("Got 50"),
        Some(10) => println!("matched y = {:?}", y),
        _ => println!("Default case {:?}", x),
    }

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let q = 5;

    match q {
        1 ... 5 => println!("one through five"),
        _ => println!("something else"),
    }


    let ch = 'c';

    match ch {
        'a' ... 'j' => println!("early ascii letter"),
        'k' ... 'z' => println!("late ascii letter"),
        _ => println!("something else"),
    }
    //('a'...'k').iter().inspect(|s| println!("{:?}", s)).collect::<Vec<_>>();

    let p = Point {x: 0, y: 7};

    let Point {x, y} = p;
    assert_eq!(0, x);

    let points = vec![
        Point {x: 0, y: 0},
        Point {x: 1, y: 5},
        Point {x: 10, y: 3},
    ];

    let sum_of_squares: i32 =
        points.iter()
            .map(|&Point {x, y}| x*x+ y*y).sum(
        );


    let s = Some(String::from("Hello"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    let origin = Point3{x: 0, y: 0, z: 0};

    match origin {
        Point3 {x, ..} => println!("x is {}", x),
    }

    let numbers = (2,3,4,5,6);

    match numbers {
        (first, .., last) => {
            println!("{}, .., {}", first, last);
        },
    }

    let robot_name = Some(String::from("Bors"));

    match robot_name {
        // if not ref, raise error, use of partially moved value: `robot_name`.
        Some(ref name) => println!("found a name: {}", name),
        None => (),
    }

    println!("{:?}", robot_name);
}


struct Point {
    x: i32,
    y: i32,
}

struct Point3 {
    x: i32, y: i32, z: i32,
}