// Integer #define constants exercising the token-based literal parser:
// hexadecimal, decimal, negation, type suffixes (U/UL/LL/ULL), and the C
// integer-constant typing rules under LLP64 (`long` = 32-bit). A hex/octal
// literal that overflows `int` takes the unsigned type (faithful to its C
// type), while a decimal literal never selects an unsigned type.
#define DEC 42
#define NEG -5
#define HEX 0x1F
#define HEX_UPPER 0XAB
#define PARENS ( 7 )
#define UINT 100U
#define ULONG 4000000000UL
#define LONGLONG 0x100000000LL
#define ULONGLONG 0xFFFFFFFFFFFFFFFFULL
#define HRESULT_STYLE 0x80004002L
#define HEX_OVERFLOW 0x80000000
#define HEX_FFFF 0xFFFFFFFF
#define DEC_OVERFLOW 2147483648
