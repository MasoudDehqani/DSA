mod algorithms;
mod data_structures;
use algorithms::{factorial::factorial, fibonacci::fibonacci};
use data_structures::singly_linked_list::SinglyLinkedList::{self, Nil, Node};

fn main() {
    let lst = Node(1, Box::new(Node(2, Box::new(Nil))));
    let v1 = lst.read(1);
    let f = factorial(6);
    let fv = fibonacci(8);
    println!("{:?}", v1);
    println!("{}", f);
    println!("{}", fv)
}
