// use std::{thread, time::Duration};

#[allow(unused_variables)]
fn combination_sum_aux(
    arr: &Vec<i32>,
    target: i32,
    current_index: usize,
    comb_index: usize,
    acc: i32,
    comb: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    // if current_index >= arr.len() || comb_index >= arr.len() || target - acc < 0 {
    //     return;
    // }

    // println!("CURR_INDEX: {}, COMB_INDEX: {}", current_index, comb_index);

    // thread::sleep(Duration::from_millis(500));

    // let next_comb_index = if acc + arr[comb_index] < target {
    //     current_index
    // } else {
    //     comb_index + 1
    // };

    // combination_sum_aux(
    //     arr,
    //     target,
    //     current_index,
    //     next_comb_index,
    //     0,
    //     &mut vec![],
    //     res,
    // );

    // combination_sum_aux(
    //     arr,
    //     target,
    //     current_index + 1,
    //     comb_index + 1,
    //     0,
    //     &mut vec![],
    //     res,
    // );

    // combination_sum_aux(
    //     arr,
    //     target,
    //     current_index,
    //     comb_index + 1,
    //     0,
    //     &mut vec![],
    //     res,
    // );
}

pub fn combination_sum(arr: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    combination_sum_aux(&arr, target, 0, 0, 0, &mut vec![], &mut res);
    res
}
