export default function binaryToDecimal(n: Array<number>): number {
  if (n.length === 0) return 0;

  const f = n.length - 1;

  return binaryToDecimal(n.slice(1)) + n[0] * 2 ** f;
}
