//! namespace Redeclared
//! library api.dll
//! file shared.h
extern "C" int Shared(int original);
//! input api.h
#include "shared.h"
extern "C" int Shared(int first);
extern "C" int Shared(int second);
