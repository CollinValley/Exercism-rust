pub fn is_armstrong_number(num: u32) -> bool {
    let number_string = num.to_string();
    let sum : u32 = number_string.chars().map(|c| u32::from(c).pow(number_string.len() as u32)).sum();
    sum == num
}

#[test]
fn test_stuff() {
    let number_string = 55_u32.to_string();
    assert_eq!("55", number_string);
    let sum :u32 = number_string.chars().map(|c| u32::from(c) ).sum();
    assert_eq!(10, sum);
}
