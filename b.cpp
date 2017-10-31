#include "test.hpp"

#include <stdio.h>

A::~A() {
  printf("~A\n");
}
B::~B() {
  printf("~B\n");
}


B new_B() {
  return B();
}
