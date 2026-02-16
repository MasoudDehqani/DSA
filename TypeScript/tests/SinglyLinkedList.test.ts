import { SinglyLinkedList } from "../dataStructures/SinglyLinkedList";
import { expect, test, describe, it, vi } from "vitest";

test("toArray", () => {
  const ls = new SinglyLinkedList();
  ls.appendInPlace(0);
  ls.appendInPlace(1);
  ls.appendInPlace(2);

  expect(ls.toArray()).toEqual([0, 1, 2]);
});

test("append", () => {
  const ls = new SinglyLinkedList(1);
  ls.appendInPlace(2);
  ls.appendInPlace(3);
  ls.appendInPlace(4);

  expect(ls.append(5).toArray()).toEqual([1, 2, 3, 4, 5]);
  expect(ls.toArray()).toEqual([1, 2, 3, 4]);
});

test("appendInPlace", () => {
  const ls = new SinglyLinkedList(1);
  ls.appendInPlace(2);
  ls.appendInPlace(3);
  ls.appendInPlace(4);

  expect(ls.toArray()).toEqual([1, 2, 3, 4]);
});

test("prepend", () => {
  const ls = new SinglyLinkedList(1);
  ls.appendInPlace(2);
  ls.appendInPlace(3);
  ls.appendInPlace(4);

  expect(ls.prepend(0).toArray()).toEqual([0, 1, 2, 3, 4]);
  expect(ls.toArray()).toEqual([1, 2, 3, 4]);
});

test("prependInPlace", () => {
  const ls = new SinglyLinkedList(1);
  ls.appendInPlace(2);
  ls.appendInPlace(3);
  ls.appendInPlace(4);
  ls.prependInPlace(0);

  expect(ls.toArray()).toEqual([0, 1, 2, 3, 4]);
});

test("removeInPlace by index", () => {
  const ls = new SinglyLinkedList(1);
  ls.removeByIndexInPlace(0);

  const ls2 = new SinglyLinkedList(1);
  ls.removeByIndexInPlace(1);

  const ls3 = new SinglyLinkedList(1);
  ls3.appendInPlace(2);
  ls3.appendInPlace(3);
  ls3.appendInPlace(4);
  ls3.appendInPlace(5);
  ls3.removeByIndexInPlace(4);

  const ls4 = new SinglyLinkedList();
  ls4.removeByIndexInPlace(0);

  const ls5 = new SinglyLinkedList(1);
  ls5.appendInPlace(2);
  ls5.appendInPlace(3);
  ls5.removeByIndexInPlace(0);

  const ls6 = new SinglyLinkedList(1);
  ls6.appendInPlace(2);
  ls6.appendInPlace(3);
  ls6.removeByIndexInPlace(1);

  expect(ls.toArray()).toEqual([]);
  expect(ls2.toArray()).toEqual([1]);
  expect(ls3.toArray()).toEqual([1, 2, 3, 4]);
  expect(ls4.toArray()).toEqual([]);
  expect(ls5.toArray()).toEqual([2, 3]);
  expect(ls6.toArray()).toEqual([1, 3]);
});

test("remove by index", () => {
  const ls = new SinglyLinkedList(1);

  const ls2 = new SinglyLinkedList(1);

  const ls3 = new SinglyLinkedList(1);
  ls3.appendInPlace(2);
  ls3.appendInPlace(3);
  ls3.appendInPlace(4);
  ls3.appendInPlace(5);

  const ls4 = new SinglyLinkedList();

  const ls5 = new SinglyLinkedList(1);
  ls5.appendInPlace(2);
  ls5.appendInPlace(3);

  const ls6 = new SinglyLinkedList(1);
  ls6.appendInPlace(2);
  ls6.appendInPlace(3);

  expect(ls.removeByIndex(0).toArray()).toEqual([]);
  expect(ls2.removeByIndex(1).toArray()).toEqual([1]);
  expect(ls3.removeByIndex(4).toArray()).toEqual([1, 2, 3, 4]);
  expect(ls4.removeByIndex(0).toArray()).toEqual([]);
  expect(ls5.removeByIndex(0).toArray()).toEqual([2, 3]);
  expect(ls6.removeByIndex(1).toArray()).toEqual([1, 3]);
});

describe("display", () => {
  it("logs the display", () => {
    const spy = vi.spyOn(console, "log").mockImplementation(() => {});

    const ls = new SinglyLinkedList(1);
    ls.appendInPlace(2);
    ls.appendInPlace(3);
    ls.appendInPlace(4);

    ls.display();

    expect(spy).toHaveBeenCalledWith("1 -> 2 -> 3 -> 4 -> null");

    spy.mockRestore();
  });
});
