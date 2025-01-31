export class SinglyLinkedListNode<T> {
  value: T;
  next: SinglyLinkedListNode<T> | null;

  constructor(value: T, next?: SinglyLinkedListNode<T> | null) {
    this.value = value;
    this.next = next == null ? null : next;
  }
}

export default class SinglyLinkedList<T> {
  head: SinglyLinkedListNode<T>;

  constructor(headNodeValue: SinglyLinkedListNode<T>) {
    this.head = headNodeValue;
  }
}
