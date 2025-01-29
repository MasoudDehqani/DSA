pub fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 1;
    }

    factorial(n - 1) * n
}
