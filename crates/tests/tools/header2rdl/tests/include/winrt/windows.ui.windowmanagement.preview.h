
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
#ifndef __windows2Eui2Ewindowmanagement2Epreview_h__
#define __windows2Eui2Ewindowmanagement2Epreview_h__
#ifndef __windows2Eui2Ewindowmanagement2Epreview_p_h__
#define __windows2Eui2Ewindowmanagement2Epreview_p_h__


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
#include "Windows.UI.WindowManagement.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                namespace Preview {
                    interface IWindowManagementPreview;
                } /* Preview */
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview ABI::Windows::UI::WindowManagement::Preview::IWindowManagementPreview

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                namespace Preview {
                    interface IWindowManagementPreviewStatics;
                } /* Preview */
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics ABI::Windows::UI::WindowManagement::Preview::IWindowManagementPreviewStatics

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Size Size;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                class AppWindow;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IAppWindow;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow ABI::Windows::UI::WindowManagement::IAppWindow

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__

/*
 *
 * Interface Windows.UI.WindowManagement.Preview.IWindowManagementPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.Preview.WindowManagementPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_Preview_IWindowManagementPreview[] = L"Windows.UI.WindowManagement.Preview.IWindowManagementPreview";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                namespace Preview {
                    MIDL_INTERFACE("4ef55b0d-561d-513c-a67c-2c02b69cef41")
                    IWindowManagementPreview : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IWindowManagementPreview = __uuidof(IWindowManagementPreview);
                } /* Preview */
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.Preview.IWindowManagementPreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.Preview.WindowManagementPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_Preview_IWindowManagementPreviewStatics[] = L"Windows.UI.WindowManagement.Preview.IWindowManagementPreviewStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                namespace Preview {
                    MIDL_INTERFACE("0f9725c6-c004-5a23-8fd2-8d092ce2704a")
                    IWindowManagementPreviewStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetPreferredMinSize(
                            ABI::Windows::UI::WindowManagement::IAppWindow* window,
                            ABI::Windows::Foundation::Size preferredFrameMinSize
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWindowManagementPreviewStatics = __uuidof(IWindowManagementPreviewStatics);
                } /* Preview */
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.Preview.WindowManagementPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.WindowManagement.Preview.IWindowManagementPreviewStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.Preview.IWindowManagementPreview ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_Preview_WindowManagementPreview_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_Preview_WindowManagementPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_Preview_WindowManagementPreview[] = L"Windows.UI.WindowManagement.Preview.WindowManagementPreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef struct __x_ABI_CWindows_CFoundation_CSize __x_ABI_CWindows_CFoundation_CSize;

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__

/*
 *
 * Interface Windows.UI.WindowManagement.Preview.IWindowManagementPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.Preview.WindowManagementPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_Preview_IWindowManagementPreview[] = L"Windows.UI.WindowManagement.Preview.IWindowManagementPreview";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.Preview.IWindowManagementPreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.Preview.WindowManagementPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_Preview_IWindowManagementPreviewStatics[] = L"Windows.UI.WindowManagement.Preview.IWindowManagementPreviewStatics";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetPreferredMinSize)(__x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* window,
        struct __x_ABI_CWindows_CFoundation_CSize preferredFrameMinSize);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStaticsVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_SetPreferredMinSize(This, window, preferredFrameMinSize) \
    ((This)->lpVtbl->SetPreferredMinSize(This, window, preferredFrameMinSize))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CPreview_CIWindowManagementPreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.Preview.WindowManagementPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.WindowManagement.Preview.IWindowManagementPreviewStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.Preview.IWindowManagementPreview ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_Preview_WindowManagementPreview_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_Preview_WindowManagementPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_Preview_WindowManagementPreview[] = L"Windows.UI.WindowManagement.Preview.WindowManagementPreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Ewindowmanagement2Epreview_p_h__

#endif // __windows2Eui2Ewindowmanagement2Epreview_h__
