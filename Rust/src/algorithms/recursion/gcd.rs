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
