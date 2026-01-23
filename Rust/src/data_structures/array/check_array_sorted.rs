pub fn is_array_sorted(arr: &[i32]) -> bool {
    for i in 1..arr.len() {
        if arr[i] < arr[i - 1] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_array_sorted;

    #[test]
    fn is_array_sorted_empty_array() {
        let arr = [];
        assert_eq!(is_array_sorted(&arr), true)
    }

    #[test]
    fn is_array_sorted_single_element_array() {
        let arr = [0];
        assert_eq!(is_array_sorted(&arr), true)
    }

    #[test]
    fn is_array_sorted_sorted_array() {
        let arr = [0, 1, 2, 3, 4, 5];
        assert_eq!(is_array_sorted(&arr), true)
    }

    #[test]
    fn is_array_sorted_sorted_array_with_duplicate() {
        let arr = [0, 0, 1, 2, 3, 3, 3, 4];
        assert_eq!(is_array_sorted(&arr), true)
    }

    #[test]
    fn is_array_sorted_not_sorted_array() {
        let arr = [0, -1, 1, 2, 3, 4];
        assert_eq!(is_array_sorted(&arr), false)
    }

    #[test]
    fn is_array_sorted_not_sorted_array_with_duplicate() {
        let arr = [0, -1, -1, 1, 2, 3, 4, 4, 4];
        assert_eq!(is_array_sorted(&arr), false)
    }
}
