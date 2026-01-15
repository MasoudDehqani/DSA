use std::cmp::Ordering;

use crate::problems::binary_search::{lower_bound, upper_bound};

fn first_occ(arr: &[i32], target: i32) -> Option<usize> {
    let mut start = 0;
    let mut end = arr.len();
    let mut f = None;

    while start < end {
        let mid = (start + end) / 2;

        match target.cmp(&arr[mid]) {
            Ordering::Equal => {
                f = Some(mid);
                end = mid;
            }
            Ordering::Greater => start = mid + 1,
            Ordering::Less => end = mid,
        }
    }

    f
}

fn last_occ(arr: &[i32], target: i32) -> Option<usize> {
    let mut start = 0;
    let mut end = arr.len();
    let mut l = None;

    while start < end {
        let mid = (start + end) / 2;

        match target.cmp(&arr[mid]) {
            Ordering::Equal => {
                l = Some(mid);
                start = mid + 1;
            }
            Ordering::Greater => start = mid + 1,
            Ordering::Less => end = mid,
        }
    }

    l
}

// pub fn first_and_last_occ(arr: &[i32], target: i32) -> (Option<usize>, Option<usize>) {
//     (first_occ(arr, target), last_occ(arr, target))
// }

pub fn first_and_last_occ(arr: &[i32], target: i32) -> (Option<usize>, Option<usize>) {
    let lb = lower_bound(arr, target);
    // let f =  if lb == arr.len() || arr[lb] != target {None} else {Some(lb)};
    let f = (lb < arr.len() && arr[lb] == target).then_some(lb);
    (f, Some(upper_bound(arr, target) - 1))
}
