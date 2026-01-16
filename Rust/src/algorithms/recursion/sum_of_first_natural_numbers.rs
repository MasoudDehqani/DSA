pub fn sum_of_first_natural_numbers(n: u32) -> u32 {
    if n < 1 {
        return n;
    }

    n + sum_of_first_natural_numbers(n - 1)
}

pub fn sum_of_first_natural_numbers_tail_recursive(n: u32) -> u32 {
    fn aux(n: u32, acc: u32) -> u32 {
        if n < 1 {
            return acc;
        }

        aux(n - 1, acc + n)
    }

    aux(n, 0)
}
