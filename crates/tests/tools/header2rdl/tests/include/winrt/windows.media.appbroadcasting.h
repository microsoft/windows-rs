
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
#ifndef __windows2Emedia2Eappbroadcasting_h__
#define __windows2Emedia2Eappbroadcasting_h__
#ifndef __windows2Emedia2Eappbroadcasting_p_h__
#define __windows2Emedia2Eappbroadcasting_p_h__


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

#if !defined(WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION)
#define WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION)

#if !defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)
#define WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.System.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppBroadcasting {
                interface IAppBroadcastingMonitor;
            } /* AppBroadcasting */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor ABI::Windows::Media::AppBroadcasting::IAppBroadcastingMonitor

#endif // ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppBroadcasting {
                interface IAppBroadcastingStatus;
            } /* AppBroadcasting */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus ABI::Windows::Media::AppBroadcasting::IAppBroadcastingStatus

#endif // ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppBroadcasting {
                interface IAppBroadcastingStatusDetails;
            } /* AppBroadcasting */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails ABI::Windows::Media::AppBroadcasting::IAppBroadcastingStatusDetails

#endif // ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppBroadcasting {
                interface IAppBroadcastingUI;
            } /* AppBroadcasting */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI ABI::Windows::Media::AppBroadcasting::IAppBroadcastingUI

#endif // ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppBroadcasting {
                interface IAppBroadcastingUIStatics;
            } /* AppBroadcasting */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics ABI::Windows::Media::AppBroadcasting::IAppBroadcastingUIStatics

#endif // ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppBroadcasting {
                class AppBroadcastingMonitor;
            } /* AppBroadcasting */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("dc64118b-04f9-5161-a7c0-e6a96070a8d1"))
ITypedEventHandler<ABI::Windows::Media::AppBroadcasting::AppBroadcastingMonitor*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::AppBroadcasting::AppBroadcastingMonitor*, ABI::Windows::Media::AppBroadcasting::IAppBroadcastingMonitor*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.AppBroadcasting.AppBroadcastingMonitor, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::AppBroadcasting::AppBroadcastingMonitor*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable_USE */

#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace System {
            class User;
        } /* System */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            interface IUser;
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CIUser ABI::Windows::System::IUser

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppBroadcasting {
                class AppBroadcastingStatus;
            } /* AppBroadcasting */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppBroadcasting {
                class AppBroadcastingStatusDetails;
            } /* AppBroadcasting */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppBroadcasting {
                class AppBroadcastingUI;
            } /* AppBroadcasting */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.Media.AppBroadcasting.IAppBroadcastingMonitor
 *
 * Introduced to Windows.Media.AppBroadcasting.AppBroadcastingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppBroadcasting.AppBroadcastingMonitor
 *
 */
#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppBroadcasting_IAppBroadcastingMonitor[] = L"Windows.Media.AppBroadcasting.IAppBroadcastingMonitor";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppBroadcasting {
                MIDL_INTERFACE("00f95a68-8907-48a0-b8ef-24d208137542")
                IAppBroadcastingMonitor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsCurrentAppBroadcasting(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_IsCurrentAppBroadcastingChanged(
                        __FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_IsCurrentAppBroadcastingChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppBroadcastingMonitor = __uuidof(IAppBroadcastingMonitor);
            } /* AppBroadcasting */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppBroadcasting.IAppBroadcastingStatus
 *
 * Introduced to Windows.Media.AppBroadcasting.AppBroadcastingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppBroadcasting.AppBroadcastingStatus
 *
 */
#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppBroadcasting_IAppBroadcastingStatus[] = L"Windows.Media.AppBroadcasting.IAppBroadcastingStatus";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppBroadcasting {
                MIDL_INTERFACE("1225e4df-03a1-42f8-8b80-c9228cd9cf2e")
                IAppBroadcastingStatus : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CanStartBroadcast(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Details(
                        ABI::Windows::Media::AppBroadcasting::IAppBroadcastingStatusDetails** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppBroadcastingStatus = __uuidof(IAppBroadcastingStatus);
            } /* AppBroadcasting */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppBroadcasting.IAppBroadcastingStatusDetails
 *
 * Introduced to Windows.Media.AppBroadcasting.AppBroadcastingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppBroadcasting.AppBroadcastingStatusDetails
 *
 */
#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppBroadcasting_IAppBroadcastingStatusDetails[] = L"Windows.Media.AppBroadcasting.IAppBroadcastingStatusDetails";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppBroadcasting {
                MIDL_INTERFACE("069dada4-b573-4e3c-8e19-1bafacd09713")
                IAppBroadcastingStatusDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsAnyAppBroadcasting(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsCaptureResourceUnavailable(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsGameStreamInProgress(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsGpuConstrained(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsAppInactive(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsBlockedForApp(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsDisabledByUser(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsDisabledBySystem(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppBroadcastingStatusDetails = __uuidof(IAppBroadcastingStatusDetails);
            } /* AppBroadcasting */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppBroadcasting.IAppBroadcastingUI
 *
 * Introduced to Windows.Media.AppBroadcasting.AppBroadcastingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppBroadcasting.AppBroadcastingUI
 *
 */
#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppBroadcasting_IAppBroadcastingUI[] = L"Windows.Media.AppBroadcasting.IAppBroadcastingUI";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppBroadcasting {
                MIDL_INTERFACE("e56f9f8f-ee99-4dca-a3c3-70af3db44f5f")
                IAppBroadcastingUI : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetStatus(
                        ABI::Windows::Media::AppBroadcasting::IAppBroadcastingStatus** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowBroadcastUI(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppBroadcastingUI = __uuidof(IAppBroadcastingUI);
            } /* AppBroadcasting */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppBroadcasting.IAppBroadcastingUIStatics
 *
 * Introduced to Windows.Media.AppBroadcasting.AppBroadcastingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppBroadcasting.AppBroadcastingUI
 *
 */
#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppBroadcasting_IAppBroadcastingUIStatics[] = L"Windows.Media.AppBroadcasting.IAppBroadcastingUIStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppBroadcasting {
                MIDL_INTERFACE("55a8a79d-23cb-4579-9c34-886fe02c045a")
                IAppBroadcastingUIStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::Media::AppBroadcasting::IAppBroadcastingUI** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::Media::AppBroadcasting::IAppBroadcastingUI** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppBroadcastingUIStatics = __uuidof(IAppBroadcastingUIStatics);
            } /* AppBroadcasting */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppBroadcasting.AppBroadcastingMonitor
 *
 * Introduced to Windows.Media.AppBroadcasting.AppBroadcastingContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Media.AppBroadcasting.AppBroadcastingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppBroadcasting.IAppBroadcastingMonitor ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppBroadcasting_AppBroadcastingMonitor_DEFINED
#define RUNTIMECLASS_Windows_Media_AppBroadcasting_AppBroadcastingMonitor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppBroadcasting_AppBroadcastingMonitor[] = L"Windows.Media.AppBroadcasting.AppBroadcastingMonitor";
#endif
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppBroadcasting.AppBroadcastingStatus
 *
 * Introduced to Windows.Media.AppBroadcasting.AppBroadcastingContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppBroadcasting.IAppBroadcastingStatus ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppBroadcasting_AppBroadcastingStatus_DEFINED
#define RUNTIMECLASS_Windows_Media_AppBroadcasting_AppBroadcastingStatus_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppBroadcasting_AppBroadcastingStatus[] = L"Windows.Media.AppBroadcasting.AppBroadcastingStatus";
#endif
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppBroadcasting.AppBroadcastingStatusDetails
 *
 * Introduced to Windows.Media.AppBroadcasting.AppBroadcastingContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppBroadcasting.IAppBroadcastingStatusDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppBroadcasting_AppBroadcastingStatusDetails_DEFINED
#define RUNTIMECLASS_Windows_Media_AppBroadcasting_AppBroadcastingStatusDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppBroadcasting_AppBroadcastingStatusDetails[] = L"Windows.Media.AppBroadcasting.AppBroadcastingStatusDetails";
#endif
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppBroadcasting.AppBroadcastingUI
 *
 * Introduced to Windows.Media.AppBroadcasting.AppBroadcastingContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.AppBroadcasting.IAppBroadcastingUIStatics interface starting with version 1.0 of the Windows.Media.AppBroadcasting.AppBroadcastingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppBroadcasting.IAppBroadcastingUI ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppBroadcasting_AppBroadcastingUI_DEFINED
#define RUNTIMECLASS_Windows_Media_AppBroadcasting_AppBroadcastingUI_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppBroadcasting_AppBroadcastingUI[] = L"Windows.Media.AppBroadcasting.AppBroadcastingUI";
#endif
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor;

#endif // ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus;

#endif // ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails;

#endif // ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI;

#endif // ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics;

#endif // ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable __FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable* This,
        __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

/*
 *
 * Interface Windows.Media.AppBroadcasting.IAppBroadcastingMonitor
 *
 * Introduced to Windows.Media.AppBroadcasting.AppBroadcastingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppBroadcasting.AppBroadcastingMonitor
 *
 */
#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppBroadcasting_IAppBroadcastingMonitor[] = L"Windows.Media.AppBroadcasting.IAppBroadcastingMonitor";
typedef struct __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsCurrentAppBroadcasting)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* add_IsCurrentAppBroadcastingChanged)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor* This,
        __FITypedEventHandler_2_Windows__CMedia__CAppBroadcasting__CAppBroadcastingMonitor_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_IsCurrentAppBroadcastingChanged)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitorVtbl;

interface __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_get_IsCurrentAppBroadcasting(This, value) \
    ((This)->lpVtbl->get_IsCurrentAppBroadcasting(This, value))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_add_IsCurrentAppBroadcastingChanged(This, handler, token) \
    ((This)->lpVtbl->add_IsCurrentAppBroadcastingChanged(This, handler, token))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_remove_IsCurrentAppBroadcastingChanged(This, token) \
    ((This)->lpVtbl->remove_IsCurrentAppBroadcastingChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingMonitor_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppBroadcasting.IAppBroadcastingStatus
 *
 * Introduced to Windows.Media.AppBroadcasting.AppBroadcastingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppBroadcasting.AppBroadcastingStatus
 *
 */
#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppBroadcasting_IAppBroadcastingStatus[] = L"Windows.Media.AppBroadcasting.IAppBroadcastingStatus";
typedef struct __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CanStartBroadcast)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Details)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus* This,
        __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusVtbl;

interface __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_get_CanStartBroadcast(This, value) \
    ((This)->lpVtbl->get_CanStartBroadcast(This, value))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_get_Details(This, value) \
    ((This)->lpVtbl->get_Details(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppBroadcasting.IAppBroadcastingStatusDetails
 *
 * Introduced to Windows.Media.AppBroadcasting.AppBroadcastingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppBroadcasting.AppBroadcastingStatusDetails
 *
 */
#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppBroadcasting_IAppBroadcastingStatusDetails[] = L"Windows.Media.AppBroadcasting.IAppBroadcastingStatusDetails";
typedef struct __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsAnyAppBroadcasting)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsCaptureResourceUnavailable)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsGameStreamInProgress)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsGpuConstrained)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsAppInactive)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsBlockedForApp)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsDisabledByUser)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsDisabledBySystem)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetailsVtbl;

interface __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_get_IsAnyAppBroadcasting(This, value) \
    ((This)->lpVtbl->get_IsAnyAppBroadcasting(This, value))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_get_IsCaptureResourceUnavailable(This, value) \
    ((This)->lpVtbl->get_IsCaptureResourceUnavailable(This, value))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_get_IsGameStreamInProgress(This, value) \
    ((This)->lpVtbl->get_IsGameStreamInProgress(This, value))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_get_IsGpuConstrained(This, value) \
    ((This)->lpVtbl->get_IsGpuConstrained(This, value))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_get_IsAppInactive(This, value) \
    ((This)->lpVtbl->get_IsAppInactive(This, value))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_get_IsBlockedForApp(This, value) \
    ((This)->lpVtbl->get_IsBlockedForApp(This, value))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_get_IsDisabledByUser(This, value) \
    ((This)->lpVtbl->get_IsDisabledByUser(This, value))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_get_IsDisabledBySystem(This, value) \
    ((This)->lpVtbl->get_IsDisabledBySystem(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatusDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppBroadcasting.IAppBroadcastingUI
 *
 * Introduced to Windows.Media.AppBroadcasting.AppBroadcastingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppBroadcasting.AppBroadcastingUI
 *
 */
#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppBroadcasting_IAppBroadcastingUI[] = L"Windows.Media.AppBroadcasting.IAppBroadcastingUI";
typedef struct __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetStatus)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI* This,
        __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingStatus** result);
    HRESULT (STDMETHODCALLTYPE* ShowBroadcastUI)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI* This);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIVtbl;

interface __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_GetStatus(This, result) \
    ((This)->lpVtbl->GetStatus(This, result))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_ShowBroadcastUI(This) \
    ((This)->lpVtbl->ShowBroadcastUI(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppBroadcasting.IAppBroadcastingUIStatics
 *
 * Introduced to Windows.Media.AppBroadcasting.AppBroadcastingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppBroadcasting.AppBroadcastingUI
 *
 */
#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppBroadcasting_IAppBroadcastingUIStatics[] = L"Windows.Media.AppBroadcasting.IAppBroadcastingUIStatics";
typedef struct __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics* This,
        __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI** result);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUI** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#define __x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_GetForUser(This, user, result) \
    ((This)->lpVtbl->GetForUser(This, user, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppBroadcasting_CIAppBroadcastingUIStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppBroadcasting.AppBroadcastingMonitor
 *
 * Introduced to Windows.Media.AppBroadcasting.AppBroadcastingContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Media.AppBroadcasting.AppBroadcastingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppBroadcasting.IAppBroadcastingMonitor ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppBroadcasting_AppBroadcastingMonitor_DEFINED
#define RUNTIMECLASS_Windows_Media_AppBroadcasting_AppBroadcastingMonitor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppBroadcasting_AppBroadcastingMonitor[] = L"Windows.Media.AppBroadcasting.AppBroadcastingMonitor";
#endif
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppBroadcasting.AppBroadcastingStatus
 *
 * Introduced to Windows.Media.AppBroadcasting.AppBroadcastingContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppBroadcasting.IAppBroadcastingStatus ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppBroadcasting_AppBroadcastingStatus_DEFINED
#define RUNTIMECLASS_Windows_Media_AppBroadcasting_AppBroadcastingStatus_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppBroadcasting_AppBroadcastingStatus[] = L"Windows.Media.AppBroadcasting.AppBroadcastingStatus";
#endif
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppBroadcasting.AppBroadcastingStatusDetails
 *
 * Introduced to Windows.Media.AppBroadcasting.AppBroadcastingContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppBroadcasting.IAppBroadcastingStatusDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppBroadcasting_AppBroadcastingStatusDetails_DEFINED
#define RUNTIMECLASS_Windows_Media_AppBroadcasting_AppBroadcastingStatusDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppBroadcasting_AppBroadcastingStatusDetails[] = L"Windows.Media.AppBroadcasting.AppBroadcastingStatusDetails";
#endif
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppBroadcasting.AppBroadcastingUI
 *
 * Introduced to Windows.Media.AppBroadcasting.AppBroadcastingContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.AppBroadcasting.IAppBroadcastingUIStatics interface starting with version 1.0 of the Windows.Media.AppBroadcasting.AppBroadcastingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppBroadcasting.IAppBroadcastingUI ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppBroadcasting_AppBroadcastingUI_DEFINED
#define RUNTIMECLASS_Windows_Media_AppBroadcasting_AppBroadcastingUI_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppBroadcasting_AppBroadcastingUI[] = L"Windows.Media.AppBroadcasting.AppBroadcastingUI";
#endif
#endif // WINDOWS_MEDIA_APPBROADCASTING_APPBROADCASTINGCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Emedia2Eappbroadcasting_p_h__

#endif // __windows2Emedia2Eappbroadcasting_h__
