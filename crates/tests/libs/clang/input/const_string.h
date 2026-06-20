// String #define constants exercising the narrow and wide string literal
// paths, including a parenthesized string literal. These emit a
// NativeEncoding attribute in the generated RDL.
#define NARROW "hello"
#define WIDE L"wide"
#define PARENS_STRING ( "paren" )
