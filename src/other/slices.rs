pub fn first_word_len(s: &str)-> usize {
    s.as_bytes()
        .iter()
        .enumerate()
        .filter_map(|(idx, &elem)| if elem == b' ' {Some(idx as usize)} else {None})
        .nth(0)
        .unwrap_or(s.len())
}

pub fn first_word(s: &str) -> &str {
    s.as_bytes()
        .iter()
        .enumerate()
        .filter_map(|(idx, &elem)|if elem == b' ' {Some(&s[0..idx])} else {None})
        .nth(0)
        .unwrap_or(&s[..])
}