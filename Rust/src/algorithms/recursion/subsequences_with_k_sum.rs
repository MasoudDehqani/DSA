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
