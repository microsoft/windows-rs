//! namespace PropertyKeys
//! no-library

struct PROPERTYKEY { int value; };
             struct DEVPROPKEY { int value; };
             #define DEFINE_PROPERTYKEY(name, ...)
             #define DEFINE_DEVPROPKEY(name, ...)
             DEFINE_PROPERTYKEY(PKEY_Test, 0x12345678, 0x1234, 0xabcd, 0x98, 0x76,                  0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 4)
             DEFINE_DEVPROPKEY(DEVPKEY_Test, 0x87654321, 0xabcd, 0x1234, 0x01, 0x23,                  0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 7)
