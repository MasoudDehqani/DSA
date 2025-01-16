open SinglyLinkedList

// Console.log(readByIndex(Node({value: 1, next: Some({value: 2, next: Some({value: 3, next: Some({value: 4, next: None})})})}), 2))
Console.log(Node(makeNode(1))->append(2)->append(3)->append(4)->search(2))
// Console.log("TEST")
