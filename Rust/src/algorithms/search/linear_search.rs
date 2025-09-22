pub fn linear_search(arr: &[i32], val: i32) -> Option<usize> {
    for i in 0..arr.len() {
        if arr[i] == val {
            return Some(i);
        }
    }

    None
}
