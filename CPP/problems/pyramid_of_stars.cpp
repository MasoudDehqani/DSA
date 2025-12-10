#include <iostream>

int main()
{
  int rows;
  std::cin >> rows;

  for (int r = 0; r < rows; r++)
  {
    for (int c = 1; c < rows * 2; c++)
    {
      if (c >= rows - r && c <= rows + r)
      {
        std::cout << "*";
      }
      else
      {
        std::cout << " ";
      }
    }

    std::cout << std::endl;
  }

  return 0;
}