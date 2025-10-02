pub fn largest(arr: Vec<i32>) -> i32 {
    let mut largest = arr[0];
    for n in arr {
        if n > largest {
            largest = n
        }
    }

    largest
}
