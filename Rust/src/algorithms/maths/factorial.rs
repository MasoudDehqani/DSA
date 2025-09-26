pub fn factorial(n: u64) -> u64 {
    match n < 2 {
        true => 1,
        false => factorial(n - 1) * n,
    }
}
