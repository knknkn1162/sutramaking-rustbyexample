struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str {"red"}
fn blue<T: Blue>(_: &T)-> &'static str {"blue"}

pub fn test() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    // the trait bound `generics::empty_bounds::Turkey: generics::empty_bounds::Red` is not satisfied
    //println!("A turkey is {}", red(&_turkey));
}