// pub fn second_smallest_number(nums: Vec<i32>) -> i32 {
//     let mut smallest = nums[0];
//     let mut second_smallest = nums[1];
//     for n in nums {
//         if n < smallest {
//             second_smallest = smallest;
//             smallest = n;
//         }
//     }

//     second_smallest
// }

// pub fn second_largest_number(nums: Vec<i32>) -> i32 {
//     let mut largest = nums[0];
//     let mut second_largest = nums[1];
//     for n in nums {
//         if n > largest {
//             second_largest = largest;
//             largest = n
//         }
//     }

//     second_largest
// }

pub fn second_smallest_number(nums: Vec<i32>) -> i32 {
    let mut smallest = nums[0];
    let mut second_smallest = nums[1];
    for n in nums {
        if second_smallest == smallest && n != smallest {
            second_smallest = n;
        }
        if n < smallest {
            second_smallest = smallest;
            smallest = n;
        }
    }

    second_smallest
}

pub fn second_largest_number(nums: Vec<i32>) -> i32 {
    let mut largest = nums[0];
    let mut second_largest = nums[1];
    for n in nums {
        if second_largest == largest && n != largest {
            second_largest = n;
        }
        if n > largest {
            second_largest = largest;
            largest = n
        }
    }

    second_largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second_smallest_number() {
        assert_eq!(second_smallest_number(vec![1, 2, 3, 4, 5]), 2);
        assert_eq!(second_smallest_number(vec![5, 4, 3, 2, 1]), 2);
        assert_eq!(second_smallest_number(vec![1, 1, 2, 2, 3]), 2);
        assert_eq!(second_smallest_number(vec![1, 2, 2, 3, 3]), 2);
        assert_eq!(second_smallest_number(vec![1, 2, 3, 4, 5]), 2);
    }

    #[test]
    fn test_second_largest_number() {
        assert_eq!(second_largest_number(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(second_largest_number(vec![5, 4, 3, 2, 1]), 4);
        assert_eq!(second_largest_number(vec![1, 1, 2, 2, 3]), 2);
        assert_eq!(second_largest_number(vec![1, 2, 2, 3, 3]), 2);
        assert_eq!(second_largest_number(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn test_second_smallest_number_with_duplicates() {
        assert_eq!(second_smallest_number(vec![1, 1, 1, 1, 1]), 1);
        assert_eq!(second_smallest_number(vec![1, 1, 1, 2, 2]), 2);
        assert_eq!(second_smallest_number(vec![1, 1, 2, 2, 2]), 2);
        assert_eq!(second_smallest_number(vec![1, 2, 2, 2, 2]), 2);
        assert_eq!(second_smallest_number(vec![2, 2, 2, 2, 2]), 2);
    }

    #[test]
    fn test_second_largest_number_with_duplicates() {
        assert_eq!(second_largest_number(vec![1, 1, 1, 1, 1]), 1);
        assert_eq!(second_largest_number(vec![1, 1, 1, 2, 2]), 1);
        assert_eq!(second_largest_number(vec![1, 1, 2, 2, 2]), 1);
        assert_eq!(second_largest_number(vec![1, 2, 2, 2, 2]), 1);
        assert_eq!(second_largest_number(vec![2, 2, 2, 2, 2]), 2);
    }
}

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
