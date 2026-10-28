// Regression for windows-rs#4943: support declarations are dependencies, not automatic roots.
#include "root_support\declarations.h"
struct PublicRoot {
    SupportRecord* value;
};
extern "C" int SharedFunction(SupportRecord* value);
#ifdef REJECTED_ROOT
struct RejectedRoot {
    UnusedSupport* value;
    const int unsupported;
};
#endif
