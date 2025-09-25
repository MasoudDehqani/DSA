pub fn count_digits(n: u32) -> u32 {
    fn helper(n: u32, acc: u32) -> u32 {
        if n / 10 < 1 {
            return acc + 1;
        }

        helper(n / 10, acc + 1)
    }

    helper(n, 0)
}
