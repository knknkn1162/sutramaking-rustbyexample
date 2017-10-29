struct Point<T> {
    x: T, y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn point_test() {
        let p = Point {x: 5, y: 10};

        assert_eq!(p.x(), &5);
    }
}