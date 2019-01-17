fn say_zero() -> String {
    "zero".to_string()
}

fn say_ones(n: u64) -> String {
    match n {
        0 => String::new(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        _ => "ERROR ONES NUMBER OUT OF BOUNDS".to_string(),
    }
}

fn say_teens(n: u64) -> String {
    match n {
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        _ => "ERROR TEENS NUMBER OUT OF BOUNDS".to_string(),
    }
}

fn say_tens(n: u64) -> String {
    let ones = n % 10;
    let mut ret_string;
    match n / 10 {
        0 => { ret_string = String::new(); },
        1 => { return say_teens(n); },
        2 => { ret_string = "twenty".to_string(); },
        3 => { ret_string = "thirty".to_string(); },
        4 => { ret_string = "forty".to_string(); },
        5 => { ret_string = "fifty".to_string(); },
        6 => { ret_string = "sixty".to_string(); },
        7 => { ret_string = "seventy".to_string(); },
        8 => { ret_string = "eighty".to_string(); },
        9 => { ret_string = "ninety".to_string(); },
        _ => { return "ERROR TENS OUT OF BOUNDS".to_string(); },
    }
    if ones != 0{
        if !ret_string.is_empty() {
            ret_string.push_str("-");
        }
        ret_string.push_str(&say_ones(ones));
    }
    ret_string
}

fn say_hundreds(n: u64) -> String {
    let hundreds = n / 100;
    let tens = n % 100;
    let mut ret_string = String::new();
    if hundreds != 0{
        ret_string = say_ones(hundreds);
        ret_string.push_str(" hundred");
    }
    if tens != 0 {
        if !ret_string.is_empty() {
            ret_string.push_str(" ");
        }
        ret_string.push_str(&say_tens(tens));
    }
    ret_string
}

fn say_big_number(n: u64) -> String {
    let mut num = n;
    let mut ret_string: String = String::new();

    let quintillions: u64 = num / 1_000_000_000_000_000_000;
    if quintillions != 0 {
        if !ret_string.is_empty() {
            ret_string.push_str(" ");
        }
        ret_string.push_str(&say_hundreds(quintillions));
        ret_string.push_str(" quintillion");
    }
    num = num % 1_000_000_000_000_000_000;

    let quadrillions: u64 = num / 1_000_000_000_000_000;
    if quadrillions != 0 {
        if !ret_string.is_empty() {
            ret_string.push_str(" ");
        }
        ret_string.push_str(&say_hundreds(quadrillions));
        ret_string.push_str(" quadrillion");
    }
    num = num % 1_000_000_000_000_000;

    let trillions: u64 = num / 1_000_000_000_000;
    if trillions != 0 {
        if !ret_string.is_empty() {
            ret_string.push_str(" ");
        }
        ret_string.push_str(&say_hundreds(trillions));
        ret_string.push_str(" trillion");
    }
    num = num % 1_000_000_000_000;

    let billions: u64 = num / 1_000_000_000;
    if billions != 0 {
        if !ret_string.is_empty() {
            ret_string.push_str(" ");
        }
        ret_string.push_str(&say_hundreds(billions));
        ret_string.push_str(" billion");
    }
    num = num % 1_000_000_000;

    let millions: u64 = num / 1_000_000;
    if millions != 0 {
        if !ret_string.is_empty() {
            ret_string.push_str(" ");
        }
        ret_string.push_str(&say_hundreds(millions));
        ret_string.push_str(" million");
    }
    num = num % 1_000_000;

    let thousands: u64 = num / 1_000;
    if thousands != 0 {
        if !ret_string.is_empty() {
            ret_string.push_str(" ");
        }
        ret_string.push_str(&say_hundreds(thousands));
        ret_string.push_str(" thousand");
    }
    num = num % 1000;
    if num != 0 {
        if !ret_string.is_empty() {
            ret_string.push_str(" ");
        }
        ret_string.push_str(&say_hundreds(num));
    }
    ret_string
}

pub fn encode(n: u64) -> String {
    if n == 0 {
        say_zero()
    } else if n < 10 {
        say_ones(n)
    } else if n < 20 {
        say_teens(n)
    } else if n < 100 {
        say_tens(n)
    } else if n < 1000 {
        say_hundreds(n)
    } else {
        say_big_number(n)
    }
}
