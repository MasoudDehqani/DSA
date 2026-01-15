pub fn floor(arr: &[i32], target: i32) -> i32 {
    let mut start = 0;
    let mut end = arr.len();
    let mut answer = 0;

    while end >= start {
        let mid = (start + end) / 2;

        if arr[mid] <= target {
            answer = arr[mid];
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }

    answer
}
