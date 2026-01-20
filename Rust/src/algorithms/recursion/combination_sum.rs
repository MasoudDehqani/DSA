fn combination_sum_aux(
    arr: &Vec<i32>,
    target: i32,
    i: usize,
    mut sum: i32,
    comb: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    if i >= arr.len() || sum >= target {
        if sum == target {
            res.push(comb.clone());
        }

        return;
    }

    comb.push(arr[i]);
    sum += arr[i];
    combination_sum_aux(arr, target, i, sum, comb, res);

    comb.pop();
    sum -= arr[i];
    combination_sum_aux(arr, target, i + 1, sum, comb, res);
}

pub fn combination_sum(arr: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    combination_sum_aux(&arr, target, 0, 0, &mut vec![], &mut res);
    res
}
