import { reverseNumber } from "../reverseNumber/reverseNumber.js";

export function isNumberPalindrome(n: number): boolean {
  if (n < 10) return true;

  return reverseNumber(n) === n;
}
