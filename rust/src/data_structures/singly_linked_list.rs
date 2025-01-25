#[derive(Debug)]
pub enum SinglyLinkedList<T> {
    Nil,
    Node(T, Box<SinglyLinkedList<T>>),
}

use crate::SinglyLinkedList::{Nil, Node};

impl<T: Copy> SinglyLinkedList<T> {
    pub fn read(&self, index: u32) -> Option<T> {
        fn read_aux<T: Copy>(
            lst: &SinglyLinkedList<T>,
            index: u32,
            current_index: u32,
        ) -> Option<T> {
            match lst {
                Nil => None,
                Node(v, nxt) => {
                    if index == current_index {
                        Some(*v)
                    } else {
                        read_aux(nxt, index, current_index + 1)
                    }
                }
            }
        }

        read_aux(&self, index, 0)
    }
}
