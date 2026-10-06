#define DECLSPEC_UUID(x) __declspec(uuid(x))
#define MIDL_INTERFACE(x) struct DECLSPEC_UUID(x)

MIDL_INTERFACE("00000000-0000-0000-c000-000000000046")
IBase
{
public:
    virtual long __stdcall Query(int value) = 0;
    virtual int __stdcall Count() = 0;
};

MIDL_INTERFACE("00000000-0000-0000-c000-000000000047")
IDerived : public IBase
{
public:
    virtual void __stdcall Notify(
        int __attribute__((annotate("_In_"))) value,
        int* __attribute__((annotate("_Out_"))) result) = 0;
};

extern "C" void Use(IDerived* value);
