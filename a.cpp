#include<stdio.h>

#include "test.hpp"

extern "C" void _ZN1BD0Ev(B *b);

int main() {
  B *b = new B();
  _ZN1BD0Ev(b);
  //delete b;
  return 0;
}
