#![allow(dead_code, unused_imports)]

// use data_structures::singly_linked_list::{
//     List,
//     SinglyLinkedList::{Nil, Node},
// };
use rust::{algorithms, data_structures};

fn main() {
    // let lst = Node(
    //     1,
    //     Box::new(Node(2, Box::new(Node(3, Box::new(Node(4, Box::new(Nil))))))),
    // );
    // println!("{}", lst.size());
    // let v1 = lst.read(2);
    // println!("{:?}", v1);
    // println!("{:?}", lst.reverse());
    // println!("{:?}", lst);
    // println!("{:?}", lst.find(1));

    // let f = algorithms::search::binary_search(&[4], 4);
    let r = algorithms::prime::is_prime(4);

    println!("{:?}", r);
}
