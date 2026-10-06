//! filter api.h
//! file types.h
struct NODE {
    NODE* next;
    VALUE value;
};
typedef NODE ARG;
//! file api.h
#include "types.h"
extern "C" void F(ARG* value);
//! input a.h
#define VALUE int
#include "api.h"
//! input b.h
#define VALUE float
#include "api.h"
