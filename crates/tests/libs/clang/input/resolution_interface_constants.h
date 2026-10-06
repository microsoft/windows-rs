//! filter iface.h
//! filter second.h
//! file iface.h
struct IUnknown { virtual void Method() = 0; };
//! file aliases.h
#include "iface.h"
typedef IUnknown* LPUNKNOWN;
typedef LPUNKNOWN UNKNOWNPTR;
typedef UNKNOWNPTR UNKNOWNPTR2;
//! input first.h
#include "iface.h"
//! input second.h
#include "aliases.h"
#define UNKNOWN_NULL ((LPUNKNOWN)0)
#define UNKNOWN_ALIAS ((UNKNOWNPTR2)-1)
#define INTEGER_CONSTANT 42
