#include <stdio.h>

void print_floyd_triangle(int rows)
{
  int columns = 1;
  int start = 0;

  while (columns <= rows)
  {
    for (int i = 1; i <= columns; i++)
    {
      start++;
      printf("%d ", start);
    }

    columns++;
    printf("\n");
  }
}

int main()
{
  printf("Enter number rows you want to see from Floyd triangle:\n");
  int rows;

  scanf("%d", &rows);
  print_floyd_triangle(rows);
  return 0;
}