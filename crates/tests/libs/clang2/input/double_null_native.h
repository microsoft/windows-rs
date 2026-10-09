#ifdef __clang__
#define SAL(text) __attribute__((annotate(text)))
#else
#define SAL(text)
#endif
typedef unsigned short WCHAR;

extern "C" unsigned MultiLength(
    SAL("_In_") SAL("_Pre_") SAL("_NullNull_terminated_") const WCHAR* value);
extern "C" void MakeMulti(
    SAL("_Out_writes_(6)") SAL("_Post_") SAL("_NullNull_terminated_") WCHAR* value);
