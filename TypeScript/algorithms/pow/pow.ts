export function powRecursive(n: number, exp: number): number {
  if (exp === 0) return 1;

  return exp > 0 ? powRecursive(n, exp - 1) * n : powRecursive(n, exp + 1) / n;
}

const powTailRecursiveAux = (n: number, exp: number, acc: number): number => {
  if (exp === 0) return acc;
  return powTailRecursiveAux(n, exp - 1, acc * n);
};

export function powTailRecursive(n: number, exp: number): number {
  const res = powTailRecursiveAux(n, exp >= 0 ? exp : -exp, 1);
  return exp > 0 ? res : 1 / res;
}

export function powTailRecursiveSquaring(n: number, exp: number): number {
  if (exp === 0) return 1;
  if (exp < 0) return 1 / powTailRecursiveSquaring(n, -exp);

  const half = powTailRecursiveAux(n, Math.floor(exp / 2), 1);
  const squared = half * half;

  return exp % 2 === 0 ? squared : squared * n;
}
