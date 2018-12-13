fn is_yelling(message: &str) -> bool {
    let mut has_uppercase_nonstart = false;
    for (i, c) in message.chars().enumerate() {
        match (i, c) {
            ( _, c) if c.is_lowercase() => {
                return false
            },
            ( i, c) if c.is_uppercase() && i > 0  => {
                has_uppercase_nonstart = true;
            }
            _ => {}
        }
    }
    has_uppercase_nonstart
}

fn is_question(message: &str) -> bool {
    for c in message.chars().rev() {
        match c {
            c if c.is_whitespace() => {},
            '?' => {
                return true
            },
            _ => {
                return false
            }
        }
    }
    false
}

#[test]
fn test_is_question() {
    assert!(is_question(":) ?"));
}

fn is_silent(message: &str) -> bool {
    for c in message.chars() {
        match c {
            c if c.is_whitespace() => {},
            _ => {
                return false
            }
        }
    }
    true
}
pub fn reply(message: &str) -> &str {
    let question = is_question(message);
    let yell = is_yelling(message);
    let silent = is_silent(message);

    match (question, yell, silent) {
        ( _, _, true) => {
            return "Fine. Be that way!"
        }
        ( true, true, _ ) => {
            return "Calm down, I know what I'm doing!"
        }
        ( _, true, _) => {
            return "Whoa, chill out!"
        }
        (true, _, _) => {
            return "Sure."
        }
        ( _, _, _) => {
            return "Whatever."
        }
    }
}
