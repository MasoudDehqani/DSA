export default function countDigits(n: number): number {
  if (n < 10) return 1;

  return countDigits(Math.trunc(n / 10)) + 1;
}
