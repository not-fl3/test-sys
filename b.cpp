#include "test.hpp"

#include <stdio.h>

A::~A() {
  printf("~A\n");
}
B::~B() {
  printf("~B\n");
}

A new_A() {
  return A();
}

B new_B() {
  return B();
}
