//! namespace InterfaceBase
//! no-library
//! args -x c++ --target=x86_64-pc-windows-msvc

struct __declspec(uuid("00000000-0000-0000-C000-000000000046")) IFACE {
                 virtual int Query() = 0;
             };
             typedef struct RESULT : IFACE { void* value; } RESULT;
