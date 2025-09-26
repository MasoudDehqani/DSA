pub fn fibonacci(n: u64) -> u64 {
    match n < 2 {
        true => n,
        false => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
