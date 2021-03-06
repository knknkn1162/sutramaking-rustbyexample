pub fn test() {
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("collected (0..10) into : {:?}", collected_iterator);

    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    println!("push 5 into the vector");
    xs.push(4);
    println!("vector: {:?}", xs);

    //collected_iterator.push(0); //error because of immutable.

    println!("second element : {}", xs.len());
    println!("second element: {}", xs[1]);

    println!("Pop last element: {:?}", xs.pop()); // Some(4)
    for x in xs.iter() {
        println!("> {}", x);
    }

    for (i, x) in xs.iter().enumerate() {
        println!("in position {} we have value {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 3;
    }

    println!("Updated vector: {:?}",xs);
}