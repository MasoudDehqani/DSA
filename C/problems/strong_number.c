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

int factorial_of_digits(int num)
{
  int res = 0;
  while (num > 0)
  {
    res += factorial(num % 10);
    num /= 10;
  }

  return res;
}

int main()
{
  printf("Enter the number to check if it's strong:\n");

  int input_num;
  scanf("%d", &input_num);

  if (factorial_of_digits(input_num) == input_num)
  {
    printf("The number %d is a strong number\n", input_num);
  }
  else
  {
    printf("The number %d is not a strong number\n", input_num);
  }

  return 0;
}