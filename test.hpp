//#include "btScalar.h"

//ATTRIBUTE_ALIGNED16(class) A {
class A {
public:
  virtual ~A();
};

struct B : A {
public:
  B() {}
  ~B();
};




