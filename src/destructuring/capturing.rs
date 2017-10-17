pub fn test() {
    use std::mem;

    let color = "green";

    let print = || println!("color : {}", color);

    print();
    print();

    let mut count = 0;

    // borrows `count`
    let mut inc = || {
        count += 1;
        println!("count : {}", count);
    };

    inc();
    inc();

    // cannot borrow `count` as mutable more than once at a time
    //let reborrow = &mut count;
    //println!("{:?}", reborrow);


    let movable = Box::new(3);

    let consume = || {
        println!("movable : {:?}", movable);
        mem::drop(movable);
    };

    consume();
    //consume();
}