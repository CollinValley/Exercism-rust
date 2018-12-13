pub fn reverse(input: &str) -> String {
    let mut output = String::new();
    for character in input.chars().rev() {
        output.push(character);
    }
    output
}
