class SinglyLinkedListNode<T> {
  value: T;
  next: SinglyLinkedListNode<T> | null;

  constructor(value: T, next: SinglyLinkedListNode<T> | null = null) {
    this.value = value;
    this.next = next;
  }
}

export class SinglyLinkedList<T> {
  head: SinglyLinkedListNode<T> | null;

  constructor(headData: T | null = null) {
    this.head = headData != null ? new SinglyLinkedListNode(headData) : null;
  }

  appendInPlace(newValue: T): void {
    const newNode = new SinglyLinkedListNode(newValue);

    if (this.head == null) {
      this.head = newNode;
      return;
    }

    let curr = this.head;

    while (curr.next) {
      curr = curr.next;
    }

    curr.next = newNode;
  }

  append(newValue: T): SinglyLinkedList<T> {
    if (this.head == null) {
      return new SinglyLinkedList(newValue);
    }

    const newList: SinglyLinkedList<T> = new SinglyLinkedList();
    newList.head = new SinglyLinkedListNode(this.head.value);

    let currOld = this.head.next;
    let currNew = newList.head;

    while (currOld && currNew) {
      currNew.next = new SinglyLinkedListNode(currOld.value);
      currNew = currNew.next;
      currOld = currOld.next;
    }

    currNew.next = new SinglyLinkedListNode(newValue);

    return newList;
  }

  // prepend(newValue: T): SinglyLinkedList<T> {
  //   const newList = new SinglyLinkedList<T>();
  //   newList.head = new SinglyLinkedListNode(newValue, this.head);
  //   return newList;
  // }

  prepend(newValue: T): SinglyLinkedList<T> {
    let newList = new SinglyLinkedList(newValue);

    let oldCurr = this.head;
    let newCurr = newList.head;

    while (oldCurr && newCurr) {
      newCurr.next = new SinglyLinkedListNode(oldCurr.value);
      oldCurr = oldCurr.next;
      newCurr = newCurr.next;
    }

    return newList;
  }

  prependInPlace(newValue: T) {
    this.head = new SinglyLinkedListNode(newValue, this.head);
  }

  removeByIndexInPlace(index: number) {
    if (index < 0) return;
    if (this.head == null) return;

    if (index === 0) {
      this.head = this.head.next;
      return;
    }

    let currIndex = index - 1;
    let curr = this.head;

    while (curr.next) {
      if (currIndex === 0) {
        curr.next = curr.next.next;
        return;
      }

      currIndex -= 1;
      curr = curr.next;
    }
  }

  removeByIndex(index: number): SinglyLinkedList<T> {
    if (index < 0) throw new Error("Negative index");

    const newList = new SinglyLinkedList<T>();

    const aux = (
      i: number,
      node: SinglyLinkedListNode<T> | null,
    ): SinglyLinkedListNode<T> | null => {
      if (node == null) {
        return new SinglyLinkedList<T>().head;
      }

      if (i === 0) {
        return node.next;
      }

      const newNextNode =
        i === 1 ? node.next?.next || null : aux(i - 1, node.next);

      return new SinglyLinkedListNode(node.value, newNextNode);
    };

    newList.head = aux(index, this.head);
    return newList;
  }

  // removeByIndex(index: number): SinglyLinkedList<T> {
  //   if (this.head == null) return new SinglyLinkedList<T>();
  //   if (index === 0) {
  //     if (this.head.next) {
  //       const res = new SinglyLinkedList(this.head.next.value);
  //       res.head = this.head.next.next;
  //       return res;
  //     }

  //     return new SinglyLinkedList();
  //   }

  //   if (index < 0) {
  //     throw new Error("negative index");
  //   }

  //   let curr = this.head;
  //   let currIndex = index - 1;
  //   let newList = new SinglyLinkedList(this.head.value);
  //   let newCurr = this.head;

  //   while (curr.next) {
  //     if (currIndex === 0) {
  //       newCurr.next = curr.next;
  //     }

  //     newCurr.next = curr;
  //     curr = curr.next;
  //   }
  // }

  toArray(): T[] {
    let arr = [];
    let curr = this.head;

    while (curr) {
      arr.push(curr.value);
      curr = curr.next;
    }

    return arr;
  }

  display() {
    let str = "";
    let curr = this.head;
    while (curr) {
      str = str + curr.value + " -> ";
      curr = curr.next;
    }

    str = str + "null";

    console.log(str);
  }
}
