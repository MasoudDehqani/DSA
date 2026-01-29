import { SinglyLinkedListNode } from "../dataStructures/SinglyLinkedList";
import { expect, test } from "vitest";

test("test", () => {
  const ls = new SinglyLinkedListNode(1);
  const res = new SinglyLinkedListNode(1, new SinglyLinkedListNode(2));

  expect(ls.append(2)).toMatchObject(res);
});

test("test 2", () => {
  const ls = new SinglyLinkedListNode(1, new SinglyLinkedListNode(2));
  const res = new SinglyLinkedListNode(
    1,
    new SinglyLinkedListNode(2, new SinglyLinkedListNode(3)),
  );

  expect(ls.append(3)).toMatchObject(res);
});
