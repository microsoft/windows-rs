// Integer #define constants exercising the token-based literal parser:
// hexadecimal, decimal, negation, type suffixes (U/UL/LL/ULL), and the
// LLP64 bit-pattern reinterpretation for large `L`-suffixed values.
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
