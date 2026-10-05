//! namespace Interfaces
//! args -x c++ --target=x86_64-pc-windows-msvc -fms-extensions
//! input first.h
struct IFoo;
struct UsesFoo { IFoo* value; };
//! input second.h
struct __declspec(uuid("00000000-0000-0000-c000-000000000046")) IFoo {
    virtual void Method() = 0;
};
struct __declspec(uuid("11111111-1111-1111-1111-111111111111")) IBar : IFoo {};
