import { expect, test, describe, it, vi } from "vitest";
import {
  append,
  deleteByIndex,
  deleteByValue,
  display,
  insert,
  prepend,
  read,
  reverse,
  search,
  toArray,
} from "../dataStructures/FunctionalSinglyLinkedList.ts";

test("append", () => {
  const ls = { value: 1, next: { value: 2, next: null } };
  const ex = { value: 1, next: { value: 2, next: { value: 3, next: null } } };

  expect(append(3, ls)).toMatchObject(ex);
});

test("prepend", () => {
  const ls = { value: 1, next: { value: 2, next: null } };
  const ex = { value: 0, next: { value: 1, next: { value: 2, next: null } } };

  expect(prepend(0, ls)).toMatchObject(ex);
});

test("insert", () => {
  const ls = { value: 1, next: { value: 3, next: null } };
  const ex = { value: 1, next: { value: 2, next: { value: 3, next: null } } };
  const ex2 = {
    value: 1,
    next: { value: 2, next: { value: 3, next: { value: 4, next: null } } },
  };

  expect(insert(2, 1, ls)).toMatchObject(ex);
  expect(insert(4, 3, ex)).toMatchObject(ex2);
});

test("toArray", () => {
  const ls = { value: 1, next: { value: 2, next: { value: 3, next: null } } };

  expect(toArray(ls)).toEqual([1, 2, 3]);
});

test("reverse", () => {
  const ls = { value: 1, next: { value: 2, next: { value: 3, next: null } } };
  const ex = { value: 3, next: { value: 2, next: { value: 1, next: null } } };

  expect(reverse(ls)).toMatchObject(ex);
});

test("read", () => {
  const ls = { value: 1, next: { value: 2, next: { value: 3, next: null } } };

  expect(read(1, ls)).toBe(2);
  expect(read(0, ls)).toBe(1);
  expect(read(2, ls)).toBe(3);
  expect(read(3, ls)).toBe(null);
});

test("search", () => {
  const ls = { value: 1, next: { value: 2, next: { value: 3, next: null } } };

  expect(search(1, ls)).toBe(0);
  expect(search(2, ls)).toBe(1);
  expect(search(3, ls)).toBe(2);
  expect(search(0, ls)).toBe(null);
});

test("delete by index", () => {
  const ls = { value: 1, next: { value: 2, next: { value: 3, next: null } } };

  expect(deleteByIndex(1, ls)).toMatchObject({
    value: 1,
    next: { value: 3, next: null },
  });
  expect(deleteByIndex(0, ls)).toMatchObject({
    value: 2,
    next: { value: 3, next: null },
  });
  expect(deleteByIndex(2, ls)).toMatchObject({
    value: 1,
    next: { value: 2, next: null },
  });
  expect(deleteByIndex(3, ls)).toMatchObject({
    value: 1,
    next: { value: 2, next: { value: 3, next: null } },
  });
});

test("delete by value", () => {
  const ls = { value: 1, next: { value: 2, next: { value: 3, next: null } } };

  expect(deleteByValue(1, ls)).toMatchObject({
    value: 2,
    next: { value: 3, next: null },
  });
  expect(deleteByValue(2, ls)).toMatchObject({
    value: 1,
    next: { value: 3, next: null },
  });
  expect(deleteByValue(3, ls)).toMatchObject({
    value: 1,
    next: { value: 2, next: null },
  });

  expect(deleteByValue(4, ls)).toMatchObject({
    value: 1,
    next: { value: 2, next: { value: 3, next: null } },
  });
});

describe("display", () => {
  it("logs the display", () => {
    const spy = vi.spyOn(console, "log").mockImplementation(() => {});

    const ls = { value: 1, next: { value: 2, next: { value: 3, next: null } } };
    display(ls);

    expect(spy).toHaveBeenCalledWith("1 -> 2 -> 3 -> null");

    spy.mockRestore();
  });
});
