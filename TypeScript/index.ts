import { append } from "./dataStructures/FunctionalSinglyLinkedList.ts";

const ls = { value: 1, next: { value: 2, next: null } };
console.log(append(3, ls));
