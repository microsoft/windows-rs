// Typed-cast #define constants exercising parse_named_cast: single- and
// double-parenthesized casts and the negated cast form. The cast target type
// is a local typedef so the generated RDL references a real type.
typedef long LONG;
typedef unsigned long DWORD;
#define CAST_SINGLE (DWORD)0x80004005
#define CAST_DOUBLE ((DWORD)0x80004005)
#define CAST_NEG_SINGLE (LONG)-1
#define CAST_NEG_DOUBLE ((LONG)-2)
