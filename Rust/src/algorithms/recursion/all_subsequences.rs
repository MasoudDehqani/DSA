pub fn print_all_subsequences(arr: &Vec<i32>) {
    fn aux(arr: &Vec<i32>, curr: usize, acc: &mut Vec<i32>) {
        if curr >= arr.len() {
            println!("{:?}", acc);
            return;
        }

        acc.push(arr[curr]);
        aux(arr, curr + 1, acc);
        acc.pop();
        aux(arr, curr + 1, acc)
    }

    aux(arr, 0, &mut vec![])
}

pub fn all_subsequences(arr: &Vec<i32>) -> Vec<Vec<i32>> {
    fn aux(arr: &Vec<i32>, curr: usize, acc: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if curr >= arr.len() {
            res.push(acc.clone());
            return;
        }

        acc.push(arr[curr]);
        aux(arr, curr + 1, acc, res);
        acc.pop();
        aux(arr, curr + 1, acc, res)
    }

    let mut res = vec![];
    aux(arr, 0, &mut vec![], &mut res);
    res
}
