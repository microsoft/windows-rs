//! filter api.h
//! file api.h
struct IFoo;
typedef IFoo* PFoo;
typedef PFoo Pointer;
struct Right;
struct Left { Right* next; Pointer value; IFoo* tag; };
struct Right { Left* next; VALUE value; };
extern "C" void Use(Left* value);
//! input first.h
#define VALUE int
#include "api.h"
//! input second.h
#define VALUE float
struct IFoo { virtual void Method() = 0; };
#include "api.h"
