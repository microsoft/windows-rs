#define SAL(name, args) __attribute__((annotate( \
    "windows_clang:sal:2:" name "\n(" args ")\n(" args ")")))

#if CONTRACT == 0
#define ATTRIBUTES
#elif CONTRACT == 1
#define ATTRIBUTES SAL("_Return_type_success_", "return >= 0")
#elif CONTRACT == 2
#define ATTRIBUTES SAL("_Must_inspect_result_", "")
#elif CONTRACT == 3
#define ATTRIBUTES SAL("_Return_type_success_", "return >= 0") SAL("_Must_inspect_result_", "")
#else
#define ATTRIBUTES SAL("_Return_type_success_", "return > 0")
#endif

typedef ATTRIBUTES int Status;
typedef SAL("_Return_type_success_", "return >= 0") void AnnotatedVoid;
struct UsesVoid {
    AnnotatedVoid* value;
};
