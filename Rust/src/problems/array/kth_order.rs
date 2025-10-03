// pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
//     for i in 0..nums.len() {
//         let mut count = 0;
//         for j in 0..nums.len() {
//             if nums[j] > nums[i] {
//                 count += 1;
//             }
//             if nums[j] == nums[i] {
//                 if j > i {
//                     count += 1;
//                 }
//             }
//         }

//         if count == k - 1 {
//             return nums[i];
//         }
//     }

//     nums[0]
// }

// pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
//     let mut kth_largest_index = 0;

//     for kth in 1..=k {
//         kth_largest_index = 0;
//         for i in 0..nums.len() {
//             if nums[i] >= nums[kth_largest_index] {
//                 kth_largest_index = i;
//             }
//         }

//         if kth != k {
//             nums.remove(kth_largest_index);
//         }
//     }

//     nums[kth_largest_index]
// }

// }

// pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
//     nums.sort();
//     nums.into_iter().rev().enumerate().fold(
//         0,
//         |acc, (i, curr)| {
//             if i == (k - 1) as usize {
//                 curr
//             } else {
//                 acc
//             }
//         },
//     )
// }
