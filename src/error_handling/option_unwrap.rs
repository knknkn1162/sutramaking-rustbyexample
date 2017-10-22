fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck"),
        Some(inner) => println!("{}? How nice", inner),
        None => println!("No gift?"),
    }
}

fn give_princess(gift: Option<&str>) {
    let inside = gift.unwrap();
    if inside == "snake" {panic!("aaaa");}
    println!("I love {}s !!!", inside);
}

pub fn test() {
    let food = Some("cabbage");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    // type annotations needed
    //let nothing = None;

    give_princess(bird);
    //give_princess(nothing);
}