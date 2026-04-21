#include "typedef_in_struct_dep.h"

// Structs that reference typedefs defined in an included (non-main) header.
// Before the fix these typedefs were emitted as undefined names in the RDL.
struct TypedefInStruct {
    MY_BYTE value;
    MY_WORD count;
};
