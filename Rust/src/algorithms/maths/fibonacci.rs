use std::collections::HashMap;

pub fn fibonacci(n: u64) -> u64 {
    match n < 2 {
        true => n,
        false => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn fibonacci_memoization(n: u64) -> u64 {
    let mut mem = HashMap::new();

    match mem.get(&n) {
        Some(&f) => f,
        None => {
            let f = fibonacci(n);
            mem.insert(n, f);

            f
        }
    }
}

// pub fn fibonacci_memoization(n: u64) -> u64 {
//     let mut mem = HashMap::new();

//     fn aux(n: u64, mem: &mut HashMap<u64, u64>) -> u64 {
//         if n < 2 {
//             return n;
//         }

//         match mem.get(&n) {
//             Some(&f) => f,
//             None => {
//                 let f = aux(n - 1, mem) + aux(n - 2, mem);
//                 mem.insert(n, f);

//                 f
//             }
//         }
//     }

//     aux(n, &mut mem)
// }
