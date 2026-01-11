#include <stdio.h>
#include <math.h>

double custom_pow_iterative(double base, double exp)
{
  int is_exp_negative = exp < 0;
  exp = is_exp_negative ? -exp : exp;
  double res = 1.0;
  while (exp > 0)
  {
    res *= base;
    exp--;
  }

  return is_exp_negative ? 1.0 / res : res;
}

double custom_pow_recursive_aux(double base, double exp)
{
  if (exp < 2.0)
    return exp == 0.0 ? 1.0 : base;

  double half = custom_pow_recursive_aux(base, floor(exp / 2.0));
  double res = (int)exp % (int)2.0 == 0.0 ? half * half : half * half * base;

  return res;
}

double custom_pow_recursive(double base, double exp)
{
  return exp < 0.0 ? 1.0 / custom_pow_recursive_aux(base, -exp) : custom_pow_recursive_aux(base, exp);
}

int main()
{
  printf("Enter the base:\n");
  double base;
  scanf("%lf", &base);
  printf("Enter the exponent:\n");
  double exponent;
  scanf("%lf", &exponent);

  printf("%lf raised to the power of %lf is: %lf\n", base, exponent, custom_pow_iterative(base, exponent));
}