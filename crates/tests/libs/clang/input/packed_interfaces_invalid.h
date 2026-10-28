#include "packed_interfaces.h"

struct __declspec(uuid("1b20803b-05c1-4100-b61b-b12917e6c101")) IData : IPacked {
    int data;
};

struct __declspec(uuid("1b20803b-05c1-4100-b61b-b12917e6c102"))
IMultiple : IPacked, IDerived {};

struct __declspec(uuid("1b20803b-05c1-4100-b61b-b12917e6c103"))
__declspec(align(16)) IAligned : IPacked {};
