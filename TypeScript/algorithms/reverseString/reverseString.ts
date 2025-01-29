export default function reverseString(s: string): string {
  if (s.length < 2) return s;

  return reverseString(s.slice(1)) + s[0];
}
