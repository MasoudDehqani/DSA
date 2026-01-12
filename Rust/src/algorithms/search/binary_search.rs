// fn binary_search_helper(arr: &[i32], value: i32, start: usize, end: usize) -> Option<usize> {
//     let middle = (start + end).div_ceil(2);

//     if start >= end {
//         if value == arr[middle] {
//             return Some(middle);
//         } else {
//             return None;
//         }
//     }

//     match value.cmp(&arr[middle]) {
//         Ordering::Equal => Some(middle),
//         Ordering::Greater => binary_search_helper(arr, value, middle + 1, end),
//         Ordering::Less => binary_search_helper(arr, value, start, middle - 1),
//     }
// }

// fn binary_search_helper(arr: &[i32], value: i32, start: usize, end: usize) -> Option<usize> {
//     if start > end {
//         return None;
//     }

//     let middle = (start + end) / 2;

//     match value.cmp(&arr[middle]) {
//         Ordering::Equal => Some(middle),
//         Ordering::Greater => binary_search_helper(arr, value, middle + 1, end),
//         Ordering::Less => {
//             if middle == 0 {
//                 None // prevent underflow
//             } else {
//                 binary_search_helper(arr, value, start, middle - 1)
//             }
//         }
//     }
// }

// pub fn binary_search(arr: &[i32], value: i32) -> Option<usize> {
//     match arr.is_empty() {
//         true => None,
//         false => binary_search_helper(arr, value, 0, arr.len() - 1),
//     }
// }

/*
  leetcode problem
*/
// pub fn search(nums: Vec<i32>, target: i32) -> i32 {
//     let mut start = 0;
//     let mut end: isize = (nums.len() - 1) as isize;

//     while end >= start {
//         let mid = (start + end) / 2;
//         let mid_val = nums[mid as usize];

//         if target > mid_val {
//             start = mid + 1;
//         } else if target < mid_val {
//             end = mid - 1;
//         } else {
//             return mid as i32;
//         }
//     }

//     -1
// }

use std::cmp::Ordering;

pub fn binary_search(arr: &[i32], value: i32) -> Option<usize> {
    let mut start = 0;
    let mut end = arr.len();

    while start < end {
        let middle = (start + end) / 2;
        match value.cmp(&arr[middle]) {
            Ordering::Equal => return Some(middle),
            Ordering::Greater => start = middle + 1,
            Ordering::Less => end = middle,
        }
    }

    None
}

pub fn binary_search_recursive(arr: &[i32], n: i32) -> Option<usize> {
    fn aux(arr: &[i32], n: i32, start: usize, end: usize) -> Option<usize> {
        if start >= end {
            return None;
        }

        let mid = (start + end) / 2;

        match n.cmp(&arr[mid]) {
            Ordering::Equal => Some(mid),
            Ordering::Greater => aux(&arr, n, mid + 1, end),
            Ordering::Less => aux(&arr, n, start, mid),
        }
    }

    aux(arr, n, 0, arr.len())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_array() {
        let arr = &[];

        assert_eq!(None, binary_search(arr, 10));
        assert_eq!(None, binary_search(arr, 0));
        assert_eq!(None, binary_search(arr, -1));
    }

    #[test]
    fn single_element_array() {
        let arr = &[8];

        assert_eq!(None, binary_search(arr, -3));
        assert_eq!(None, binary_search(arr, 0));
        assert_eq!(None, binary_search(arr, 4));
        assert_eq!(Some(0), binary_search(arr, 8))
    }

    #[test]
    fn small_array() {
        let arr = &[1, 2];

        assert_eq!(Some(0), binary_search(arr, 1));
        assert_eq!(Some(1), binary_search(arr, 2));
        assert_eq!(None, binary_search(arr, 3));
        assert_eq!(None, binary_search(arr, -1));
    }

    #[test]
    fn typical_array() {
        let arr: [i32; 1000] = (1..=1000).collect::<Vec<i32>>().try_into().unwrap();
        // let arr: [u32; 1000] = std::array::from_fn(|i| (i + 1) as u32);

        assert_eq!(Some(42), binary_search(&arr, 43));
        assert_eq!(Some(999), binary_search(&arr, 1000));
    }

    #[test]
    fn array_with_negative_and_positive_numbers() {
        let arr: [i32; 1000] = (-500..500).collect::<Vec<i32>>().try_into().unwrap();
        assert_eq!(Some(0), binary_search(&arr, -500));
        assert_eq!(Some(388), binary_search(&arr, -112));
        assert_eq!(Some(999), binary_search(&arr, 499));
        assert_eq!(None, binary_search(&arr, 500));

        let arr2: [i32; 1001] = (-500..=500).collect::<Vec<i32>>().try_into().unwrap();
        assert_eq!(Some(0), binary_search(&arr2, -500));
        assert_eq!(Some(735), binary_search(&arr2, 235));
        assert_eq!(Some(1000), binary_search(&arr2, 500));
        assert_eq!(None, binary_search(&arr2, 501));
    }

    #[test]
    fn ordered_non_consecutive_array() {
        let arr = &[3, 8, 26, 89, 117, 259, 808, 1222];
        assert_eq!(Some(4), binary_search(arr, 117));
        assert_eq!(Some(0), binary_search(arr, 3));
        assert_eq!(Some(7), binary_search(arr, 1222));
        assert_eq!(None, binary_search(arr, -1));
        assert_eq!(None, binary_search(arr, 1223));
    }

    #[test]
    fn very_long_array() {
        let arr: [i32; 100_000] = (-50_000..50_000).collect::<Vec<i32>>().try_into().unwrap();
        assert_eq!(Some(0), binary_search(&arr, -50_000));
        assert_eq!(Some(26970), binary_search(&arr, -23_030));
        assert_eq!(Some(99999), binary_search(&arr, 49999));
        assert_eq!(None, binary_search(&arr, -50_001));
        assert_eq!(None, binary_search(&arr, 50_000));
    }

    #[test]
    fn array_with_duplicates() {
        let arr = &[1, 1, 2, 3, 4, 4, 4];
        assert_eq!(Some(3), binary_search(arr, 3));
        assert_eq!(Some(1), binary_search(arr, 1));
        assert_eq!(Some(5), binary_search(arr, 4));
        assert_eq!(None, binary_search(arr, -1));
        assert_eq!(None, binary_search(arr, 5));

        let arr2 = &[2, 2, 2, 2];
        assert_eq!(Some(2), binary_search(arr2, 2));
        // assert_eq!(Some(1), binary_search(arr2, 2));
        assert_eq!(None, binary_search(arr2, 1));
        assert_eq!(None, binary_search(arr2, 3));
    }
}
