pub fn bubble_sort(arr: &mut [i32]) {
    fn helper(arr: &mut [i32], start: usize, end: usize, mut swapped: bool) {
        if end < 1 {
            return;
        }

        let second = start + 1;

        if second > end {
            if !swapped {
                return;
            }

            helper(arr, 0, end - 1, swapped);
            return;
        }

        if arr[start] > arr[second] {
            arr.swap(start, second);
            swapped = true;
        }
        helper(arr, start + 1, end, swapped);
    }

    helper(arr, 0, arr.len().saturating_sub(1), false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = [5, 3, 8, 4, 2];
        bubble_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 8]);
    }

    #[test]
    fn test_bubble_sort_empty() {
        let mut arr = [];
        bubble_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_bubble_sort_single_element() {
        let mut arr = [5];
        bubble_sort(&mut arr);
        assert_eq!(arr, [5]);
    }

    #[test]
    fn test_bubble_sort_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubble_sort_reverse() {
        let mut arr = [5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubble_sort_duplicate() {
        let mut arr = [5, 3, 8, 4, 2, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 5, 8]);
    }

    #[test]
    fn test_bubble_sort_duplicate_sorted() {
        let mut arr = [2, 3, 4, 5, 5, 8];
        bubble_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 5, 8]);
    }

    #[test]
    fn test_bubble_sort_duplicate_sorted_reverse() {
        let mut arr = [8, 5, 5, 4, 3, 2];
        bubble_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 5, 8]);
    }

    #[test]
    fn test_bubble_sort_duplicate_sorted_reverse_duplicate() {
        let mut arr = [8, 5, 5, 4, 3, 2, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 5, 5, 8]);
    }
}
