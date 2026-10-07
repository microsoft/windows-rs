#define _In_opt_ __attribute__((annotate("_In_opt_")))
#define _Out_opt_ __attribute__((annotate("_Out_opt_")))
#define _Inout_opt_ __attribute__((annotate("_Inout_opt_")))

extern "C" void Use(_In_opt_ const int* input, _Out_opt_ int* output, _Inout_opt_ int* update);

struct __declspec(uuid("00000003-0000-0000-c000-000000000046")) IOptional {
    virtual void __stdcall Use(
        _In_opt_ const int* input, _Out_opt_ int* output, _Inout_opt_ int* update) = 0;
};
