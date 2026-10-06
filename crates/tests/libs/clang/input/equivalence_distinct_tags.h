//! filter api.h
//! file api.h
struct TAG;
typedef TAG* POINTER;
struct Uses { POINTER value; };
extern "C" void Use(Uses* value);
//! input first.h
#define TAG First
#include "api.h"
//! input second.h
#define TAG Second
#include "api.h"
