fn give_princess(gift: &str) {
    if gift == "snake" {panic!("AAAaaaaa!!!");}
    println!("I love {}s!!", gift);
}

pub fn test() {
    give_princess("teddy bear");
    give_princess("snake");
}