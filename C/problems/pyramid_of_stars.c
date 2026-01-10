#include <stdio.h>

int main()
{
  int rows;
  printf("Enter number of rows:\n");
  scanf("%d", &rows);

  for (int r = 0; r < rows; r++)
  {
    for (int c = 0; c <= (rows * 2) - 1; c++)
    {
      if (c >= rows - r && c <= rows + r)
      {
        printf("*");
      }
      else
      {
        printf(" ");
      }
    }

    printf("\n");
  }

  return 0;
}