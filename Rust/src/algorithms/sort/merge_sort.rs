// fn helper(arr: &mut [i32], start: usize, end: usize) {
//     if end - start < 1 {
//         return;
//     }

//     let mid = (start + end) / 2;

//     helper(arr, start, mid);
//     helper(arr, mid + 1, end);

//     println!("start: {start}, mid: {mid}, end: {end}");

//     let mut sub = vec![];
//     for i in start..=mid {
//         for j in mid..=end {
//             println!("i: {i}, j: {j}");
//             if arr[i] < arr[j] && i != j {
//                 sub.push((i, arr[i]));
//             } else {
//                 sub.push((j, arr[j]));
//             }
//         }
//     }
//     println!("VEC: {sub:?}");
//     sub.into_iter().for_each(|(i, e)| arr[i] = e);
//     println!("ARR: {arr:?}");
// }

// pub fn merge_sort(arr: &mut [i32]) {
//     println!("{:?}", arr);
//     let end = arr.len().saturating_sub(1);
//     helper(arr, 0, end);
// }

pub fn merge_sort(arr: &[i32]) -> Vec<i32> {
    if arr.len() < 2 {
        return arr.to_vec();
    }

    let mid = arr.len() / 2;

    let left = merge_sort(&arr[0..mid]);
    let right = merge_sort(&arr[mid..]);

    // let mut res = vec![];
    let mut res = Vec::with_capacity(arr.len());
    let mut left_current = 0;
    let mut right_current = 0;

    loop {
        match (left.get(left_current), right.get(right_current)) {
            (Some(&l), Some(&r)) => {
                if l < r {
                    res.push(l);
                    left_current += 1;
                } else {
                    res.push(r);
                    right_current += 1;
                }
            }
            (Some(&l), None) => {
                res.push(l);
                left_current += 1;
            }
            (None, Some(&r)) => {
                res.push(r);
                right_current += 1;
            }
            (None, None) => break res,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let arr = [5, 3, 8, 4, 2];
        assert_eq!(merge_sort(&arr), [2, 3, 4, 5, 8]);
    }

    #[test]
    fn test_merge_sort_empty() {
        let arr = [];
        assert_eq!(merge_sort(&arr), []);
    }

    #[test]
    fn test_merge_sort_single_element() {
        let arr = [5];
        assert_eq!(merge_sort(&arr), [5]);
    }

    #[test]
    fn test_merge_sort_sorted() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(merge_sort(&arr), [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_reverse() {
        let arr = [5, 4, 3, 2, 1];
        assert_eq!(merge_sort(&arr), [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_duplicate() {
        let arr = [5, 3, 8, 4, 2, 5];
        assert_eq!(merge_sort(&arr), [2, 3, 4, 5, 5, 8]);
    }

    #[test]
    fn test_merge_sort_duplicate_sorted() {
        let arr = [2, 3, 4, 5, 5, 8];
        assert_eq!(merge_sort(&arr), [2, 3, 4, 5, 5, 8]);
    }

    #[test]
    fn test_merge_sort_duplicate_sorted_reverse() {
        let arr = [8, 5, 5, 4, 3, 2];
        assert_eq!(merge_sort(&arr), [2, 3, 4, 5, 5, 8]);
    }

    #[test]
    fn test_merge_sort_duplicate_sorted_reverse_duplicate() {
        let arr = [8, 5, 5, 4, 3, 2, 5];
        assert_eq!(merge_sort(&arr), [2, 3, 4, 5, 5, 5, 8]);
    }
}
