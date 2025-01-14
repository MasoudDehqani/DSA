import reverseString from "../reverseString/reverseString.js";

export default function isStringPalindrome(s: string): boolean {
  if (s.length < 2) return true;

  return reverseString(s) === s;
}
