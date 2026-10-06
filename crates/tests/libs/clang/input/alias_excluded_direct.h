//! reference alias_excluded_types.rdl
//! reference alias_excluded_canonical.rdl
//! reference alias_excluded_direct.rdl
typedef char* LPSTR;
typedef const char* LPCSTR;
typedef void* PVOID;
typedef LPSTR KnownText;
typedef LPCSTR KnownConstText;
typedef PVOID KnownPointer;
struct Example {
    LPSTR* text;
    LPCSTR* constant;
    PVOID direct;
    PVOID* output;
    PVOID const* input;
};
