#include "test.hpp"

#include <stdio.h>


foo::foo() {
    printf("foo foo foo foo foo foo\n");
}

foo::foo(bar *bar) {
    printf("foo bar foo bar foo bar\n");
}

void foo::hihihi() {
    printf("WTF WTF WTF\n");
}
