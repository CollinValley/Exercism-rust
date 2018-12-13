pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() == 0 || len > digits.len() {
        return vec![];
    } else if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }
    // given a string of digits, output all the series into a vector of strings
    let mut output = Vec::<String>::new();
    for i in 0..digits.len() {
        if let Some(sub_str) = digits.get(i..(i+len)) {
            output.push(sub_str.to_string());
        }
    }
    output
}
