//! filter api.h
//! file api.h
struct IFoo;
typedef IFoo* PFoo;
typedef PFoo Array[COUNT];
union Storage { Array array; void* padding[2]; };
struct Uses { Storage value; IFoo* tag; };
extern "C" void Use(Uses* value);
//! input first.h
#define COUNT 1
#include "api.h"
//! input second.h
#define COUNT 2
struct IFoo { virtual void Method() = 0; };
#include "api.h"
