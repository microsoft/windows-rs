//! namespace AnnotationOrder
//! library api.dll
//! args -x c++ -fms-extensions
#define OUT_ATTRIBUTE __attribute__((annotate("_Out_")))
extern "C" void Use(OUT_ATTRIBUTE /* [iid_is] */ void **value);
typedef void (*Callback)(OUT_ATTRIBUTE /* [iid_is] */ void **value);
struct __declspec(uuid("12345678-1234-abcd-9876-0123456789ab")) ITest {
    virtual int Method(OUT_ATTRIBUTE /* [iid_is] */ void **value) = 0;
};
