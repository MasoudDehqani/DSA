mod data_structures;
mod algorithms;
use data_structures::singly_linked_list::SinglyLinkedList::{self, Nil, Node};
use algorithms::factorial::factorial;

fn main() {
    let lst = Node(1, Box::new(Node(2, Box::new(Nil))));
    let v1 = lst.read(1);
    let f = factorial(6);
    println!("{:?}", v1);
    println!("{}", f)
}
