// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let mut is_even = true;
    let mut sum = 0;

    let cc_number: String = cc_number.chars().filter(|c| !c.is_whitespace()).collect();
    if cc_number.len() < 2 {
        return false;
    }

    println!("{}", cc_number);
    for c in cc_number.chars().rev() {
        is_even = !is_even;

        let digit = c.to_digit(10);
        if digit == None {
            return false
        }
        let mut n = digit.unwrap();

        if is_even {
            n = n*2;
            if n > 9 {
                n = n/10 + n%10;
            }
        }
        sum += n;
        println!("{}: {}", is_even, n);
    }
    return sum % 10 == 0;
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

fn main() {
    let cc_number = "8273 1232 7352 0569";
    luhn(cc_number);
}