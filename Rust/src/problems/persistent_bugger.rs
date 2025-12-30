fn multiply_of_digits(mut num: u64) -> u64 {
    let mut res = 1;

    while num > 0 {
        res *= num % 10;
        num /= 10;
    }

    res
}

pub fn persistence(mut num: u64) -> u64 {
    let mut c = 0;

    while num > 9 {
        num = multiply_of_digits(num);
        c += 1;
    }

    c
}
