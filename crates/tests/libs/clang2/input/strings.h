#define _In_z_ __attribute__((annotate("_In_z_")))
#define _In_opt_z_ __attribute__((annotate("_In_opt_z_")))
#define _Out_z_ __attribute__((annotate("_Out_z_")))
#define _Inout_z_ __attribute__((annotate("_Inout_z_")))

typedef const char* Narrow;
typedef Narrow Alias;
typedef const wchar_t* Wide;

extern "C" void Strings(
    _In_z_ Alias input,
    _In_z_ unsigned char* mutable_input,
    _Out_z_ char* output,
    _Inout_z_ unsigned short* update,
    _In_opt_z_ Wide optional,
    char* raw);

struct __declspec(uuid("00000004-0000-0000-c000-000000000046")) IStrings {
    virtual void __stdcall Strings(
        _In_z_ Alias input,
        _In_z_ unsigned char* mutable_input,
        _Out_z_ char* output,
        _Inout_z_ unsigned short* update,
        _In_opt_z_ Wide optional,
        char* raw) = 0;
};
