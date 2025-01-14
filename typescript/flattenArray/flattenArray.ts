export default function flattenArray(arr: Array<any>) {
  return arr.reduce(
    (acc, curr) =>
      Array.isArray(curr) ? acc.concat(flattenArray(curr)) : acc.concat([curr]),
    []
  );
}
