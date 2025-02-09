import isStringPalindrome from "../isStringPalindrome/isStringPalindrome.js";

export function isNumberPalindrome(n: number): boolean {
  if (n < 10) return true;

  return isStringPalindrome(n.toString());
}
