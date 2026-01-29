import {
  SinglyLinkedListNode,
  default as SinglyLinkedList,
} from "../dataStructures/SinglyLinkedList";
import { expect, test } from "vitest";

test("test", () => {
  const ls = new SinglyLinkedListNode(1, new SinglyLinkedListNode(2));
  const l = new SinglyLinkedList(ls);
  const t = new SinglyLinkedList(
    new SinglyLinkedListNode(
      1,
      new SinglyLinkedListNode(2, new SinglyLinkedListNode(3)),
    ),
  );
  expect(l.append(3)).toMatchObject(t);
});
