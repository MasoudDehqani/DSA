export default function binarySearchOnOrderedArray(
  orderedArray: Array<number>,
  n: number,
  from: number = 0,
  to: number = orderedArray.length - 1
): number | null {
  if (from > to) return null;

  const midIndex = Math.floor((from + to) / 2);

  if (orderedArray[midIndex] === n) return midIndex;
  if (n > orderedArray[midIndex])
    return binarySearchOnOrderedArray(orderedArray, n, midIndex + 1, to);
  // if (n < midIndex)
  return binarySearchOnOrderedArray(orderedArray, n, from, midIndex - 1);
}
