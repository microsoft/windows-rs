#define SAL(text) __attribute__((annotate(text)))
typedef unsigned short WCHAR;
extern "C" void MultiString(
    SAL("_Post_") SAL("_NullNull_terminated_") WCHAR* value);
