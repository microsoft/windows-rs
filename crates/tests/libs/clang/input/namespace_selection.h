#include "namespace_partial.h"
namespace Selected {
    struct Root { Right::Item* value; };
    struct __declspec(uuid("12345678-1234-1234-1234-123456789abc")) IRoot {
        virtual void __stdcall Use(Root* value) = 0;
    };
    extern "C" void __stdcall Flat(Right::Dependent* value);
    static inline int Helper() { return 0; }
    enum Mode { First, Second };
}
