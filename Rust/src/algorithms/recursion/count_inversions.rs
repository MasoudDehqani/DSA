// pub fn count_inversions(arr: &Vec<i32>) -> i32 {
//     fn aux(arr: &Vec<i32>, i: usize, pair: &mut Vec<i32>, count: &mut i32) {
//         if pair.len() == 2 {
//             if pair[0] > pair[1] {
//                 *count += 1;
//             }

//             return;
//         }

//         if i >= arr.len() {
//             return;
//         }

//         pair.push(arr[i]);
//         aux(arr, i + 1, pair, count);

//         pair.pop();
//         aux(arr, i + 1, pair, count);
//     }

//     let mut count = 0;
//     aux(arr, 0, &mut vec![], &mut count);
//     count
// }

pub fn count_inversions(arr: &Vec<i32>) -> i32 {
    fn aux(arr: &Vec<i32>, i: usize, pair: &mut Vec<i32>) -> i32 {
        if pair.len() == 2 {
            if pair[0] > pair[1] {
                return 1;
            }

            return 0;
        }

        if i >= arr.len() {
            return 0;
        }

        pair.push(arr[i]);
        let l = aux(arr, i + 1, pair);

        pair.pop();
        let r = aux(arr, i + 1, pair);

        l + r
    }

    aux(arr, 0, &mut vec![])
}
