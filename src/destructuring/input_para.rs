fn apply_fnonce<F>(f: F) where
    F: FnOnce() {
    f();
}

fn apply_fn<F>(f: F) where
    F: Fn() {
    f();
}

// consider changing this to `mut f`
fn apply_fnmut<F>(mut f: F) where
    F: FnMut() {
    f(); // if not mut, cannot borrow mutably
}


fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
        f(3)
}

pub fn test() {
    use std::mem;

    let greeting = "hello";

    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greeting);

        // requires FnMut
        farewell.push_str("!!!");

        println!("Then I screamed {}", farewell);
        println!("Now I can sleep. zzz");

        // requires FnOnce
        //mem::drop(farewell);
    };

    // but this closure only implements `FnOnce`
    apply_fnmut(diary);

    let double = |x| 2 *x;

    println!("3 doubled: {}", apply_to_3(double));
}