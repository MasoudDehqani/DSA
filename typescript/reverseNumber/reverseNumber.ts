// import countDigits from "../countDigits/countDigits.js"

function countDigits(n: number) {
  return n === 0 ? n : Math.trunc(Math.log10(Math.abs(n))) + 1;
}

// export default function reverseNumber(n: number): number {
//   if (n < 10) return n

//   const q = Math.trunc(n / 10)
//   return reverseNumber(q) + ((n % 10) * (10 ** countDigits(q)))
// }

export default function reverseNumber(
  n: number,
  digitCount: number = countDigits(n)
): number {
  if (n < 10) return n;

  const q = Math.trunc(n / 10);
  return reverseNumber(q, digitCount - 1) + (n % 10) * 10 ** (digitCount - 1);
}
