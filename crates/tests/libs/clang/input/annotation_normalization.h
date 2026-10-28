#ifdef LEGACY
#define CONTRACT(current, legacy) __attribute__((annotate("windows_clang:sal:1:" legacy "\n()\n()")))
#else
#define CONTRACT(current, legacy) __attribute__((annotate("windows_clang:sal:2:" current "\n()\n()")))
#endif
#define READ CONTRACT("_In_", "__in")
#define OPTIONAL_READ CONTRACT("_In_opt_", "__in_opt")
#define WRITE CONTRACT("_Out_", "__out")
#define OPTIONAL_WRITE CONTRACT("_Out_opt_", "__out_opt")
#define UPDATE CONTRACT("_Inout_", "__inout")
#define OPTIONAL_UPDATE CONTRACT("_Inout_opt_", "__inout_opt")

#define PARAMETERS READ int* input, OPTIONAL_READ int* optional_input, \
    WRITE int* output, OPTIONAL_WRITE int* optional_output, \
    UPDATE int* both, OPTIONAL_UPDATE int* optional_both

typedef void (*Normalizer)(PARAMETERS);
extern "C" void NormalizeFunction(PARAMETERS);
struct __declspec(uuid("11111111-2222-3333-4444-555555555555")) INormalized {
    virtual void __stdcall Apply(PARAMETERS) = 0;
};
