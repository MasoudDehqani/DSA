pub fn fibonacci(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}
