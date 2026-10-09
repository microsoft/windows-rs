#define SAL(text) __attribute__((annotate(text)))
typedef unsigned short WCHAR;

extern "C" void Bare(SAL("_NullNull_terminated_") const WCHAR* value);
extern "C" void Input(SAL("_Pre_") SAL("_NullNull_terminated_") const char* value);
extern "C" void Output(SAL("_Post_") SAL("_NullNull_terminated_") WCHAR* value);
extern "C" void Single(SAL("_Pre_") SAL("_Null_terminated_") const WCHAR* value);
extern "C" void Mixed(SAL("_In_z_") SAL("_Post_") SAL("_NullNull_terminated_") WCHAR* value);
extern "C" void Counted(SAL("_In_") SAL("_Pre_") SAL("_NullNull_terminated_")
    SAL("_In_reads_(count)") const WCHAR* value, unsigned count);
extern "C" void Optional(SAL("_In_opt_") SAL("_Pre_") SAL("_NullNull_terminated_")
    const WCHAR* value);
extern "C" void Ordinary(SAL("_In_z_") const WCHAR* value);
extern "C" void Combined(SAL("_In_z_") SAL("_Pre_") SAL("_NullNull_terminated_")
    const WCHAR* value);
