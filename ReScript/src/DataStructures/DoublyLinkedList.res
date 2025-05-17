type rec doublyLinkedList<'a> = Empty | Node('a, doublyLinkedList<'a>, doublyLinkedList<'a>)
