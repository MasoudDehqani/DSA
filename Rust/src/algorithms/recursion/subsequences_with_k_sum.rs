pub fn print_subsequences_with_k_sum(seq: &Vec<i32>, k: i32) {
    fn aux(seq: &Vec<i32>, k: i32, i: usize, sub: &mut Vec<i32>) {
        if i >= seq.len() {
            if sub.iter().sum::<i32>() == k {
                println!("{:?}", sub);
            }

            return;
        }

        sub.push(seq[i]);
        aux(seq, k, i + 1, sub);
        sub.pop();
        aux(seq, k, i + 1, sub);
    }

    aux(seq, k, 0, &mut vec![]);
}

pub fn subsequences_with_k_sum(seq: &Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    fn aux(
        seq: &Vec<i32>,
        k: i32,
        i: usize,
        sub: &mut Vec<i32>,
        mut sum: i32,
        res: &mut Vec<Vec<i32>>,
    ) {
        if i >= seq.len() {
            if sum == k {
                res.push(sub.clone());
            }

            return;
        }

        sub.push(seq[i]);
        sum += seq[i];
        aux(seq, k, i + 1, sub, sum, res);
        sub.pop();
        sum -= seq[i];
        aux(seq, k, i + 1, sub, sum, res);
    }

    let mut res = vec![];
    aux(seq, k, 0, &mut vec![], 0, &mut res);
    res
}

pub fn any_one_subsequence_with_k_sum(seq: &Vec<i32>, k: i32) -> Vec<i32> {
    fn aux(seq: &Vec<i32>, k: i32, i: usize, sub: &mut Vec<i32>, mut sum: i32) -> bool {
        if i >= seq.len() {
            if sum == k {
                return true;
            }

            return false;
        }

        sub.push(seq[i]);
        sum += seq[i];
        if aux(seq, k, i + 1, sub, sum) {
            return true;
        }
        sub.pop();
        sum -= seq[i];
        if aux(seq, k, i + 1, sub, sum) {
            return true;
        }

        return false;
    }

    let mut sub = vec![];
    aux(seq, k, 0, &mut sub, 0);
    sub
}

// pub fn count_subsequences_with_k_sum(seq: &Vec<i32>, k: i32) -> i32 {
//     fn aux(seq: &Vec<i32>, k: i32, i: usize, count: &mut i32, mut sum: i32) {
//         if i >= seq.len() {
//             if sum == k {
//                 *count += 1;
//             }

//             return;
//         }

//         sum += seq[i];
//         aux(seq, k, i + 1, count, sum);

//         sum -= seq[i];
//         aux(seq, k, i + 1, count, sum);
//     }

//     let mut count = 0;
//     aux(seq, k, 0, &mut count, 0);
//     count
// }

pub fn count_subsequences_with_k_sum(seq: &Vec<i32>, k: i32) -> i32 {
    fn aux(seq: &Vec<i32>, k: i32, i: usize, mut sum: i32) -> i32 {
        // only applicable on positive sequences
        // if sum > k {
        //     return 0;
        // }
        if i >= seq.len() {
            if sum == k {
                return 1;
            }

            return 0;
        }

        sum += seq[i];
        let l = aux(seq, k, i + 1, sum);

        sum -= seq[i];
        let r = aux(seq, k, i + 1, sum);

        l + r
    }

    aux(seq, k, 0, 0)
}
