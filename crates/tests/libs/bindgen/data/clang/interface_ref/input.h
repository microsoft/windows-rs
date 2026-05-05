// IUnknown is in Windows.Win32.System.Com in the reference metadata.
// Declaring a local stub so IMyInterface can inherit from it; the definition
// is suppressed because IUnknown already exists in the reference.
struct __declspec(uuid("00000000-0000-0000-c000-000000000046")) IUnknown {
    virtual int __stdcall QueryInterface(void* riid, void** ppvObject) = 0;
    virtual unsigned int __stdcall AddRef() = 0;
    virtual unsigned int __stdcall Release() = 0;
};

class __declspec(uuid("cafecafe-1234-5678-9abc-def012345678"))
IMyInterface : public IUnknown {
public:
    virtual int __stdcall Greet(int x) = 0;
};
