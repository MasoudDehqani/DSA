export default function insertionSort(arr: Array<number>, arrayLength: number) {
  for (let i = 0; i < arrayLength - 1; i++) {
    const key = arr[i + 1];
    for (let j = i - 1; j > 0; j--) {
      if (key < arr[j]) {
        arr[i + 1] = arr[j]
        arr[j] = key
      }
    }
  }

  return arr
}