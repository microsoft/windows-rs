#pragma once
#include "included.h"
#include "other\source.h"

struct NativeRecord;
typedef NativeRecord RecordAlias;
struct NativeRecord {
    int value;
    struct Nested {
        unsigned int bits;
    } nested;
};

int Echo(int value);
float Echo(float value);
int Repeated(int value);
int Repeated(int value);

struct NativeInterface {
    virtual int Echo(int value) = 0;
};

enum NativeEnum { First = 1, Second = 2 };
typedef void (*Callback)(RecordAlias* value);

#define VALUE 1
#undef VALUE
#define VALUE 2
#define HELPER(value) ((value) + 1)
#define DECLARE(name) int name(int value);
DECLARE(FromMacro)
