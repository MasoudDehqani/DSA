mod data_structures;
use data_structures::singly_linked_list::SinglyLinkedList::{self, Node, Nil};
use data_structures::singly_linked_list::ListNode;
fn main() {
    let n = ListNode {value: 1, next: Box::new(None)};
    let l: SinglyLinkedList<i64> = Node(n);
    println!("{:?}", l);
}
