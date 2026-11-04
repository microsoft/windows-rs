#include <sal.h>

#define _Win32_PreserveResult_ __attribute__((annotate("Win32Metadata.PreserveResult")))

typedef long HRESULT;

extern "C" _Win32_PreserveResult_
_Success_(return >= 0)
HRESULT __stdcall ReadValue(/* [out][retval] */ _Out_ unsigned *value);

_Win32_PreserveResult_
typedef HRESULT (__stdcall *Callback)(_Out_ unsigned *value);

typedef Callback CallbackAlias;
_Win32_PreserveResult_ typedef Callback CallbackAgain;

_Win32_PreserveResult_
typedef HRESULT __stdcall CallbackFunction(_Out_ unsigned *value);
typedef CallbackFunction *CallbackPointer;

struct Callbacks {
    Callback direct;
    CallbackAlias alias;
    CallbackAgain marked_alias;
    CallbackPointer function;
};

extern "C" void __stdcall UseCallback(CallbackAlias callback);

struct __declspec(uuid("10000000-0000-0000-0000-000000000001")) Results {
    _Win32_PreserveResult_
    virtual HRESULT __stdcall Status() = 0;
    _Win32_PreserveResult_
    _Success_(return >= 0)
    virtual HRESULT __stdcall Read(/* [out][retval] */ _Out_ unsigned *value) = 0;
};
