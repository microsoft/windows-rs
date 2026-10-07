//! namespace AnnotationOrder
//! library api.dll
//! args -x c++ -fms-extensions
#define OUT
extern "C" void First(/* [iid_is] */ OUT void **value);
extern "C" void Second(OUT /* [iid_is] */ void **value);
typedef void (*FirstCallback)(/* [iid_is] */ OUT void **value);
typedef void (*SecondCallback)(OUT /* [iid_is] */ void **value);
struct __declspec(uuid("12345678-1234-abcd-9876-0123456789ab")) ITest {
    virtual int First(/* [iid_is] */ OUT void **value) = 0;
    virtual int Second(OUT /* [iid_is] */ void **value) = 0;
};
