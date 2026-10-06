//! filter api.h
//! file api.h
struct IFoo;
typedef IFoo Alias1;
typedef Alias1 Alias2;
typedef Alias2 Alias3;
typedef Alias3* PFoo;
typedef PFoo PFoo2;
typedef PFoo2 (*Callback)(PFoo2 value, PFoo2* slot);
struct IUse { virtual PFoo2 Call(PFoo2 value) = 0; };
struct Inner {
    IFoo* tag;
    PFoo2 value;
    PFoo2* slot;
    PFoo2 array[2];
};
struct Uses {
    Uses* next;
    Inner inner;
    IFoo* tag;
    Callback callback;
    IUse* listener;
};
extern "C" void Use(Uses* value);
//! input first.h
#include "api.h"
//! input second.h
struct IFoo { virtual void Method() = 0; };
#include "api.h"
