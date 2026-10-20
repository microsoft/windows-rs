#define IN __attribute__((annotate("windows_clang:sal:2:_In_\n()\n()")))
#define OUT __attribute__((annotate("windows_clang:sal:2:_Out_\n()\n()")))
typedef const char* PCSTR;
typedef char* LPSTR;
typedef char* PSTR;
typedef const unsigned short* PCWSTR;
typedef unsigned short* LPWSTR;
typedef unsigned short* PWSTR;
extern "C" void Strings(
    IN LPSTR legacy_input,
    OUT LPSTR legacy_output,
    IN PSTR explicit_input,
    IN LPWSTR wide_input,
    OUT PWSTR wide_output);
typedef void (*CALLBACK)(IN LPSTR input, OUT LPWSTR output);
