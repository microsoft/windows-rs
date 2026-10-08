#define VOLUME_PREFIX L"\\\\?\\Volume"
#define VOLUME_PREFIX_LEN (sizeof(VOLUME_PREFIX) / sizeof(VOLUME_PREFIX[0]) - 1)
#define NARROW "a\0b"
#define UTF8 u8"\u03A9\U0001F600"
#define WIDE L"\x1234" L"A\0z\U0001F600"
#define UTF16 u"\u03A9\U0001F600"
#define RAW R"(a\n"b)"
#define EMPTY L""
#define CONCAT L"a" "b" L"c"
#define ESCAPES "\a\b\t\n\v\f\r\\\"\000"
const wchar_t* Pointer __attribute__((annotate("_Literal_contract_"))) = L"pointer";
