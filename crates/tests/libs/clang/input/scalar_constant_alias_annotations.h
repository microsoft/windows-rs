#define SAL(name, args) __attribute__((annotate( \
    "windows_clang:sal:2:" name "\n(" args ")\n(" args ")")))
typedef SAL("_Return_type_success_", "return >= 0") unsigned long ULongA;
