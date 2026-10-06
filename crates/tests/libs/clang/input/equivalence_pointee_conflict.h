//! filter api.h
//! file api.h
struct IFoo;
typedef VALUE Scalar;
typedef Scalar* Pointer;
struct Inner { Pointer value; };
struct Uses { Inner inner; IFoo* tag; };
extern "C" void Use(Uses* value);
//! input first.h
#define VALUE int
#include "api.h"
//! input second.h
#define VALUE float
struct IFoo { virtual void Method() = 0; };
#include "api.h"
