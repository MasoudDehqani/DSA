open SinglyLinkedList

let app = () => {
  Console.log(Node(makeNode(1))->append(2)->append(3)->append(4)->insertAtIndex(5, 2))
}
