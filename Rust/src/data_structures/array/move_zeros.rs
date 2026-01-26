// pub fn move_zeroes(nums: &mut Vec<i32>) {
//     let mut i = 0;

//     for j in 0..nums.len() {
//         if nums[i] == 0 && nums[j] != 0 {
//             nums.swap(i, j);
//             i += 1;
//         }

//         if nums[i] != 0 {
//             i += 1;
//         }
//     }
// }

pub fn move_zeroes_to_end(numbers: &mut [i32]) {
    let mut i = 0;
    let mut rep = i + 1;

    while rep < numbers.len() {
        if numbers[rep] == 0 {
            rep += 1;
            continue;
        }

        if numbers[i] == 0 {
            numbers.swap(i, rep);
            i += 1;
            continue;
        }

        i += 1;
    }
}
