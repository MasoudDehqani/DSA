pub fn print_all_subsequences(seq: &Vec<i32>) {
    fn aux(seq: &Vec<i32>, curr: usize, acc: &mut Vec<i32>) {
        if curr >= seq.len() {
            println!("{:?}", acc);
            return;
        }

        acc.push(seq[curr]);
        aux(seq, curr + 1, acc);
        acc.pop();
        aux(seq, curr + 1, acc)
    }

    aux(seq, 0, &mut vec![])
}

pub fn all_subsequences(seq: &Vec<i32>) -> Vec<Vec<i32>> {
    fn aux(seq: &Vec<i32>, curr: usize, acc: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if curr >= seq.len() {
            res.push(acc.clone());
            return;
        }

        acc.push(seq[curr]);
        aux(seq, curr + 1, acc, res);
        acc.pop();
        aux(seq, curr + 1, acc, res)
    }

    let mut res = vec![];
    aux(seq, 0, &mut vec![], &mut res);
    res
}
