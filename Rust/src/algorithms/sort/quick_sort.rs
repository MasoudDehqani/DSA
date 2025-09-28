// fn helper(arr: &mut [i32], start: usize, end: usize) {
//     if start >= end {
//         return;
//     }

//     let mut pivot = end;
//     let mut i: i32 = start as i32 - 1;
//     let mut j = start;

//     while j < pivot {
//         if arr[j] >= arr[pivot] {
//             j += 1;
//             continue;
//         } else {
//             i += 1;
//             arr.swap(i as usize, j);
//             j += 1;
//         }
//     }

//     i += 1;
//     arr.swap(i as usize, pivot);
//     pivot = i as usize;

//     helper(arr, start, pivot.saturating_sub(1));
//     helper(arr, pivot + 1, end);
// }

// pub fn quick_sort(arr: &mut [i32]) {
//     let end = arr.len().saturating_sub(1);
//     helper(arr, 0, end);
// }

fn partition(arr: &mut [i32], start: usize, end: usize) -> usize {
    let pivot = arr[end];
    let mut i = start;

    for j in start..end {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, end);
    i
}

fn quick_sort_helper(arr: &mut [i32], start: usize, end: usize) {
    if start < end {
        let pivot = partition(arr, start, end);
        if pivot > 0 {
            quick_sort_helper(arr, start, pivot - 1);
        }
        quick_sort_helper(arr, pivot + 1, end);
    }
}

pub fn quick_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }
    let end = arr.len() - 1;
    quick_sort_helper(arr, 0, end);
}

#[cfg(test)]
mod tests {
    use super::quick_sort;

    #[test]
    fn test_quick_sort() {
        let mut arr = [5, 3, 8, 4, 2];
        quick_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 8]);
    }

    #[test]
    fn test_quick_sort_empty() {
        let mut arr = [];
        quick_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_quick_sort_single_element() {
        let mut arr = [5];
        quick_sort(&mut arr);
        assert_eq!(arr, [5]);
    }

    #[test]
    fn test_quick_sort_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_reverse() {
        let mut arr = [5, 4, 3, 2, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_duplicate() {
        let mut arr = [5, 3, 8, 4, 2, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 5, 8]);
    }

    #[test]
    fn test_quick_sort_duplicate_sorted() {
        let mut arr = [2, 3, 4, 5, 5, 8];
        quick_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 5, 8]);
    }

    #[test]
    fn test_quick_sort_duplicate_sorted_reverse() {
        let mut arr = [8, 5, 5, 4, 3, 2];
        quick_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 5, 8]);
    }

    #[test]
    fn test_quick_sort_duplicate_sorted_reverse_duplicate() {
        let mut arr = [8, 5, 5, 4, 3, 2, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 5, 5, 8]);
    }
}
