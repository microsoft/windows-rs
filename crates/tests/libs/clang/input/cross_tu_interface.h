//! namespace Interfaces
//! args -x c++ --target=x86_64-pc-windows-msvc -fms-extensions
//! input first.h
struct IFoo;
typedef IFoo* PFoo;
struct UsesFoo {
    IFoo* value;
    PFoo alias;
    IFoo** slot;
    IFoo* array[2];
};
extern "C" IFoo* FromForward(IFoo* value, IFoo** output);
typedef IFoo* (*FooCallback)(IFoo* value);
struct IUseForward { virtual IFoo* Use(IFoo* value) = 0; };
//! input second.h
struct __declspec(uuid("00000000-0000-0000-c000-000000000046")) IFoo {
    virtual void Method() = 0;
};
struct __declspec(uuid("11111111-1111-1111-1111-111111111111")) IBar : IFoo {};
struct UsesComplete { IFoo* value; };
extern "C" IFoo* FromComplete(IFoo* value, IFoo** output);
