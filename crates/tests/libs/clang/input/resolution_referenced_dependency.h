//! reference resolution_interfaces.rdl
//! filter api.h
//! file api.h
struct IFoo { IFoo(); virtual void Method() = 0; };
extern "C" void Referenced(IFoo* value);
//! input first.h
#include "api.h"
//! input second.h
#include "api.h"
