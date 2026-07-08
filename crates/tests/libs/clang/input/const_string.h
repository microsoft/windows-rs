// String #define constants exercising the narrow and wide string literal
// paths, including a parenthesized string literal. These emit a
// NativeEncoding attribute in the generated RDL.
#define NARROW "hello"
#define WIDE L"wide"
#define PARENS_STRING ( "paren" )

// Escape sequences must be decoded to their actual bytes, not stored verbatim.
#define ESC_NEWLINE "a\nb"
#define ESC_BACKSLASH "a\\b"
#define ESC_NUL "x\0y"
#define ESC_HEX_ASCII "\x41\x42"
#define ESC_OCTAL "\101"

// A raw byte array (bytes >= 0x80) has no faithful UTF-8 String form and is omitted.
#define ESC_BYTES "\xaa\xbb"
