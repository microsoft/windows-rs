#define IN_Z __attribute__((annotate("_In_z_")))
#define OUT_Z __attribute__((annotate("_Out_z_")))
#define INOUT_Z __attribute__((annotate("_Inout_z_")))
extern "C" void Strings(
    IN_Z char* input,
    IN_Z const char* constant,
    OUT_Z char* output,
    INOUT_Z char* both,
    IN_Z unsigned short* wide_input,
    IN_Z const unsigned short* wide_constant,
    OUT_Z unsigned short* wide_output);
typedef void (*CALLBACK)(IN_Z char* input, OUT_Z char* output);
struct IStrings {
    virtual void Strings(IN_Z char* input, OUT_Z char* output) = 0;
};
