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

// pub fn second_smallest_number(nums: Vec<i32>) -> i32 {
//     let mut smallest = nums[0];
//     let mut second_smallest = nums[1];
//     for n in nums {
//         if second_smallest == smallest && n != smallest {
//             second_smallest = n;
//         }
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
//         if second_largest == largest && n != largest {
//             second_largest = n;
//         }
//         if n > largest {
//             second_largest = largest;
//             largest = n
//         }
//     }

//     second_largest
// }

// pub fn second_largest_number(numbers: &Vec<i32>) -> Option<i32> {
//     let mut maybe_largest = None;
//     let mut maybe_second_largest = None;

//     for &n in numbers {
//         let Some(largest) = maybe_largest else {
//             maybe_largest = Some(n);
//             continue;
//         };

//         if n > largest {
//             maybe_second_largest = maybe_largest;
//             maybe_largest = Some(n);
//             continue;
//         }

//         if Some(n).gt(&maybe_second_largest) && n != largest {
//             maybe_second_largest = Some(n)
//         }
//     }

//     maybe_second_largest
// }

// pub fn second_smallest_number(numbers: &Vec<i32>) -> Option<i32> {
//     let mut maybe_smallest = None;
//     let mut maybe_second_smallest = None;

//     for &n in numbers {
//         let Some(smallest) = maybe_smallest else {
//             maybe_smallest = Some(n);
//             continue;
//         };

//         println!("{n}, {smallest}");
//         if n < smallest {
//             maybe_second_smallest = maybe_smallest;
//             maybe_smallest = Some(n);
//             continue;
//         }

//         match maybe_second_smallest {
//             Some(second_smallest) => {
//                 if n < second_smallest && n != smallest {
//                     maybe_second_smallest = Some(n)
//                 }
//             }
//             None => {
//                 if n > smallest {
//                     maybe_second_smallest = Some(n)
//                 }
//             }
//         }
//     }

//     maybe_second_smallest
// }

/*
    Reviewed and written by chatgpt
*/
pub fn second_largest_number(numbers: &[i32]) -> Option<i32> {
    let mut largest = None;
    let mut second = None;

    for &n in numbers {
        match largest {
            None => largest = Some(n),

            Some(l) if n > l => {
                second = largest;
                largest = Some(n);
            }

            Some(l) if n < l => {
                if second.map_or(true, |s| n > s) {
                    second = Some(n);
                }
            }
            _ => {}
        }
    }

    second
}

pub fn second_smallest_number(numbers: &[i32]) -> Option<i32> {
    let mut smallest = None;
    let mut second = None;

    for &n in numbers {
        match smallest {
            None => smallest = Some(n),

            Some(s) if n < s => {
                second = smallest;
                smallest = Some(n);
            }

            Some(s) if n > s => {
                if second.map_or(true, |x| n < x) {
                    second = Some(n);
                }
            }

            _ => {} // n == smallest → ignore
        }
    }

    second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second_smallest_number_empty_array() {
        assert_eq!(second_smallest_number(&vec![]), None);
    }

    #[test]
    fn test_second_largest_number_empty_array() {
        assert_eq!(second_largest_number(&vec![]), None);
    }

    #[test]
    fn test_second_smallest_number_single_element_array() {
        assert_eq!(second_smallest_number(&vec![3]), None);
    }

    #[test]
    fn test_second_largest_number_single_element_array() {
        assert_eq!(second_largest_number(&vec![3]), None);
    }

    #[test]
    fn test_second_smallest_number() {
        assert_eq!(second_smallest_number(&vec![0, 1, 2, 3, 4, 5]), Some(1));
        assert_eq!(second_smallest_number(&vec![5, 4, 3, 2, 1, 0]), Some(1));
        assert_eq!(second_smallest_number(&vec![5, 4, 0, 2, 1, 3]), Some(1));
    }

    #[test]
    fn test_second_largest_number() {
        assert_eq!(second_largest_number(&vec![0, 1, 2, 3, 4, 5]), Some(4));
        assert_eq!(second_largest_number(&vec![5, 4, 3, 2, 1, 0]), Some(4));
        assert_eq!(second_largest_number(&vec![5, 4, 0, 2, 1, 3]), Some(4));
    }

    #[test]
    fn test_second_smallest_number_with_duplicates() {
        assert_eq!(second_smallest_number(&vec![1, 1, 1, 1, 1]), None);
        assert_eq!(second_smallest_number(&vec![1, 1, 1, 2, 2]), Some(2));
        assert_eq!(second_smallest_number(&vec![1, 1, 2, 2, 2]), Some(2));
        assert_eq!(second_smallest_number(&vec![2, 2, 2, 2, 2]), None);
        assert_eq!(second_smallest_number(&vec![1, 2, 2, 3, 3, 3]), Some(2));
    }

    #[test]
    fn test_second_largest_number_with_duplicates() {
        assert_eq!(second_largest_number(&vec![1, 1, 1, 1, 1]), None);
        assert_eq!(second_largest_number(&vec![1, 1, 1, 2, 2]), Some(1));
        assert_eq!(second_largest_number(&vec![1, 1, 2, 2, 2]), Some(1));
        assert_eq!(second_largest_number(&vec![2, 2, 2, 2, 2]), None);
        assert_eq!(second_largest_number(&vec![1, 2, 2, 3, 3, 3]), Some(2));
    }

    #[test]
    fn test_second_smallest_number_negative() {
        assert_eq!(second_smallest_number(&vec![-1, 0, -2, -3]), Some(-2));
    }

    #[test]
    fn test_second_largest_number_negative() {
        assert_eq!(second_largest_number(&vec![-1, 0, -2, -3]), Some(-1));
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
