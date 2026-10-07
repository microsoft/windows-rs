//! reference alias_excluded_types.rdl
//! reference alias_excluded_canonical.rdl
#define IN __attribute__((annotate("_In_")))
#define OUT __attribute__((annotate("_Out_")))
typedef char* LPSTR;
typedef const char* LPCSTR;
typedef unsigned short* LPWSTR;
typedef const unsigned short* LPCWSTR;
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
extern "C" LPCSTR Text(IN LPSTR input, OUT LPWSTR output);
