#![allow(dead_code, unused_imports)]

use data_structures::list::List;
use data_structures::singly_linked_list::SinglyLinkedList::{Nil, Node};
use rust::{algorithms, data_structures};

fn main() {
    // let lst = Node(
    //     1,
    //     Box::new(Node(2, Box::new(Node(3, Box::new(Node(4, Box::new(Nil))))))),
    // );
    // println!("{:?}", lst.filter(|e| e % 2 == 0));
    // println!("{:?}", lst.map(|e| e * 2));
    // let a = algorithms::search::binary_search(&[1, 2, 3, 4, 5, 6, 7], 5);
    // println!("{:?}", a);
    // println!("{}", lst.size());
    // let v1 = lst.read(2);
    // println!("{:?}", v1);
    // println!("{:?}", lst.reverse());
    // println!("{:?}", lst);
    // println!("{:?}", lst.find(1));

    // let f = algorithms::search::binary_search(&[4], 4);
    // let r = algorithms::prime_numbers::is_prime(4);

    // println!("{:?}", r);

    // println!(
    //     "{:?}",
    //     algorithms::recursion::palindrome::is_palindrome_string(String::from(
    //         "A man, a plan, a canal: Panama"
    //     ))
    // )

    // let mut arr = [1, 0, 120, 4, 8, 37, 93];

    // algorithms::sort::bubble_sort(&mut arr);
    // println!("{:?}", arr);

    // println!("{}", algorithms::recursion::count_digits(1094758291));

    // println!(
    //     "{}",
    //     algorithms::recursion::reverse_number::reverse_number_recursive(100)
    // );

    // println!(
    //     "{}",
    //     algorithms::recursion::reverse_number::reverse_number_non_tail_rec(1234)
    // )

    // println!(
    //     "{}",
    //     algorithms::recursion::count_digits_non_tail_rec(7231039459093487509)
    // )

    // let arr = [3, 8, 5, 4, 2, 5];
    // println!("{:?}", algorithms::sort::merge_sort(&arr));
    // println!("{:?}", arr);
    // println!("{}", check(vec![2, 1]));
    let mut v = vec![1, 1, 2];
    println!("{}", remove_duplicates(&mut v));
    println!("{v:?}");
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut prev = 0;
    let mut i = 0;
    while i < nums.len() {
        if prev == nums[i] {
            nums.remove(i);
        } else {
            prev = nums[i];
            i += 1;
        }
    }

    nums.len().try_into().unwrap()
}

pub fn check(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut drops = 0;

    for i in 1..n {
        if nums[i] < nums[i - 1] {
            drops += 1;
            if drops > 1 {
                return false;
            }
        }
    }

    if drops == 1 && nums[0] < nums[n - 1] {
        return false;
    }

    true
}

// pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
//     nums.sort();
//     nums.into_iter().rev().enumerate().fold(
//         0,
//         |acc, (i, curr)| {
//             if i == k as usize {
//                 curr
//             } else {
//                 acc
//             }
//         },
//     )
// }
