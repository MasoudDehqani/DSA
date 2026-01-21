pub fn count_1_to_n(n: u32) {
    if n < 1 {
        return;
    }

    count_1_to_n(n - 1);
    println!("{}", n)
}

pub fn count_1_to_n_linearly(n: u32) {
    fn aux(n: u32, i: u32) {
        if i > n {
            return;
        }

        println!("{}", i);
        aux(n, i + 1)
    }

    aux(n, 1)
}

pub fn count_n_to_1(n: u32) {
    if n < 1 {
        return;
    }

    println!("{}", n);
    count_n_to_1(n - 1)
}
