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

// pub fn reverse_number_iterative(mut n: u64) -> u64 {
//     let mut res = 0;

//     while n > 0 {
//         let quotient = n / 10;
//         let rem = n % 10;
//         let rem_digit_place_value = n.ilog10();

//         res = res + (rem * 10_u64.pow(rem_digit_place_value));

//         n = quotient;
//     }

//     res
// }

// pub fn reverse_number_iterative(mut n: u64) -> u64 {
//     let mut rev = 0;
//     while n > 0 {
//         rev = rev * 10 + n % 10;
//         n /= 10;
//     }
//     rev
// }

// pub fn reverse(x: i32) -> i32 {
//     let mut n = x.abs();
//     let mut res: i32 = 0;

//     while n > 0 {
//         let quotient = n / 10;
//         let rem = n % 10;
//         let rem_digit_place_value = n.ilog10();

//         let (f, is_overflowing_1) = 10_i32.overflowing_pow(rem_digit_place_value);
//         let (s, is_overflowing_2) = rem.overflowing_mul(f);
//         let (new_res, is_overflowing_3) = res.overflowing_add(s);

//         if is_overflowing_1 || is_overflowing_2 || is_overflowing_3 {
//             return 0;
//         }

//         res = new_res;

//         n = quotient;
//     }

//     if x < 0 {
//         -res
//     } else {
//         res
//     }
// }

pub fn reverse_number_iterative(mut n: i32) -> i32 {
    let factor = if n.is_negative() { -1 } else { 1 };
    n = n.abs();
    let mut rev: i32 = 0;

    while n > 0 {
        let (multiplied, is_overflowing) = rev.overflowing_mul(10);
        if is_overflowing {
            return 0;
        }

        rev = multiplied + n % 10;
        n /= 10;
    }

    rev * factor
}
