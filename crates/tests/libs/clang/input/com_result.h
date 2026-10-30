#define SAL(name, args) __attribute__((annotate("windows_clang:sal:2:" name "\n(" args ")\n(" args ")")))
#define COM_OUT SAL("_COM_Outptr_", "")

typedef SAL("_Return_type_success_", "return >= 0") long Status;
typedef SAL("_Return_type_success_", "return == 0") long Code;

struct __declspec(uuid("11111111-1111-1111-1111-111111111111")) IObject {
    virtual void __stdcall Run() = 0;
};

struct __declspec(uuid("22222222-2222-2222-2222-222222222222")) IFactory {
    virtual Status __stdcall Create(int flags, COM_OUT IObject** result) = 0;
    virtual Status __stdcall Optional(SAL("_COM_Outptr_opt_", "") IObject** result) = 0;
    virtual Status __stdcall MaybeNull(SAL("_COM_Outptr_result_maybenull_", "") IObject** result) = 0;
    virtual Status __stdcall NotCom(SAL("_Outptr_result_nullonfailure_", "") IObject** result) = 0;
    virtual Status __stdcall NotLast(COM_OUT IObject** result, int flags) = 0;
    virtual Status __stdcall Untyped(COM_OUT void** result) = 0;
    virtual SAL("_Success_", "return == 0") Status __stdcall Exact(COM_OUT IObject** result) = 0;
    virtual Code __stdcall Zero(COM_OUT IObject** result) = 0;
    virtual long __stdcall Unspecified(COM_OUT IObject** result) = 0;
    virtual void __stdcall Void(COM_OUT IObject** result) = 0;
};

extern "C" Status CreateObject(COM_OUT IObject** result);
