pub fn count_digits(mut n: u64) -> u64 {
    let mut count = 1;

    while n > 9 {
        n /= 10;
        count += 1;
    }

    count
}
