// export default function insertionSort(arr: Array<number>, arrayLength: number) {
//   for (let i = 1; i < arrayLength; i++) {
//     const key = arr[i];
//     for (let j = i - 1; j >= 0; j--) {
//       if (key < arr[j]) {
//         [arr[j], arr[j + 1]] = [key, arr[j]];
//       }
//     }
//   }

//   return arr;
// }

export default function insertionSort(arr: Array<number>) {
  
  for (let i = 1; i < arr.length; i++) {

    const key = arr[i]
    let j = i - 1

    while (j >= 0 && key < arr[j]) {
      arr[j + 1] = arr[j]
      j--
    }

    arr[j + 1] = key
  }

  return arr;
}