// DEFINE_GUID / DEFINE_OLEGUID named GUID constants (parse_define_guid_tokens).
//
// Without INITGUID the SDK's DEFINE_GUID expands to a valueless
// `extern "C" const GUID name` declaration, so the faithful value lives only in
// the macro arguments. The scraper reads those from the DEFINE_GUID macro
// expansion and emits a `const NAME: GUID = 0x...;`. DEFINE_OLEGUID is the
// shorthand whose trailing eight bytes are the fixed OLE sequence
// `{ 0xC0, 0, 0, 0, 0, 0, 0, 0x46 }`.
typedef struct _GUID {
    unsigned long Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
} GUID;

#define DEFINE_GUID(name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8) \
    extern "C" const GUID name
#define DEFINE_OLEGUID(name, l, w1, w2) \
    DEFINE_GUID(name, l, w1, w2, 0xC0, 0, 0, 0, 0, 0, 0, 0x46)

// All-zero GUID.
DEFINE_GUID(GUID_NULL, 0x00000000, 0x0000, 0x0000, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);

// A fully-specified GUID.
DEFINE_GUID(CLSID_Sample, 0x12345678, 0x1234, 0x5678, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44);

// The OLE shorthand: only data1/data2/data3 are spelled; the trailing bytes are
// filled with the canonical OLE sequence.
DEFINE_OLEGUID(IID_ISample, 0x00020400, 0x0000, 0x0000);
