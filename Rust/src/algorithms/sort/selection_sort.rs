pub fn selection_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        let mut current_smallest_index = i;

        for j in i..arr.len() {
            if arr[j] < arr[current_smallest_index] {
                current_smallest_index = j;
            }
        }

        arr.swap(i, current_smallest_index);
    }
}

#[cfg(test)]
mod tests {
    use super::selection_sort;

    #[test]
    fn test_selection_sort() {
        let mut arr = [5, 3, 8, 4, 2];
        selection_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 8]);
    }

    #[test]
    fn test_selection_sort_empty() {
        let mut arr = [];
        selection_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_selection_sort_single_element() {
        let mut arr = [5];
        selection_sort(&mut arr);
        assert_eq!(arr, [5]);
    }

    #[test]
    fn test_selection_sort_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_selection_sort_reverse() {
        let mut arr = [5, 4, 3, 2, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_selection_sort_duplicate() {
        let mut arr = [5, 3, 8, 4, 2, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 5, 8]);
    }

    #[test]
    fn test_selection_sort_duplicate_sorted() {
        let mut arr = [2, 3, 4, 5, 5, 8];
        selection_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 5, 8]);
    }

    #[test]
    fn test_selection_sort_duplicate_sorted_reverse() {
        let mut arr = [8, 5, 5, 4, 3, 2];
        selection_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 5, 8]);
    }

    #[test]
    fn test_selection_sort_duplicate_sorted_reverse_duplicate() {
        let mut arr = [8, 5, 5, 4, 3, 2, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 5, 5, 8]);
    }
}
