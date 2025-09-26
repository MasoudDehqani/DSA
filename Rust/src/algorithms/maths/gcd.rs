pub fn gcd(n1: i32, n2: i32) -> i32 {
    fn helper(n1: i32, n2: i32) -> i32 {
        if n1 == 0 {
            return n2;
        } else if n2 == 0 {
            return n1;
        }

        helper(n2, n1 % n2)
    }

    match n1 > n2 {
        true => helper(n1, n2),
        false => helper(n2, n1),
    }
}

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn gcd_six_and_eight() {
        assert_eq!(2, gcd(6, 8))
    }

    #[test]
    fn gcd_twenty_and_fourty() {
        assert_eq!(20, gcd(40, 20))
    }

    #[test]
    fn gcd_twelve_and_sixteen() {
        assert_eq!(4, gcd(12, 16))
    }

    #[test]
    fn gcd_eleven_and_thirteen() {
        assert_eq!(1, gcd(13, 11))
    }

    #[test]
    fn gcd_eighteen_and_twelve() {
        assert_eq!(6, gcd(18, 12))
    }

    #[test]
    fn gcd_large_numbers() {
        assert_eq!(12, gcd(123456, 789012))
    }

    #[test]
    #[ignore]
    fn gcd_negative_numbers() {
        assert_eq!(4, gcd(-8, 12));
        assert_eq!(6, gcd(-18, -24));
        assert_eq!(1, gcd(-7, 13))
    }
}
