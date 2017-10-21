struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    // return Option<T>
    fn next(&mut self)-> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn fibonacci()-> Fibonacci {
    Fibonacci {curr: 1, next: 1}
}

pub fn test() {
    let mut sequence = 0..3;

    println!("Four consecutive next call on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    println!("Iterator through 0..3 using for");

    for i in 0..3 {
        println!("> {}", i);
    }

    println!("take");
    // for construct turns some collections into iterators using the .into_iterator()
    // if &i, mismatched types
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    println!("next");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    println!("iter: {:?}", &array);

    // iter method produces an Iterator over an array/slice

    for i in array.iter() {
        println!("> {}", i);
    }

}