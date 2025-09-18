pub trait List<T> {
    fn read(&self, index: usize) -> Option<&T>;
}

pub enum SinglyLinkedList<T> {
    Node(T, Box<SinglyLinkedList<T>>),
    Nil,
}

impl<T> List<T> for SinglyLinkedList<T> {
    fn read(&self, index: usize) -> Option<&T> {
        match self {
            SinglyLinkedList::Nil => None,
            SinglyLinkedList::Node(head, tail) => {
                if index == 0 {
                    Some(head)
                } else {
                    tail.read(index - 1)
                }
            }
        }
    }
}
