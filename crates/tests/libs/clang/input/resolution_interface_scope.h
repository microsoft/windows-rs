//! filter api.h
//! file types.h
#ifdef DEFINE_INTERFACE
struct IFoo { virtual void Method() = 0; };
typedef IFoo* Shared;
#else
struct Shared { int value; };
#endif
//! file api.h
#include "types.h"
#ifdef DEFINE_INTERFACE
extern "C" void InterfaceUse(IFoo* value);
#else
extern "C" void RecordUse(Shared value, Shared* pointer);
#endif
//! input first.h
#define DEFINE_INTERFACE
#include "api.h"
//! input second.h
#include "api.h"
