//! reference alias_excluded_types.rdl
typedef void* PVOID;
typedef void* LPVOID;
typedef PVOID KnownPointer;
typedef LPVOID KnownLongPointer;
typedef unsigned long long ULONGLONG;
typedef ULONGLONG KnownScalar;
typedef void* CUSTOM;
typedef CUSTOM KnownCustom;
typedef void* AMBIGUOUS;
typedef AMBIGUOUS KnownFirst;
typedef AMBIGUOUS KnownSecond;
struct LocalRecord { int value; };
typedef LocalRecord KnownRecord;
struct Example {
    PVOID direct;
    PVOID* output;
    PVOID const* input;
    LPVOID const* long_input;
    PVOID const* const* nested;
    PVOID array[2];
    ULONGLONG scalar;
    CUSTOM custom;
    AMBIGUOUS ambiguous;
    LocalRecord record;
};
extern "C" PVOID const* Compare(PVOID direct, PVOID* output, PVOID const* input);
typedef void (*CALLBACK)(LPVOID const* input, LPVOID* output);
