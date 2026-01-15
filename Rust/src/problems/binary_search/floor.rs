pub fn floor(arr: &[i32], target: i32) -> Option<i32> {
    let mut start = 0;
    let mut end = arr.len();
    let mut answer = None;

    while start < end {
        let mid = (start + end) / 2;

        if arr[mid] <= target {
            answer = Some(arr[mid]);
            start = mid + 1;
        } else {
            end = mid;
        }
    }

    answer
}
