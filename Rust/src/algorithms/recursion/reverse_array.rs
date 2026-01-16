pub fn reverse_array<T: Clone + Copy>(mut arr: Vec<T>) -> Vec<T> {
    if arr.len() < 2 {
        return arr;
    }

    let arr_but_first = arr.split_off(1);
    let reversed = reverse_array(arr_but_first);
    vec![reversed, arr].concat()
}
