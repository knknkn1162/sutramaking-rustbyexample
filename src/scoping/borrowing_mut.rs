#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    // can change 'year' to 2014 due to mutability
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

pub fn test() {
    let immutabook = Book {
        author: "Douglas Hofstadter",
        title: "Godel, Escher, Bach",
        year: 1979,
    };

    // impl Copy trait
    let mut mutabook = immutabook;

    borrow_book(&immutabook);

    borrow_book(&mutabook);

    // write &mut explicitly
    new_edition(&mut mutabook);

    // cannot borrow an immutable object as mutable
    //new_edition(&mut immutabook);
}