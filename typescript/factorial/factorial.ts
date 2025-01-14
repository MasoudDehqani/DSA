export default function factorial(n: number): number {
  if (n < 3) return n;

  return factorial(n - 1) * n;
}
