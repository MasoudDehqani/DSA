open SinglyLinkedList

// Console.log(Node(makeNode(1))->append(2)->append(3)->append(4)->deleteByIndex(1))
// Console.log(Node(makeNode(1))->append(2)->append(4)->insertAtIndex(2, 3))

let myList = Node(1, Node(2, Node(2, Node(3, Empty))))
let myList2 = Node(1, Node(1, Node(2, Node(3, Empty))))
let myList3 = Node(1, Node(2, Node(3, Node(3, Empty))))
let myList4 = Node(1, Node(2, Node(3, Node(4, Empty))))
let myList5 = Empty
let myList6 = Node(1, Empty)

Console.log(read(myList6, 1))
