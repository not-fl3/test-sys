#include<stdio.h>

#include "test.hpp"

int main() {
  B *b = new B();
  delete b;
  return 0;
}
