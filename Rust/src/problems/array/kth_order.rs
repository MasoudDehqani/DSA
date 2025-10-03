// pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
//     for i in 0..nums.len() {
//         let mut count = 0;
//         for j in 0..nums.len() {
//             if i != j && nums[j] >= nums[i] {
//                 count += 1;
//             }
//             println!("i: {}, j: {}", nums[i], nums[j]);
//         }

//         println!("count: {count}, k: {k}");

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
// class Solution {
//     secondLargestElement(nums) {
//         let largest = nums[0];
//         let secondLargest;

//         for (let i = 0; i < nums.length; i++) {
//             if (nums[i] !== largest && !secondLargest) {
//                 secondLargest = nums[i];
//             }
//             if (nums[i] > largest) {
//                 secondLargest = largest;
//                 largest = nums[i];
//             }
//         }

//         return secondLargest ? secondLargest : -1;
//     }
// }

// [8, 8, 7, 6, 5]
// [10, 10, 10, 10, 10]
// [7, 7, 2, 2, 10, 10, 10]
