
#[derive(Debug)]
pub struct ListNode<T> {
  pub value: T,
  pub next: Box<Option<ListNode<T>>>
}

#[derive(Debug)]
pub enum SinglyLinkedList<T> {
  Node(ListNode<T>),
  Nil
}

impl<T: Copy> SinglyLinkedList<T> {
  pub fn read(&self, _index: u32) -> Option<T> {
    match self {
      SinglyLinkedList::Node(list_node) => Some(list_node.value),
      SinglyLinkedList::Nil => None
    }
  }
}
