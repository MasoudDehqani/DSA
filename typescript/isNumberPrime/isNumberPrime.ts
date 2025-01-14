export default function isNumberPrime(n: number, acc: number = 2): boolean {
  if (n <= 1) return false;
  if (n === 2) return true;
  if (n % acc === 0) return false;
  if (acc * acc > n) return true; // checking acc up to radical of n (square root of n) is enough

  return isNumberPrime(n, acc === 2 ? 3 : acc + 2);
}
