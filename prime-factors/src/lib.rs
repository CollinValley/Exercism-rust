use std::ops::Not;

fn is_prime(n: u64) -> bool {
    if n == 2 { return true }
    (2 .. (n/2 + 1)).any(|a| n % a == 0).not()
}

#[test]
fn test_is_prime() {
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(!is_prime(8));
    assert!(is_prime(17));
}

fn get_next_prime(mut n: u64) -> u64{
    n += 1;
    while !is_prime(n) {
        n += 1;
    }
    n
}

#[test]
fn test_get_next_prime() {
    assert_eq!(3, get_next_prime(2));
    assert_eq!(7, get_next_prime(5));
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut num = n;
    let mut prime_factor = 2;
    let mut prime_factors = Vec::new();

    if num < 2 { return vec![] }
    if num == 2 { return vec![2] }

    while num != prime_factor {
        if num % prime_factor == 0 {
            num = num / prime_factor ;
            prime_factors.push(prime_factor);
        } else {
            prime_factor = get_next_prime(prime_factor);
        }
    }
    prime_factors.push(prime_factor);
    prime_factors
}
