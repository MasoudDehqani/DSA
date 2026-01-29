export class SinglyLinkedListNode<T> {
  value: T;
  next: SinglyLinkedListNode<T> | null;

  constructor(value: T, next?: SinglyLinkedListNode<T> | null) {
    this.value = value;
    this.next = next || null;
  }

  appendInplace(newValue: T) {
    if (this.next == null) {
      this.next = new SinglyLinkedListNode(newValue);
      return;
    }

    this.next.appendInplace(newValue);
  }

  append(newValue: T): SinglyLinkedListNode<T> {
    let cp = { ...this };
    let curr = cp.next || cp;

    while (curr != null && curr.next != null) {
      curr = curr.next;
    }

    curr.next = new SinglyLinkedListNode(newValue);

    return cp;
  }

  // append(newValue: T): SinglyLinkedListNode<T> {
  //   const newNode = new SinglyLinkedListNode(newValue);
  //   if (this.value == null) return this;

  //   const newHead = new SinglyLinkedListNode(this.value);
  //   let newListCurrent = newHead;
  //   let currentNode = this.next;

  //   while (currentNode != null) {
  //     newListCurrent.next = this;
  //     newListCurrent = newListCurrent.next;
  //     currentNode = currentNode.next;
  //   }

  //   newListCurrent.next = newNode;
  //   return new SinglyLinkedList(newHead);
  // }

  // appendInPlace(newValue: T): void {
  //   if (this.value == null) return;

  //   let lastNode = this.value;
  //   while (lastNode.next != null) {
  //     lastNode = lastNode.next;
  //   }

  //   lastNode.next = new SinglyLinkedListNode(newValue);
  // }

  // read(index: number): T | null {
  //   if (this.value == null) return null;
  //   if (index < 0) throw new Error("negative index error");

  //   let currentNode: SinglyLinkedListNode<T> | null = this.value;

  //   while (currentNode != null) {
  //     if (index === 0) return currentNode.value;
  //     currentNode = currentNode.next;
  //     index -= 1;
  //   }

  //   return null;
  // }

  // search(val: T): number | null {
  //   let currentNode = this.value;
  //   let currentIndex = 0;

  //   while (currentNode != null) {
  //     if (currentNode.value === val) {
  //       return currentIndex;
  //     }

  //     currentNode = currentNode.next;
  //     currentIndex += 1;
  //   }

  //   return null;
  // }

  // delete(index: number): SinglyLinkedList<T> {
  //   if (this.value == null) throw new Error("list is empty");
  //   if (index === 0) return new SinglyLinkedList(this.value.next);
  //   if (index < 0) throw new Error("negative index error");

  //   const newHead = new SinglyLinkedListNode(this.value);
  //   let currentNewNode = newHead;
  //   let currentNode = this.value.next;

  //   for (let i = 1; i <= index; i++) {
  //     if (currentNode == null) throw new Error("out of bound index");

  //     if (i === index) {
  //       currentNewNode.next = currentNode.next;
  //       break;
  //     } else {
  //       currentNewNode.next = new SinglyLinkedListNode(currentNode.value);
  //     }

  //     currentNewNode = currentNewNode.next;
  //     currentNode = currentNode.next;
  //   }

  //   return new SinglyLinkedList(newHead);
  // }

  // deleteInPlace(index: number): void {
  //   if (this.value == null) throw new Error("empty list");
  //   if (index === 0) {
  //     this.value = this.value.next;
  //     return;
  //   }

  //   if (index < 0) throw new Error("negative index");

  //   let currentNode: SinglyLinkedListNode<T> | null = this.value;

  //   for (let i = 0; i <= index; i++) {
  //     if (currentNode == null) throw new Error("out of bound index");

  //     if (i === index - 1 && currentNode.next != null) {
  //       currentNode.next = currentNode.next.next;
  //       break;
  //     }

  //     currentNode = currentNode.next;
  //   }
  // }

  // reverse(): SinglyLinkedList<T> | null {
  //   if (this.value == null) return null;
  //   if (this.value.next == null) return new SinglyLinkedList(this.value);

  //   let currentNode = this.value;
  //   let newListNode = new SinglyLinkedListNode(this.value);

  //   while (currentNode.next != null) {
  //     newListNode = new SinglyLinkedListNode(
  //       currentNode.next.value,
  //       newListNode,
  //     );

  //     currentNode = currentNode.next;
  //   }

  //   return new SinglyLinkedList(newListNode);
  // }

  // reverseInPlace(): void {
  //   if (this.value == null || this.value.next == null) return;

  //   let prevNode = null;
  //   let currentNode: SinglyLinkedListNode<T> | null = this.value;

  //   while (currentNode != null) {
  //     const nxt: SinglyLinkedListNode<T> | null = currentNode.next;
  //     currentNode.next = prevNode;
  //     prevNode = currentNode;
  //     currentNode = nxt;
  //   }

  //   this.value = prevNode;
  // }

  // reverseRecursiveInPlace() {
  //   if (this.value == null || this.value.next == null) return;

  //   const reverseRecursiveInPlaceAux = (
  //     listHead: SinglyLinkedListNode<T>,
  //   ): SinglyLinkedListNode<T> => {
  //     if (listHead.next == null) return listHead;

  //     const newHead = reverseRecursiveInPlaceAux(listHead.next);

  //     const hd = listHead;
  //     const nxt = listHead.next;
  //     const rest = listHead.next.next;

  //     listHead = nxt;
  //     listHead.next = hd;
  //     listHead.next.next = rest;

  //     return newHead;
  //   };

  //   this.value = reverseRecursiveInPlaceAux(this.value);
  // }

  // reverseTailRecursive(): SinglyLinkedList<T> {
  //   if (this.value == null || this.value.next == null)
  //     return new SinglyLinkedList(this.value);

  //   const reverseTailRecursiveAux = (
  //     listHead: SinglyLinkedListNode<T>,
  //     acc: SinglyLinkedList<T>,
  //   ): SinglyLinkedList<T> => {
  //     const newAcc = new SinglyLinkedList(
  //       new SinglyLinkedListNode(listHead.value, acc.value),
  //     );

  //     if (listHead.next == null) return newAcc;

  //     return reverseTailRecursiveAux(listHead.next, newAcc);
  //   };

  //   return reverseTailRecursiveAux(this.value, new SinglyLinkedList());
  // }
}

export type SinglyLinkedList<T> = SinglyLinkedListNode<T> | null;
