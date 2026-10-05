//! namespace MethodAlias
//! args -x c++ -fms-extensions
#define PublicMethod ExpandedMethod
#define DECLARE_METHOD(name) virtual void name() = 0
struct __declspec(uuid("12345678-1234-abcd-9876-0123456789ab")) ITest {
    DECLARE_METHOD(PublicMethod);
};
