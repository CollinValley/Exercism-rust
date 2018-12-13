fn is_prime(num: u32) -> bool {
    let mut ret = true;
    for i in  2 .. num {
        if num % i  == 0 {
            ret = false;
            break;
        }
    }
    ret
}

pub fn nth(n: u32) -> u32 {
    let mut max_prime = 2;
    let mut number = 2;
    let mut nth = n;
    while nth > 0 {
        number = number + 1;
        if is_prime(number) {
            max_prime = number;
            nth = nth - 1;
        }
    }
    max_prime
}
