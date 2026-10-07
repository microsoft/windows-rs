#define IN_Z __attribute__((annotate("_In_z_")))
#define OUT_Z __attribute__((annotate("_Out_z_")))
typedef char* PSTR;
typedef const char* PCSTR;
typedef unsigned short* PWSTR;
typedef const unsigned short* PCWSTR;
extern "C" void Strings(IN_Z char* input, IN_Z const char* constant, OUT_Z char* output,
                       IN_Z unsigned short* wide, IN_Z const unsigned short* wide_constant);
typedef void (*CALLBACK)(IN_Z char* input, OUT_Z char* output);
struct IStrings {
    virtual void Strings(IN_Z char* input, OUT_Z char* output) = 0;
};
