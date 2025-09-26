use super::count_digits;

pub fn is_armstrong_number(mut num: u64) -> bool {
    let exp = count_digits(num);
    let original_number = num;
    let mut sum = 0;

    while num > 0 {
        let current_digit = num % 10;
        sum += current_digit.pow(exp as u32);
        num /= 10;
    }

    original_number == sum
}
