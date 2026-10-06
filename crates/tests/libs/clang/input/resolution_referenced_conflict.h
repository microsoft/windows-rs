//! reference resolution_interfaces.rdl
//! filter api.h
//! file types.h
struct Data { VALUE value; };
//! file api.h
#include "types.h"
extern "C" void Referenced(Data* value);
//! input first.h
#define VALUE int
#include "api.h"
//! input second.h
#define VALUE double
#include "api.h"
