// import countDigits from "../countDigits/countDigits.js"

function countDigits(n: number) {
  return n === 0 ? n : Math.trunc(Math.log10(Math.abs(n))) + 1;
}

// export default function reverseNumber(n: number): number {
//   if (n < 10) return n

//   const q = Math.trunc(n / 10)
//   return reverseNumber(q) + ((n % 10) * (10 ** countDigits(q)))
// }

export function reverseNumber(
  n: number,
  digitCount: number = countDigits(n)
): number {
  if (n < 10) return n;

  const q = Math.trunc(n / 10);
  return reverseNumber(q, digitCount - 1) + (n % 10) * 10 ** (digitCount - 1);
}

const reverseNumberTailRecursiveAux = (n: number, acc: number) => {
  if (n === 0) return acc;

  const quotient = Math.trunc(n / 10);
  const rem = n % 10;
  // const exponent = quotient === 0 ? 0 : Math.trunc(Math.log10(quotient)) + 1;

  // return reverseNumberTailRecursiveAux(quotient, acc + rem * 10 ** exponent);

  return reverseNumberTailRecursiveAux(quotient, acc * 10 + rem);
};

export function reverseNumberTailRecursive(n: number) {
  if (!Number.isInteger(n)) {
    throw new Error("Input must be an integer.");
  }

  const isNegative = n < 0;
  const positiveNumber = isNegative ? -n : n;
  const res = reverseNumberTailRecursiveAux(positiveNumber, 0);
  return isNegative ? -res : res;
}
