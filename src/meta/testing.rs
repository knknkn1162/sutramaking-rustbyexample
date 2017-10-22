#[cfg(not(test))]
fn main() {
    println!("if you see this");
}

#[cfg(test)]
mod test {
    fn distance(a: (f32, f32), b: (f32, f32))-> f32 {
        ((b.0-a.0).powi(2)+(b.1-a.1).powi(2)).sqrt()
    }

    #[test]
    fn distance_test() {
        assert!(distance((0_f32, 0_f32), (1_f32, 1_f32)) == 2_f32.sqrt());
    }

    #[test]
    #[should_panic]
    fn failing_test() {
        assert!(1_i32 == 2_i32);
    }
}