#ifndef DIRECTION
#define DIRECTION "_In_"
#endif
#define CONTRACT __attribute__((annotate("windows_clang:sal:2:" DIRECTION "\n()\n()")))
typedef void (*Reader)(CONTRACT int* value);
typedef void (**Nested)(CONTRACT int* value);
struct UsesNested {
    Nested value;
};
