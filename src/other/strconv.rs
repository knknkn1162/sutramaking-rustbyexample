use std::str;

fn strconv(b: &[u8])->&str {
    str::from_utf8(b).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strconv_test() {
        let b = "あいaib".as_bytes();
        let res = strconv(b);

        assert_eq!("あいaib", res);
    }
}