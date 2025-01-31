export class SinglyLinkedListNode<T> {
  value: T;
  next: SinglyLinkedListNode<T> | null;

  constructor(value: T, next?: SinglyLinkedListNode<T> | null) {
    this.value = value;
    this.next = next == null ? null : next;
  }
}

export default class SinglyLinkedList<T> {
  head?: SinglyLinkedListNode<T>;

  constructor(headNodeValue?: SinglyLinkedListNode<T>) {
    this.head = headNodeValue;
  }

  private getLastNode() {
    if (this.head == null) return this.head;

    let lastNode = this.head;
    while (lastNode.next != null) {
      lastNode = lastNode.next;
    }

    return lastNode;
  }

  append(newValue: T) {
    let lastNode = this.getLastNode();
    if (lastNode == null) {
      lastNode = new SinglyLinkedListNode(newValue);
      return lastNode;
    }

    lastNode.next = new SinglyLinkedListNode(newValue);
    return this.head;
  }
}
