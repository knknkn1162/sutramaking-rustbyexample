macro_rules! test {
($left:expr; and $right:expr) => (
    println!("{:?} and {:?} is {:?}",
        stringify!($left),
        stringify!($right),
        $left && $right
    )
);
($left: expr; or $right:expr) => (
    println!("{:?} or {:?} is {:?}",
        stringify!($left),
        stringify!($right),
        $left || $right
    )
);

}

pub fn tes() {
    test!(1_i32 + 1 == 2_i32; and 2_i32 * 2 == 4_i32);
    test!(true; and  false);
    test!(true; or false);
}