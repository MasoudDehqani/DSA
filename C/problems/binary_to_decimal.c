#include <stdio.h>

int custom_pow(int base, int exp)
{
  int res = 1;
  while (exp > 0)
  {
    res *= base;
    exp--;
  }

  return res;
}

int binary_to_decimal(int binary_number)
{
  int order = 0;
  int res = 0;

  while (binary_number > 0)
  {
    res += (binary_number % 10) * custom_pow(2, order);
    binary_number /= 10;
    order++;
  }

  return res;
}

int main()
{
  printf("Enter the binary number you want to convert to decimal:\n");

  int input_binary;
  scanf("%d", &input_binary);

  printf("The decimal of binary number %d is: %d\n", input_binary, binary_to_decimal(input_binary));
}