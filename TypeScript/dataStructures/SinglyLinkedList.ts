export class SinglyLinkedListNode<T> {
  value: T;
  next: SinglyLinkedListNode<T> | null;

  constructor(value: T, next?: SinglyLinkedListNode<T> | null) {
    this.value = value;
    this.next = next || null;
  }
}

export default class SinglyLinkedList<T> {
  head: SinglyLinkedListNode<T> | null;

  constructor(headNodeValue?: SinglyLinkedListNode<T> | null) {
    this.head = headNodeValue || null;
  }

  append(newValue: T): SinglyLinkedList<T> {
    const newNode = new SinglyLinkedListNode(newValue);
    if (this.head == null) return new SinglyLinkedList(newNode);

    const newHead = new SinglyLinkedListNode(this.head.value);
    let newListCurrent = newHead;
    let currentNode = this.head.next;

    while (currentNode != null) {
      newListCurrent.next = new SinglyLinkedListNode(currentNode.value);
      newListCurrent = newListCurrent.next;
      currentNode = currentNode.next;
    }

    newListCurrent.next = newNode;
    return new SinglyLinkedList(newHead);
  }

  appendInPlace(newValue: T): void {
    if (this.head == null) return;

    let lastNode = this.head;
    while (lastNode.next != null) {
      lastNode = lastNode.next;
    }

    lastNode.next = new SinglyLinkedListNode(newValue);
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
    let currentNode = this.head;
    let currentIndex = 0;

    while (currentNode != null) {
      if (currentNode.value === val) {
        return currentIndex;
      }

      currentNode = currentNode.next;
      currentIndex += 1;
    }

    return null;
  }

  delete(index: number) {
    if (this.head == null) throw new Error("list is empty");
    if (index === 0) return new SinglyLinkedList(this.head.next);
    if (index < 0) throw new Error("negative index error");

    let newHead = new SinglyLinkedListNode(this.head.value);
    let currentNode = this.head.next;

    for (let i = 1; i <= index; i++) {
      if (currentNode == null) throw new Error("out of bound index");

      if (i === index) {
        newHead.next = currentNode.next;
        break;
      } else {
        newHead.next = currentNode;
      }

      newHead = newHead.next;
      currentNode = currentNode.next;
    }

    return new SinglyLinkedList(newHead);
  }

  deleteInPlace(index: number) {
    if (this.head == null) throw new Error("empty list");
    if (index === 0) {
      this.head = this.head.next;
      return;
    }

    if (index < 0) throw new Error("negative index");

    let currentNode: SinglyLinkedListNode<T> | null = this.head;

    for (let i = 0; i <= index; i++) {
      if (currentNode == null) throw new Error("out of bound index");

      if (i === index - 1 && currentNode.next != null) {
        currentNode.next = currentNode.next.next;
        break;
      }

      currentNode = currentNode.next;
    }
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
