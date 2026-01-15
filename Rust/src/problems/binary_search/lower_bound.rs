pub fn lower_bound(arr: &[i32], target: i32) -> usize {
    let mut start = 0;
    let mut end = arr.len();
    let mut answer = 0;

    while start <= end {
        let middle = (start + end) / 2;

        if arr[middle] >= target {
            answer = middle;
            end = middle - 1;
        } else {
            start = middle + 1;
        }
    }

    answer
}
