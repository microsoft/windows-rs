// Regression boundary for windows-rs#5042, using the configured SAL transport.
#define SAL(text) __attribute__((annotate("windows_clang:sal:2:" text "\n()\n()")))
typedef unsigned short WCHAR;
extern "C" void MultiString(SAL("_Post_") SAL("_NullNull_terminated_") WCHAR* value);
