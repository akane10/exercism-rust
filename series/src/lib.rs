pub fn series(digits: &str, len: usize) -> Vec<String> {
    let digits_len = digits.len();
    let mut vec: Vec<String> = Vec::new();
    let mut start = 0;
    let mut end = len;

    match len {
        0 => vec!["".to_string(); 6],
        _ => {
            loop {
                if digits_len < end {
                    break;
                }
                let x = digits[start..end].to_string();
                vec.push(x);
                start += 1;
                end += 1;
            }
            vec
        }
    }
}
