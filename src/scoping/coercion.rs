fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// 'a >= 'b,  first is coerced to 'b lifetime.
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 { first }


pub fn test() {
    let first = 2;
    {
        // shorter lifetime
        let second = 3;
        // 'a is shorter lifetime.
        println!("The product is {}", multiply(&first, &second));

        println!("{} is the first", choose_first(&first, &second));
    }
}