#include <stdio.h>

int factorial(int num)
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

int main()
{
  printf("Enter the number you want its factorial:\n");
  int input_num;

  scanf("%d", &input_num);
  int fac = factorial(input_num);
  printf("%d\n", fac);
}