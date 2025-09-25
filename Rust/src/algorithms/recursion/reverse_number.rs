pub fn reverse_number_recursive(n: u64) -> u64 {
    if n < 10 {
        return n;
    }

    let quotient = n / 10;
    let rem = n % 10;
    let rem_digit_place_value = quotient.ilog10() + 1;

    reverse_number_recursive(quotient) + (rem * 10_u64.pow(rem_digit_place_value))
}

pub fn reverse_number_iterative(mut n: u64) -> u64 {
    let mut res = 0;

    while n > 0 {
        let quotient = n / 10;
        let rem = n % 10;
        let rem_digit_place_value = n.ilog10();

        res = res + (rem * 10_u64.pow(rem_digit_place_value));

        n = quotient;
    }

    res
}

// pub fn reverse_number_iterative(mut n: u64) -> u64 {
//     let mut rev = 0;
//     while n > 0 {
//         rev = rev * 10 + n % 10;
//         n /= 10;
//     }
//     rev
// }
