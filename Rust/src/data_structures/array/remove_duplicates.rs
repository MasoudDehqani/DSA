// pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//     let mut prev = i32::MIN;
//     let mut i = 0;
//     while i < nums.len() {
//         if prev == nums[i] {
//             nums.remove(i);
//         } else {
//             prev = nums[i];
//             i += 1;
//         }
//     }

//     nums.len() as _
// }

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i = 1;
    let mut j = 0;
    while i < nums.len() {
        if nums[i] != nums[j] {
            j += 1;
            nums[j] = nums[i];
            i += 1;
        } else {
            i += 1;
        }
    }

    (j + 1) as _
}
