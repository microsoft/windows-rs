//! reference alias_excluded_types.rdl
#define IN __attribute__((annotate("_In_")))
#define OUT __attribute__((annotate("_Out_")))
typedef char* PSTR;
typedef PSTR LPSTR;
typedef const char* LPCSTR;
typedef LPCSTR PCSTR;
typedef unsigned short* PWSTR;
typedef PWSTR LPWSTR;
typedef const unsigned short* PCWSTR;
typedef PCWSTR LPCWSTR;
typedef LPSTR KnownText;
typedef LPCSTR KnownConstText;
typedef LPWSTR KnownWide;
typedef LPCWSTR KnownConstWide;
struct Example {
    LPSTR* text;
    LPCSTR* constant;
    LPWSTR* wide;
    LPCWSTR* wide_constant;
};
extern "C" PCSTR Text(IN LPSTR input, OUT LPWSTR output);
