//! namespace Iids
//! args -x c++ -fms-extensions
#define DEFINE_GUID(name, ...)
struct __declspec(uuid("12345678-1234-abcd-9876-0123456789ab")) ITest {
    virtual int Method() = 0;
};
DEFINE_GUID(IID_ITest, 0x12345678, 0x1234, 0xabcd, 0x98, 0x76, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab)
