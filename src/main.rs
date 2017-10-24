mod other;

fn main() {
    let s = "12345 67 8".to_owned();
    let res = other::slices::first_word(&s);
    let t = "12345 6 78";
    //first_word(s: &String) -> &str
    let another_res = other::slices::first_word(t);
    println!("{:?}", res);

    let a = [1,2,3,4,5];
    let ref slice = a[1..3];


    let plus_1 = other::option::plus_one(Some(5));
    println!("{:?}", plus_1);

    //other::switch_coin::add(Coin::Quarter);
}
