type rec doublyLinkedList<'a, 'b> = Empty | Node('a, 'b, doublyLinkedList<'a, 'b>)
