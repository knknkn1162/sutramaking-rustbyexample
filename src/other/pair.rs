use std::fmt::Display;

struct Pair<T> {
    x: T, y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T)-> Self {
        Self {
            x, y
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self)-> String {
        if self.x >= self.y {
            format!("The largest member is x = {}", self.x)
        } else {
            format!("The largest member is y = {}", self.y)
        }
    }
}

// only traits defined in the current crate can be implemented for a type parameter
impl<T: Display> ToString for T {
    fn to_string(&self)-> String {
        format!("{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_tostring() {
        let s = 3.to_string();

        assert_eq!(s, "3");
    }
}