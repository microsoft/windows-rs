
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
#ifndef __windows2Eui2Enotifications2Epreview_h__
#define __windows2Eui2Enotifications2Epreview_h__
#ifndef __windows2Eui2Enotifications2Epreview_p_h__
#define __windows2Eui2Enotifications2Epreview_p_h__


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
#include "Windows.UI.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                namespace Preview {
                    interface IToastOcclusionManagerPreviewStatics;
                } /* Preview */
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics ABI::Windows::UI::Notifications::Preview::IToastOcclusionManagerPreviewStatics

#endif // ____x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct WindowId WindowId;
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.UI.Notifications.Preview.IToastOcclusionManagerPreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.Preview.ToastOcclusionManagerPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_Preview_IToastOcclusionManagerPreviewStatics[] = L"Windows.UI.Notifications.Preview.IToastOcclusionManagerPreviewStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                namespace Preview {
                    MIDL_INTERFACE("507e5c83-50f9-5412-8953-b65c18cfab12")
                    IToastOcclusionManagerPreviewStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetToastWindowMargin(
                            ABI::Windows::UI::WindowId appWindowId,
                            DOUBLE margin
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IToastOcclusionManagerPreviewStatics = __uuidof(IToastOcclusionManagerPreviewStatics);
                } /* Preview */
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.UI.Notifications.Preview.ToastOcclusionManagerPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Notifications.Preview.IToastOcclusionManagerPreviewStatics interface starting with version 13.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_Preview_ToastOcclusionManagerPreview_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_Preview_ToastOcclusionManagerPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_Preview_ToastOcclusionManagerPreview[] = L"Windows.UI.Notifications.Preview.ToastOcclusionManagerPreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics __x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef struct __x_ABI_CWindows_CUI_CWindowId __x_ABI_CWindows_CUI_CWindowId;

/*
 *
 * Interface Windows.UI.Notifications.Preview.IToastOcclusionManagerPreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.Preview.ToastOcclusionManagerPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_Preview_IToastOcclusionManagerPreviewStatics[] = L"Windows.UI.Notifications.Preview.IToastOcclusionManagerPreviewStatics";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetToastWindowMargin)(__x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics* This,
        struct __x_ABI_CWindows_CUI_CWindowId appWindowId,
        DOUBLE margin);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStaticsVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_SetToastWindowMargin(This, appWindowId, margin) \
    ((This)->lpVtbl->SetToastWindowMargin(This, appWindowId, margin))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CPreview_CIToastOcclusionManagerPreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.UI.Notifications.Preview.ToastOcclusionManagerPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Notifications.Preview.IToastOcclusionManagerPreviewStatics interface starting with version 13.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_Preview_ToastOcclusionManagerPreview_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_Preview_ToastOcclusionManagerPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_Preview_ToastOcclusionManagerPreview[] = L"Windows.UI.Notifications.Preview.ToastOcclusionManagerPreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Enotifications2Epreview_p_h__

#endif // __windows2Eui2Enotifications2Epreview_h__
