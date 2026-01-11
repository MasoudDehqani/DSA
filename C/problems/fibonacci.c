#include <stdio.h>

int fib_iterative(int term)
{
  if (term == 1)
    return 0;
  if (term == 2)
    return 1;

  int term_minus_one = 0;
  int term_minus_two = 1;
  int res = 0;

  while (term > 2)
  {
    res = term_minus_one + term_minus_two;
    term_minus_one = term_minus_two;
    term_minus_two = res;
    term--;
  }

  return res;
}

int fib_recursive(int term)
{
  if (term == 1)
    return 0;
  if (term == 2)
    return 1;

  return fib_rec(term - 1) + fib_rec(term - 2);
}

int main()
{
  printf("Enter the fibonacci term you want to calculate:\n");
  int input_term;
  scanf("%d", &input_term);

  printf("The %d of fibonacci series is: %d\n", input_term, fib_iterative(input_term));
}