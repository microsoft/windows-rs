//! filter api.h
//! filter first.h
//! filter second.h
//! file api.h
struct IFoo;
typedef IFoo* PFoo;
typedef PFoo Pointer;
struct Right;
struct Left { Right* next; Pointer value; IFoo* tag; };
struct Right { Left* next; int value; };
extern "C" void Use(Left* value);
//! input first.h
#include "api.h"
//! input second.h
struct IFoo { virtual void Method() = 0; };
#include "api.h"
