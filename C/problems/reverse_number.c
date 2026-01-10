#include <stdio.h>

int reverse(int n)
{
  int rev = 0;

  while (n > 0)
  {
    rev = (rev * 10) + (n % 10);
    n /= 10;
  }

  return rev;
}

int main()
{
  printf("Enter the number:\n");
  int num;
  scanf("%d", &num);
  int reversed = reverse(num);
  printf("Reverse of %d is: %d\n", num, reversed);
}