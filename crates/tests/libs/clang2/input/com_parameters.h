#define _COM_Outptr_ __attribute__((annotate("_COM_Outptr_")))
#define _In_opt_ __attribute__((annotate("_In_opt_")))

struct IUnknown;

extern "C" void Create(
    _COM_Outptr_ void** untyped,
    _COM_Outptr_ IUnknown** typed,
    _In_opt_ IUnknown* outer);

struct __declspec(uuid("00000001-0000-0000-c000-000000000046")) IFactory {
    virtual void __stdcall Create(
        _COM_Outptr_ void** untyped,
        _COM_Outptr_ IUnknown** typed,
        _In_opt_ IUnknown* outer) = 0;
};
