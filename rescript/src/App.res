open SinglyLinkedList

// Console.log(Node(makeNode(1))->append(2)->append(3)->append(4)->deleteByIndex(1))
// Console.log(Node(makeNode(1))->append(2)->append(4)->insertAtIndex(2, 3))

// let myList = Node(1, Node(2, Node(3, Empty)))
// let myList2 = Node(1, Node(3, Node(4, Empty)))
// let myList3 = Node(2, Node(3, Node(4, Empty)))
// let myList4 = Node(1, Node(2, Node(3, Node(4, Empty))))

let myList = Node(1, Node(2, Node(2, Node(3, Empty))))
let myList2 = Node(1, Node(1, Node(2, Node(3, Empty))))
let myList3 = Node(1, Node(2, Node(3, Node(3, Empty))))
let myList4 = Node(1, Node(2, Node(3, Node(4, Empty))))

// Console.log(read(myList, -1))
// Console.log(search(myList, 2))
// Console.log(append(myList, 4))
// Console.log(insert(myList, 2, 1))
// Console.log(insert(myList2, 2, 1))
// Console.log(insert(myList3, 1, 0))
// Console.log(insert(myList4, 5, 4))
Console.log(delete(myList, 1))
Console.log(delete(myList2, 0))
Console.log(delete(myList3, 2))
Console.log(delete(myList4, 3))
Console.log(delete(myList4, 4))
