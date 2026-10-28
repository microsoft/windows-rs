struct GUID {
    unsigned int Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
};

struct __declspec(uuid("00000000-0000-0000-C000-000000000046")) IUnknown {
    virtual long __stdcall QueryInterface(const GUID& iid, void** result) = 0;
    virtual unsigned long __stdcall AddRef() = 0;
    virtual unsigned long __stdcall Release() = 0;
};

struct Primary {
    virtual unsigned int __stdcall Read(unsigned int value) = 0;
};

struct Combined : Primary, IUnknown {};

#pragma pack(push, 1)
struct PackedCombined : Primary, IUnknown {};
#pragma pack(pop)

extern "C" Combined* __stdcall NativeObject();
extern "C" unsigned int __stdcall ReferenceCount();

#if NEGATIVE_BASES == 1
struct WithData : Primary, IUnknown { int value; };
#elif NEGATIVE_BASES == 2
struct WithMethod : Primary, IUnknown {
    virtual void __stdcall Extra() = 0;
};
#elif NEGATIVE_BASES == 3
struct PrivateBase : Primary, private IUnknown {};
#elif NEGATIVE_BASES == 4
struct VirtualBase : Primary, virtual IUnknown {};
#elif NEGATIVE_BASES == 5
struct ContainsObject { Combined value; };
#elif NEGATIVE_BASES == 6
extern "C" void __stdcall ByValue(Combined value);
#elif NEGATIVE_BASES == 7
struct Further : Combined {};
#endif
