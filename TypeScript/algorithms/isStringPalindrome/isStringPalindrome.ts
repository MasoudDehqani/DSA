const isStringPalindromeAux = (s: string, f: number, last: number) => {
  if (f > Math.trunc(s.length / 2)) return true;

  if (s[f] !== s[last]) return false;
  return isStringPalindromeAux(s, f + 1, last - 1);
};

export default function isStringPalindrome(s: string): boolean {
  if (s.length < 2) return true;

  return isStringPalindromeAux(s, 0, s.length - 1);
}
