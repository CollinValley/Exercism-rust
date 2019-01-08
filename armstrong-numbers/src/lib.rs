pub fn is_armstrong_number(num: u32) -> bool {
    let number_string = num.to_string();
    num == number_string.chars()
        .map( |c| c.to_digit(10).unwrap().pow(number_string.len() as u32))
        .sum()
}
