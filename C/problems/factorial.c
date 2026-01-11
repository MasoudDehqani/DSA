#include <stdio.h>

int factorial_iterative(int num)
{
  int res = num;

  while (num > 1)
  {
    int prev = num - 1;
    res *= prev;
    num = prev;
  }

  return res;
}

int factorial_recursive(int num)
{
  if (num <= 2)
    return num;

  return num * factorial_recursive(num - 1);
}

int main()
{
  printf("Enter the number you want its factorial:\n");
  int input_num;

  scanf("%d", &input_num);
  int fac = factorial_recursive(input_num);
  printf("Factorial of %d is: %d\n", input_num, fac);
}