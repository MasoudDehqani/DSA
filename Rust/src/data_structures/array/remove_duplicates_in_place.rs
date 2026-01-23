pub fn remove_duplicates_in_place_from_sorted_array(arr: &mut Vec<i32>) {
    let mut i = 1;

    while i < arr.len() {
        if arr[i] == arr[i - 1] {
            arr.remove(i);
            i -= 1;
        }

        i += 1;
    }
}
