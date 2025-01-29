export default function sumOfDigis(n: number): number {
  if (n < 10) return n;

  return sumOfDigis(Math.trunc(n / 10)) + (n % 10);
}
