// export default function gcd(n1: number, n2: number): number {
//   return n2 === 0 ? n1 : gcd(n2, n1 % n2);
// }

const gcd = (n1: number, n2: number): number =>
  n2 === 0 ? n1 : gcd(n2, n1 % n2);

export default gcd;
