//! filter api.h
//! file api.h
struct IFoo;
typedef IFoo Alias;
typedef Alias POINTERS Pointer;
struct Uses { Pointer value; IFoo* tag; };
extern "C" void Use(Uses* value);
//! input first.h
#define POINTERS *
#include "api.h"
//! input second.h
#define POINTERS **
struct IFoo { virtual void Method() = 0; };
#include "api.h"
