fn apply<F>(f: F) where
    F: Fn() {
    f();
}

pub fn test() {
    let x = 7;

    let print = || println!("{}", x);

    apply(print);
}