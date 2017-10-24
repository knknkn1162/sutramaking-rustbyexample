pub fn first_word(s: &str)-> usize {
    s.as_bytes()
        .iter()
        .enumerate()
        .filter_map(|(idx, &elem)| if elem == b' ' {Some(idx as usize)} else {None})
        .nth(0)
        .unwrap_or(s.len())
}