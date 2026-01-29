import { SinglyLinkedListNode } from "../dataStructures/SinglyLinkedList";
import { expect, test } from "vitest";

test("test", () => {
  const ls = new SinglyLinkedListNode(1, new SinglyLinkedListNode(2));
  // ls.appendInplace(3);
  // ls.append(3)
  const t = new SinglyLinkedListNode(
    1,
    new SinglyLinkedListNode(2, new SinglyLinkedListNode(3)),
  );

  // console.log(ls.append(3));
  // console.log(ls);

  expect(ls.append(3)).toMatchObject(t);
});
