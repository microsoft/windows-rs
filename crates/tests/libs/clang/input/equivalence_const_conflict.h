//! filter api.h
//! file api.h
struct IFoo;
typedef IFoo Alias;
typedef QUALIFIER Alias* Pointer;
struct Uses { Pointer value; IFoo* tag; };
extern "C" void Use(Uses* value);
//! input first.h
#define QUALIFIER const
#include "api.h"
//! input second.h
#define QUALIFIER
struct IFoo { virtual void Method() = 0; };
#include "api.h"
