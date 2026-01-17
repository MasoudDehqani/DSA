// pub fn reverse_array<T: Clone>(mut arr: Vec<T>) -> Vec<T> {
//     if arr.len() < 2 {
//         return arr;
//     }

//     let arr_but_first = arr.split_off(1);
//     let reversed = reverse_array(arr_but_first);
//     vec![reversed, arr].concat()
// }

pub fn reverse_array<T>(mut arr: Vec<T>) -> Vec<T> {
    if arr.len() < 2 {
        return arr;
    }

    let arr_but_first = arr.split_off(1);
    let mut reversed = reverse_array(arr_but_first);
    reversed.append(&mut arr);
    reversed
}

pub fn reverse_array_in_place<T>(arr: &mut Vec<T>) {
    fn aux<T>(arr: &mut Vec<T>, curr: usize, len: usize) {
        if curr >= len / 2 {
            return;
        }

        arr.swap(curr, len - 1 - curr);
        aux(arr, curr + 1, len);
    }

    aux(arr, 0, arr.len());
}
