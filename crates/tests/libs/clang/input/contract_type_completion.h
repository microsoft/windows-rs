//! filter api.h
//! file types.h
#ifdef COMPLETE
struct IFoo { virtual void Method() = 0; };
#else
struct IFoo;
#endif
typedef IFoo* PFoo;
//! file api.h
#include "types.h"
struct Packet {
    PFoo value;
    PFoo* slot;
};
//! input first.h
#include "api.h"
//! input second.h
#define COMPLETE
#include "api.h"
