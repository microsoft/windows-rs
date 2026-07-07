// Typed-cast #define constants exercising parse_named_cast.
//
// A cast to a fundamental scalar typedef (DWORD/LONG) collapses to the underlying
// primitive — the same fundamentals that collapse everywhere else — so the
// constant is a plain typed integer (`u32`/`i32`), never a dangling reference to a
// `DWORD`/`LONG` type that the metadata does not preserve. A cast to a real,
// non-collapsing named type (an enum here) keeps that type as the constant's type.
typedef long LONG;
typedef unsigned long DWORD;
enum COLOR_MODE { COLOR_MODE_A = 0 };
#define CAST_SINGLE (DWORD)0x80004005
#define CAST_DOUBLE ((DWORD)0x80004005)
#define CAST_NEG_SINGLE (LONG)-1
#define CAST_NEG_DOUBLE ((LONG)-2)
#define CAST_ENUM (COLOR_MODE)1
