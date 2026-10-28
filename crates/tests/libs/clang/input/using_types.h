#pragma once
#ifdef TYPEDEF_SYNTAX
typedef unsigned long Number;
typedef Number* NumberPointer;
typedef struct _UsingRecord { Number value; } UsingRecord;
#else
using Number = unsigned long;
using NumberPointer = Number*;
struct _UsingRecord { Number value; };
using UsingRecord = _UsingRecord;
#endif
