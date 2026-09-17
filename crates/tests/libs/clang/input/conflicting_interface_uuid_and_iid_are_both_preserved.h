//! namespace Iids
//! no-library
//! args -x c++ -fms-extensions

#define DEFINE_GUID(name, ...)
             struct __declspec(uuid("12345678-1234-abcd-9876-0123456789ab")) ITest {
                 virtual int Method() = 0;
             };
             DEFINE_GUID(IID_ITest, 0x87654321, 0x4321, 0xdcba, 0x67, 0x89, 0xfe,                  0xdc, 0xba, 0x98, 0x76, 0x54)
