#include <stdio.h>

int main()
{
  int rows;
  scanf("%d", &rows);

  for (int r = 0; r < rows; r++)
  {
    for (int c = 1; c < rows * 2; c++)
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