use std::cmp::Ordering;

fn binary_search_helper(arr: &[i32], value: i32, start: usize, end: usize) -> Option<usize> {
    let middle = (start + end).div_ceil(2);

    if start >= end {
        if value == arr[middle] {
            return Some(middle);
        } else {
            return None;
        }
    }

    match value.cmp(&arr[middle]) {
        Ordering::Equal => Some(middle),
        Ordering::Greater => binary_search_helper(arr, value, middle + 1, end),
        Ordering::Less => binary_search_helper(arr, value, start, middle - 1),
    }
}

pub fn binary_search(arr: &[i32], value: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    binary_search_helper(arr, value, 0, arr.len() - 1)
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
        assert_eq!(Some(388), binary_search(&arr, -112));
        assert_eq!(Some(999), binary_search(&arr, 499));
    }

    #[test]
    fn ordered_non_consecutive_array() {
        let arr = &[3, 8, 26, 89, 117, 259, 808, 1222];
        assert_eq!(Some(4), binary_search(arr, 117));
    }
}
