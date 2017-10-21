#[derive(Debug, Clone, Copy)]
struct Nil;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

pub fn test() {
    let nil = Nil;
    // copy. there are no resources to move
    let copied_nil = nil;

    println!("original : {:?}", nil);
    println!("copy: {:?}", copied_nil);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);
    //println!("copy: {:?}", pair); //use clone method

    let cloned_pair = moved_pair.clone(); // need to defind Copy trait

    drop(moved_pair);

    println!("clone : {:?}", cloned_pair);
}