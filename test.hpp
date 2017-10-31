//#include "btScalar.h"

//ATTRIBUTE_ALIGNED16(class) A {
class A {
public:
  virtual ~A() = 0;
};

struct B : A {
public:
  B() {}
  ~B();
};




