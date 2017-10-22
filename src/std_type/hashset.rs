use std::collections::HashSet;

pub fn test() {
    let mut a: HashSet<i32> = vec!(1_i32, 2, 3).into_iter().collect();
    let mut b: HashSet<i32> = vec!(2_i32, 3, 4).into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    //assert!(b.insert(4), "value 4 is already in set B");

    b.insert(5);

    println!("A: {:?}", a);
    println!("B: {:?}", b);

    println!("union: {:?}", a.union(&b).collect::<Vec<_>>());
    println!("difference : {:?}", a.difference(&b).collect::<Vec<_>>());

    println!("Intersection : {:?}", a.intersection(&b).collect::<Vec<_>>());
    println!("symmetric diff : {:?}", a.symmetric_difference(&b).collect::<Vec<_>>());
}