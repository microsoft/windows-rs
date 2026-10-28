#include "macro_discovery.h"
#define GOOD1 1
#define BAD_TYPE unsigned int
#define BAD_TOKENS ) ]
#define EMPTY
#define ERASED 1
#undef ERASED
const int ERASED = 7;
#define REPLACED 1
#undef REPLACED
const int REPLACED = 9;
#define REPLACED(value) (value)
#if defined(_M_IX86)
#define ARCHFAIL unsigned int
#else
#define ARCHFAIL 12
#endif
#define GOOD2 (NUMBER + 1)
