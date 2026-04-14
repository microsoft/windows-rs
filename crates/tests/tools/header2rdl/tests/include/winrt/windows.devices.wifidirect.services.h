
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
#ifndef __windows2Edevices2Ewifidirect2Eservices_h__
#define __windows2Edevices2Ewifidirect2Eservices_h__
#ifndef __windows2Edevices2Ewifidirect2Eservices_p_h__
#define __windows2Edevices2Ewifidirect2Eservices_p_h__


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

#if !defined(WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION)
#define WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Devices.Enumeration.h"
#include "Windows.Networking.h"
#include "Windows.Networking.Sockets.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    interface IWiFiDirectService;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectService

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    interface IWiFiDirectServiceAdvertiser;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceAdvertiser

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    interface IWiFiDirectServiceAdvertiserFactory;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceAdvertiserFactory

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    interface IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    interface IWiFiDirectServiceProvisioningInfo;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceProvisioningInfo

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    interface IWiFiDirectServiceRemotePortAddedEventArgs;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceRemotePortAddedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    interface IWiFiDirectServiceSession;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceSession

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    interface IWiFiDirectServiceSessionDeferredEventArgs;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceSessionDeferredEventArgs

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    interface IWiFiDirectServiceSessionRequest;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceSessionRequest

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    interface IWiFiDirectServiceSessionRequestedEventArgs;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceSessionRequestedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    interface IWiFiDirectServiceStatics;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceStatics

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    class WiFiDirectService;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c4fa2ae8-4ff7-5aa0-af97-ed85ea66f9ae"))
IAsyncOperation<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectService*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectService*, ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectService*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.WiFiDirect.Services.WiFiDirectService>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectService*> __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_t;
#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f505a3c8-4837-5e0e-8a4d-1e2af5477e5c"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectService*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectService*, ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectService*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.WiFiDirect.Services.WiFiDirectService>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectService*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    class WiFiDirectServiceProvisioningInfo;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d7fa4dc4-4730-506e-bff0-801eb4a831a8"))
IAsyncOperation<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceProvisioningInfo*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceProvisioningInfo*, ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceProvisioningInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.WiFiDirect.Services.WiFiDirectServiceProvisioningInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceProvisioningInfo*> __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_t;
#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("94cb9568-040a-5186-a3c9-52680ee17984"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceProvisioningInfo*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceProvisioningInfo*, ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceProvisioningInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.WiFiDirect.Services.WiFiDirectServiceProvisioningInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceProvisioningInfo*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    class WiFiDirectServiceSession;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c2da4e97-728b-5401-a9d9-3a0185450af2"))
IAsyncOperation<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSession*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSession*, ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceSession*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSession>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSession*> __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_t;
#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b29de711-60b8-59da-8f4d-fc79d8ccd422"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSession*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSession*, ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceSession*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSession>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSession*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIIterator_1_HSTRING_USE
#define DEF___FIIterator_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8c304ebb-6615-50a4-8829-879ecd443236"))
IIterator<HSTRING> : IIterator_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<HSTRING> __FIIterator_1_HSTRING_t;
#define __FIIterator_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_HSTRING_USE */



#ifndef DEF___FIIterable_1_HSTRING_USE
#define DEF___FIIterable_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e2fcc7c1-3bfc-5a0b-b2b0-72e769d1cb7e"))
IIterable<HSTRING> : IIterable_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<HSTRING> __FIIterable_1_HSTRING_t;
#define __FIIterable_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    typedef enum WiFiDirectServiceConfigurationMethod : int WiFiDirectServiceConfigurationMethod;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_USE
#define DEF___FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("19889f5e-49ae-5e31-b059-083f9f1532c3"))
IIterator<enum ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceConfigurationMethod> : IIterator_impl<enum ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceConfigurationMethod>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.WiFiDirect.Services.WiFiDirectServiceConfigurationMethod>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceConfigurationMethod> __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_t;
#define __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_USE
#define DEF___FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d9773b1a-a148-58bf-9c4b-afeac9be3ab4"))
IIterable<enum ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceConfigurationMethod> : IIterable_impl<enum ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceConfigurationMethod>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.WiFiDirect.Services.WiFiDirectServiceConfigurationMethod>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceConfigurationMethod> __FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_t;
#define __FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            class EndpointPair;
        } /* Networking */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CNetworking_CIEndpointPair_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIEndpointPair_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            interface IEndpointPair;
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CIEndpointPair ABI::Windows::Networking::IEndpointPair

#endif // ____x_ABI_CWindows_CNetworking_CIEndpointPair_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CEndpointPair_USE
#define DEF___FIIterator_1_Windows__CNetworking__CEndpointPair_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c899ff9f-e6f5-5673-810c-04e2ff98704f"))
IIterator<ABI::Windows::Networking::EndpointPair*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::EndpointPair*, ABI::Windows::Networking::IEndpointPair*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.EndpointPair>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::EndpointPair*> __FIIterator_1_Windows__CNetworking__CEndpointPair_t;
#define __FIIterator_1_Windows__CNetworking__CEndpointPair ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CEndpointPair_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CEndpointPair_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CEndpointPair_USE
#define DEF___FIIterable_1_Windows__CNetworking__CEndpointPair_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d7ec83c4-a17b-51bf-8997-aa33b9102dc9"))
IIterable<ABI::Windows::Networking::EndpointPair*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::EndpointPair*, ABI::Windows::Networking::IEndpointPair*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.EndpointPair>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::EndpointPair*> __FIIterable_1_Windows__CNetworking__CEndpointPair_t;
#define __FIIterable_1_Windows__CNetworking__CEndpointPair ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CEndpointPair_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CEndpointPair_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIVectorView_1_HSTRING_USE
#define DEF___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2f13c006-a03a-5f69-b090-75a43e33423e"))
IVectorView<HSTRING> : IVectorView_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<HSTRING> __FIVectorView_1_HSTRING_t;
#define __FIVectorView_1_HSTRING ABI::Windows::Foundation::Collections::__FIVectorView_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_HSTRING_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_USE
#define DEF___FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("dc710fe1-7f04-515b-8ac1-1c5d3c0d2b28"))
IVectorView<enum ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceConfigurationMethod> : IVectorView_impl<enum ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceConfigurationMethod>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.WiFiDirect.Services.WiFiDirectServiceConfigurationMethod>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceConfigurationMethod> __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_t;
#define __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CEndpointPair_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CEndpointPair_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8780a851-6d48-5006-9288-81f3d7045a96"))
IVectorView<ABI::Windows::Networking::EndpointPair*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::EndpointPair*, ABI::Windows::Networking::IEndpointPair*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.EndpointPair>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::EndpointPair*> __FIVectorView_1_Windows__CNetworking__CEndpointPair_t;
#define __FIVectorView_1_Windows__CNetworking__CEndpointPair ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CEndpointPair_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CEndpointPair_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIVector_1_HSTRING_USE
#define DEF___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("98b9acc1-4b56-532e-ac73-03d5291cca90"))
IVector<HSTRING> : IVector_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<HSTRING> __FIVector_1_HSTRING_t;
#define __FIVector_1_HSTRING ABI::Windows::Foundation::Collections::__FIVector_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_HSTRING_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_USE
#define DEF___FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f6a6f91c-0579-565d-be07-4538a55690be"))
IVector<enum ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceConfigurationMethod> : IVector_impl<enum ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceConfigurationMethod>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Devices.WiFiDirect.Services.WiFiDirectServiceConfigurationMethod>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<enum ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceConfigurationMethod> __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_t;
#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    class WiFiDirectServiceSessionDeferredEventArgs;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fc3dfc2c-9cfa-5822-ba3f-ff3afb65777e"))
ITypedEventHandler<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectService*, ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSessionDeferredEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectService*, ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectService*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSessionDeferredEventArgs*, ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceSessionDeferredEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.WiFiDirect.Services.WiFiDirectService, Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionDeferredEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectService*, ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSessionDeferredEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    class WiFiDirectServiceAdvertiser;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("67fc3121-c1a0-5c23-af58-ecb7f2a7d773"))
ITypedEventHandler<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceAdvertiser*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceAdvertiser*, ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceAdvertiser*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertiser, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceAdvertiser*, IInspectable*> __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    class WiFiDirectServiceAutoAcceptSessionConnectedEventArgs;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3be2d508-a856-5c09-9998-522597b44b07"))
ITypedEventHandler<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceAdvertiser*, ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceAutoAcceptSessionConnectedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceAdvertiser*, ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceAdvertiser*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceAutoAcceptSessionConnectedEventArgs*, ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertiser, Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAutoAcceptSessionConnectedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceAdvertiser*, ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceAutoAcceptSessionConnectedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    class WiFiDirectServiceSessionRequestedEventArgs;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cb98fd74-871d-5730-91fe-81ef947fe78f"))
ITypedEventHandler<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceAdvertiser*, ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSessionRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceAdvertiser*, ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceAdvertiser*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSessionRequestedEventArgs*, ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceSessionRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertiser, Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceAdvertiser*, ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSessionRequestedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("10c33301-e31c-5cce-b2a0-c1dc2d8d0e13"))
ITypedEventHandler<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSession*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSession*, ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceSession*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSession, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSession*, IInspectable*> __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    class WiFiDirectServiceRemotePortAddedEventArgs;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8326a337-3c19-57a7-80ec-cca2ea62ef12"))
ITypedEventHandler<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSession*, ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceRemotePortAddedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSession*, ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceSession*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceRemotePortAddedEventArgs*, ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceRemotePortAddedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSession, Windows.Devices.WiFiDirect.Services.WiFiDirectServiceRemotePortAddedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSession*, ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceRemotePortAddedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceInformation;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceInformation;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation ABI::Windows::Devices::Enumeration::IDeviceInformation

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IAsyncAction;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIAsyncAction ABI::Windows::Foundation::IAsyncAction

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IClosable;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIClosable ABI::Windows::Foundation::IClosable

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class DatagramSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IDatagramSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket ABI::Windows::Networking::Sockets::IDatagramSocket

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class StreamSocketListener;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocketListener;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener ABI::Windows::Networking::Sockets::IStreamSocketListener

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IBuffer;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer ABI::Windows::Storage::Streams::IBuffer

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    typedef enum WiFiDirectServiceAdvertisementStatus : int WiFiDirectServiceAdvertisementStatus;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    typedef enum WiFiDirectServiceError : int WiFiDirectServiceError;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    typedef enum WiFiDirectServiceIPProtocol : int WiFiDirectServiceIPProtocol;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    typedef enum WiFiDirectServiceSessionErrorStatus : int WiFiDirectServiceSessionErrorStatus;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    typedef enum WiFiDirectServiceSessionStatus : int WiFiDirectServiceSessionStatus;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    typedef enum WiFiDirectServiceStatus : int WiFiDirectServiceStatus;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    class WiFiDirectServiceSessionRequest;
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertisementStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    enum WiFiDirectServiceAdvertisementStatus : int
                    {
                        WiFiDirectServiceAdvertisementStatus_Created = 0,
                        WiFiDirectServiceAdvertisementStatus_Started = 1,
                        WiFiDirectServiceAdvertisementStatus_Stopped = 2,
                        WiFiDirectServiceAdvertisementStatus_Aborted = 3,
                    };
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFiDirect.Services.WiFiDirectServiceConfigurationMethod
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    enum WiFiDirectServiceConfigurationMethod : int
                    {
                        WiFiDirectServiceConfigurationMethod_Default = 0,
                        WiFiDirectServiceConfigurationMethod_PinDisplay = 1,
                        WiFiDirectServiceConfigurationMethod_PinEntry = 2,
                    };
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFiDirect.Services.WiFiDirectServiceError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    enum WiFiDirectServiceError : int
                    {
                        WiFiDirectServiceError_Success = 0,
                        WiFiDirectServiceError_RadioNotAvailable = 1,
                        WiFiDirectServiceError_ResourceInUse = 2,
                        WiFiDirectServiceError_UnsupportedHardware = 3,
                        WiFiDirectServiceError_NoHardware = 4,
                    };
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFiDirect.Services.WiFiDirectServiceIPProtocol
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    enum WiFiDirectServiceIPProtocol : int
                    {
                        WiFiDirectServiceIPProtocol_Tcp = 6,
                        WiFiDirectServiceIPProtocol_Udp = 17,
                    };
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionErrorStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    enum WiFiDirectServiceSessionErrorStatus : int
                    {
                        WiFiDirectServiceSessionErrorStatus_Ok = 0,
                        WiFiDirectServiceSessionErrorStatus_Disassociated = 1,
                        WiFiDirectServiceSessionErrorStatus_LocalClose = 2,
                        WiFiDirectServiceSessionErrorStatus_RemoteClose = 3,
                        WiFiDirectServiceSessionErrorStatus_SystemFailure = 4,
                        WiFiDirectServiceSessionErrorStatus_NoResponseFromRemote = 5,
                    };
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    enum WiFiDirectServiceSessionStatus : int
                    {
                        WiFiDirectServiceSessionStatus_Closed = 0,
                        WiFiDirectServiceSessionStatus_Initiated = 1,
                        WiFiDirectServiceSessionStatus_Requested = 2,
                        WiFiDirectServiceSessionStatus_Open = 3,
                    };
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFiDirect.Services.WiFiDirectServiceStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    enum WiFiDirectServiceStatus : int
                    {
                        WiFiDirectServiceStatus_Available = 0,
                        WiFiDirectServiceStatus_Busy = 1,
                        WiFiDirectServiceStatus_Custom = 2,
                    };
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectService[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectService";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    MIDL_INTERFACE("50aabbb8-5f71-45ec-84f1-a1e4fc7879a3")
                    IWiFiDirectService : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_RemoteServiceInfo(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SupportedConfigurationMethods(
                            __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PreferGroupOwnerMode(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PreferGroupOwnerMode(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SessionInfo(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SessionInfo(
                            ABI::Windows::Storage::Streams::IBuffer* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceError(
                            ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceError* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SessionDeferred(
                            __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SessionDeferred(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetProvisioningInfoAsync(
                            ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceConfigurationMethod selectedConfigurationMethod,
                            __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ConnectAsync(
                            __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ConnectAsyncWithPin(
                            HSTRING pin,
                            __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWiFiDirectService = __uuidof(IWiFiDirectService);
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAdvertiser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertiser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceAdvertiser[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAdvertiser";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    MIDL_INTERFACE("a4aa1ee1-9d8f-4f4f-93ee-7ddea2e37f46")
                    IWiFiDirectServiceAdvertiser : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceNamePrefixes(
                            __FIVector_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceInfo(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ServiceInfo(
                            ABI::Windows::Storage::Streams::IBuffer* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AutoAcceptSession(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AutoAcceptSession(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PreferGroupOwnerMode(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PreferGroupOwnerMode(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PreferredConfigurationMethods(
                            __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceStatus(
                            ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ServiceStatus(
                            ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceStatus value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CustomServiceStatusCode(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CustomServiceStatusCode(
                            UINT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DeferredSessionInfo(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DeferredSessionInfo(
                            ABI::Windows::Storage::Streams::IBuffer* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AdvertisementStatus(
                            ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceAdvertisementStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceError(
                            ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceError* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SessionRequested(
                            __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SessionRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_AutoAcceptSessionConnected(
                            __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_AutoAcceptSessionConnected(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_AdvertisementStatusChanged(
                            __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_AdvertisementStatusChanged(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ConnectAsync(
                            ABI::Windows::Devices::Enumeration::IDeviceInformation* deviceInfo,
                            __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ConnectAsyncWithPin(
                            ABI::Windows::Devices::Enumeration::IDeviceInformation* deviceInfo,
                            HSTRING pin,
                            __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWiFiDirectServiceAdvertiser = __uuidof(IWiFiDirectServiceAdvertiser);
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAdvertiserFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertiser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceAdvertiserFactory[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAdvertiserFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    MIDL_INTERFACE("3106ac0d-b446-4f13-9f9a-8ae925feba2b")
                    IWiFiDirectServiceAdvertiserFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateWiFiDirectServiceAdvertiser(
                            HSTRING serviceName,
                            ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceAdvertiser** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWiFiDirectServiceAdvertiserFactory = __uuidof(IWiFiDirectServiceAdvertiserFactory);
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAutoAcceptSessionConnectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    MIDL_INTERFACE("dcd9e01e-83df-43e5-8f43-cbe8479e84eb")
                    IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Session(
                            ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceSession** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SessionInfo(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs = __uuidof(IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs);
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceProvisioningInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectServiceProvisioningInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceProvisioningInfo[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceProvisioningInfo";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    MIDL_INTERFACE("8bdb7cfe-97d9-45a2-8e99-db50910fb6a6")
                    IWiFiDirectServiceProvisioningInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SelectedConfigurationMethod(
                            ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceConfigurationMethod* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsGroupFormationNeeded(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWiFiDirectServiceProvisioningInfo = __uuidof(IWiFiDirectServiceProvisioningInfo);
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceRemotePortAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectServiceRemotePortAddedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceRemotePortAddedEventArgs[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceRemotePortAddedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    MIDL_INTERFACE("d4cebac1-3fd3-4f0e-b7bd-782906f44411")
                    IWiFiDirectServiceRemotePortAddedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EndpointPairs(
                            __FIVectorView_1_Windows__CNetworking__CEndpointPair** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Protocol(
                            ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceIPProtocol* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWiFiDirectServiceRemotePortAddedEventArgs = __uuidof(IWiFiDirectServiceRemotePortAddedEventArgs);
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSession
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceSession[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSession";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    MIDL_INTERFACE("81142163-e426-47cb-8640-e1b3588bf26f")
                    IWiFiDirectServiceSession : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSessionStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ErrorStatus(
                            ABI::Windows::Devices::WiFiDirect::Services::WiFiDirectServiceSessionErrorStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SessionId(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AdvertisementId(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceAddress(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SessionAddress(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetConnectionEndpointPairs(
                            __FIVectorView_1_Windows__CNetworking__CEndpointPair** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SessionStatusChanged(
                            __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SessionStatusChanged(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AddStreamSocketListenerAsync(
                            ABI::Windows::Networking::Sockets::IStreamSocketListener* value,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AddDatagramSocketAsync(
                            ABI::Windows::Networking::Sockets::IDatagramSocket* value,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_RemotePortAdded(
                            __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_RemotePortAdded(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWiFiDirectServiceSession = __uuidof(IWiFiDirectServiceSession);
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionDeferredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionDeferredEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceSessionDeferredEventArgs[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionDeferredEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    MIDL_INTERFACE("8dfc197f-1201-4f1f-b6f4-5df1b7b9fb2e")
                    IWiFiDirectServiceSessionDeferredEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DeferredSessionInfo(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWiFiDirectServiceSessionDeferredEventArgs = __uuidof(IWiFiDirectServiceSessionDeferredEventArgs);
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequest
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceSessionRequest[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionRequest";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    MIDL_INTERFACE("a0e27c8b-50cb-4a58-9bcf-e472b99fba04")
                    IWiFiDirectServiceSessionRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DeviceInformation(
                            ABI::Windows::Devices::Enumeration::IDeviceInformation** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ProvisioningInfo(
                            ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceProvisioningInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SessionInfo(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWiFiDirectServiceSessionRequest = __uuidof(IWiFiDirectServiceSessionRequest);
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceSessionRequestedEventArgs[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    MIDL_INTERFACE("74bdcc11-53d6-4999-b4f8-6c8ecc1771e7")
                    IWiFiDirectServiceSessionRequestedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetSessionRequest(
                            ABI::Windows::Devices::WiFiDirect::Services::IWiFiDirectServiceSessionRequest** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWiFiDirectServiceSessionRequestedEventArgs = __uuidof(IWiFiDirectServiceSessionRequestedEventArgs);
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceStatics[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFiDirect {
                namespace Services {
                    MIDL_INTERFACE("7db40045-fd74-4688-b725-5dce86acf233")
                    IWiFiDirectServiceStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetSelector(
                            HSTRING serviceName,
                            HSTRING* serviceSelector
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetSelectorWithFilter(
                            HSTRING serviceName,
                            ABI::Windows::Storage::Streams::IBuffer* serviceInfoFilter,
                            HSTRING* serviceSelector
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                            HSTRING deviceId,
                            __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService** asyncOp
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWiFiDirectServiceStatics = __uuidof(IWiFiDirectServiceStatics);
                } /* Services */
            } /* WiFiDirect */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFiDirect.Services.WiFiDirectService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFiDirect.Services.IWiFiDirectService ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectService_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectService_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFiDirect_Services_WiFiDirectService[] = L"Windows.Devices.WiFiDirect.Services.WiFiDirectService";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertiser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAdvertiserFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAdvertiser ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceAdvertiser_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceAdvertiser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceAdvertiser[] = L"Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertiser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAutoAcceptSessionConnectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceAutoAcceptSessionConnectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceAutoAcceptSessionConnectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceAutoAcceptSessionConnectedEventArgs[] = L"Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAutoAcceptSessionConnectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFiDirect.Services.WiFiDirectServiceProvisioningInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceProvisioningInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceProvisioningInfo_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceProvisioningInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceProvisioningInfo[] = L"Windows.Devices.WiFiDirect.Services.WiFiDirectServiceProvisioningInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFiDirect.Services.WiFiDirectServiceRemotePortAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceRemotePortAddedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceRemotePortAddedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceRemotePortAddedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceRemotePortAddedEventArgs[] = L"Windows.Devices.WiFiDirect.Services.WiFiDirectServiceRemotePortAddedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSession ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSession_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSession[] = L"Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionDeferredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionDeferredEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSessionDeferredEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSessionDeferredEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSessionDeferredEventArgs[] = L"Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionDeferredEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionRequest ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSessionRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSessionRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSessionRequest[] = L"Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSessionRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSessionRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSessionRequestedEventArgs[] = L"Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService;

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser;

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory;

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo;

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession;

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest;

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics;

#endif // ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService* This,
        __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService* This,
        __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo* This,
        __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfoVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo* This,
        __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfoVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession* This,
        __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession* This,
        __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIIterator_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1_HSTRING __FIIterator_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_HSTRING;

typedef struct __FIIterator_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_HSTRINGVtbl;

interface __FIIterator_1_HSTRING
{
    CONST_VTBL struct __FIIterator_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1_HSTRING __FIIterable_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_HSTRING;

typedef struct __FIIterable_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_HSTRING* This,
        __FIIterator_1_HSTRING** result);

    END_INTERFACE
} __FIIterable_1_HSTRINGVtbl;

interface __FIIterable_1_HSTRING
{
    CONST_VTBL struct __FIIterable_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_HSTRING_INTERFACE_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceConfigurationMethod __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceConfigurationMethod;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod;

typedef struct __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethodVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceConfigurationMethod* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceConfigurationMethod* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethodVtbl;

interface __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethodVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod __FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod;

typedef struct __FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethodVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        __FIIterator_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethodVtbl;

interface __FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethodVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CNetworking_CIEndpointPair_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIEndpointPair_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CIEndpointPair __x_ABI_CWindows_CNetworking_CIEndpointPair;

#endif // ____x_ABI_CWindows_CNetworking_CIEndpointPair_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CEndpointPair_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CEndpointPair_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CEndpointPair __FIIterator_1_Windows__CNetworking__CEndpointPair;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CEndpointPair;

typedef struct __FIIterator_1_Windows__CNetworking__CEndpointPairVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CEndpointPair* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CEndpointPair* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CEndpointPair* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CEndpointPair* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CEndpointPair* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CEndpointPair* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CEndpointPair* This,
        __x_ABI_CWindows_CNetworking_CIEndpointPair** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CEndpointPair* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CEndpointPair* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CEndpointPair* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CIEndpointPair** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CEndpointPairVtbl;

interface __FIIterator_1_Windows__CNetworking__CEndpointPair
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CEndpointPairVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CEndpointPair_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CEndpointPair_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CEndpointPair_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CEndpointPair_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CEndpointPair_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CEndpointPair_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CEndpointPair_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CEndpointPair_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CEndpointPair_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CEndpointPair_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CEndpointPair_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CEndpointPair_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CEndpointPair_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CEndpointPair __FIIterable_1_Windows__CNetworking__CEndpointPair;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CEndpointPair;

typedef struct __FIIterable_1_Windows__CNetworking__CEndpointPairVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CEndpointPair* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CEndpointPair* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CEndpointPair* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CEndpointPair* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CEndpointPair* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CEndpointPair* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CEndpointPair* This,
        __FIIterator_1_Windows__CNetworking__CEndpointPair** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CEndpointPairVtbl;

interface __FIIterable_1_Windows__CNetworking__CEndpointPair
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CEndpointPairVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CEndpointPair_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CEndpointPair_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CEndpointPair_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CEndpointPair_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CEndpointPair_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CEndpointPair_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CEndpointPair_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CEndpointPair_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_HSTRING __FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_HSTRING;

typedef struct __FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_HSTRINGVtbl;

interface __FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod;

typedef struct __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethodVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        UINT32 index,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceConfigurationMethod* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceConfigurationMethod value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceConfigurationMethod* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethodVtbl;

interface __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethodVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CEndpointPair_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CEndpointPair_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CEndpointPair __FIVectorView_1_Windows__CNetworking__CEndpointPair;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CEndpointPair;

typedef struct __FIVectorView_1_Windows__CNetworking__CEndpointPairVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CEndpointPair* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CEndpointPair* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CEndpointPair* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CEndpointPair* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CEndpointPair* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CEndpointPair* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CEndpointPair* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CIEndpointPair** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CEndpointPair* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CEndpointPair* This,
        __x_ABI_CWindows_CNetworking_CIEndpointPair* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CEndpointPair* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CIEndpointPair** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CEndpointPairVtbl;

interface __FIVectorView_1_Windows__CNetworking__CEndpointPair
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CEndpointPairVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CEndpointPair_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CEndpointPair_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CEndpointPair_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CEndpointPair_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CEndpointPair_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CEndpointPair_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CEndpointPair_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CEndpointPair_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CEndpointPair_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CEndpointPair_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CEndpointPair_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVector_1_HSTRING __FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_HSTRING;

typedef struct __FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_HSTRING* This,
        __FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_HSTRING* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_HSTRING* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items);

    END_INTERFACE
} __FIVector_1_HSTRINGVtbl;

interface __FIVector_1_HSTRING
{
    CONST_VTBL struct __FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_HSTRING_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_HSTRING_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_HSTRING_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_HSTRING_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_HSTRING_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_HSTRING_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod;

typedef struct __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethodVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        UINT32 index,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceConfigurationMethod* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceConfigurationMethod value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        UINT32 index,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceConfigurationMethod value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        UINT32 index,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceConfigurationMethod value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceConfigurationMethod value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceConfigurationMethod* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceConfigurationMethod* items);

    END_INTERFACE
} __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethodVtbl;

interface __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod
{
    CONST_VTBL struct __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethodVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs* This,
        __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* sender,
        __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable* This,
        __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs* This,
        __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* sender,
        __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs* This,
        __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* sender,
        __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable* This,
        __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs* This,
        __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* sender,
        __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceAdvertisementStatus __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceAdvertisementStatus;

typedef enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceError __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceError;

typedef enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceIPProtocol __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceIPProtocol;

typedef enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceSessionErrorStatus __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceSessionErrorStatus;

typedef enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceSessionStatus __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceSessionStatus;

typedef enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceStatus __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceStatus;

/*
 *
 * Struct Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertisementStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceAdvertisementStatus
{
    WiFiDirectServiceAdvertisementStatus_Created = 0,
    WiFiDirectServiceAdvertisementStatus_Started = 1,
    WiFiDirectServiceAdvertisementStatus_Stopped = 2,
    WiFiDirectServiceAdvertisementStatus_Aborted = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFiDirect.Services.WiFiDirectServiceConfigurationMethod
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceConfigurationMethod
{
    WiFiDirectServiceConfigurationMethod_Default = 0,
    WiFiDirectServiceConfigurationMethod_PinDisplay = 1,
    WiFiDirectServiceConfigurationMethod_PinEntry = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFiDirect.Services.WiFiDirectServiceError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceError
{
    WiFiDirectServiceError_Success = 0,
    WiFiDirectServiceError_RadioNotAvailable = 1,
    WiFiDirectServiceError_ResourceInUse = 2,
    WiFiDirectServiceError_UnsupportedHardware = 3,
    WiFiDirectServiceError_NoHardware = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFiDirect.Services.WiFiDirectServiceIPProtocol
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceIPProtocol
{
    WiFiDirectServiceIPProtocol_Tcp = 6,
    WiFiDirectServiceIPProtocol_Udp = 17,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionErrorStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceSessionErrorStatus
{
    WiFiDirectServiceSessionErrorStatus_Ok = 0,
    WiFiDirectServiceSessionErrorStatus_Disassociated = 1,
    WiFiDirectServiceSessionErrorStatus_LocalClose = 2,
    WiFiDirectServiceSessionErrorStatus_RemoteClose = 3,
    WiFiDirectServiceSessionErrorStatus_SystemFailure = 4,
    WiFiDirectServiceSessionErrorStatus_NoResponseFromRemote = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceSessionStatus
{
    WiFiDirectServiceSessionStatus_Closed = 0,
    WiFiDirectServiceSessionStatus_Initiated = 1,
    WiFiDirectServiceSessionStatus_Requested = 2,
    WiFiDirectServiceSessionStatus_Open = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFiDirect.Services.WiFiDirectServiceStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceStatus
{
    WiFiDirectServiceStatus_Available = 0,
    WiFiDirectServiceStatus_Busy = 1,
    WiFiDirectServiceStatus_Custom = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectService[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectService";
typedef struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RemoteServiceInfo)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedConfigurationMethods)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* This,
        __FIVectorView_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod** value);
    HRESULT (STDMETHODCALLTYPE* get_PreferGroupOwnerMode)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_PreferGroupOwnerMode)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_SessionInfo)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* put_SessionInfo)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceError)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* This,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceError* value);
    HRESULT (STDMETHODCALLTYPE* add_SessionDeferred)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* This,
        __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionDeferredEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SessionDeferred)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* GetProvisioningInfoAsync)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* This,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceConfigurationMethod selectedConfigurationMethod,
        __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceProvisioningInfo** result);
    HRESULT (STDMETHODCALLTYPE* ConnectAsync)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* This,
        __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession** result);
    HRESULT (STDMETHODCALLTYPE* ConnectAsyncWithPin)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService* This,
        HSTRING pin,
        __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceVtbl;

interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_get_RemoteServiceInfo(This, value) \
    ((This)->lpVtbl->get_RemoteServiceInfo(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_get_SupportedConfigurationMethods(This, value) \
    ((This)->lpVtbl->get_SupportedConfigurationMethods(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_get_PreferGroupOwnerMode(This, value) \
    ((This)->lpVtbl->get_PreferGroupOwnerMode(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_put_PreferGroupOwnerMode(This, value) \
    ((This)->lpVtbl->put_PreferGroupOwnerMode(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_get_SessionInfo(This, value) \
    ((This)->lpVtbl->get_SessionInfo(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_put_SessionInfo(This, value) \
    ((This)->lpVtbl->put_SessionInfo(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_get_ServiceError(This, value) \
    ((This)->lpVtbl->get_ServiceError(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_add_SessionDeferred(This, handler, token) \
    ((This)->lpVtbl->add_SessionDeferred(This, handler, token))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_remove_SessionDeferred(This, token) \
    ((This)->lpVtbl->remove_SessionDeferred(This, token))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_GetProvisioningInfoAsync(This, selectedConfigurationMethod, result) \
    ((This)->lpVtbl->GetProvisioningInfoAsync(This, selectedConfigurationMethod, result))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_ConnectAsync(This, result) \
    ((This)->lpVtbl->ConnectAsync(This, result))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_ConnectAsyncWithPin(This, pin, result) \
    ((This)->lpVtbl->ConnectAsyncWithPin(This, pin, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectService_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAdvertiser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertiser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceAdvertiser[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAdvertiser";
typedef struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ServiceName)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceNamePrefixes)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceInfo)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* put_ServiceInfo)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* value);
    HRESULT (STDMETHODCALLTYPE* get_AutoAcceptSession)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AutoAcceptSession)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_PreferGroupOwnerMode)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_PreferGroupOwnerMode)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_PreferredConfigurationMethods)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        __FIVector_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceConfigurationMethod** value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceStatus)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceStatus* value);
    HRESULT (STDMETHODCALLTYPE* put_ServiceStatus)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceStatus value);
    HRESULT (STDMETHODCALLTYPE* get_CustomServiceStatusCode)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_CustomServiceStatusCode)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_DeferredSessionInfo)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* put_DeferredSessionInfo)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* value);
    HRESULT (STDMETHODCALLTYPE* get_AdvertisementStatus)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceAdvertisementStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceError)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceError* value);
    HRESULT (STDMETHODCALLTYPE* add_SessionRequested)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSessionRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SessionRequested)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_AutoAcceptSessionConnected)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAutoAcceptSessionConnectedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AutoAcceptSessionConnected)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_AdvertisementStatusChanged)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceAdvertiser_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AdvertisementStatusChanged)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* ConnectAsync)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* deviceInfo,
        __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession** result);
    HRESULT (STDMETHODCALLTYPE* ConnectAsyncWithPin)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* deviceInfo,
        HSTRING pin,
        __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession** result);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser* This);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserVtbl;

interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_get_ServiceName(This, value) \
    ((This)->lpVtbl->get_ServiceName(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_get_ServiceNamePrefixes(This, value) \
    ((This)->lpVtbl->get_ServiceNamePrefixes(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_get_ServiceInfo(This, value) \
    ((This)->lpVtbl->get_ServiceInfo(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_put_ServiceInfo(This, value) \
    ((This)->lpVtbl->put_ServiceInfo(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_get_AutoAcceptSession(This, value) \
    ((This)->lpVtbl->get_AutoAcceptSession(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_put_AutoAcceptSession(This, value) \
    ((This)->lpVtbl->put_AutoAcceptSession(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_get_PreferGroupOwnerMode(This, value) \
    ((This)->lpVtbl->get_PreferGroupOwnerMode(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_put_PreferGroupOwnerMode(This, value) \
    ((This)->lpVtbl->put_PreferGroupOwnerMode(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_get_PreferredConfigurationMethods(This, value) \
    ((This)->lpVtbl->get_PreferredConfigurationMethods(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_get_ServiceStatus(This, value) \
    ((This)->lpVtbl->get_ServiceStatus(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_put_ServiceStatus(This, value) \
    ((This)->lpVtbl->put_ServiceStatus(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_get_CustomServiceStatusCode(This, value) \
    ((This)->lpVtbl->get_CustomServiceStatusCode(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_put_CustomServiceStatusCode(This, value) \
    ((This)->lpVtbl->put_CustomServiceStatusCode(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_get_DeferredSessionInfo(This, value) \
    ((This)->lpVtbl->get_DeferredSessionInfo(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_put_DeferredSessionInfo(This, value) \
    ((This)->lpVtbl->put_DeferredSessionInfo(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_get_AdvertisementStatus(This, value) \
    ((This)->lpVtbl->get_AdvertisementStatus(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_get_ServiceError(This, value) \
    ((This)->lpVtbl->get_ServiceError(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_add_SessionRequested(This, handler, token) \
    ((This)->lpVtbl->add_SessionRequested(This, handler, token))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_remove_SessionRequested(This, token) \
    ((This)->lpVtbl->remove_SessionRequested(This, token))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_add_AutoAcceptSessionConnected(This, handler, token) \
    ((This)->lpVtbl->add_AutoAcceptSessionConnected(This, handler, token))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_remove_AutoAcceptSessionConnected(This, token) \
    ((This)->lpVtbl->remove_AutoAcceptSessionConnected(This, token))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_add_AdvertisementStatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_AdvertisementStatusChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_remove_AdvertisementStatusChanged(This, token) \
    ((This)->lpVtbl->remove_AdvertisementStatusChanged(This, token))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_ConnectAsync(This, deviceInfo, result) \
    ((This)->lpVtbl->ConnectAsync(This, deviceInfo, result))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_ConnectAsyncWithPin(This, deviceInfo, pin, result) \
    ((This)->lpVtbl->ConnectAsyncWithPin(This, deviceInfo, pin, result))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAdvertiserFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertiser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceAdvertiserFactory[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAdvertiserFactory";
typedef struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWiFiDirectServiceAdvertiser)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory* This,
        HSTRING serviceName,
        __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiser** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_CreateWiFiDirectServiceAdvertiser(This, serviceName, result) \
    ((This)->lpVtbl->CreateWiFiDirectServiceAdvertiser(This, serviceName, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAdvertiserFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAutoAcceptSessionConnectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Session)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs* This,
        __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession** value);
    HRESULT (STDMETHODCALLTYPE* get_SessionInfo)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_get_Session(This, value) \
    ((This)->lpVtbl->get_Session(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_get_SessionInfo(This, value) \
    ((This)->lpVtbl->get_SessionInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceProvisioningInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectServiceProvisioningInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceProvisioningInfo[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceProvisioningInfo";
typedef struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SelectedConfigurationMethod)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo* This,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceConfigurationMethod* value);
    HRESULT (STDMETHODCALLTYPE* get_IsGroupFormationNeeded)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfoVtbl;

interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_get_SelectedConfigurationMethod(This, value) \
    ((This)->lpVtbl->get_SelectedConfigurationMethod(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_get_IsGroupFormationNeeded(This, value) \
    ((This)->lpVtbl->get_IsGroupFormationNeeded(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceRemotePortAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectServiceRemotePortAddedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceRemotePortAddedEventArgs[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceRemotePortAddedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EndpointPairs)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs* This,
        __FIVectorView_1_Windows__CNetworking__CEndpointPair** value);
    HRESULT (STDMETHODCALLTYPE* get_Protocol)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceIPProtocol* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_get_EndpointPairs(This, value) \
    ((This)->lpVtbl->get_EndpointPairs(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_get_Protocol(This, value) \
    ((This)->lpVtbl->get_Protocol(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceRemotePortAddedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSession
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceSession[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSession";
typedef struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ServiceName)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceSessionStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_ErrorStatus)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This,
        enum __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CWiFiDirectServiceSessionErrorStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_SessionId)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_AdvertisementId)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceAddress)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SessionAddress)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetConnectionEndpointPairs)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This,
        __FIVectorView_1_Windows__CNetworking__CEndpointPair** value);
    HRESULT (STDMETHODCALLTYPE* add_SessionStatusChanged)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This,
        __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SessionStatusChanged)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* AddStreamSocketListenerAsync)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener* value,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* AddDatagramSocketAsync)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* value,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* add_RemotePortAdded)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This,
        __FITypedEventHandler_2_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceSession_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectServiceRemotePortAddedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RemotePortAdded)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionVtbl;

interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_get_ServiceName(This, value) \
    ((This)->lpVtbl->get_ServiceName(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_get_ErrorStatus(This, value) \
    ((This)->lpVtbl->get_ErrorStatus(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_get_SessionId(This, value) \
    ((This)->lpVtbl->get_SessionId(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_get_AdvertisementId(This, value) \
    ((This)->lpVtbl->get_AdvertisementId(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_get_ServiceAddress(This, value) \
    ((This)->lpVtbl->get_ServiceAddress(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_get_SessionAddress(This, value) \
    ((This)->lpVtbl->get_SessionAddress(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_GetConnectionEndpointPairs(This, value) \
    ((This)->lpVtbl->GetConnectionEndpointPairs(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_add_SessionStatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_SessionStatusChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_remove_SessionStatusChanged(This, token) \
    ((This)->lpVtbl->remove_SessionStatusChanged(This, token))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_AddStreamSocketListenerAsync(This, value, result) \
    ((This)->lpVtbl->AddStreamSocketListenerAsync(This, value, result))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_AddDatagramSocketAsync(This, value, result) \
    ((This)->lpVtbl->AddDatagramSocketAsync(This, value, result))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_add_RemotePortAdded(This, handler, token) \
    ((This)->lpVtbl->add_RemotePortAdded(This, handler, token))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_remove_RemotePortAdded(This, token) \
    ((This)->lpVtbl->remove_RemotePortAdded(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionDeferredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionDeferredEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceSessionDeferredEventArgs[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionDeferredEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeferredSessionInfo)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_get_DeferredSessionInfo(This, value) \
    ((This)->lpVtbl->get_DeferredSessionInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionDeferredEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequest
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceSessionRequest[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionRequest";
typedef struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceInformation)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** value);
    HRESULT (STDMETHODCALLTYPE* get_ProvisioningInfo)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest* This,
        __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceProvisioningInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_SessionInfo)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestVtbl;

interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_get_DeviceInformation(This, value) \
    ((This)->lpVtbl->get_DeviceInformation(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_get_ProvisioningInfo(This, value) \
    ((This)->lpVtbl->get_ProvisioningInfo(This, value))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_get_SessionInfo(This, value) \
    ((This)->lpVtbl->get_SessionInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceSessionRequestedEventArgs[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionRequestedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetSessionRequest)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs* This,
        __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequest** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_GetSessionRequest(This, value) \
    ((This)->lpVtbl->GetSessionRequest(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceSessionRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFiDirect.Services.WiFiDirectService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFiDirect_Services_IWiFiDirectServiceStatics[] = L"Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceStatics";
typedef struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetSelector)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics* This,
        HSTRING serviceName,
        HSTRING* serviceSelector);
    HRESULT (STDMETHODCALLTYPE* GetSelectorWithFilter)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics* This,
        HSTRING serviceName,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* serviceInfoFilter,
        HSTRING* serviceSelector);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics* This,
        HSTRING deviceId,
        __FIAsyncOperation_1_Windows__CDevices__CWiFiDirect__CServices__CWiFiDirectService** asyncOp);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_GetSelector(This, serviceName, serviceSelector) \
    ((This)->lpVtbl->GetSelector(This, serviceName, serviceSelector))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_GetSelectorWithFilter(This, serviceName, serviceInfoFilter, serviceSelector) \
    ((This)->lpVtbl->GetSelectorWithFilter(This, serviceName, serviceInfoFilter, serviceSelector))

#define __x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_FromIdAsync(This, deviceId, asyncOp) \
    ((This)->lpVtbl->FromIdAsync(This, deviceId, asyncOp))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFiDirect_CServices_CIWiFiDirectServiceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFiDirect.Services.WiFiDirectService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFiDirect.Services.IWiFiDirectService ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectService_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectService_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFiDirect_Services_WiFiDirectService[] = L"Windows.Devices.WiFiDirect.Services.WiFiDirectService";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertiser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAdvertiserFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAdvertiser ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceAdvertiser_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceAdvertiser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceAdvertiser[] = L"Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertiser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAutoAcceptSessionConnectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceAutoAcceptSessionConnectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceAutoAcceptSessionConnectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceAutoAcceptSessionConnectedEventArgs[] = L"Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAutoAcceptSessionConnectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFiDirect.Services.WiFiDirectServiceProvisioningInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceProvisioningInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceProvisioningInfo_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceProvisioningInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceProvisioningInfo[] = L"Windows.Devices.WiFiDirect.Services.WiFiDirectServiceProvisioningInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFiDirect.Services.WiFiDirectServiceRemotePortAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceRemotePortAddedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceRemotePortAddedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceRemotePortAddedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceRemotePortAddedEventArgs[] = L"Windows.Devices.WiFiDirect.Services.WiFiDirectServiceRemotePortAddedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSession ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSession_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSession[] = L"Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionDeferredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionDeferredEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSessionDeferredEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSessionDeferredEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSessionDeferredEventArgs[] = L"Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionDeferredEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionRequest ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSessionRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSessionRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSessionRequest[] = L"Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSessionRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSessionRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFiDirect_Services_WiFiDirectServiceSessionRequestedEventArgs[] = L"Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Edevices2Ewifidirect2Eservices_p_h__

#endif // __windows2Edevices2Ewifidirect2Eservices_h__
