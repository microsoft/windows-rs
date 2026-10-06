//! reference contract_reference_pointer.rdl
//! filter api.h
//! filter types.h
//! file types.h
#ifdef COMPLETE
struct Base { virtual void Method() = 0; };
#else
struct Base;
#endif
typedef Base ObjectAlias;
typedef ObjectAlias* Handle;
typedef Handle Copy;
typedef Copy* Slot;
//! file api.h
#include "types.h"
extern "C" Handle Use(Handle value, Copy copy, Handle* output, Slot slot);
#define HANDLE_NULL ((Handle)0)
#define COPY_NULL ((Copy)0)
#define INTEGER_CONSTANT 42
//! input first.h
#include "api.h"
//! input second.h
#define COMPLETE
#include "api.h"
