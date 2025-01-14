export default function decimalToBinary(n: number): Array<number> {
  if (n === 0) return [];

  const b = n % 2 === 0 ? 0 : 1;

  return decimalToBinary(Math.floor(n / 2)).concat([b]);
}
