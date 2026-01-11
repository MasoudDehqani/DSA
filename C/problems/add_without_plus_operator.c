#include <stdio.h>

int add_with_inc_dec_operators(int x, int y)
{
  int is_y_negative = y < 0;
  while (y != 0)
  {
    is_y_negative ? y++ : y--;
    is_y_negative ? x-- : x++;
  }

  return x;
}

int half_adder(int x, int y)
{
  int sum, carry;
  while (y != 0)
  {
    sum = x ^ y;
    carry = (x & y) << 1;
    x = sum;
    y = carry;
  }

  return sum;
}

int main()
{
  printf("Enter two number you want to add: \n");
  int x, y;
  scanf("%d %d", &x, &y);

  // int res = add_with_inc_dec_operators(x, y);
  int res = half_adder(x, y);
  printf("%d + %d : %d\n", x, y, res);
}