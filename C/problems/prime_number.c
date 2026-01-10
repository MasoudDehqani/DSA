#include <stdio.h>

int custom_log2(int num)
{
  int curr = 1;
  int count = 0;

  while (curr < num)
  {
    curr *= 2;
    count++;
  }

  return count;
}

int is_prime(int num)
{
  int curr = 2;
  int threshold = custom_log2(num);
  for (int curr = 2; curr <= threshold; curr++)
  {
    if (num % curr == 0)
    {
      return 0;
    }
  }

  return 1;
}

int main()
{
  printf("Enter the number to check if it's prime:\n");

  int input_num;
  scanf("%d", &input_num);

  if (is_prime(input_num))
  {
    printf("The number %d is a prime number\n", input_num);
  }
  else
  {
    printf("The number %d is not a prime number\n", input_num);
  }

  return 0;
}