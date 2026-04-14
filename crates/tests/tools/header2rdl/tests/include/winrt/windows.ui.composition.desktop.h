
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
#ifndef __windows2Eui2Ecomposition2Edesktop_h__
#define __windows2Eui2Ecomposition2Edesktop_h__
#ifndef __windows2Eui2Ecomposition2Edesktop_p_h__
#define __windows2Eui2Ecomposition2Edesktop_p_h__


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
#if !defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)
#define WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)
#define WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION 0x130000
#endif // defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Desktop {
                    interface IDesktopWindowTarget;
                } /* Desktop */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget ABI::Windows::UI::Composition::Desktop::IDesktopWindowTarget

#endif // ____x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
/*
 *
 * Interface Windows.UI.Composition.Desktop.IDesktopWindowTarget
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Desktop.DesktopWindowTarget
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Desktop_IDesktopWindowTarget[] = L"Windows.UI.Composition.Desktop.IDesktopWindowTarget";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Desktop {
                    MIDL_INTERFACE("6329d6ca-3366-490e-9db3-25312929ac51")
                    IDesktopWindowTarget : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsTopmost(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDesktopWindowTarget = __uuidof(IDesktopWindowTarget);
                } /* Desktop */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.Composition.Desktop.DesktopWindowTarget
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Desktop.IDesktopWindowTarget ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Desktop_DesktopWindowTarget_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Desktop_DesktopWindowTarget_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Desktop_DesktopWindowTarget[] = L"Windows.UI.Composition.Desktop.DesktopWindowTarget";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget __x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget;

#endif // ____x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

/*
 *
 * Interface Windows.UI.Composition.Desktop.IDesktopWindowTarget
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Desktop.DesktopWindowTarget
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Desktop_IDesktopWindowTarget[] = L"Windows.UI.Composition.Desktop.IDesktopWindowTarget";
typedef struct __x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTargetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsTopmost)(__x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTargetVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTargetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_get_IsTopmost(This, value) \
    ((This)->lpVtbl->get_IsTopmost(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CDesktop_CIDesktopWindowTarget_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.Composition.Desktop.DesktopWindowTarget
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Desktop.IDesktopWindowTarget ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Desktop_DesktopWindowTarget_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Desktop_DesktopWindowTarget_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Desktop_DesktopWindowTarget[] = L"Windows.UI.Composition.Desktop.DesktopWindowTarget";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Ecomposition2Edesktop_p_h__

#endif // __windows2Eui2Ecomposition2Edesktop_h__
