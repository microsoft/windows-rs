#pragma once
#include "declaration_macros.h"

typedef struct _Record {
    int value;
} Record, *PRecord, **PPRecord, RecordArray[2], *PointerArray[3];

DECLARE_TAG(Handle);
WRAP_TAG(OtherHandle);
