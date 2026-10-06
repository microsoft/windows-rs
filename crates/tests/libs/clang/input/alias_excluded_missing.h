//! reference alias_excluded_types.rdl
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
    LPSTR buffers[2];
};
extern "C" LPCSTR Text(IN LPSTR input, OUT LPWSTR output);
typedef LPCWSTR (*CALLBACK)(IN LPSTR input, OUT LPWSTR output);
#define EMPTY_TEXT ((LPCSTR)0)
