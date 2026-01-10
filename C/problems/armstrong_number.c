#include <stdio.h>

int custom_pow(int base, int exp)
{
  int res = 1;
  while (exp > 0)
  {
    res *= base;
    exp -= 1;
  }

  return res;
}

int number_of_digits(int num)
{
  int res = 0;

  while (num > 0)
  {
    res += 1;
    num /= 10;
  }

  return res;
}

int to_order_pow(int num)
{
  int order = number_of_digits(num);
  int sum = 0;

  while (num > 0)
  {
    sum += custom_pow(num % 10, order);
    num /= 10;
  }

  return sum;
}

int main()
{
  printf("Enter the number to check if it's armstrong:\n");

  int input_num;
  scanf("%d", &input_num);

  if (input_num == to_order_pow(input_num))
  {
    printf("The number %d is an armstrong number", input_num);
  }
  else
  {
    printf("The number %d is not an armstrong number", input_num);
  }

  return 0;
}