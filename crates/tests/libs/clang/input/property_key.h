//! flat
// A `PROPERTYKEY`/`DEVPROPKEY` constant is defined via the SDK `DEFINE_PROPERTYKEY`/
// `DEFINE_DEVPROPKEY` macros, which expand to `{ { fmtid }, pid }`. The scraper matches
// the macro expansion (like `DEFINE_GUID`) and emits a `#[guid(fmtid)] const NAME: TYPE = pid;`
// carrying the `fmtid` on a `#[guid]` attribute and the `pid` as the value.
typedef struct _GUID {
    unsigned long Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
} GUID;

typedef struct _tagpropertykey {
    GUID fmtid;
    unsigned long pid;
} PROPERTYKEY;

typedef struct _DEVPROPKEY {
    GUID fmtid;
    unsigned long pid;
} DEVPROPKEY;

#define EXTERN_C extern "C"
#define DECLSPEC_SELECTANY __declspec(selectany)

#define DEFINE_PROPERTYKEY(name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8, pid) EXTERN_C const PROPERTYKEY DECLSPEC_SELECTANY name = { { l, w1, w2, { b1, b2, b3, b4, b5, b6, b7, b8 } }, pid }

#define DEFINE_DEVPROPKEY(name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8, pid) EXTERN_C const DEVPROPKEY DECLSPEC_SELECTANY name = { { l, w1, w2, { b1, b2, b3, b4, b5, b6, b7, b8 } }, pid }

DEFINE_PROPERTYKEY(PKEY_Test_Sample, 0x540b947e, 0x8b40, 0x45bc, 0xa8, 0xa2, 0x6a, 0x0b, 0x89, 0x4c, 0xbd, 0xa2, 100);
DEFINE_DEVPROPKEY(DEVPKEY_Test_Sample, 0x26e3e2a2, 0x1234, 0x5678, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 26);
