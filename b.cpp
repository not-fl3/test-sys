#include <iostream>

using namespace std;

extern "C" void b() {
  cout << "IOSTREAM!" << endl;
}
