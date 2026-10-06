//! filter api.h
//! file types.h
#ifdef COMPLETE
struct IFoo { virtual void Method() = 0; };
#else
struct IFoo;
#endif
typedef IFoo* PFoo;
typedef PFoo Pointer;
//! file api.h
#include "types.h"
#define POINTER_NULL ((Pointer)0)
#define INTEGER_CONSTANT 42
//! input first.h
#include "api.h"
//! input second.h
#define COMPLETE
#include "api.h"
