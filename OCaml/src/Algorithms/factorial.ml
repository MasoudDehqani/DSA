let rec factorial = function
  | 1 -> 1
  | 2 -> 2
  | n -> factorial (n - 1) * n
