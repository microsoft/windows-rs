//! filter api.h
//! file api.h
struct IFoo;
typedef IFoo* PFoo;
typedef PFoo Pointer;
#pragma pack(push, PACKING)
struct Inner { char prefix; Pointer value; };
#pragma pack(pop)
struct Uses { Inner inner; IFoo* tag; };
extern "C" void Use(Uses* value);
//! input first.h
#define PACKING 1
#include "api.h"
//! input second.h
#define PACKING 8
struct IFoo { virtual void Method() = 0; };
#include "api.h"
