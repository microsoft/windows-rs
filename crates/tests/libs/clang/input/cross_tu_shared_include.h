//! namespace Stable
//! file shared.h
struct IShared { virtual void Method() = 0; };
//! input first.h
#include "shared.h"
extern "C" IShared* GetShared();
//! input second.h
#include "shared.h"
struct HOLDER { IShared* value; };
