export default function sumOfFirstN(n: number): number {
  if (n < 1) return n;

  return sumOfFirstN(n - 1) + n;
}
