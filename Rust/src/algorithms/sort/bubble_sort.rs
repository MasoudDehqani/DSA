pub fn bubble_sort(arr: &mut [i32]) {
    let mut end = arr.len();

    loop {
        let mut no_swap = true;
        let mut f = 0;
        let mut s = 1;

        for _ in 1..end {
            if arr[f] > arr[s] {
                arr.swap(f, s);
                no_swap = false;
            }

            f += 1;
            s += 1;
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
    fn empty_array() {
        let mut arr = [];
        bubble_sort(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn single_element_array() {
        let mut arr = [2];
        bubble_sort(&mut arr);
        assert_eq!([2], arr);
    }

    #[test]
    fn multi_element_array() {
        let mut arr = [1, 0, 120, 4, 8, 37, 93];
        bubble_sort(&mut arr);
        assert_eq!([0, 1, 4, 8, 37, 93, 120], arr);
    }
}
