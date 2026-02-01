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
