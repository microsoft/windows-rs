//! filter api.h
//! file api.h
struct IFoo;
typedef IFoo Alias;
typedef Alias INDIRECTION Argument;
extern "C" void Use(Argument value);
//! input first.h
#define INDIRECTION &
#include "api.h"
//! input second.h
#define INDIRECTION *
struct IFoo { virtual void Method() = 0; };
#include "api.h"
