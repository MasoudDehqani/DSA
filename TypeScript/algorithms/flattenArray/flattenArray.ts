// export default function flattenArray(arr: Array<any>) {
//   return arr.reduce(
//     (acc, curr) =>
//       Array.isArray(curr) ? acc.concat(flattenArray(curr)) : acc.concat([curr]),
//     []
//   );
// }

const flattenArrayAux = <T>(
  arr: Array<T | Array<T>>,
  acc: Array<T>,
  currentIndex: number
): Array<T> => {
  if (arr[currentIndex] == null) return acc;

  if (Array.isArray(arr[currentIndex])) {
    acc.concat(flattenArrayAux(arr[currentIndex], acc, 0));
  } else {
    acc.push(arr[currentIndex]);
  }

  return flattenArrayAux(arr, acc, currentIndex + 1);
};

export default function flattenArray<T>(arr: Array<T | Array<T>>): Array<T> {
  return flattenArrayAux(arr, [], 0);
}
