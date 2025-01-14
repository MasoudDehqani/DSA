export default function pow(n: number, p: number): number {
  if (p === 0) return 1;
  if (n === 0) return 0;

  if (p < 0) return 1 / pow(n, -p);

  const halfPower = pow(n, Math.trunc(p / 2));
  const res = halfPower * halfPower;

  return p % 2 === 0 ? res : res * n;
}
