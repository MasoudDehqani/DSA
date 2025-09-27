pub fn bubble_sort(arr: &mut [i32]) {
    let mut end = arr.len();

    while end > 1 {
        let mut no_swap = true;

        for i in 0..end - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                no_swap = false;
            }
        }

        if no_swap {
            break;
        }

        end -= 1;
    }
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
