fn create_fn() -> Box<Fn()> {
    let text = "Fn".to_owned();

    // without move, error: it borrows `text`
    // move keyword is signals that captures occur by value;
    Box::new(move || println!("This is a {}", text))
}

fn create_fnmut() -> Box<FnMut()> {
    let text = "FnMut".to_owned();

    Box::new(move || println!("This is a : {}", text))
}

pub fn test() {
    let fn_plain = create_fn();

    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();
}