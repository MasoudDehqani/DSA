pub fn digital_root(n: i64) -> i64 {
    let mut sum = 0;
    let mut curr = n;

    while curr > 0 {
        let rem = curr % 10;
        sum += rem;

        if curr / 10 <= 0 && sum >= 10 {
            curr = sum;
            sum = 0;
        } else {
            curr /= 10;
        }
    }

    sum
}
