pub trait List<T: Copy> {
    fn read(&self, index: usize) -> Option<&T>;
    fn size(&self) -> usize;
    fn reverse(&self) -> Self;
}

#[derive(Debug)]
pub enum SinglyLinkedList<T> {
    Node(T, Box<SinglyLinkedList<T>>),
    Nil,
}

use SinglyLinkedList::*;

impl<T: Copy> SinglyLinkedList<T> {
    fn reverse_aux(&self, acc: SinglyLinkedList<T>) -> SinglyLinkedList<T> {
        match self {
            Nil => acc,
            Node(head, tail) => tail.reverse_aux(Node(head.clone(), Box::new(acc))),
        }
    }
}

impl<T: Copy> List<T> for SinglyLinkedList<T> {
    fn read(&self, index: usize) -> Option<&T> {
        match (self, index == 0) {
            (Nil, _) => None,
            (Node(head, _), true) => Some(head),
            (Node(_, tail), false) => tail.read(index - 1),
        }
    }

    fn size(&self) -> usize {
        let mut current = self;
        let mut size = 0_usize;
        while let Node(_, tail) = current {
            size += 1;
            current = tail;
        }

        size
    }

    fn reverse(&self) -> Self {
        self.reverse_aux(Nil)
    }
}
