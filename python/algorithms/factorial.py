def fac(n):
  if (n < 3):
    return n
  
  return fac(n - 1) * n
