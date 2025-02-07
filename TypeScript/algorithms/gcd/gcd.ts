// const gcd = (n1: number, n2: number) => {
//   if (n1 === n2) return n1;
//   if (n1 === 0) return n2;
//   if (n2 === 0) return n1;

//   if (n1 > n2) return gcd(n1 % n2, n2);
//   return gcd(n2 % n1, n1);
// };

const gcd = (n1: number, n2: number): number =>
  n2 === 0 ? n1 : gcd(n2, n1 % n2);

export default gcd;
