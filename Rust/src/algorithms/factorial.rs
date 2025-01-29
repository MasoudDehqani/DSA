pub fn factorial(n: u64) -> u64 {
  if n < 3 {
    return n
  }

  factorial(n - 1) * n
}