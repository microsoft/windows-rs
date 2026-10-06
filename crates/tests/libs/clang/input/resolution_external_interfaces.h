//! reference resolution_interfaces.rdl
//! filter aliases.h
//! filter first.h
//! filter second.h
//! file aliases.h
struct _IFoo;
typedef _IFoo IFoo;
typedef IFoo Alias1;
typedef Alias1 Alias2;
typedef Alias2 Alias3;
typedef Alias3* PFoo;
typedef PFoo PFoo2;
typedef PFoo2* PPFoo;
struct Uses {
    _IFoo* tag;
    Alias3* value;
    PFoo2 pointer;
    PFoo2* slot;
    Alias3* array[2];
};
//! input first.h
#include "aliases.h"
extern "C" Alias3* Forward(Alias3* value, PFoo2 pointer, PFoo2* slot);
extern "C" void Tags(_IFoo* tag, IFoo* alias, _IFoo** slot);
//! input second.h
#include "aliases.h"
extern "C" Alias3* Complete(Alias3* value, PFoo2 pointer, PFoo2* slot);
