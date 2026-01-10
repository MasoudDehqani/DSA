#include <stdio.h>

int reverse_number(int n)
{
  int rev = 0;

  while (n > 0)
  {
    rev = (rev * 10) + (n % 10);
    n /= 10;
  }

  return rev;
}

int is_palindrome_number()
{
  printf("Enter the number to check if it's palindrome: ");
  int num;
  scanf("%d", &num);

  return reverse_number(num) == num;
}

int main()
{
  printf("%d\n", is_palindrome_number());
}