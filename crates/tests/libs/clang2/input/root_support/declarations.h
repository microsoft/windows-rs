#include "other.h"
const char InterfaceName_Public[] = "Public";
#ifdef CONFLICT
typedef float SupportWord;
#else
typedef unsigned long SupportWord;
#endif
struct SupportRecord {
    SupportWord value;
};
struct UnusedSupport {
    int value;
};
template <typename T> class UnusedTemplate {};
extern "C" int SharedFunction(SupportRecord* value);
#define SUPPORT_CONSTANT 17
