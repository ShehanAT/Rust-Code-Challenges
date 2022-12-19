#![allow(unused)]
use std::str::FromStr;

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}


#[derive(Debug, PartialEq)]
enum InvalidInput {
    TooShort,
    TooLong, 
    InvalidCharacter(u8, char),
    FailedCheckSum,
}

impl FromStr for Isbn {
    type Err = InvalidInput; 

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut digits_w_removed_hypens = s.replace("-", "");

        digits_w_removed_hypens.remove(digits_w_removed_hypens.chars().count() - 1); // Removing last digit
        
        let mut filtered_digits = Vec::<u8>::from([]);

        for (i, c) in digits_w_removed_hypens.char_indices() {
            if !c.eq(&'-') && !c.is_numeric() {
                return Err(InvalidInput::InvalidCharacter(i as u8, c))
            }
            filtered_digits.push(c.to_digit(10).unwrap() as u8);
        }
        
        let check_digit = calculate_check_digit(&filtered_digits);

        if s.chars().count() > 17 {
            return Err(InvalidInput::TooLong);
        } else if s.chars().count() < 17 {
            return Err(InvalidInput::TooShort);
        }
        if filtered_digits[filtered_digits.len() - 1] != check_digit {
            return Err(InvalidInput::FailedCheckSum);
        }

        Ok(Isbn { raw: s.to_string(), digits: filtered_digits })
    }
}

impl std::fmt::Display for Isbn {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

fn calculate_check_digit(digits: &[u8]) -> u8 {
    let mut one_multiplication_nums = vec![];
    let mut three_multiplication_nums = vec![];
    
    let digits_length = digits.len();

    let mut digit_sum = 0 as u8;

    for even_num in (0..digits_length).step_by(2) {
        let weighted_num = digits.get(even_num).unwrap() * 1;
        one_multiplication_nums.push(weighted_num);
        digit_sum += weighted_num;
    } 
    for odd_num in (1..digits_length).step_by(2) {
        let weighted_num = digits.get(odd_num).unwrap() * 3;
        three_multiplication_nums.push(weighted_num);
        digit_sum += weighted_num;
    } 
    let digit_sum_remainder = digit_sum % 10;
    let last_digit = ((10 - digit_sum_remainder) % 10) as u8;

    last_digit
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();
    println!("Rust in Action's ISBN-13 ({})is valid!", rust_in_action);
}

#[test]
fn can_correctly_calcuate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}? {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }

}

#[test]
fn rust_in_action() {
    // calculate_check_digit_test(&[9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 0, 6]);
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
    
}