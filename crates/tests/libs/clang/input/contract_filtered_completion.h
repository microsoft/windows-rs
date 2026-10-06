//! filter api.h
//! file types.h
#ifdef COMPLETE
struct IFoo { virtual void Method() = 0; };
#else
struct IFoo;
#endif
typedef IFoo Alias;
typedef Alias Alias2;
typedef Alias2* PFoo;
typedef PFoo Pointer;
typedef Pointer* Slot;
struct Packet {
    Pointer value;
    Slot slot;
    IFoo* tag;
};
//! file api.h
#include "types.h"
extern "C" Pointer Use(Alias2* tag, Alias2** slot, Pointer value, Pointer* output, Slot alias, Packet* packet);
//! input first.h
#include "api.h"
//! input second.h
#define COMPLETE
#include "api.h"
