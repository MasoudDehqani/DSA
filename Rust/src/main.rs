#![allow(dead_code, unused_imports)]

use data_structures::list::List;
use data_structures::singly_linked_list::SinglyLinkedList::{Nil, Node};
use rust::algorithms::maths::{fibonacci, fibonacci_memoization, reverse_number};
use rust::algorithms::recursion::basic_recursion::{
    count_1_to_n, count_1_to_n_linearly, count_n_to_1,
};
use rust::algorithms::recursion::{
    all_subsequences, any_one_subsequence_with_k_sum, combination_sum, count_inversions,
    count_subsequences_with_k_sum, is_string_palindrome, pow, print_all_subsequences,
    print_subsequences_with_k_sum, reverse_array_in_place, reverse_string, subsequences_with_k_sum,
    sum_of_first_natural_numbers, sum_of_first_natural_numbers_tail_recursive,
};
use rust::algorithms::search::binary_search::{
    ceil, first_and_last_occ, floor, lower_bound, upper_bound,
};
use rust::data_structures::array::remove_duplicates_in_place::remove_duplicates_in_place_from_sorted_array;
use rust::data_structures::array::rotate::left_rotate_by_one_place;
use rust::data_structures::array::second_order::second_largest_number;
use rust::data_structures::array::{is_array_sorted, kth_order};
use rust::problems::brackets_check::balance;
use rust::problems::{self, pascal_triangle_at, pascal_triangle_at_mem, pyramid_of_stars};
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
    // let mut v = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    // println!("{}", remove_duplicates(&mut v));
    // println!("{v:?}");
    // let mut arr = vec![1, 2];
    // rotate(&mut arr, 7);
    // println!("{arr:?}");

    // pyramid_of_stars();
    // println!("{}", reverse_string(String::from("Hello!")));
    // println!("{}", reverse_number(123));
    // println!("{}", balance(vec!['(', ')', '(']))

    // println!("{}", pascal_triangle_at_mem(40, 30));
    // println!("{}", pascal_triangle_at(40, 30))

    // println!("{}", pascal_triangle_at(0, 0));
    // println!("{}", pascal_triangle_at(1, 0));
    // println!("{}", pascal_triangle_at(1, 1));

    // println!("{}", pascal_triangle_at(2, 0));
    // println!("{}", pascal_triangle_at(2, 1));
    // println!("{}", pascal_triangle_at(2, 2));

    // println!("{}", pascal_triangle_at(3, 0));
    // println!("{}", pascal_triangle_at(3, 1));
    // println!("{}", pascal_triangle_at(3, 2));
    // println!("{}", pascal_triangle_at(3, 3));

    // println!("{}", pascal_triangle_at(4, 0));
    // println!("{}", pascal_triangle_at(4, 4));
    // println!("{}", pascal_triangle_at(7, 0));
    // println!("{}", pascal_triangle_at(7, 7));

    // println!("{}", pascal_triangle_at(15, 0));
    // println!("{}", pascal_triangle_at(15, 15));

    // println!("{}", pascal_triangle_at(4, 1));
    // println!("{}", pascal_triangle_at(4, 2));
    // println!("{}", pascal_triangle_at(4, 3));

    // println!("{}", pascal_triangle_at(5, 2));
    // println!("{}", pascal_triangle_at(6, 3));
    // println!("{}", pascal_triangle_at(7, 3));
    // println!("{}", pascal_triangle_at(7, 4));

    // println!("{}", pascal_triangle_at(8, 2));
    // println!("{}", pascal_triangle_at(9, 3));
    // println!("{}", pascal_triangle_at(10, 4));

    // println!("{}", pascal_triangle_at(10, 5));
    // println!("{}", pascal_triangle_at(15, 7));
    // println!("{}", pascal_triangle_at(20, 10));
    // println!("{}", pascal_triangle_at(25, 12));
    // println!("{}", pascal_triangle_at(30, 15));

    // println!("{}", pascal_triangle_at(4, 3));
    // println!("{}", pascal_triangle_at(8, 5));
    // println!("{}", pascal_triangle_at(0, -1));
    // println!("{}", pascal_triangle_at(-1, 4));
    // println!("{}", pascal_triangle_at(-3, -2));

    // println!("0: {}", pow::pow_recursive_squaring(2.0, 0.0));
    // println!("1: {}", pow::pow_recursive_squaring(2.0, 1.0));
    // println!("2: {}", pow::pow_recursive_squaring(2.0, 2.0));
    // println!("3: {}", pow::pow_recursive_squaring(2.0, 3.0));
    // println!("4: {}", pow::pow_recursive_squaring(2.0, 4.0));
    // println!("5: {}", pow::pow_recursive_squaring(2.0, 5.0));
    // println!("6: {}", pow::pow_recursive_squaring(2.0, 6.0));
    // println!("7: {}", pow::pow_recursive_squaring(2.0, 7.0));
    // println!("8: {}", pow::pow_recursive_squaring(2.0, 8.0));
    // println!("9: {}", pow::pow_recursive_squaring(2.0, 9.0));
    // println!("10: {}", pow::pow_recursive_squaring(2.0, 10.0));
    // println!("11: {}", pow::pow_recursive_squaring(2.0, 11.0));
    // println!("12: {}", pow::pow_recursive_squaring(2.0, 12.0));
    // println!("13: {}", pow::pow_recursive_squaring(2.0, 13.0));
    // println!("14: {}", pow::pow_recursive_squaring(2.0, 14.0));

    // println!(
    //     "{}",
    //     binary_search::search_insert_position(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15], 11)
    // );

    // println!(
    //     "{}",
    //     binary_search::search_insert_position(&[0, 22, 29, 84, 125, 130, 561, 777, 1231, 1800], 25)
    // );

    // println!(
    //     "{}",
    //     binary_search::search_insert_position(
    //         &[0, 22, 29, 84, 125, 130, 561, 777, 1231, 1800],
    //         561
    //     )
    // );

    // println!("{:?}", ceil(&[10, 20, 30, 40, 50], 25));
    // println!("{:?}", ceil(&[10, 20, 30, 40, 50], 0));
    // println!("{:?}", ceil(&[10, 20, 30, 40, 50], 60));
    // println!("{:?}", ceil(&[10, 20, 30, 40, 50], 10));
    // println!("{:?}", ceil(&[10, 20, 30, 40, 50], 50));

    // println!("{:?}", floor(&[10, 20, 30, 40, 50], 25));
    // println!("{:?}", floor(&[10, 20, 30, 40, 50], 0));
    // println!("{:?}", floor(&[10, 20, 30, 40, 50], 60));
    // println!("{:?}", floor(&[10, 20, 30, 40, 50], 10));
    // println!("{:?}", floor(&[10, 20, 30, 40, 50], 50));
    // println!("{}", floor(&[10, 20, 30, 40, 50], 25))

    // println!("{:?}", first_and_last_occ(&[0, 1, 2, 2, 2, 3, 4, 5], 2));
    // println!("{:?}", first_and_last_occ(&[0, 1, 2, 2, 2, 4, 5, 6], 3));
    // println!("{:?}", first_and_last_occ(&[0, 1, 2, 2, 2, 3, 4, 5], 0));
    // println!("{:?}", first_and_last_occ(&[0, 1, 2, 2, 2, 3, 4, 5], 5));
    // println!(
    //     "{:?}",
    //     first_and_last_occ(&[0, 0, 0, 0, 1, 2, 2, 2, 3, 4, 5], 0)
    // );
    // println!(
    //     "{:?}",
    //     first_and_last_occ(&[0, 1, 2, 2, 2, 3, 4, 5, 5, 5], 5)
    // );

    // println!(
    //     "{}",
    //     lower_bound(&[0, 22, 29, 84, 125, 130, 561, 777, 1231, 1800], 182)
    // );

    // println!("{}", upper_bound(&[0, 1, 2, 2, 2, 3, 4, 5, 5, 5], 5));
    // println!("{}", upper_bound(&[], 5));
    // println!("{}", lower_bound(&[0, 1, 2, 2, 2, 3, 4, 5, 5, 5], 5));

    // println!("{}", lower_bound(&[1, 2, 2, 2, 3, 4, 5, 5, 5], 0));
    // println!("{}", lower_bound(&[1, 2, 2, 2, 3, 4, 5, 5, 5], 6));
    // println!("{}", lower_bound(&[1, 2, 2, 2, 3, 4, 5, 5, 5], 5));
    // println!("{}", lower_bound(&[1, 2, 2, 2, 3, 4, 5, 5, 5], 4));
    // println!("{}", lower_bound(&[1, 2, 2, 2, 3, 4, 5, 5, 5], 2));
    // println!("{}", lower_bound(&[1, 2, 2, 2, 3, 4, 5, 5, 5], 1));

    // println!("----------------------------------------");

    // println!("{}", upper_bound(&[1, 2, 2, 2, 3, 4, 5, 5, 5], 0));
    // println!("{}", upper_bound(&[1, 2, 2, 2, 3, 4, 5, 5, 5], 6));
    // println!("{}", upper_bound(&[1, 2, 2, 2, 3, 4, 5, 5, 5], 5));
    // println!("{}", upper_bound(&[1, 2, 2, 2, 3, 4, 5, 5, 5], 4));
    // println!("{}", upper_bound(&[1, 2, 2, 2, 3, 4, 5, 5, 5], 2));
    // println!("{}", upper_bound(&[1, 2, 2, 2, 3, 4, 5, 5, 5], 1))

    // println!("{:?}", combination_sum(vec![2, 3, 5], 8));
    // println!("{:?}", combination_sum(vec![2, 3, 6, 7], 7));
    // println!("{}", sum_of_first_natural_numbers_recursive(5));
    // println!("{}", sum_of_first_natural_numbers_tail_recursive(5));
    // let mut arr: Vec<i32> = vec![1, 2, 3, 4];
    // reverse_array_in_place(&mut arr);
    // println!("{:?}", arr);
    // println!("{:?}", is_string_palindrome("ABCBA"))

    // println!("{}", fibonacci(45));
    // println!("{}", fibonacci_memoization(45))
    // let seq = vec![1, 2, 3, 4, 5];
    // let seq = vec![3, 1, 2];
    // println!("{:?}", count_subsequences_with_k_sum(&seq, 5));
    // print_all_subsequences(&seq);
    // let seq = vec![10, 1, 2, 7, 6, 1, 5];
    // println!("{:?}", subsequences_with_k_sum(&seq, 8));
    // let seq2 = vec![2, 5, 2, 1, 2];
    // println!("{:?}", subsequences_with_k_sum(&seq2, 5));
    // print_subsequences_with_k_sum(&seq, 8);

    // println!("{:?}", all_subsequences(&arr))

    // let arr = vec![5, 3, 2, 4, 1];
    // println!("{}", count_inversions(&arr))

    // count_1_to_n(8);
    // count_n_to_1(8);

    // count_1_to_n_linearly(8);

    // let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 9, 9];
    // println!("{:?}", kth_order::find_kth_largest(&numbers, 9))
    // println!("{:?}", second_largest_number(&vec![1, 2, 3, 4, 4, 5, 5, 5]));
    // let mut arr = vec![1, 1, 1, 2, 3];
    // let mut arr = vec![1, 1, 1, 2, 3];
    // let mut arr = vec![3, 3, 3, 2, 2, 0, -1];
    // remove_duplicates_in_place_from_sorted_array(&mut arr);
    // println!("{:?}", arr)
    // let mut arr = vec![1, 2, 3, 4, 5];
    // left_rotate_by_one_place(&mut arr);
    // println!("{arr:?}")
}
