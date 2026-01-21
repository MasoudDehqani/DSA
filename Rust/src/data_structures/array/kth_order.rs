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

pub fn find_kth_largest(numbers: &Vec<i32>, mut k: u32) -> Option<i32> {
    if k == 0 || numbers.len() == 0 {
        return None;
    }

    let mut largest = numbers[0];

    for &n in numbers {
        if n > largest {
            largest = n;
        }
    }

    let mut kth_largest = Some(largest);

    let mut last_largest = largest;
    k -= 1;
    while k > 0 {
        let mut current_largest = numbers[0];
        for &n in numbers {
            if n > current_largest && n < last_largest {
                current_largest = n;
            }
        }

        if current_largest != last_largest {
            kth_largest = Some(current_largest);
        } else {
            kth_largest = None;
        }

        last_largest = current_largest;

        k -= 1;
    }

    kth_largest
}
