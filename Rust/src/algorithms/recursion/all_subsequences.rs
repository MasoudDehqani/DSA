pub fn print_all_subsequences(seq: &Vec<i32>) {
    fn aux(seq: &Vec<i32>, i: usize, sub: &mut Vec<i32>) {
        if i >= seq.len() {
            println!("{:?}", sub);
            return;
        }

        sub.push(seq[i]);
        aux(seq, i + 1, sub);
        sub.pop();
        aux(seq, i + 1, sub)
    }

    aux(seq, 0, &mut vec![])
}

pub fn all_subsequences(seq: &Vec<i32>) -> Vec<Vec<i32>> {
    fn aux(seq: &Vec<i32>, i: usize, sub: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if i >= seq.len() {
            res.push(sub.clone());
            return;
        }

        sub.push(seq[i]);
        aux(seq, i + 1, sub, res);
        sub.pop();
        aux(seq, i + 1, sub, res)
    }

    let mut res = vec![];
    aux(seq, 0, &mut vec![], &mut res);
    res
}
