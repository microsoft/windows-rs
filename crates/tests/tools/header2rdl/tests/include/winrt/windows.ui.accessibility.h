
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
#ifndef __windows2Eui2Eaccessibility_h__
#define __windows2Eui2Eaccessibility_h__
#ifndef __windows2Eui2Eaccessibility_p_h__
#define __windows2Eui2Eaccessibility_p_h__


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
#ifndef ____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Accessibility {
                interface IScreenReaderPositionChangedEventArgs;
            } /* Accessibility */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs ABI::Windows::UI::Accessibility::IScreenReaderPositionChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Accessibility {
                interface IScreenReaderService;
            } /* Accessibility */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService ABI::Windows::UI::Accessibility::IScreenReaderService

#endif // ____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Accessibility {
                class ScreenReaderService;
            } /* Accessibility */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Accessibility {
                class ScreenReaderPositionChangedEventArgs;
            } /* Accessibility */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("51b01f42-eefc-56d2-8185-23de8f080e0a"))
ITypedEventHandler<ABI::Windows::UI::Accessibility::ScreenReaderService*, ABI::Windows::UI::Accessibility::ScreenReaderPositionChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Accessibility::ScreenReaderService*, ABI::Windows::UI::Accessibility::IScreenReaderService*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Accessibility::ScreenReaderPositionChangedEventArgs*, ABI::Windows::UI::Accessibility::IScreenReaderPositionChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Accessibility.ScreenReaderService, Windows.UI.Accessibility.ScreenReaderPositionChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Accessibility::ScreenReaderService*, ABI::Windows::UI::Accessibility::ScreenReaderPositionChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.UI.Accessibility.IScreenReaderPositionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Accessibility.ScreenReaderPositionChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Accessibility_IScreenReaderPositionChangedEventArgs[] = L"Windows.UI.Accessibility.IScreenReaderPositionChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Accessibility {
                MIDL_INTERFACE("557eb5e5-54d0-5ccd-9fc5-ed33357f8a9f")
                IScreenReaderPositionChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ScreenPositionInRawPixels(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsReadingText(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IScreenReaderPositionChangedEventArgs = __uuidof(IScreenReaderPositionChangedEventArgs);
            } /* Accessibility */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Accessibility.IScreenReaderService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Accessibility.ScreenReaderService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Accessibility_IScreenReaderService[] = L"Windows.UI.Accessibility.IScreenReaderService";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Accessibility {
                MIDL_INTERFACE("19475427-eac0-50d3-bdd9-9b487a226256")
                IScreenReaderService : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentScreenReaderPosition(
                        ABI::Windows::UI::Accessibility::IScreenReaderPositionChangedEventArgs** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ScreenReaderPositionChanged(
                        __FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ScreenReaderPositionChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IScreenReaderService = __uuidof(IScreenReaderService);
            } /* Accessibility */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService;
#endif /* !defined(____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.Accessibility.ScreenReaderPositionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Accessibility.IScreenReaderPositionChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_UI_Accessibility_ScreenReaderPositionChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Accessibility_ScreenReaderPositionChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Accessibility_ScreenReaderPositionChangedEventArgs[] = L"Windows.UI.Accessibility.ScreenReaderPositionChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.Accessibility.ScreenReaderService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Accessibility.IScreenReaderService ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_UI_Accessibility_ScreenReaderService_DEFINED
#define RUNTIMECLASS_Windows_UI_Accessibility_ScreenReaderService_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Accessibility_ScreenReaderService[] = L"Windows.UI.Accessibility.ScreenReaderService";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService;

#endif // ____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService* sender,
        __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

/*
 *
 * Interface Windows.UI.Accessibility.IScreenReaderPositionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Accessibility.ScreenReaderPositionChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Accessibility_IScreenReaderPositionChangedEventArgs[] = L"Windows.UI.Accessibility.IScreenReaderPositionChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ScreenPositionInRawPixels)(__x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* get_IsReadingText)(__x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_get_ScreenPositionInRawPixels(This, value) \
    ((This)->lpVtbl->get_ScreenPositionInRawPixels(This, value))

#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_get_IsReadingText(This, value) \
    ((This)->lpVtbl->get_IsReadingText(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Accessibility.IScreenReaderService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Accessibility.ScreenReaderService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Accessibility_IScreenReaderService[] = L"Windows.UI.Accessibility.IScreenReaderService";
typedef struct __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderServiceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CurrentScreenReaderPosition)(__x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService* This,
        __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderPositionChangedEventArgs** value);
    HRESULT (STDMETHODCALLTYPE* add_ScreenReaderPositionChanged)(__x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService* This,
        __FITypedEventHandler_2_Windows__CUI__CAccessibility__CScreenReaderService_Windows__CUI__CAccessibility__CScreenReaderPositionChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ScreenReaderPositionChanged)(__x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderServiceVtbl;

interface __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderServiceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_get_CurrentScreenReaderPosition(This, value) \
    ((This)->lpVtbl->get_CurrentScreenReaderPosition(This, value))

#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_add_ScreenReaderPositionChanged(This, handler, token) \
    ((This)->lpVtbl->add_ScreenReaderPositionChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_remove_ScreenReaderPositionChanged(This, token) \
    ((This)->lpVtbl->remove_ScreenReaderPositionChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService;
#endif /* !defined(____x_ABI_CWindows_CUI_CAccessibility_CIScreenReaderService_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.Accessibility.ScreenReaderPositionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Accessibility.IScreenReaderPositionChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_UI_Accessibility_ScreenReaderPositionChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Accessibility_ScreenReaderPositionChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Accessibility_ScreenReaderPositionChangedEventArgs[] = L"Windows.UI.Accessibility.ScreenReaderPositionChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.Accessibility.ScreenReaderService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Accessibility.IScreenReaderService ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_UI_Accessibility_ScreenReaderService_DEFINED
#define RUNTIMECLASS_Windows_UI_Accessibility_ScreenReaderService_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Accessibility_ScreenReaderService[] = L"Windows.UI.Accessibility.ScreenReaderService";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Eaccessibility_p_h__

#endif // __windows2Eui2Eaccessibility_h__
