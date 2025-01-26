open SinglyLinkedList

// Console.log(Node(makeNode(1))->append(2)->append(3)->append(4)->deleteByIndex(1))
// Console.log(Node(makeNode(1))->append(2)->append(4)->insertAtIndex(2, 3))

let myList = Node(1, Node(2, Node(3, Empty)))

Console.log(readByIndex(myList, 2))
