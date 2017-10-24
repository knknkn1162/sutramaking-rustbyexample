mod other;

fn main() {
    let s = "12345 67 8".to_owned();
    let res = other::slices::first_word(&s);
    println!("{:?}", res);
}
