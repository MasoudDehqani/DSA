export class SinglyLinkedListNode<T> {
  value: T;
  next: SinglyLinkedListNode<T> | null;

  constructor(value: T, next?: SinglyLinkedListNode<T> | null) {
    this.value = value;
    this.next = next == null ? null : next;
  }
}

export default class SinglyLinkedList<T> {
  head?: SinglyLinkedListNode<T> | null;

  constructor(headNodeValue?: SinglyLinkedListNode<T> | null) {
    this.head = headNodeValue;
  }

  append(newValue: T): SinglyLinkedList<T> {
    const newNode = new SinglyLinkedListNode(newValue);
    if (this.head == null) return new SinglyLinkedList(newNode);

    const newHead = new SinglyLinkedListNode(this.head.value);
    let newListCurrent = newHead;
    let currentNode = this.head.next;

    while (currentNode != null) {
      newListCurrent.next = new SinglyLinkedListNode(currentNode.value)
      newListCurrent = newListCurrent.next;
      currentNode = currentNode.next;
    }

    newListCurrent.next = newNode
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
    if (this.head == null || this.head.next == null || index < 0) return null;
    if (index === 0) return new SinglyLinkedList(this.head.next)

    const newHead = new SinglyLinkedListNode(this.head.value);
    let newCurrentNode = newHead;
    let currentNode: SinglyLinkedListNode<T> | null = this.head.next;

    for (let i = 1; i <= index; i++) {
      if (currentNode == null) break

      if (i === index) {
        newCurrentNode.next = currentNode.next
        break
      } else {
        newCurrentNode.next = new SinglyLinkedListNode(currentNode.value)
      }

      newCurrentNode = newCurrentNode.next
      currentNode = currentNode.next
    }

    return new SinglyLinkedList(newHead)
  }

  deleteInPlace(index: number) {
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
