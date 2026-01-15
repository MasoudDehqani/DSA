// export default function flattenArray(arr: Array<any>) {
//   return arr.reduce(
//     (acc, curr) =>
//       Array.isArray(curr) ? acc.concat(flattenArray(curr)) : acc.concat([curr]),
//     []
//   );
// }

// const flattenArrayAux = <T>(
//   arr: Array<T | Array<T>>,
//   acc: Array<T>,
//   currentIndex: number
// ): Array<T> => {
//   if (arr[currentIndex] == null) return acc;

//   if (Array.isArray(arr[currentIndex])) {
//     acc.concat(flattenArrayAux(arr[currentIndex], acc, 0));
//   } else {
//     acc.push(arr[currentIndex]);
//   }

//   return flattenArrayAux(arr, acc, currentIndex + 1);
// };

// export default function flattenArray<T>(arr: Array<T | Array<T>>): Array<T> {
//   return flattenArrayAux(arr, [], 0);
// }

function flattenArrayAux<T>(
  arr: Array<T>,
  currentIndex: number,
  acc: Array<T>
) {
  const curr = arr[currentIndex];
  if (curr == null) return;

  if (Array.isArray(curr)) {
    flattenArrayAux(curr, 0, acc);
  } else {
    acc.push(curr);
  }

  flattenArrayAux(arr, currentIndex + 1, acc);
}

export function flattenArray<T>(arr: Array<T>) {
  let acc: Array<T> = [];
  flattenArrayAux(arr, 0, acc);
  return acc;
}
