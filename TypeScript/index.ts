// import reverseString from "./reverseString/reverseString.js"
// import countDigits from "./countDigits/countDigits.js"
// import sumOfDigis from "./sumOfDigits/sumOfDigits.js";
// import reverseNumber from "./reverseNumber/reverseNumber.js";
// import factorial from "./factorial/factorial.js";
// import fibonacci from "./fibonacci/fibonacci.js";
// import sumOfFirstN from "./sumOfFirstN/sumOfFirstN.js";
// import isStringPalindrome from "./isStringPalindrome/isStringPalindrome.js";
// import { isNumberPalindrome } from "./isNumberPalindrome/isNumberPalindrome.js";
// import pow from "./pow/pow.js";
// import flattenArray from "./flattenArray/flattenArray.js";
// import binarySearchOnOrderedArray from "./binarySearchOnOrderedArray/binarySearchOnOrderedArray.js";
// import gcd from "./gcd/gcd.js";
// import isNumberPrime from "./isNumberPrime/isNumberPrime.js";

import SinglyLinkedList, {
  SinglyLinkedListNode,
} from "./dataStructures/SinglyLinkedList.js";

// import binaryToDecimal from "./binaryToDecimal/binaryToDecimal.js";
// import decimalToBinary from "./decimalToBinary/decimalToBinary.js";

// console.log(reverseString("Hello!"))
// console.log(countDigits(3259))
// console.log(sumOfDigis(5461))
// console.log(reverseNumber(1234))
// console.log(factorial(5))
// console.log(fibonacci(6))
// console.log(sumOfFirstN(8))
// console.log(isStringPalindrome("rotator"))
// console.log(isNumberPalindrome(4224))
// console.log(pow(5, 4))
// console.log(
//   flattenArray([
//     [[[[[0]]]]],
//     1,
//     2,
//     [3, [4]],
//     5,
//     6,
//     7,
//     [[[8]], 9],
//     [10, 11, 12, [[[[13]]]]],
//   ])
// );
// console.log(binarySearchOnOrderedArray([1, 2], 4));
// console.log(gcd(8, 6));
// console.log(isNumberPrime(71));
// console.log(decimalToBinary(23));

const newNode = new SinglyLinkedListNode(
  1,
  new SinglyLinkedListNode(
    2,
    new SinglyLinkedListNode(3, new SinglyLinkedListNode(4))
  )
);

// console.log(
//   new SinglyLinkedList(newNode).append(4).append(5).append(6).reverse()
// );
const lst = new SinglyLinkedList(newNode);
// lst.appendInPlace(4)
// const n = lst.append(3);
// const d = lst.delete(1);
// console.log(n);
// console.log(d);
// lst.deleteInPlace(3);
// lst.appendInPlace(5);
// console.log(lst.search(2));
// console.log(lst.search(3));
// console.log(lst.append(4));
// console.log(lst.delete(2));
// lst.deleteInPlace(2);
lst.reverseInPlace();

console.log(lst);
