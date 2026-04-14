
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
#ifndef __windows2Edevices2Ewifi_h__
#define __windows2Edevices2Ewifi_h__
#ifndef __windows2Edevices2Ewifi_p_h__
#define __windows2Edevices2Ewifi_p_h__


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

#if !defined(WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION)
#define WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Networking.Connectivity.h"
#include "Windows.Security.Credentials.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                interface IWiFiAdapter;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter ABI::Windows::Devices::WiFi::IWiFiAdapter

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                interface IWiFiAdapter2;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2 ABI::Windows::Devices::WiFi::IWiFiAdapter2

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                interface IWiFiAdapterStatics;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics ABI::Windows::Devices::WiFi::IWiFiAdapterStatics

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                interface IWiFiAvailableNetwork;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork ABI::Windows::Devices::WiFi::IWiFiAvailableNetwork

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                interface IWiFiConnectionResult;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult ABI::Windows::Devices::WiFi::IWiFiConnectionResult

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                interface IWiFiNetworkReport;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport ABI::Windows::Devices::WiFi::IWiFiNetworkReport

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                interface IWiFiOnDemandHotspotConnectTriggerDetails;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails ABI::Windows::Devices::WiFi::IWiFiOnDemandHotspotConnectTriggerDetails

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                interface IWiFiOnDemandHotspotConnectionResult;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult ABI::Windows::Devices::WiFi::IWiFiOnDemandHotspotConnectionResult

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                interface IWiFiOnDemandHotspotNetwork;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork ABI::Windows::Devices::WiFi::IWiFiOnDemandHotspotNetwork

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                interface IWiFiOnDemandHotspotNetworkProperties;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties ABI::Windows::Devices::WiFi::IWiFiOnDemandHotspotNetworkProperties

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                interface IWiFiOnDemandHotspotNetworkStatics;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics ABI::Windows::Devices::WiFi::IWiFiOnDemandHotspotNetworkStatics

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                interface IWiFiWpsConfigurationResult;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult ABI::Windows::Devices::WiFi::IWiFiWpsConfigurationResult

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                typedef enum WiFiAccessStatus : int WiFiAccessStatus;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f8c75a3a-739a-57aa-986d-1f7604d7e386"))
IAsyncOperation<enum ABI::Windows::Devices::WiFi::WiFiAccessStatus> : IAsyncOperation_impl<enum ABI::Windows::Devices::WiFi::WiFiAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.WiFi.WiFiAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Devices::WiFi::WiFiAccessStatus> __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_t;
#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("65e889ca-f6c9-5c75-bef9-05ab88a49a54"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Devices::WiFi::WiFiAccessStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Devices::WiFi::WiFiAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.WiFi.WiFiAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Devices::WiFi::WiFiAccessStatus> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                class WiFiAdapter;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1dcf739d-10b7-59e9-ab47-8b0277e20193"))
IAsyncOperation<ABI::Windows::Devices::WiFi::WiFiAdapter*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFi::WiFiAdapter*, ABI::Windows::Devices::WiFi::IWiFiAdapter*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.WiFi.WiFiAdapter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::WiFi::WiFiAdapter*> __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter_t;
#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("35362f75-6aae-560d-b3d0-3fae9c7260a8"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::WiFi::WiFiAdapter*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFi::WiFiAdapter*, ABI::Windows::Devices::WiFi::IWiFiAdapter*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.WiFi.WiFiAdapter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::WiFi::WiFiAdapter*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                class WiFiConnectionResult;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ffa41f49-4c30-50d3-9549-e4f055b417b4"))
IAsyncOperation<ABI::Windows::Devices::WiFi::WiFiConnectionResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFi::WiFiConnectionResult*, ABI::Windows::Devices::WiFi::IWiFiConnectionResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.WiFi.WiFiConnectionResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::WiFi::WiFiConnectionResult*> __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_t;
#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f380eb8d-1e52-5350-a288-861c963a84f0"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::WiFi::WiFiConnectionResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFi::WiFiConnectionResult*, ABI::Windows::Devices::WiFi::IWiFiConnectionResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.WiFi.WiFiConnectionResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::WiFi::WiFiConnectionResult*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                class WiFiOnDemandHotspotConnectionResult;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("81ec4b01-3e45-5a39-baec-0bda4ab1e3ae"))
IAsyncOperation<ABI::Windows::Devices::WiFi::WiFiOnDemandHotspotConnectionResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFi::WiFiOnDemandHotspotConnectionResult*, ABI::Windows::Devices::WiFi::IWiFiOnDemandHotspotConnectionResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.WiFi.WiFiOnDemandHotspotConnectionResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::WiFi::WiFiOnDemandHotspotConnectionResult*> __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_t;
#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("60e43845-d589-5456-bdba-f0cd24a5237e"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::WiFi::WiFiOnDemandHotspotConnectionResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFi::WiFiOnDemandHotspotConnectionResult*, ABI::Windows::Devices::WiFi::IWiFiOnDemandHotspotConnectionResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.WiFi.WiFiOnDemandHotspotConnectionResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::WiFi::WiFiOnDemandHotspotConnectionResult*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                class WiFiWpsConfigurationResult;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4b721d74-0289-583c-a81d-f3be03ea596d"))
IAsyncOperation<ABI::Windows::Devices::WiFi::WiFiWpsConfigurationResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFi::WiFiWpsConfigurationResult*, ABI::Windows::Devices::WiFi::IWiFiWpsConfigurationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.WiFi.WiFiWpsConfigurationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::WiFi::WiFiWpsConfigurationResult*> __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_t;
#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("33fa345b-28cd-58a8-bcfc-be4cfd108e91"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::WiFi::WiFiWpsConfigurationResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFi::WiFiWpsConfigurationResult*, ABI::Windows::Devices::WiFi::IWiFiWpsConfigurationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.WiFi.WiFiWpsConfigurationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::WiFi::WiFiWpsConfigurationResult*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE
#define DEF___FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("144136c6-b502-5a52-90fc-22a09318f932"))
IIterator<ABI::Windows::Devices::WiFi::WiFiAdapter*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFi::WiFiAdapter*, ABI::Windows::Devices::WiFi::IWiFiAdapter*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.WiFi.WiFiAdapter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::WiFi::WiFiAdapter*> __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter_t;
#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE
#define DEF___FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e0bc76c4-8d0c-53fc-bcd4-228f47210ace"))
IIterable<ABI::Windows::Devices::WiFi::WiFiAdapter*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFi::WiFiAdapter*, ABI::Windows::Devices::WiFi::IWiFiAdapter*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.WiFi.WiFiAdapter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::WiFi::WiFiAdapter*> __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter_t;
#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE
#define DEF___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("670a3c41-ecc8-55c2-84d4-51864496a328"))
IVectorView<ABI::Windows::Devices::WiFi::WiFiAdapter*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFi::WiFiAdapter*, ABI::Windows::Devices::WiFi::IWiFiAdapter*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.WiFi.WiFiAdapter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::WiFi::WiFiAdapter*> __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_t;
#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3140802b-987c-5c56-a430-90fbc1898dda"))
IAsyncOperation<__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.WiFi.WiFiAdapter>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter*> __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("92902a07-2f18-56e9-87fb-24fe19f70688"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.WiFi.WiFiAdapter>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                class WiFiAvailableNetwork;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_USE
#define DEF___FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("468677c4-ebb9-5196-836d-72faa9fe673e"))
IIterator<ABI::Windows::Devices::WiFi::WiFiAvailableNetwork*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFi::WiFiAvailableNetwork*, ABI::Windows::Devices::WiFi::IWiFiAvailableNetwork*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.WiFi.WiFiAvailableNetwork>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::WiFi::WiFiAvailableNetwork*> __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_t;
#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_USE
#define DEF___FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f17484ea-c71e-5d3e-b74c-3a0e61dd9c20"))
IIterable<ABI::Windows::Devices::WiFi::WiFiAvailableNetwork*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFi::WiFiAvailableNetwork*, ABI::Windows::Devices::WiFi::IWiFiAvailableNetwork*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.WiFi.WiFiAvailableNetwork>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::WiFi::WiFiAvailableNetwork*> __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_t;
#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                typedef enum WiFiWpsKind : int WiFiWpsKind;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind_USE
#define DEF___FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9b19593c-602d-57d9-a852-a48a8204ff42"))
IIterator<enum ABI::Windows::Devices::WiFi::WiFiWpsKind> : IIterator_impl<enum ABI::Windows::Devices::WiFi::WiFiWpsKind>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.WiFi.WiFiWpsKind>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Devices::WiFi::WiFiWpsKind> __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind_t;
#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind_USE
#define DEF___FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("41e16513-a8f2-55ed-9be4-5665167d49d7"))
IIterable<enum ABI::Windows::Devices::WiFi::WiFiWpsKind> : IIterable_impl<enum ABI::Windows::Devices::WiFi::WiFiWpsKind>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.WiFi.WiFiWpsKind>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Devices::WiFi::WiFiWpsKind> __FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind_t;
#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_USE
#define DEF___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7c65d286-7285-5d63-bdea-5ef951bdf618"))
IVectorView<ABI::Windows::Devices::WiFi::WiFiAvailableNetwork*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFi::WiFiAvailableNetwork*, ABI::Windows::Devices::WiFi::IWiFiAvailableNetwork*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.WiFi.WiFiAvailableNetwork>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::WiFi::WiFiAvailableNetwork*> __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_t;
#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind_USE
#define DEF___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("aae7e5e5-27f0-5b28-8c58-9039356d3dc7"))
IVectorView<enum ABI::Windows::Devices::WiFi::WiFiWpsKind> : IVectorView_impl<enum ABI::Windows::Devices::WiFi::WiFiWpsKind>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.WiFi.WiFiWpsKind>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Devices::WiFi::WiFiWpsKind> __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind_t;
#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000


#ifndef DEF___FIReference_1_UINT32_USE
#define DEF___FIReference_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("513ef3af-e784-5325-a91e-97c2b8111cf3"))
IReference<UINT32> : IReference_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<UINT32> __FIReference_1_UINT32_t;
#define __FIReference_1_UINT32 ABI::Windows::Foundation::__FIReference_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_UINT32_USE */


namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                typedef enum WiFiOnDemandHotspotCellularBars : int WiFiOnDemandHotspotCellularBars;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#ifndef DEF___FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars_USE
#define DEF___FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d5cc7e52-2520-5460-84e6-8258c83d0c68"))
IReference<enum ABI::Windows::Devices::WiFi::WiFiOnDemandHotspotCellularBars> : IReference_impl<enum ABI::Windows::Devices::WiFi::WiFiOnDemandHotspotCellularBars>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Devices.WiFi.WiFiOnDemandHotspotCellularBars>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<enum ABI::Windows::Devices::WiFi::WiFiOnDemandHotspotCellularBars> __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars_t;
#define __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars ABI::Windows::Foundation::__FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f6c02d1b-43e8-5fc8-8e8e-ee7b8094b683"))
ITypedEventHandler<ABI::Windows::Devices::WiFi::WiFiAdapter*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::WiFi::WiFiAdapter*, ABI::Windows::Devices::WiFi::IWiFiAdapter*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.WiFi.WiFiAdapter, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::WiFi::WiFiAdapter*, IInspectable*> __FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IPropertyValue;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIPropertyValue ABI::Windows::Foundation::IPropertyValue

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class NetworkAdapter;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface INetworkAdapter;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter ABI::Windows::Networking::Connectivity::INetworkAdapter

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class NetworkSecuritySettings;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface INetworkSecuritySettings;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings ABI::Windows::Networking::Connectivity::INetworkSecuritySettings

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                class PasswordCredential;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IPasswordCredential;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential ABI::Windows::Security::Credentials::IPasswordCredential

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                typedef enum WiFiConnectionMethod : int WiFiConnectionMethod;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                typedef enum WiFiConnectionStatus : int WiFiConnectionStatus;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                typedef enum WiFiNetworkKind : int WiFiNetworkKind;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                typedef enum WiFiOnDemandHotspotAvailability : int WiFiOnDemandHotspotAvailability;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                typedef enum WiFiOnDemandHotspotConnectStatus : int WiFiOnDemandHotspotConnectStatus;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                typedef enum WiFiPhyKind : int WiFiPhyKind;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                typedef enum WiFiReconnectionKind : int WiFiReconnectionKind;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                typedef enum WiFiWpsConfigurationStatus : int WiFiWpsConfigurationStatus;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                class WiFiNetworkReport;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                class WiFiOnDemandHotspotNetwork;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                class WiFiOnDemandHotspotNetworkProperties;
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.WiFi.WiFiAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                enum WiFiAccessStatus : int
                {
                    WiFiAccessStatus_Unspecified = 0,
                    WiFiAccessStatus_Allowed = 1,
                    WiFiAccessStatus_DeniedByUser = 2,
                    WiFiAccessStatus_DeniedBySystem = 3,
                };
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiConnectionMethod
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                enum WiFiConnectionMethod : int
                {
                    WiFiConnectionMethod_Default = 0,
                    WiFiConnectionMethod_WpsPin = 1,
                    WiFiConnectionMethod_WpsPushButton = 2,
                };
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiConnectionStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                enum WiFiConnectionStatus : int
                {
                    WiFiConnectionStatus_UnspecifiedFailure = 0,
                    WiFiConnectionStatus_Success = 1,
                    WiFiConnectionStatus_AccessRevoked = 2,
                    WiFiConnectionStatus_InvalidCredential = 3,
                    WiFiConnectionStatus_NetworkNotAvailable = 4,
                    WiFiConnectionStatus_Timeout = 5,
                    WiFiConnectionStatus_UnsupportedAuthenticationProtocol = 6,
                };
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiNetworkKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                enum WiFiNetworkKind : int
                {
                    WiFiNetworkKind_Any = 0,
                    WiFiNetworkKind_Infrastructure = 1,
                    WiFiNetworkKind_Adhoc = 2,
                };
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiOnDemandHotspotAvailability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                enum WiFiOnDemandHotspotAvailability : int
                {
                    WiFiOnDemandHotspotAvailability_Available = 0,
                    WiFiOnDemandHotspotAvailability_Unavailable = 1,
                };
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiOnDemandHotspotCellularBars
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                enum WiFiOnDemandHotspotCellularBars : int
                {
                    WiFiOnDemandHotspotCellularBars_ZeroBars = 0,
                    WiFiOnDemandHotspotCellularBars_OneBar = 1,
                    WiFiOnDemandHotspotCellularBars_TwoBars = 2,
                    WiFiOnDemandHotspotCellularBars_ThreeBars = 3,
                    WiFiOnDemandHotspotCellularBars_FourBars = 4,
                    WiFiOnDemandHotspotCellularBars_FiveBars = 5,
                };
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiOnDemandHotspotConnectStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                enum WiFiOnDemandHotspotConnectStatus : int
                {
                    WiFiOnDemandHotspotConnectStatus_UnspecifiedFailure = 0,
                    WiFiOnDemandHotspotConnectStatus_Success = 1,
                    WiFiOnDemandHotspotConnectStatus_AppTimedOut = 2,
                    WiFiOnDemandHotspotConnectStatus_InvalidCredential = 3,
                    WiFiOnDemandHotspotConnectStatus_NetworkNotAvailable = 4,
                    WiFiOnDemandHotspotConnectStatus_UnsupportedAuthenticationProtocol = 5,
                    WiFiOnDemandHotspotConnectStatus_BluetoothConnectFailed = 6,
                    WiFiOnDemandHotspotConnectStatus_BluetoothTransmissionError = 7,
                    WiFiOnDemandHotspotConnectStatus_OperationCanceledByUser = 8,
                    WiFiOnDemandHotspotConnectStatus_EntitlementCheckFailed = 9,
                    WiFiOnDemandHotspotConnectStatus_NoCellularSignal = 10,
                    WiFiOnDemandHotspotConnectStatus_CellularDataTurnedOff = 11,
                    WiFiOnDemandHotspotConnectStatus_WlanConnectFailed = 12,
                    WiFiOnDemandHotspotConnectStatus_WlanNotVisible = 13,
                    WiFiOnDemandHotspotConnectStatus_AccessPointCannotConnect = 14,
                    WiFiOnDemandHotspotConnectStatus_CellularConnectTimedOut = 15,
                    WiFiOnDemandHotspotConnectStatus_RoamingNotAllowed = 16,
                    WiFiOnDemandHotspotConnectStatus_PairingRequired = 17,
                    WiFiOnDemandHotspotConnectStatus_DataLimitReached = 18,
                };
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiPhyKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                enum WiFiPhyKind : int
                {
                    WiFiPhyKind_Unknown = 0,
                    WiFiPhyKind_Fhss = 1,
                    WiFiPhyKind_Dsss = 2,
                    WiFiPhyKind_IRBaseband = 3,
                    WiFiPhyKind_Ofdm = 4,
                    WiFiPhyKind_Hrdsss = 5,
                    WiFiPhyKind_Erp = 6,
                    WiFiPhyKind_HT = 7,
                    WiFiPhyKind_Vht = 8,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    WiFiPhyKind_Dmg = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                    WiFiPhyKind_HE = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                    WiFiPhyKind_Eht = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                };
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiReconnectionKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                enum WiFiReconnectionKind : int
                {
                    WiFiReconnectionKind_Automatic = 0,
                    WiFiReconnectionKind_Manual = 1,
                };
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiWpsConfigurationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                enum WiFiWpsConfigurationStatus : int
                {
                    WiFiWpsConfigurationStatus_UnspecifiedFailure = 0,
                    WiFiWpsConfigurationStatus_Success = 1,
                    WiFiWpsConfigurationStatus_Timeout = 2,
                };
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiWpsKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                enum WiFiWpsKind : int
                {
                    WiFiWpsKind_Unknown = 0,
                    WiFiWpsKind_Pin = 1,
                    WiFiWpsKind_PushButton = 2,
                    WiFiWpsKind_Nfc = 3,
                    WiFiWpsKind_Ethernet = 4,
                    WiFiWpsKind_Usb = 5,
                };
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiAdapter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiAdapter[] = L"Windows.Devices.WiFi.IWiFiAdapter";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                MIDL_INTERFACE("a6c4e423-3d75-43a4-b9de-11e26b72d9b0")
                IWiFiAdapter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkAdapter(
                        ABI::Windows::Networking::Connectivity::INetworkAdapter** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ScanAsync(
                        ABI::Windows::Foundation::IAsyncAction** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkReport(
                        ABI::Windows::Devices::WiFi::IWiFiNetworkReport** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_AvailableNetworksChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable* args,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AvailableNetworksChanged(
                        EventRegistrationToken eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConnectAsync(
                        ABI::Windows::Devices::WiFi::IWiFiAvailableNetwork* availableNetwork,
                        ABI::Windows::Devices::WiFi::WiFiReconnectionKind reconnectionKind,
                        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConnectWithPasswordCredentialAsync(
                        ABI::Windows::Devices::WiFi::IWiFiAvailableNetwork* availableNetwork,
                        ABI::Windows::Devices::WiFi::WiFiReconnectionKind reconnectionKind,
                        ABI::Windows::Security::Credentials::IPasswordCredential* passwordCredential,
                        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConnectWithPasswordCredentialAndSsidAsync(
                        ABI::Windows::Devices::WiFi::IWiFiAvailableNetwork* availableNetwork,
                        ABI::Windows::Devices::WiFi::WiFiReconnectionKind reconnectionKind,
                        ABI::Windows::Security::Credentials::IPasswordCredential* passwordCredential,
                        HSTRING ssid,
                        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Disconnect(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IWiFiAdapter = __uuidof(IWiFiAdapter);
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiAdapter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiAdapter2[] = L"Windows.Devices.WiFi.IWiFiAdapter2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                MIDL_INTERFACE("5bc4501d-81e4-453d-9430-1fcafbadd6b6")
                IWiFiAdapter2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetWpsConfigurationAsync(
                        ABI::Windows::Devices::WiFi::IWiFiAvailableNetwork* availableNetwork,
                        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync(
                        ABI::Windows::Devices::WiFi::IWiFiAvailableNetwork* availableNetwork,
                        ABI::Windows::Devices::WiFi::WiFiReconnectionKind reconnectionKind,
                        ABI::Windows::Security::Credentials::IPasswordCredential* passwordCredential,
                        HSTRING ssid,
                        ABI::Windows::Devices::WiFi::WiFiConnectionMethod connectionMethod,
                        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWiFiAdapter2 = __uuidof(IWiFiAdapter2);
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiAdapterStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiAdapterStatics[] = L"Windows.Devices.WiFi.IWiFiAdapterStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                MIDL_INTERFACE("da25fddd-d24c-43e3-aabd-c4659f730f99")
                IWiFiAdapterStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FindAllAdaptersAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* deviceSelector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING deviceId,
                        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter** asyncOp
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestAccessAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWiFiAdapterStatics = __uuidof(IWiFiAdapterStatics);
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiAvailableNetwork
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiAvailableNetwork
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiAvailableNetwork[] = L"Windows.Devices.WiFi.IWiFiAvailableNetwork";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                MIDL_INTERFACE("26e96246-183e-4704-9826-71b4a2f0f668")
                IWiFiAvailableNetwork : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Uptime(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Ssid(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Bssid(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ChannelCenterFrequencyInKilohertz(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkRssiInDecibelMilliwatts(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SignalBars(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkKind(
                        ABI::Windows::Devices::WiFi::WiFiNetworkKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PhyKind(
                        ABI::Windows::Devices::WiFi::WiFiPhyKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SecuritySettings(
                        ABI::Windows::Networking::Connectivity::INetworkSecuritySettings** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BeaconInterval(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsWiFiDirect(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWiFiAvailableNetwork = __uuidof(IWiFiAvailableNetwork);
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiConnectionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiConnectionResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiConnectionResult[] = L"Windows.Devices.WiFi.IWiFiConnectionResult";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                MIDL_INTERFACE("143bdfd9-c37d-40be-a5c8-857bce85a931")
                IWiFiConnectionResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ConnectionStatus(
                        ABI::Windows::Devices::WiFi::WiFiConnectionStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWiFiConnectionResult = __uuidof(IWiFiConnectionResult);
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiNetworkReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiNetworkReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiNetworkReport[] = L"Windows.Devices.WiFi.IWiFiNetworkReport";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                MIDL_INTERFACE("9524ded2-5911-445e-8194-be4f1a704895")
                IWiFiNetworkReport : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AvailableNetworks(
                        __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWiFiNetworkReport = __uuidof(IWiFiNetworkReport);
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiOnDemandHotspotConnectTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiOnDemandHotspotConnectTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiOnDemandHotspotConnectTriggerDetails[] = L"Windows.Devices.WiFi.IWiFiOnDemandHotspotConnectTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                MIDL_INTERFACE("a268eb58-68f5-59cf-8d38-35bf44b097ef")
                IWiFiOnDemandHotspotConnectTriggerDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RequestedNetwork(
                        ABI::Windows::Devices::WiFi::IWiFiOnDemandHotspotNetwork** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReportError(
                        ABI::Windows::Devices::WiFi::WiFiOnDemandHotspotConnectStatus status
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConnectAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Connect(
                        ABI::Windows::Devices::WiFi::IWiFiOnDemandHotspotConnectionResult** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWiFiOnDemandHotspotConnectTriggerDetails = __uuidof(IWiFiOnDemandHotspotConnectTriggerDetails);
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiOnDemandHotspotConnectionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiOnDemandHotspotConnectionResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiOnDemandHotspotConnectionResult[] = L"Windows.Devices.WiFi.IWiFiOnDemandHotspotConnectionResult";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                MIDL_INTERFACE("911794a1-6c82-5de3-8a4a-f9ff22a4957a")
                IWiFiOnDemandHotspotConnectionResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Devices::WiFi::WiFiOnDemandHotspotConnectStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWiFiOnDemandHotspotConnectionResult = __uuidof(IWiFiOnDemandHotspotConnectionResult);
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiOnDemandHotspotNetwork
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiOnDemandHotspotNetwork
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiOnDemandHotspotNetwork[] = L"Windows.Devices.WiFi.IWiFiOnDemandHotspotNetwork";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                MIDL_INTERFACE("18dc7115-a04e-507c-bbaf-b78369d29fa7")
                IWiFiOnDemandHotspotNetwork : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetProperties(
                        ABI::Windows::Devices::WiFi::IWiFiOnDemandHotspotNetworkProperties** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UpdateProperties(
                        ABI::Windows::Devices::WiFi::IWiFiOnDemandHotspotNetworkProperties* newProperties
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        GUID* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWiFiOnDemandHotspotNetwork = __uuidof(IWiFiOnDemandHotspotNetwork);
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiOnDemandHotspotNetworkProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiOnDemandHotspotNetworkProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiOnDemandHotspotNetworkProperties[] = L"Windows.Devices.WiFi.IWiFiOnDemandHotspotNetworkProperties";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                MIDL_INTERFACE("c810a1f2-c81d-5852-be50-e4bd4d81e98d")
                IWiFiOnDemandHotspotNetworkProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Availability(
                        ABI::Windows::Devices::WiFi::WiFiOnDemandHotspotAvailability* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Availability(
                        ABI::Windows::Devices::WiFi::WiFiOnDemandHotspotAvailability value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemainingBatteryPercent(
                        __FIReference_1_UINT32** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RemainingBatteryPercent(
                        __FIReference_1_UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CellularBars(
                        __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CellularBars(
                        __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsMetered(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsMetered(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Ssid(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Ssid(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Password(
                        ABI::Windows::Security::Credentials::IPasswordCredential** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Password(
                        ABI::Windows::Security::Credentials::IPasswordCredential* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWiFiOnDemandHotspotNetworkProperties = __uuidof(IWiFiOnDemandHotspotNetworkProperties);
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiOnDemandHotspotNetworkStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiOnDemandHotspotNetwork
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiOnDemandHotspotNetworkStatics[] = L"Windows.Devices.WiFi.IWiFiOnDemandHotspotNetworkStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                MIDL_INTERFACE("00f5b8ac-80e7-5054-871c-8739f374e3c9")
                IWiFiOnDemandHotspotNetworkStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetOrCreateById(
                        GUID networkId,
                        ABI::Windows::Devices::WiFi::IWiFiOnDemandHotspotNetwork** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWiFiOnDemandHotspotNetworkStatics = __uuidof(IWiFiOnDemandHotspotNetworkStatics);
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiWpsConfigurationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiWpsConfigurationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiWpsConfigurationResult[] = L"Windows.Devices.WiFi.IWiFiWpsConfigurationResult";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace WiFi {
                MIDL_INTERFACE("67b49871-17ee-42d1-b14f-5a11f1226fb5")
                IWiFiWpsConfigurationResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Devices::WiFi::WiFiWpsConfigurationStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedWpsKinds(
                        __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWiFiWpsConfigurationResult = __uuidof(IWiFiWpsConfigurationResult);
            } /* WiFi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Devices.WiFi.WiFiAdapter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.WiFi.IWiFiAdapterStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFi.IWiFiAdapter ** Default Interface **
 *    Windows.Devices.WiFi.IWiFiAdapter2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFi_WiFiAdapter_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFi_WiFiAdapter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFi_WiFiAdapter[] = L"Windows.Devices.WiFi.WiFiAdapter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFi.WiFiAvailableNetwork
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFi.IWiFiAvailableNetwork ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFi_WiFiAvailableNetwork_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFi_WiFiAvailableNetwork_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFi_WiFiAvailableNetwork[] = L"Windows.Devices.WiFi.WiFiAvailableNetwork";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFi.WiFiConnectionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFi.IWiFiConnectionResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFi_WiFiConnectionResult_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFi_WiFiConnectionResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFi_WiFiConnectionResult[] = L"Windows.Devices.WiFi.WiFiConnectionResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFi.WiFiNetworkReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFi.IWiFiNetworkReport ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFi_WiFiNetworkReport_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFi_WiFiNetworkReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFi_WiFiNetworkReport[] = L"Windows.Devices.WiFi.WiFiNetworkReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFi.WiFiOnDemandHotspotConnectTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFi.IWiFiOnDemandHotspotConnectTriggerDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Devices_WiFi_WiFiOnDemandHotspotConnectTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFi_WiFiOnDemandHotspotConnectTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFi_WiFiOnDemandHotspotConnectTriggerDetails[] = L"Windows.Devices.WiFi.WiFiOnDemandHotspotConnectTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Devices.WiFi.WiFiOnDemandHotspotConnectionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFi.IWiFiOnDemandHotspotConnectionResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Devices_WiFi_WiFiOnDemandHotspotConnectionResult_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFi_WiFiOnDemandHotspotConnectionResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFi_WiFiOnDemandHotspotConnectionResult[] = L"Windows.Devices.WiFi.WiFiOnDemandHotspotConnectionResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Devices.WiFi.WiFiOnDemandHotspotNetwork
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.WiFi.IWiFiOnDemandHotspotNetworkStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFi.IWiFiOnDemandHotspotNetwork ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Devices_WiFi_WiFiOnDemandHotspotNetwork_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFi_WiFiOnDemandHotspotNetwork_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFi_WiFiOnDemandHotspotNetwork[] = L"Windows.Devices.WiFi.WiFiOnDemandHotspotNetwork";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Devices.WiFi.WiFiOnDemandHotspotNetworkProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFi.IWiFiOnDemandHotspotNetworkProperties ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Devices_WiFi_WiFiOnDemandHotspotNetworkProperties_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFi_WiFiOnDemandHotspotNetworkProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFi_WiFiOnDemandHotspotNetworkProperties[] = L"Windows.Devices.WiFi.WiFiOnDemandHotspotNetworkProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Devices.WiFi.WiFiWpsConfigurationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFi.IWiFiWpsConfigurationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Devices_WiFi_WiFiWpsConfigurationResult_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFi_WiFiWpsConfigurationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFi_WiFiWpsConfigurationResult[] = L"Windows.Devices.WiFi.WiFiWpsConfigurationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter;

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2 __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2;

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics;

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork;

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult __x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult;

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport __x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport;

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails;

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult;

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork;

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties;

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics;

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult __x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult;

#endif // ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiAccessStatus __x_ABI_CWindows_CDevices_CWiFi_CWiFiAccessStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus* This,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiAccessStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatusVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus* This,
        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResultVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult* This,
        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiConnectionResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResultVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult* This,
        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResultVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult* This,
        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter;

typedef struct __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl;

interface __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter;

typedef struct __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAdapter** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl;

interface __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter;

typedef struct __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl;

interface __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork;

typedef struct __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetworkVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetworkVtbl;

interface __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetworkVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork;

typedef struct __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetworkVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        __FIIterator_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetworkVtbl;

interface __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetworkVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiWpsKind __x_ABI_CWindows_CDevices_CWiFi_CWiFiWpsKind;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind;

typedef struct __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKindVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiWpsKind* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiWpsKind* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKindVtbl;

interface __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKindVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind __FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind;

typedef struct __FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKindVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        __FIIterator_1_Windows__CDevices__CWiFi__CWiFiWpsKind** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKindVtbl;

interface __FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKindVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CWiFi__CWiFiWpsKind_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork;

typedef struct __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetworkVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetworkVtbl;

interface __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetworkVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind;

typedef struct __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKindVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        UINT32 index,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiWpsKind* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiWpsKind value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiWpsKind* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKindVtbl;

interface __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKindVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if !defined(____FIReference_1_UINT32_INTERFACE_DEFINED__)
#define ____FIReference_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIReference_1_UINT32 __FIReference_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_UINT32;

typedef struct __FIReference_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_UINT32* This,
        UINT32* result);

    END_INTERFACE
} __FIReference_1_UINT32Vtbl;

interface __FIReference_1_UINT32
{
    CONST_VTBL struct __FIReference_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_UINT32_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_UINT32_INTERFACE_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiOnDemandHotspotCellularBars __x_ABI_CWindows_CDevices_CWiFi_CWiFiOnDemandHotspotCellularBars;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars;

typedef struct __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBarsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars* This,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiOnDemandHotspotCellularBars* result);

    END_INTERFACE
} __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBarsVtbl;

interface __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars
{
    CONST_VTBL struct __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBarsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable __FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiConnectionMethod __x_ABI_CWindows_CDevices_CWiFi_CWiFiConnectionMethod;

typedef enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiConnectionStatus __x_ABI_CWindows_CDevices_CWiFi_CWiFiConnectionStatus;

typedef enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiNetworkKind __x_ABI_CWindows_CDevices_CWiFi_CWiFiNetworkKind;

typedef enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiOnDemandHotspotAvailability __x_ABI_CWindows_CDevices_CWiFi_CWiFiOnDemandHotspotAvailability;

typedef enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiOnDemandHotspotConnectStatus __x_ABI_CWindows_CDevices_CWiFi_CWiFiOnDemandHotspotConnectStatus;

typedef enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiPhyKind __x_ABI_CWindows_CDevices_CWiFi_CWiFiPhyKind;

typedef enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiReconnectionKind __x_ABI_CWindows_CDevices_CWiFi_CWiFiReconnectionKind;

typedef enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiWpsConfigurationStatus __x_ABI_CWindows_CDevices_CWiFi_CWiFiWpsConfigurationStatus;

/*
 *
 * Struct Windows.Devices.WiFi.WiFiAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiAccessStatus
{
    WiFiAccessStatus_Unspecified = 0,
    WiFiAccessStatus_Allowed = 1,
    WiFiAccessStatus_DeniedByUser = 2,
    WiFiAccessStatus_DeniedBySystem = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiConnectionMethod
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiConnectionMethod
{
    WiFiConnectionMethod_Default = 0,
    WiFiConnectionMethod_WpsPin = 1,
    WiFiConnectionMethod_WpsPushButton = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiConnectionStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiConnectionStatus
{
    WiFiConnectionStatus_UnspecifiedFailure = 0,
    WiFiConnectionStatus_Success = 1,
    WiFiConnectionStatus_AccessRevoked = 2,
    WiFiConnectionStatus_InvalidCredential = 3,
    WiFiConnectionStatus_NetworkNotAvailable = 4,
    WiFiConnectionStatus_Timeout = 5,
    WiFiConnectionStatus_UnsupportedAuthenticationProtocol = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiNetworkKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiNetworkKind
{
    WiFiNetworkKind_Any = 0,
    WiFiNetworkKind_Infrastructure = 1,
    WiFiNetworkKind_Adhoc = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiOnDemandHotspotAvailability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiOnDemandHotspotAvailability
{
    WiFiOnDemandHotspotAvailability_Available = 0,
    WiFiOnDemandHotspotAvailability_Unavailable = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiOnDemandHotspotCellularBars
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiOnDemandHotspotCellularBars
{
    WiFiOnDemandHotspotCellularBars_ZeroBars = 0,
    WiFiOnDemandHotspotCellularBars_OneBar = 1,
    WiFiOnDemandHotspotCellularBars_TwoBars = 2,
    WiFiOnDemandHotspotCellularBars_ThreeBars = 3,
    WiFiOnDemandHotspotCellularBars_FourBars = 4,
    WiFiOnDemandHotspotCellularBars_FiveBars = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiOnDemandHotspotConnectStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiOnDemandHotspotConnectStatus
{
    WiFiOnDemandHotspotConnectStatus_UnspecifiedFailure = 0,
    WiFiOnDemandHotspotConnectStatus_Success = 1,
    WiFiOnDemandHotspotConnectStatus_AppTimedOut = 2,
    WiFiOnDemandHotspotConnectStatus_InvalidCredential = 3,
    WiFiOnDemandHotspotConnectStatus_NetworkNotAvailable = 4,
    WiFiOnDemandHotspotConnectStatus_UnsupportedAuthenticationProtocol = 5,
    WiFiOnDemandHotspotConnectStatus_BluetoothConnectFailed = 6,
    WiFiOnDemandHotspotConnectStatus_BluetoothTransmissionError = 7,
    WiFiOnDemandHotspotConnectStatus_OperationCanceledByUser = 8,
    WiFiOnDemandHotspotConnectStatus_EntitlementCheckFailed = 9,
    WiFiOnDemandHotspotConnectStatus_NoCellularSignal = 10,
    WiFiOnDemandHotspotConnectStatus_CellularDataTurnedOff = 11,
    WiFiOnDemandHotspotConnectStatus_WlanConnectFailed = 12,
    WiFiOnDemandHotspotConnectStatus_WlanNotVisible = 13,
    WiFiOnDemandHotspotConnectStatus_AccessPointCannotConnect = 14,
    WiFiOnDemandHotspotConnectStatus_CellularConnectTimedOut = 15,
    WiFiOnDemandHotspotConnectStatus_RoamingNotAllowed = 16,
    WiFiOnDemandHotspotConnectStatus_PairingRequired = 17,
    WiFiOnDemandHotspotConnectStatus_DataLimitReached = 18,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiPhyKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiPhyKind
{
    WiFiPhyKind_Unknown = 0,
    WiFiPhyKind_Fhss = 1,
    WiFiPhyKind_Dsss = 2,
    WiFiPhyKind_IRBaseband = 3,
    WiFiPhyKind_Ofdm = 4,
    WiFiPhyKind_Hrdsss = 5,
    WiFiPhyKind_Erp = 6,
    WiFiPhyKind_HT = 7,
    WiFiPhyKind_Vht = 8,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    WiFiPhyKind_Dmg = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    WiFiPhyKind_HE = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    WiFiPhyKind_Eht = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiReconnectionKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiReconnectionKind
{
    WiFiReconnectionKind_Automatic = 0,
    WiFiReconnectionKind_Manual = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiWpsConfigurationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiWpsConfigurationStatus
{
    WiFiWpsConfigurationStatus_UnspecifiedFailure = 0,
    WiFiWpsConfigurationStatus_Success = 1,
    WiFiWpsConfigurationStatus_Timeout = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Devices.WiFi.WiFiWpsKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiWpsKind
{
    WiFiWpsKind_Unknown = 0,
    WiFiWpsKind_Pin = 1,
    WiFiWpsKind_PushButton = 2,
    WiFiWpsKind_Nfc = 3,
    WiFiWpsKind_Ethernet = 4,
    WiFiWpsKind_Usb = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiAdapter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiAdapter[] = L"Windows.Devices.WiFi.IWiFiAdapter";
typedef struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NetworkAdapter)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter** value);
    HRESULT (STDMETHODCALLTYPE* ScanAsync)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* get_NetworkReport)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport** value);
    HRESULT (STDMETHODCALLTYPE* add_AvailableNetworksChanged)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter* This,
        __FITypedEventHandler_2_Windows__CDevices__CWiFi__CWiFiAdapter_IInspectable* args,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_AvailableNetworksChanged)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter* This,
        EventRegistrationToken eventCookie);
    HRESULT (STDMETHODCALLTYPE* ConnectAsync)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* availableNetwork,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiReconnectionKind reconnectionKind,
        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult** value);
    HRESULT (STDMETHODCALLTYPE* ConnectWithPasswordCredentialAsync)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* availableNetwork,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiReconnectionKind reconnectionKind,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* passwordCredential,
        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult** value);
    HRESULT (STDMETHODCALLTYPE* ConnectWithPasswordCredentialAndSsidAsync)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* availableNetwork,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiReconnectionKind reconnectionKind,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* passwordCredential,
        HSTRING ssid,
        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult** value);
    HRESULT (STDMETHODCALLTYPE* Disconnect)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter* This);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterVtbl;

interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_get_NetworkAdapter(This, value) \
    ((This)->lpVtbl->get_NetworkAdapter(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_ScanAsync(This, value) \
    ((This)->lpVtbl->ScanAsync(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_get_NetworkReport(This, value) \
    ((This)->lpVtbl->get_NetworkReport(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_add_AvailableNetworksChanged(This, args, eventCookie) \
    ((This)->lpVtbl->add_AvailableNetworksChanged(This, args, eventCookie))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_remove_AvailableNetworksChanged(This, eventCookie) \
    ((This)->lpVtbl->remove_AvailableNetworksChanged(This, eventCookie))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_ConnectAsync(This, availableNetwork, reconnectionKind, value) \
    ((This)->lpVtbl->ConnectAsync(This, availableNetwork, reconnectionKind, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_ConnectWithPasswordCredentialAsync(This, availableNetwork, reconnectionKind, passwordCredential, value) \
    ((This)->lpVtbl->ConnectWithPasswordCredentialAsync(This, availableNetwork, reconnectionKind, passwordCredential, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_ConnectWithPasswordCredentialAndSsidAsync(This, availableNetwork, reconnectionKind, passwordCredential, ssid, value) \
    ((This)->lpVtbl->ConnectWithPasswordCredentialAndSsidAsync(This, availableNetwork, reconnectionKind, passwordCredential, ssid, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_Disconnect(This) \
    ((This)->lpVtbl->Disconnect(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiAdapter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiAdapter2[] = L"Windows.Devices.WiFi.IWiFiAdapter2";
typedef struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetWpsConfigurationAsync)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* availableNetwork,
        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiWpsConfigurationResult** operation);
    HRESULT (STDMETHODCALLTYPE* ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* availableNetwork,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiReconnectionKind reconnectionKind,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* passwordCredential,
        HSTRING ssid,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiConnectionMethod connectionMethod,
        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiConnectionResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2Vtbl;

interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_GetWpsConfigurationAsync(This, availableNetwork, operation) \
    ((This)->lpVtbl->GetWpsConfigurationAsync(This, availableNetwork, operation))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync(This, availableNetwork, reconnectionKind, passwordCredential, ssid, connectionMethod, operation) \
    ((This)->lpVtbl->ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync(This, availableNetwork, reconnectionKind, passwordCredential, ssid, connectionMethod, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiAdapterStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiAdapterStatics[] = L"Windows.Devices.WiFi.IWiFiAdapterStatics";
typedef struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindAllAdaptersAsync)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAdapter** value);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics* This,
        HSTRING* deviceSelector);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics* This,
        HSTRING deviceId,
        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAdapter** asyncOp);
    HRESULT (STDMETHODCALLTYPE* RequestAccessAsync)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics* This,
        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiAccessStatus** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_FindAllAdaptersAsync(This, value) \
    ((This)->lpVtbl->FindAllAdaptersAsync(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_GetDeviceSelector(This, deviceSelector) \
    ((This)->lpVtbl->GetDeviceSelector(This, deviceSelector))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_FromIdAsync(This, deviceId, asyncOp) \
    ((This)->lpVtbl->FromIdAsync(This, deviceId, asyncOp))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_RequestAccessAsync(This, value) \
    ((This)->lpVtbl->RequestAccessAsync(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAdapterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiAvailableNetwork
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiAvailableNetwork
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiAvailableNetwork[] = L"Windows.Devices.WiFi.IWiFiAvailableNetwork";
typedef struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetworkVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Uptime)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Ssid)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Bssid)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ChannelCenterFrequencyInKilohertz)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_NetworkRssiInDecibelMilliwatts)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_SignalBars)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_NetworkKind)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* This,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiNetworkKind* value);
    HRESULT (STDMETHODCALLTYPE* get_PhyKind)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* This,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiPhyKind* value);
    HRESULT (STDMETHODCALLTYPE* get_SecuritySettings)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings** value);
    HRESULT (STDMETHODCALLTYPE* get_BeaconInterval)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_IsWiFiDirect)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetworkVtbl;

interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetworkVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_get_Uptime(This, value) \
    ((This)->lpVtbl->get_Uptime(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_get_Ssid(This, value) \
    ((This)->lpVtbl->get_Ssid(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_get_Bssid(This, value) \
    ((This)->lpVtbl->get_Bssid(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_get_ChannelCenterFrequencyInKilohertz(This, value) \
    ((This)->lpVtbl->get_ChannelCenterFrequencyInKilohertz(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_get_NetworkRssiInDecibelMilliwatts(This, value) \
    ((This)->lpVtbl->get_NetworkRssiInDecibelMilliwatts(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_get_SignalBars(This, value) \
    ((This)->lpVtbl->get_SignalBars(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_get_NetworkKind(This, value) \
    ((This)->lpVtbl->get_NetworkKind(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_get_PhyKind(This, value) \
    ((This)->lpVtbl->get_PhyKind(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_get_SecuritySettings(This, value) \
    ((This)->lpVtbl->get_SecuritySettings(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_get_BeaconInterval(This, value) \
    ((This)->lpVtbl->get_BeaconInterval(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_get_IsWiFiDirect(This, value) \
    ((This)->lpVtbl->get_IsWiFiDirect(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiAvailableNetwork_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiConnectionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiConnectionResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiConnectionResult[] = L"Windows.Devices.WiFi.IWiFiConnectionResult";
typedef struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionStatus)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult* This,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiConnectionStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResultVtbl;

interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_get_ConnectionStatus(This, value) \
    ((This)->lpVtbl->get_ConnectionStatus(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiConnectionResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiNetworkReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiNetworkReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiNetworkReport[] = L"Windows.Devices.WiFi.IWiFiNetworkReport";
typedef struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_AvailableNetworks)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport* This,
        __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiAvailableNetwork** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReportVtbl;

interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_get_AvailableNetworks(This, value) \
    ((This)->lpVtbl->get_AvailableNetworks(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiNetworkReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiOnDemandHotspotConnectTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiOnDemandHotspotConnectTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiOnDemandHotspotConnectTriggerDetails[] = L"Windows.Devices.WiFi.IWiFiOnDemandHotspotConnectTriggerDetails";
typedef struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RequestedNetwork)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork** value);
    HRESULT (STDMETHODCALLTYPE* ReportError)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails* This,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiOnDemandHotspotConnectStatus status);
    HRESULT (STDMETHODCALLTYPE* ConnectAsync)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails* This,
        __FIAsyncOperation_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotConnectionResult** operation);
    HRESULT (STDMETHODCALLTYPE* Connect)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetailsVtbl;

interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_get_RequestedNetwork(This, value) \
    ((This)->lpVtbl->get_RequestedNetwork(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_ReportError(This, status) \
    ((This)->lpVtbl->ReportError(This, status))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_ConnectAsync(This, operation) \
    ((This)->lpVtbl->ConnectAsync(This, operation))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_Connect(This, result) \
    ((This)->lpVtbl->Connect(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiOnDemandHotspotConnectionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiOnDemandHotspotConnectionResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiOnDemandHotspotConnectionResult[] = L"Windows.Devices.WiFi.IWiFiOnDemandHotspotConnectionResult";
typedef struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult* This,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiOnDemandHotspotConnectStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResultVtbl;

interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotConnectionResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiOnDemandHotspotNetwork
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiOnDemandHotspotNetwork
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiOnDemandHotspotNetwork[] = L"Windows.Devices.WiFi.IWiFiOnDemandHotspotNetwork";
typedef struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetProperties)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties** result);
    HRESULT (STDMETHODCALLTYPE* UpdateProperties)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork* This,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* newProperties);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork* This,
        GUID* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkVtbl;

interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_GetProperties(This, result) \
    ((This)->lpVtbl->GetProperties(This, result))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_UpdateProperties(This, newProperties) \
    ((This)->lpVtbl->UpdateProperties(This, newProperties))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiOnDemandHotspotNetworkProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiOnDemandHotspotNetworkProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiOnDemandHotspotNetworkProperties[] = L"Windows.Devices.WiFi.IWiFiOnDemandHotspotNetworkProperties";
typedef struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Availability)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiOnDemandHotspotAvailability* value);
    HRESULT (STDMETHODCALLTYPE* put_Availability)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiOnDemandHotspotAvailability value);
    HRESULT (STDMETHODCALLTYPE* get_RemainingBatteryPercent)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* put_RemainingBatteryPercent)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This,
        __FIReference_1_UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_CellularBars)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This,
        __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars** value);
    HRESULT (STDMETHODCALLTYPE* put_CellularBars)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This,
        __FIReference_1_Windows__CDevices__CWiFi__CWiFiOnDemandHotspotCellularBars* value);
    HRESULT (STDMETHODCALLTYPE* get_IsMetered)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsMetered)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Ssid)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Ssid)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Password)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** value);
    HRESULT (STDMETHODCALLTYPE* put_Password)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkPropertiesVtbl;

interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_get_Availability(This, value) \
    ((This)->lpVtbl->get_Availability(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_put_Availability(This, value) \
    ((This)->lpVtbl->put_Availability(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_get_RemainingBatteryPercent(This, value) \
    ((This)->lpVtbl->get_RemainingBatteryPercent(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_put_RemainingBatteryPercent(This, value) \
    ((This)->lpVtbl->put_RemainingBatteryPercent(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_get_CellularBars(This, value) \
    ((This)->lpVtbl->get_CellularBars(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_put_CellularBars(This, value) \
    ((This)->lpVtbl->put_CellularBars(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_get_IsMetered(This, value) \
    ((This)->lpVtbl->get_IsMetered(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_put_IsMetered(This, value) \
    ((This)->lpVtbl->put_IsMetered(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_get_Ssid(This, value) \
    ((This)->lpVtbl->get_Ssid(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_put_Ssid(This, value) \
    ((This)->lpVtbl->put_Ssid(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_get_Password(This, value) \
    ((This)->lpVtbl->get_Password(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_put_Password(This, value) \
    ((This)->lpVtbl->put_Password(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiOnDemandHotspotNetworkStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiOnDemandHotspotNetwork
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiOnDemandHotspotNetworkStatics[] = L"Windows.Devices.WiFi.IWiFiOnDemandHotspotNetworkStatics";
typedef struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetOrCreateById)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics* This,
        GUID networkId,
        __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetwork** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_GetOrCreateById(This, networkId, result) \
    ((This)->lpVtbl->GetOrCreateById(This, networkId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiOnDemandHotspotNetworkStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Devices.WiFi.IWiFiWpsConfigurationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.WiFi.WiFiWpsConfigurationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_WiFi_IWiFiWpsConfigurationResult[] = L"Windows.Devices.WiFi.IWiFiWpsConfigurationResult";
typedef struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult* This,
        enum __x_ABI_CWindows_CDevices_CWiFi_CWiFiWpsConfigurationStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedWpsKinds)(__x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult* This,
        __FIVectorView_1_Windows__CDevices__CWiFi__CWiFiWpsKind** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResultVtbl;

interface __x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_get_SupportedWpsKinds(This, value) \
    ((This)->lpVtbl->get_SupportedWpsKinds(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult;
#endif /* !defined(____x_ABI_CWindows_CDevices_CWiFi_CIWiFiWpsConfigurationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Devices.WiFi.WiFiAdapter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.WiFi.IWiFiAdapterStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFi.IWiFiAdapter ** Default Interface **
 *    Windows.Devices.WiFi.IWiFiAdapter2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFi_WiFiAdapter_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFi_WiFiAdapter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFi_WiFiAdapter[] = L"Windows.Devices.WiFi.WiFiAdapter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFi.WiFiAvailableNetwork
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFi.IWiFiAvailableNetwork ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFi_WiFiAvailableNetwork_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFi_WiFiAvailableNetwork_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFi_WiFiAvailableNetwork[] = L"Windows.Devices.WiFi.WiFiAvailableNetwork";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFi.WiFiConnectionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFi.IWiFiConnectionResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFi_WiFiConnectionResult_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFi_WiFiConnectionResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFi_WiFiConnectionResult[] = L"Windows.Devices.WiFi.WiFiConnectionResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFi.WiFiNetworkReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFi.IWiFiNetworkReport ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_WiFi_WiFiNetworkReport_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFi_WiFiNetworkReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFi_WiFiNetworkReport[] = L"Windows.Devices.WiFi.WiFiNetworkReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.WiFi.WiFiOnDemandHotspotConnectTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFi.IWiFiOnDemandHotspotConnectTriggerDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Devices_WiFi_WiFiOnDemandHotspotConnectTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFi_WiFiOnDemandHotspotConnectTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFi_WiFiOnDemandHotspotConnectTriggerDetails[] = L"Windows.Devices.WiFi.WiFiOnDemandHotspotConnectTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Devices.WiFi.WiFiOnDemandHotspotConnectionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFi.IWiFiOnDemandHotspotConnectionResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Devices_WiFi_WiFiOnDemandHotspotConnectionResult_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFi_WiFiOnDemandHotspotConnectionResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFi_WiFiOnDemandHotspotConnectionResult[] = L"Windows.Devices.WiFi.WiFiOnDemandHotspotConnectionResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Devices.WiFi.WiFiOnDemandHotspotNetwork
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.WiFi.IWiFiOnDemandHotspotNetworkStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFi.IWiFiOnDemandHotspotNetwork ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Devices_WiFi_WiFiOnDemandHotspotNetwork_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFi_WiFiOnDemandHotspotNetwork_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFi_WiFiOnDemandHotspotNetwork[] = L"Windows.Devices.WiFi.WiFiOnDemandHotspotNetwork";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Devices.WiFi.WiFiOnDemandHotspotNetworkProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFi.IWiFiOnDemandHotspotNetworkProperties ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Devices_WiFi_WiFiOnDemandHotspotNetworkProperties_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFi_WiFiOnDemandHotspotNetworkProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFi_WiFiOnDemandHotspotNetworkProperties[] = L"Windows.Devices.WiFi.WiFiOnDemandHotspotNetworkProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Devices.WiFi.WiFiWpsConfigurationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.WiFi.IWiFiWpsConfigurationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Devices_WiFi_WiFiWpsConfigurationResult_DEFINED
#define RUNTIMECLASS_Windows_Devices_WiFi_WiFiWpsConfigurationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_WiFi_WiFiWpsConfigurationResult[] = L"Windows.Devices.WiFi.WiFiWpsConfigurationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Edevices2Ewifi_p_h__

#endif // __windows2Edevices2Ewifi_h__
