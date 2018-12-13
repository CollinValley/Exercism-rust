pub fn verse(n: i32) -> String {
    if n > 2 {
       return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1);
    } else if n == 2 {
       return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n-1);
    } else if n == 1 {
       return format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    } else {
       return format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut output = String::new();
    for i in (end..(start + 1)).rev() {
        output.push_str(&verse(i));
        if i != end { output.push('\n'); }
    }
    output
}
