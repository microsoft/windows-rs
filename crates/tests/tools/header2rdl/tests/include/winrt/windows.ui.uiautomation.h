
#pragma warning( disable: 4049 )  /* more than 64k source lines */

/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include <rpc.h>
#include <rpcndr.h>

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */

#ifndef COM_NO_WINDOWS_H
#include <windows.h>
#include <ole2.h>
#endif /*COM_NO_WINDOWS_H*/
#ifndef __windows2Eui2Euiautomation_h__
#define __windows2Eui2Euiautomation_h__
#ifndef __windows2Eui2Euiautomation_p_h__
#define __windows2Eui2Euiautomation_p_h__


#pragma once

//
// Deprecated attribute support
//

#pragma push_macro("DEPRECATED")
#undef DEPRECATED

#if !defined(DISABLE_WINRT_DEPRECATION)
#if defined(__cplusplus)
#if __cplusplus >= 201402
#define DEPRECATED(x) [[deprecated(x)]]
#define DEPRECATEDENUMERATOR(x) [[deprecated(x)]]
#elif defined(_MSC_VER)
#if _MSC_VER >= 1900
#define DEPRECATED(x) [[deprecated(x)]]
#define DEPRECATEDENUMERATOR(x) [[deprecated(x)]]
#else
#define DEPRECATED(x) __declspec(deprecated(x))
#define DEPRECATEDENUMERATOR(x)
#endif // _MSC_VER >= 1900
#else // Not Standard C++ or MSVC, ignore the construct.
#define DEPRECATED(x)
#define DEPRECATEDENUMERATOR(x)
#endif  // C++ deprecation
#else // C - disable deprecation
#define DEPRECATED(x)
#define DEPRECATEDENUMERATOR(x)
#endif
#else // Deprecation is disabled
#define DEPRECATED(x)
#define DEPRECATEDENUMERATOR(x)
#endif  /* DEPRECATED */

// Disable Deprecation for this header, MIDL verifies that cross-type access is acceptable
#ifdef __clang__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#else
#pragma warning(push)
#pragma warning(disable: 4996)
#endif

// Ensure that the setting of the /ns_prefix command line switch is consistent for all headers.
// If you get an error from the compiler indicating "warning C4005: 'CHECK_NS_PREFIX_STATE': macro redefinition", this
// indicates that you have included two different headers with different settings for the /ns_prefix MIDL command line switch
#if !defined(DISABLE_NS_PREFIX_CHECKS)
#define CHECK_NS_PREFIX_STATE "always"
#endif // !defined(DISABLE_NS_PREFIX_CHECKS)


#pragma push_macro("MIDL_CONST_ID")
#undef MIDL_CONST_ID
#define MIDL_CONST_ID const __declspec(selectany)


//  API Contract Inclusion Definitions
#if !defined(SPECIFIC_API_CONTRACT_DEFINITIONS)
#if !defined(WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION)
#define WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                interface IAutomationConnection;
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection ABI::Windows::UI::UIAutomation::IAutomationConnection

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                interface IAutomationConnectionBoundObject;
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject ABI::Windows::UI::UIAutomation::IAutomationConnectionBoundObject

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                interface IAutomationElement;
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement ABI::Windows::UI::UIAutomation::IAutomationElement

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                interface IAutomationTextRange;
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange ABI::Windows::UI::UIAutomation::IAutomationTextRange

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                class AutomationConnection;
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.UI.UIAutomation.IAutomationConnection
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.AutomationConnection
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_IAutomationConnection[] = L"Windows.UI.UIAutomation.IAutomationConnection";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                MIDL_INTERFACE("aad262ed-0ef4-5d43-97be-a834e27b65b9")
                IAutomationConnection : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsRemoteSystem(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AppUserModelId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExecutableFileName(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAutomationConnection = __uuidof(IAutomationConnection);
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.IAutomationConnectionBoundObject
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.AutomationConnectionBoundObject
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_IAutomationConnectionBoundObject[] = L"Windows.UI.UIAutomation.IAutomationConnectionBoundObject";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                MIDL_INTERFACE("5e8558fb-ca52-5b65-9830-dd2905816093")
                IAutomationConnectionBoundObject : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Connection(
                        ABI::Windows::UI::UIAutomation::IAutomationConnection** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAutomationConnectionBoundObject = __uuidof(IAutomationConnectionBoundObject);
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.IAutomationElement
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.AutomationElement
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_IAutomationElement[] = L"Windows.UI.UIAutomation.IAutomationElement";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                MIDL_INTERFACE("a1898370-2c07-56fd-993f-61a72a08058c")
                IAutomationElement : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsRemoteSystem(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AppUserModelId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExecutableFileName(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAutomationElement = __uuidof(IAutomationElement);
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.UIAutomation.IAutomationTextRange
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.AutomationTextRange
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_IAutomationTextRange[] = L"Windows.UI.UIAutomation.IAutomationTextRange";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                MIDL_INTERFACE("7e101b65-40d3-5994-85a9-0a0cb9a4ec98")
                IAutomationTextRange : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IAutomationTextRange = __uuidof(IAutomationTextRange);
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.UIAutomation.AutomationConnection
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.IAutomationConnection ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_AutomationConnection_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_AutomationConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_AutomationConnection[] = L"Windows.UI.UIAutomation.AutomationConnection";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.UIAutomation.AutomationConnectionBoundObject
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.IAutomationConnectionBoundObject ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_AutomationConnectionBoundObject_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_AutomationConnectionBoundObject_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_AutomationConnectionBoundObject[] = L"Windows.UI.UIAutomation.AutomationConnectionBoundObject";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.UIAutomation.AutomationElement
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.IAutomationElement ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_AutomationElement_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_AutomationElement_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_AutomationElement[] = L"Windows.UI.UIAutomation.AutomationElement";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.UIAutomation.AutomationTextRange
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.IAutomationTextRange ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_AutomationTextRange_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_AutomationTextRange_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_AutomationTextRange[] = L"Windows.UI.UIAutomation.AutomationTextRange";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

/*
 *
 * Interface Windows.UI.UIAutomation.IAutomationConnection
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.AutomationConnection
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_IAutomationConnection[] = L"Windows.UI.UIAutomation.IAutomationConnection";
typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsRemoteSystem)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_AppUserModelId)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ExecutableFileName)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionVtbl;

interface __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_get_IsRemoteSystem(This, value) \
    ((This)->lpVtbl->get_IsRemoteSystem(This, value))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_get_AppUserModelId(This, value) \
    ((This)->lpVtbl->get_AppUserModelId(This, value))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_get_ExecutableFileName(This, value) \
    ((This)->lpVtbl->get_ExecutableFileName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.IAutomationConnectionBoundObject
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.AutomationConnectionBoundObject
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_IAutomationConnectionBoundObject[] = L"Windows.UI.UIAutomation.IAutomationConnectionBoundObject";
typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObjectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Connection)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject* This,
        __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnection** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObjectVtbl;

interface __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObjectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_get_Connection(This, value) \
    ((This)->lpVtbl->get_Connection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.IAutomationElement
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.AutomationElement
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_IAutomationElement[] = L"Windows.UI.UIAutomation.IAutomationElement";
typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsRemoteSystem)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_AppUserModelId)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ExecutableFileName)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElementVtbl;

interface __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_get_IsRemoteSystem(This, value) \
    ((This)->lpVtbl->get_IsRemoteSystem(This, value))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_get_AppUserModelId(This, value) \
    ((This)->lpVtbl->get_AppUserModelId(This, value))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_get_ExecutableFileName(This, value) \
    ((This)->lpVtbl->get_ExecutableFileName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.UIAutomation.IAutomationTextRange
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.AutomationTextRange
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_IAutomationTextRange[] = L"Windows.UI.UIAutomation.IAutomationTextRange";
typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRangeVtbl;

interface __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.UIAutomation.AutomationConnection
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.IAutomationConnection ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_AutomationConnection_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_AutomationConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_AutomationConnection[] = L"Windows.UI.UIAutomation.AutomationConnection";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.UIAutomation.AutomationConnectionBoundObject
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.IAutomationConnectionBoundObject ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_AutomationConnectionBoundObject_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_AutomationConnectionBoundObject_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_AutomationConnectionBoundObject[] = L"Windows.UI.UIAutomation.AutomationConnectionBoundObject";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.UIAutomation.AutomationElement
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.IAutomationElement ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_AutomationElement_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_AutomationElement_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_AutomationElement[] = L"Windows.UI.UIAutomation.AutomationElement";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.UIAutomation.AutomationTextRange
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.IAutomationTextRange ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_AutomationTextRange_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_AutomationTextRange_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_AutomationTextRange[] = L"Windows.UI.UIAutomation.AutomationTextRange";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Euiautomation_p_h__

#endif // __windows2Eui2Euiautomation_h__
