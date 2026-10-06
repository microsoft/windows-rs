//! filter aliases.h
//! filter first.h
//! filter second.h
//! file aliases.h
struct IFoo;
typedef IFoo Alias1;
typedef Alias1 Alias2;
typedef Alias2 Alias3;
typedef Alias3* PFoo;
typedef PFoo PFoo2;
typedef PFoo2* PPFoo;
//! input first.h
#include "aliases.h"
struct Uses {
    IFoo* tag;
    Alias3* value;
    PFoo2 pointer;
    PFoo2* slot;
    Alias3* array[2];
};
extern "C" Alias3* Forward(Alias3* value, PFoo2 pointer, PFoo2* slot);
//! input second.h
struct IFoo { virtual void Method() = 0; };
#include "aliases.h"
extern "C" Alias3* Complete(Alias3* value, PFoo2 pointer, PFoo2* slot);
