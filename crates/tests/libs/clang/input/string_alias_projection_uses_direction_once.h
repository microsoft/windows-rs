//! namespace StringAliases
//! library api.dll

#define IN __attribute__((annotate("_In_")))
             #define OUT __attribute__((annotate("_Out_")))
             typedef const char* PCSTR;
             typedef char* LPSTR;
             typedef char* PSTR;
             typedef const unsigned short* PCWSTR;
             typedef unsigned short* LPWSTR;
             typedef unsigned short* PWSTR;
             extern "C" void Strings(IN LPSTR legacy_input, OUT LPSTR legacy_output,                  IN PSTR explicit_input, IN LPWSTR wide_input, OUT PWSTR wide_output);
