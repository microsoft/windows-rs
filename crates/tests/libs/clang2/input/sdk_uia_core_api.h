#include <intsafe.h>
#include <windows.h>
#include <ole2.h>
#include <uiautomationcore.h>
#include <uiautomationclient.h>
#include <uiautomationcoreapi.h>

static_assert(UiaRootObjectId == -25);
static_assert(UIA_NamePropertyId == 30005);
static_assert(UIA_InvokePatternId == 10000);
static_assert(UIA_ButtonControlTypeId == 50000);
static_assert(UIA_Invoke_InvokedEventId == 20009);
static_assert(__is_same(decltype(&UiaReturnRawElementProvider),
    LRESULT(WINAPI*)(HWND, WPARAM, LPARAM, IRawElementProviderSimple*)));
static_assert(__is_same(decltype(&UiaHostProviderFromHwnd),
    HRESULT(WINAPI*)(HWND, IRawElementProviderSimple**)));
static_assert(__is_same(decltype(&UiaRaiseAutomationPropertyChangedEvent),
    HRESULT(WINAPI*)(IRawElementProviderSimple*, PROPERTYID, VARIANT, VARIANT)));
