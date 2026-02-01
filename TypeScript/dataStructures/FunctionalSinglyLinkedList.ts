export interface FunctionalSinglyLinkedListNode<T> {
  value: T;
  next: FunctionalSinglyLinkedListNode<T> | null;
}

export type FunctionalSinglyLinkedList<T> =
  FunctionalSinglyLinkedListNode<T> | null;

export function append<T>(
  value: T,
  list: FunctionalSinglyLinkedList<T>,
): FunctionalSinglyLinkedList<T> {
  if (list == null) {
    return { value, next: null };
  }

  return { value: list.value, next: append(value, list.next) };
}

export function prepend<T>(
  value: T,
  list: FunctionalSinglyLinkedList<T>,
): FunctionalSinglyLinkedList<T> {
  return { value, next: list };
}

export function insert<T>(
  value: T,
  index: number,
  list: FunctionalSinglyLinkedList<T>,
): FunctionalSinglyLinkedList<T> {
  if (list == null) {
    return index === 0 ? { value, next: null } : list;
  }

  if (index === 0) {
    return { value, next: list };
  }

  return { value: list.value, next: insert(value, index - 1, list.next) };
}

export function read<T>(
  index: number,
  list: FunctionalSinglyLinkedList<T>,
): T | null {
  if (list == null) {
    return null;
  }

  if (index === 0) {
    return list.value;
  }

  return read(index - 1, list.next);
}

export function search<T>(
  value: T,
  list: FunctionalSinglyLinkedList<T>,
): number | null {
  const aux = (i: number, ls: FunctionalSinglyLinkedList<T>): number | null => {
    if (ls == null) {
      return null;
    }

    if (value === ls.value) {
      return i;
    }

    return aux(i + 1, ls.next);
  };

  return aux(0, list);
}

export function deleteByIndex<T>(
  index: number,
  list: FunctionalSinglyLinkedList<T>,
): FunctionalSinglyLinkedList<T> {
  if (list == null) {
    return list;
  }

  if (index == 0) {
    return list.next ? { value: list.next.value, next: list.next.next } : null;
  }

  return { value: list.value, next: deleteByIndex(index - 1, list.next) };
}

export function deleteByValue<T>(
  value: T,
  list: FunctionalSinglyLinkedList<T>,
): FunctionalSinglyLinkedList<T> {
  if (list == null) {
    return list;
  }

  if (value === list.value) {
    return list.next ? { value: list.next.value, next: list.next.next } : null;
  }

  return { value: list.value, next: deleteByValue(value, list.next) };
}

export function toArray<T>(list: FunctionalSinglyLinkedList<T>): T[] {
  let arr: Array<T> = [];

  const aux = (l: FunctionalSinglyLinkedList<T>) => {
    if (l == null) {
      return;
    }

    arr.push(l.value);
    aux(l.next);
  };

  aux(list);
  return arr;
}

export function reverse<T>(
  list: FunctionalSinglyLinkedList<T>,
): FunctionalSinglyLinkedList<T> {
  const aux = (
    ls: FunctionalSinglyLinkedList<T>,
    acc: FunctionalSinglyLinkedList<T>,
  ): FunctionalSinglyLinkedList<T> => {
    if (ls == null) {
      return acc;
    }

    const newAcc = { value: ls.value, next: acc };
    return aux(ls.next, newAcc);
  };

  return aux(list, null);
}

export function display<T>(list: FunctionalSinglyLinkedList<T>) {
  let res = "";

  const aux = (list: FunctionalSinglyLinkedList<T>) => {
    if (list == null) {
      res += "null";
      return;
    }

    res = res + list.value + " -> ";
    aux(list.next);
  };

  aux(list);
  console.log(res);
}
