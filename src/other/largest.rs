use std::cmp::PartialOrd;

// return a reference to a T value in the slice
pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    // item: &T
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn largest_test() {
        let number_list = vec![34, 50, 25];

        let result = largest(&number_list);
        assert_eq!(result, &50);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);

        assert_eq!(result, &'y');
    }
}
