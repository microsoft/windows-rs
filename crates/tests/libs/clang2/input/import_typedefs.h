typedef unsigned long Count;
typedef Count Total;
typedef int* Buffer;
typedef const Count* Counts;
typedef int Row[2];
struct Record { int value; };
typedef Record RecordAlias;
enum Mode { First, Second };
typedef Mode ModeAlias;
typedef int (*Callback)(int value);
typedef int (*OtherCallback)(int value);

#ifndef CALLBACK_TYPE
#define CALLBACK_TYPE Callback
#endif

#if VARIANT == 0
extern "C" Total Convert(Buffer storage, Counts counts, Row* rows,
    RecordAlias* record, ModeAlias mode, CALLBACK_TYPE callback);
#else
extern "C" unsigned long Convert(int*, const unsigned long*, int (*)[2],
    Record*, Mode, CALLBACK_TYPE);
#endif
