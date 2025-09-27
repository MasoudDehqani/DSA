pub fn insertion_sort(arr: &mut [i32]) {
    for i in 2..arr.len() {
        for j in 0..=i {
            if arr[i] < arr[j] {
                arr.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::insertion_sort;

    #[test]
    fn test_insertion_sort() {
        let mut arr = [5, 3, 8, 4, 2];
        insertion_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 8]);
    }

    #[test]
    fn test_insertion_sort_empty() {
        let mut arr = [];
        insertion_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_insertion_sort_single_element() {
        let mut arr = [5];
        insertion_sort(&mut arr);
        assert_eq!(arr, [5]);
    }

    #[test]
    fn test_insertion_sort_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insertion_sort_reverse() {
        let mut arr = [5, 4, 3, 2, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insertion_sort_duplicate() {
        let mut arr = [5, 3, 8, 4, 2, 5];
        insertion_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 5, 8]);
    }

    #[test]
    fn test_insertion_sort_duplicate_sorted() {
        let mut arr = [2, 3, 4, 5, 5, 8];
        insertion_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 5, 8]);
    }

    #[test]
    fn test_insertion_sort_duplicate_sorted_reverse() {
        let mut arr = [8, 5, 5, 4, 3, 2];
        insertion_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 5, 8]);
    }

    #[test]
    fn test_insertion_sort_duplicate_sorted_reverse_duplicate() {
        let mut arr = [8, 5, 5, 4, 3, 2, 5];
        insertion_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 5, 5, 8]);
    }
}
