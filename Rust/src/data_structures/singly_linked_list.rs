pub trait List<T> {
    fn read(&self, index: usize) -> Option<&T>;
    fn size(&self) -> usize;
    fn reverse(&self) -> Self;
    fn find(&self, val: T) -> Option<usize>;
    fn map(&self, f: impl Fn(&T) -> T) -> Self;
    fn filter(&self, f: impl Fn(&T) -> bool) -> Self;
}

#[derive(Debug)]
pub enum SinglyLinkedList<T> {
    Node(T, Box<SinglyLinkedList<T>>),
    Nil,
}

use SinglyLinkedList::*;

impl<T: Copy> SinglyLinkedList<T> {
    fn reverse_helper(&self, acc: SinglyLinkedList<T>) -> SinglyLinkedList<T> {
        match self {
            Nil => acc,
            Node(head, tail) => tail.reverse_helper(Node(*head, Box::new(acc))),
        }
    }
}

impl<T: Copy + PartialEq> List<T> for SinglyLinkedList<T> {
    fn map(&self, func: impl Fn(&T) -> T) -> Self {
        match self {
            Nil => Nil,
            Node(head, tail) => Node(func(head), Box::new(tail.map(func))),
        }
    }

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
        self.reverse_helper(Nil)
    }

    fn find(&self, val: T) -> Option<usize> {
        let mut current_index = 0_usize;
        let mut current_node = self;

        while let Node(head, tail) = current_node {
            if val == *head {
                return Some(current_index);
            } else {
                current_node = tail;
                current_index += 1;
            }
        }

        None
    }

    fn filter(&self, f: impl Fn(&T) -> bool) -> Self {
        match self {
            Nil => Nil,
            Node(head, tail) => {
                if f(head) {
                    Node(*head, Box::new(tail.filter(f)))
                } else {
                    tail.filter(f)
                }
            }
        }
    }

    // fn reverse(&self) -> Self {
    //     fn reverse_aux<T: Copy>(
    //         lst: &SinglyLinkedList<T>,
    //         acc: SinglyLinkedList<T>,
    //     ) -> SinglyLinkedList<T> {
    //         match lst {
    //             Nil => acc,
    //             Node(head, tail) => reverse_aux(tail, Node(head.clone(), Box::new(acc))),
    //         }
    //     }

    //     reverse_aux(self, Nil)
    // }
}
