// pub fn is_prime(n: u32) -> bool {
//     let mut d = 2;

//     while d < n && d <= n.isqrt() {
//         if n % d == 0 {
//             return false;
//         }

//         d += 1;
//     }

//     n > 1
// }

pub fn is_prime(n: u32) -> bool {
    for i in 2..=n.isqrt() {
        if n % i == 0 {
            return false;
        }
    }

    n > 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_and_one_are_not_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1))
    }

    #[test]
    fn two_and_three_are_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3))
    }

    #[test]
    fn even_numbers_are_not_prime() {
        assert!(!is_prime(4));
        assert!(!is_prime(102))
    }

    #[test]
    fn random_prime_numbers() {
        assert!(is_prime(13));
        assert!(is_prime(17));
        assert!(is_prime(19));
        assert!(is_prime(23));
        assert!(is_prime(149));
        assert!(is_prime(263));
        assert!(is_prime(7841))
    }
}
