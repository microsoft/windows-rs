#define CHOICE 1
const int Before = CHOICE;
#undef CHOICE
#if STALE
const int CHOICE = 2;
#else
#define CHOICE 2
#endif
struct __windows_clang_macro_0 { int value; };
#define OTHER 3
