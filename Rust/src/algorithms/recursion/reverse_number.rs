pub fn reverse_number_non_tail_rec(n: u64) -> u64 {
    if n < 10 {
        return n;
    }

    let quotient = n / 10;
    let rem = n % 10;
    let rem_digit_place_value = quotient.ilog10() + 1;

    reverse_number_non_tail_rec(quotient) + (rem * 10_u64.pow(rem_digit_place_value))
}

pub fn reverse_number_tail_rec(n: u64) -> u64 {
    fn helper(n: u64, acc: u64) -> u64 {
        if n < 1 {
            return acc;
        }

        helper(n / 10, acc * 10 + n % 10)
    }

    helper(n, 0)
}
