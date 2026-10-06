//! filter api.h
//! file types.h
typedef VALUE ARG;
//! file api.h
#include "types.h"
extern "C" void F(ARG value);
//! input a.h
#define VALUE int
#include "api.h"
//! input b.h
#define VALUE double
#include "api.h"
