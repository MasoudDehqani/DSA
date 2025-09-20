mod algorithms;
mod data_structures;
// use data_structures::singly_linked_list::{
//     List,
//     SinglyLinkedList::{Nil, Node},
// };

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

    let f = algorithms::search::binary_search(&[4], 4);

    println!("{:?}", f);
}
