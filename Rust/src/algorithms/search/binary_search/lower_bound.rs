pub fn lower_bound(arr: &[i32], target: i32) -> usize {
    let mut start = 0;
    let mut end = arr.len();
    let mut answer = end;

    while start < end {
        let middle = (start + end) / 2;
        // println!("BEFORE: {}, {}, {}", start, middle, end);

        if arr[middle] >= target {
            answer = middle;
            end = middle;
        } else {
            start = middle + 1;
        }

        // println!("AFTER: {}, {}, {}", start, middle, end);
    }

    answer
}
