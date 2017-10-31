//#include "btScalar.h"

//ATTRIBUTE_ALIGNED16(class) A {
struct A {
    virtual ~A() = 0;
};

struct B : A {
    B() {}
    ~B();
};
