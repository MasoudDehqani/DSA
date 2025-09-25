#![allow(dead_code, unused_imports)]

use data_structures::singly_linked_list::{
    List,
    SinglyLinkedList::{Nil, Node},
};
use rust::{algorithms, data_structures};

fn main() {
    // let lst = Node(
    //     1,
    //     Box::new(Node(2, Box::new(Node(3, Box::new(Node(4, Box::new(Nil))))))),
    // );
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

    println!("{}", algorithms::recursion::count_digits(1094758291));
}
