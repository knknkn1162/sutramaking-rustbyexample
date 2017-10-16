pub fn test() {
    struct Foo {x: (u32, u32), y: u32}

    let foo = Foo {x: (1, 2), y: 3};

    //destructure
    let Foo {x: (a, b), y} = foo;

    let Foo {y: i, x: j} = foo;
    println!("i = {:?}, j = {:?}", i, j);

    let Foo {y, x:_} = foo;

    println!("y = {}", y);

}