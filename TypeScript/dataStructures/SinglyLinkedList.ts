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
      return new SinglyLinkedList(lastNode);
    }

    lastNode.next = new SinglyLinkedListNode(newValue);
    return new SinglyLinkedList(this.head);
  }

  read(index: number) {
    let currentNode = this.head;

    if (currentNode == null) return null;

    while (index > 0 && currentNode.next != null) {
      currentNode = currentNode.next;
      index -= 1;
    }

    return index === 0 ? currentNode.value : null;
  }

  search(val: T): number | null {
    let currentIndex = 0;

    let currentNode = this.head;

    if (currentNode == null) return null;

    while (currentNode.value !== val && currentNode.next != null) {
      currentNode = currentNode.next;
      currentIndex += 1;
    }

    return currentNode.value === val ? currentIndex : null;
  }

  delete(index: number) {
    if (this.head == null || this.head.next == null) return null;

    let currentNode = this.head;

    while (index > 1 && currentNode.next != null) {
      index -= 1;
      currentNode = currentNode.next;
    }

    if (index === 1) {
      currentNode.next =
        currentNode.next?.next == null ? null : currentNode.next.next;
    }

    return this.head;
  }

  reverse() {
    if (this.head == null) return null;
    if (this.head.next == null) return this.head;

    let currentNode = this.head;
    let newListNode = new SinglyLinkedListNode(this.head.value);

    while (currentNode.next != null) {
      newListNode = new SinglyLinkedListNode(
        currentNode.next.value,
        newListNode
      );

      currentNode = currentNode.next;
    }

    return new SinglyLinkedList(newListNode);
  }
}
