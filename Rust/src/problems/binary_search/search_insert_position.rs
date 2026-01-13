use std::cmp::Ordering;

pub fn search_insert_position(arr: &[i32], target: i32) -> usize {
    let mut start = 0;
    let mut end = arr.len();
    let mut middle = 0;

    while start < end {
        middle = (start + end) / 2;

        match target.cmp(&arr[middle]) {
            Ordering::Equal => return middle,
            Ordering::Greater => start = middle + 1,
            Ordering::Less => end = middle,
        }
    }

    if arr[middle] > target {
        middle
    } else {
        middle + 1
    }
}
