macro_rules! create_function {
    // ident is used for variable/function names
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name))
        }
    }
}


create_function!(foo);
create_function!(bar);

macro_rules! print_result {
// expr designator is used for expressions.
($expression:expr) => (
    println!("{:?} = {:?}", stringify!($expression), $expression)
)
}

pub fn test() {
    foo();
    bar();

    print_result!(1u32 + 1);

    print_result!({
       let x = 1u32;
        x * x + 2 * x - 1
    });
}