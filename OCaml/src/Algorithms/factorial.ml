let rec factorial = function
  | n when n <= 0 ->
      invalid_arg "Factorial is only defined for positive integers"
  | 0
  | 1 ->
      1
  | n -> n * factorial (n - 1)
