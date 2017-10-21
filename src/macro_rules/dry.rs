use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
// tt is used for operators and tokens
($a:ident, $b:ident, $func:ident, $op:tt) => (
    assert!($a.len() == $b.len(),
        "{:?}: dimension mismatch : {:?} {:?} {:?}",
        stringify!($func),
        ($a.len(),),
        stringify!($op),
        ($b.len(),)
    );
)
}

macro_rules! op {
($func:ident, $bound:ident, $op:tt, $method:ident) => (
    fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
        assert_equal_len!(xs, ys, $func, $op);

        for (x, y) in xs.iter_mut().zip(ys.iter()) {
            *x = $bound::$method(*x, *y);
        }
    }
)
}

op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

pub mod test {
    use std::iter;
    macro_rules! test {
    ($func:ident, $x:expr, $y:expr, $z:expr) => {
        #[test]
        fn $func() {
            for size in 0_usize..10 {
                let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                let y: Vec<_> = iter::repeat($y).take(size).collect();
                let z: Vec<_> = iter::repeat($z).take(size).collect();

                super::$func(&mut x, &y);
                assert_eq!(x, z);
            }
        } // end $func
    }
    }

    // try `rustc --test src/macro_rules/dry.rs && ./dry`
    test!(add_assign, 1_u32, 2_u32, 3_u32);
    test!(mul_assign, 2_u32, 3_u32, 6_u32);
    test!(sub_assign, 3_u32, 2_u32, 1_u32);
}