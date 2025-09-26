pub fn count_digits_tail_rec(n: u64) -> u64 {
    fn helper(n: u64, acc: u64) -> u64 {
        if n / 10 < 1 {
            return acc + 1;
        }

        helper(n / 10, acc + 1)
    }

    helper(n, 0)
}

pub fn count_digits_non_tail_rec(n: u64) -> u64 {
    if n < 10 {
        return 1;
    }

    count_digits_non_tail_rec(n / 10) + 1
}
