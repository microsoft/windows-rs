
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
#ifndef __windows2Edevices2Ebluetooth2Erfcomm_h__
#define __windows2Edevices2Ebluetooth2Erfcomm_h__
#ifndef __windows2Edevices2Ebluetooth2Erfcomm_p_h__
#define __windows2Edevices2Ebluetooth2Erfcomm_p_h__


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
#include "Windows.Devices.Bluetooth.h"
#include "Windows.Devices.Enumeration.h"
#include "Windows.Networking.h"
#include "Windows.Networking.Sockets.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    interface IRfcommDeviceService;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommDeviceService

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    interface IRfcommDeviceService2;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2 ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommDeviceService2

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    interface IRfcommDeviceService3;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3 ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommDeviceService3

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    interface IRfcommDeviceServiceStatics;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommDeviceServiceStatics

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    interface IRfcommDeviceServiceStatics2;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2 ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommDeviceServiceStatics2

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    interface IRfcommDeviceServicesResult;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommDeviceServicesResult

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    interface IRfcommServiceId;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    interface IRfcommServiceIdStatics;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceIdStatics

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    interface IRfcommServiceProvider;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceProvider

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    interface IRfcommServiceProvider2;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2 ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceProvider2

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    interface IRfcommServiceProviderStatics;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceProviderStatics

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    class RfcommDeviceService;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0df56bd7-c8f6-5c32-9644-aa0bcf28d78c"))
IAsyncOperation<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*, ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommDeviceService*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*> __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_t;
#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5c772518-442f-58ed-80cb-538d34b88295"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*, ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommDeviceService*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    class RfcommServiceProvider;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fed44828-e232-554d-85d1-2f04d1322e69"))
IAsyncOperation<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommServiceProvider*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommServiceProvider*, ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceProvider*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommServiceProvider*> __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_t;
#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("446a7f50-8f2e-51f0-aebb-1bc3d192905f"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommServiceProvider*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommServiceProvider*, ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceProvider*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommServiceProvider*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                typedef enum DeviceAccessStatus : int DeviceAccessStatus;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c00bc2f2-a7f8-5f3f-80d1-2808ef6bca10"))
IAsyncOperation<enum ABI::Windows::Devices::Enumeration::DeviceAccessStatus> : IAsyncOperation_impl<enum ABI::Windows::Devices::Enumeration::DeviceAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Enumeration.DeviceAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Devices::Enumeration::DeviceAccessStatus> __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_t;
#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ee154d83-805b-53e8-8469-90715036d013"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Devices::Enumeration::DeviceAccessStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Devices::Enumeration::DeviceAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Enumeration.DeviceAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Devices::Enumeration::DeviceAccessStatus> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("82a3a3b7-e04a-5395-8487-7f94f1508ce7"))
IKeyValuePair<UINT32, ABI::Windows::Storage::Streams::IBuffer*> : IKeyValuePair_impl<UINT32, ABI::Windows::Storage::Streams::IBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<UInt32, Windows.Storage.Streams.IBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<UINT32, ABI::Windows::Storage::Streams::IBuffer*> __FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a295fa0c-c99f-5109-8ab9-91534bb48c9b"))
IIterator<__FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer*> : IIterator_impl<__FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<UInt32, Windows.Storage.Streams.IBuffer>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer*> __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4fe7fe23-22b1-528c-881d-a4eceaef0f11"))
IIterable<__FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer*> : IIterable_impl<__FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<UInt32, Windows.Storage.Streams.IBuffer>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer*> __FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("57dc41e6-8b4d-5910-9703-d7c668436852"))
IMapView<UINT32, ABI::Windows::Storage::Streams::IBuffer*> : IMapView_impl<UINT32, ABI::Windows::Storage::Streams::IBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<UInt32, Windows.Storage.Streams.IBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<UINT32, ABI::Windows::Storage::Streams::IBuffer*> __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::Collections::__FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d4904ded-bc1d-5933-aecf-e42c5d465bff"))
IAsyncOperation<__FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer*> : IAsyncOperation_impl<__FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IMapView`2<UInt32, Windows.Storage.Streams.IBuffer>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer*> __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::__FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("92c2e4d0-7c25-596b-9135-10d1472e6968"))
IAsyncOperationCompletedHandler<__FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer*> : IAsyncOperationCompletedHandler_impl<__FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IMapView`2<UInt32, Windows.Storage.Streams.IBuffer>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer*> __FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE
#define DEF___FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("64ab0132-c64c-5a87-8113-613ef356924c"))
IIterator<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*, ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommDeviceService*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*> __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_t;
#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE
#define DEF___FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3378e9a6-f6e2-50ea-bfee-b8109631feca"))
IIterable<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*, ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommDeviceService*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*> __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_t;
#define __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5d2591df-48c5-5734-9ef1-bd639b032007"))
IMap<UINT32, ABI::Windows::Storage::Streams::IBuffer*> : IMap_impl<UINT32, ABI::Windows::Storage::Streams::IBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<UInt32, Windows.Storage.Streams.IBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<UINT32, ABI::Windows::Storage::Streams::IBuffer*> __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::Collections::__FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE
#define DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("97df6b82-d15c-597e-ba69-492207a1c108"))
IVectorView<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*, ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommDeviceService*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*> __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_t;
#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                typedef enum BluetoothCacheMode : int BluetoothCacheMode;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                class BluetoothDevice;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothDevice;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice ABI::Windows::Devices::Bluetooth::IBluetoothDevice

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                typedef enum BluetoothError : int BluetoothError;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceAccessInformation;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceAccessInformation;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation ABI::Windows::Devices::Enumeration::IDeviceAccessInformation

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_FWD_DEFINED__

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
            class HostName;
        } /* Networking */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            interface IHostName;
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CIHostName ABI::Windows::Networking::IHostName

#endif // ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                typedef enum SocketProtectionLevel : int SocketProtectionLevel;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    class RfcommServiceId;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommDeviceService[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    MIDL_INTERFACE("ae81ff1f-c5a1-4c40-8c28-f3efd69062f3")
                    IRfcommDeviceService : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ConnectionHostName(
                            ABI::Windows::Networking::IHostName** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ConnectionServiceName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceId(
                            ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ProtectionLevel(
                            ABI::Windows::Networking::Sockets::SocketProtectionLevel* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MaxProtectionLevel(
                            ABI::Windows::Networking::Sockets::SocketProtectionLevel* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetSdpRawAttributesAsync(
                            __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer** asyncOp
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetSdpRawAttributesWithCacheModeAsync(
                            ABI::Windows::Devices::Bluetooth::BluetoothCacheMode cacheMode,
                            __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer** asyncOp
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRfcommDeviceService = __uuidof(IRfcommDeviceService);
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommDeviceService2[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    MIDL_INTERFACE("536ced14-ebcd-49fe-bf9f-40efc689b20d")
                    IRfcommDeviceService2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Device(
                            ABI::Windows::Devices::Bluetooth::IBluetoothDevice** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRfcommDeviceService2 = __uuidof(IRfcommDeviceService2);
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService2
 *     Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommDeviceService3[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService3";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    MIDL_INTERFACE("1c22ace6-dd44-4d23-866d-8f3486ee6490")
                    IRfcommDeviceService3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DeviceAccessInformation(
                            ABI::Windows::Devices::Enumeration::IDeviceAccessInformation** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RequestAccessAsync(
                            __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRfcommDeviceService3 = __uuidof(IRfcommDeviceService3);
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommDeviceServiceStatics[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    MIDL_INTERFACE("a4a149ef-626d-41ac-b253-87ac5c27e28a")
                    IRfcommDeviceServiceStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                            HSTRING deviceId,
                            __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService** asyncOp
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                            ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId* serviceId,
                            HSTRING* selector
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRfcommDeviceServiceStatics = __uuidof(IRfcommDeviceServiceStatics);
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommDeviceServiceStatics2[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    MIDL_INTERFACE("aa8cb1c9-e78d-4be4-8076-0a3d87a0a05f")
                    IRfcommDeviceServiceStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorForBluetoothDevice(
                            ABI::Windows::Devices::Bluetooth::IBluetoothDevice* bluetoothDevice,
                            HSTRING* selector
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorForBluetoothDeviceWithCacheMode(
                            ABI::Windows::Devices::Bluetooth::IBluetoothDevice* bluetoothDevice,
                            ABI::Windows::Devices::Bluetooth::BluetoothCacheMode cacheMode,
                            HSTRING* selector
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorForBluetoothDeviceAndServiceId(
                            ABI::Windows::Devices::Bluetooth::IBluetoothDevice* bluetoothDevice,
                            ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId* serviceId,
                            HSTRING* selector
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode(
                            ABI::Windows::Devices::Bluetooth::IBluetoothDevice* bluetoothDevice,
                            ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId* serviceId,
                            ABI::Windows::Devices::Bluetooth::BluetoothCacheMode cacheMode,
                            HSTRING* selector
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRfcommDeviceServiceStatics2 = __uuidof(IRfcommDeviceServiceStatics2);
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServicesResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceServicesResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommDeviceServicesResult[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServicesResult";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    MIDL_INTERFACE("3b48388c-7ccf-488e-9625-d259a5732d55")
                    IRfcommDeviceServicesResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Error(
                            ABI::Windows::Devices::Bluetooth::BluetoothError* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Services(
                            __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService** services
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRfcommDeviceServicesResult = __uuidof(IRfcommDeviceServicesResult);
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommServiceId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommServiceId[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceId";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    MIDL_INTERFACE("22629204-7e02-4017-8136-da1b6a1b9bbf")
                    IRfcommServiceId : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Uuid(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AsShortId(
                            UINT32* shortId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AsString(
                            HSTRING* id
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRfcommServiceId = __uuidof(IRfcommServiceId);
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceIdStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommServiceId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommServiceIdStatics[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceIdStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    MIDL_INTERFACE("2a179eba-a975-46e3-b56b-08ffd783a5fe")
                    IRfcommServiceIdStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE FromUuid(
                            GUID uuid,
                            ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId** serviceId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE FromShortId(
                            UINT32 shortId,
                            ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId** serviceId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SerialPort(
                            ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId** serviceId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ObexObjectPush(
                            ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId** serviceId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ObexFileTransfer(
                            ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId** serviceId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PhoneBookAccessPce(
                            ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId** serviceId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PhoneBookAccessPse(
                            ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId** serviceId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_GenericFileTransfer(
                            ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId** serviceId
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRfcommServiceIdStatics = __uuidof(IRfcommServiceIdStatics);
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommServiceProvider[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    MIDL_INTERFACE("eadbfdc4-b1f6-44ff-9f7c-e7a82ab86821")
                    IRfcommServiceProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceId(
                            ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SdpRawAttributes(
                            __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE StartAdvertising(
                            ABI::Windows::Networking::Sockets::IStreamSocketListener* listener
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE StopAdvertising(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRfcommServiceProvider = __uuidof(IRfcommServiceProvider);
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommServiceProvider2[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    MIDL_INTERFACE("736bdfc6-3c81-4d1e-baf2-ddbb81284512")
                    IRfcommServiceProvider2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE StartAdvertisingWithRadioDiscoverability(
                            ABI::Windows::Networking::Sockets::IStreamSocketListener* listener,
                            boolean radioDiscoverable
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRfcommServiceProvider2 = __uuidof(IRfcommServiceProvider2);
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProviderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommServiceProviderStatics[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProviderStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    MIDL_INTERFACE("98888303-69ca-413a-84f7-4344c7292997")
                    IRfcommServiceProviderStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateAsync(
                            ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId* serviceId,
                            __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider** asyncOp
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRfcommServiceProviderStatics = __uuidof(IRfcommServiceProviderStatics);
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService ** Default Interface **
 *    Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService2
 *    Windows.Foundation.IClosable
 *    Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Rfcomm_RfcommDeviceService_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Rfcomm_RfcommDeviceService_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Rfcomm_RfcommDeviceService[] = L"Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceServicesResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServicesResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Rfcomm_RfcommDeviceServicesResult_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Rfcomm_RfcommDeviceServicesResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Rfcomm_RfcommDeviceServicesResult[] = L"Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceServicesResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Devices.Bluetooth.Rfcomm.RfcommServiceId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceIdStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceId ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Rfcomm_RfcommServiceId_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Rfcomm_RfcommServiceId_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Rfcomm_RfcommServiceId[] = L"Windows.Devices.Bluetooth.Rfcomm.RfcommServiceId";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProviderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider ** Default Interface **
 *    Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Rfcomm_RfcommServiceProvider_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Rfcomm_RfcommServiceProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Rfcomm_RfcommServiceProvider[] = L"Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2 __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3 __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2 __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2 __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProviderVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider* This,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProviderVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceAccessStatus __x_ABI_CWindows_CDevices_CEnumeration_CDeviceAccessStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceAccessStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatusVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer __FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);

    END_INTERFACE
} __FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        __FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer __FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        __FIIterator_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        UINT32 key,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        UINT32 key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer** first,
        __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer** second);

    END_INTERFACE
} __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer __FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        __FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        __FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer __FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService;

typedef struct __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl;

interface __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService;

typedef struct __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl;

interface __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        UINT32 key,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        UINT32 key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        __FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        UINT32 key,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This,
        UINT32 key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer* This);

    END_INTERFACE
} __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService;

typedef struct __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl;

interface __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothCacheMode __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothCacheMode;

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothError __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothError;

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CIHostName __x_ABI_CWindows_CNetworking_CIHostName;

#endif // ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CNetworking_CSockets_CSocketProtectionLevel __x_ABI_CWindows_CNetworking_CSockets_CSocketProtectionLevel;

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_FWD_DEFINED__

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommDeviceService[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionHostName)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionServiceName)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceId)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId** value);
    HRESULT (STDMETHODCALLTYPE* get_ProtectionLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketProtectionLevel* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxProtectionLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketProtectionLevel* value);
    HRESULT (STDMETHODCALLTYPE* GetSdpRawAttributesAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService* This,
        __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer** asyncOp);
    HRESULT (STDMETHODCALLTYPE* GetSdpRawAttributesWithCacheModeAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothCacheMode cacheMode,
        __FIAsyncOperation_1___FIMapView_2_UINT32_Windows__CStorage__CStreams__CIBuffer** asyncOp);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_get_ConnectionHostName(This, value) \
    ((This)->lpVtbl->get_ConnectionHostName(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_get_ConnectionServiceName(This, value) \
    ((This)->lpVtbl->get_ConnectionServiceName(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_get_ServiceId(This, value) \
    ((This)->lpVtbl->get_ServiceId(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_get_ProtectionLevel(This, value) \
    ((This)->lpVtbl->get_ProtectionLevel(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_get_MaxProtectionLevel(This, value) \
    ((This)->lpVtbl->get_MaxProtectionLevel(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_GetSdpRawAttributesAsync(This, asyncOp) \
    ((This)->lpVtbl->GetSdpRawAttributesAsync(This, asyncOp))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_GetSdpRawAttributesWithCacheModeAsync(This, cacheMode, asyncOp) \
    ((This)->lpVtbl->GetSdpRawAttributesWithCacheModeAsync(This, cacheMode, asyncOp))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommDeviceService2[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService2";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Device)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_get_Device(This, value) \
    ((This)->lpVtbl->get_Device(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService2
 *     Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommDeviceService3[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService3";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceAccessInformation)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation** value);
    HRESULT (STDMETHODCALLTYPE* RequestAccessAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3* This,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_get_DeviceAccessInformation(This, value) \
    ((This)->lpVtbl->get_DeviceAccessInformation(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_RequestAccessAsync(This, value) \
    ((This)->lpVtbl->RequestAccessAsync(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommDeviceServiceStatics[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics* This,
        HSTRING deviceId,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService** asyncOp);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId* serviceId,
        HSTRING* selector);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_FromIdAsync(This, deviceId, asyncOp) \
    ((This)->lpVtbl->FromIdAsync(This, deviceId, asyncOp))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_GetDeviceSelector(This, serviceId, selector) \
    ((This)->lpVtbl->GetDeviceSelector(This, serviceId, selector))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommDeviceServiceStatics2[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics2";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorForBluetoothDevice)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* bluetoothDevice,
        HSTRING* selector);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorForBluetoothDeviceWithCacheMode)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* bluetoothDevice,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothCacheMode cacheMode,
        HSTRING* selector);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorForBluetoothDeviceAndServiceId)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* bluetoothDevice,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId* serviceId,
        HSTRING* selector);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* bluetoothDevice,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId* serviceId,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothCacheMode cacheMode,
        HSTRING* selector);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_GetDeviceSelectorForBluetoothDevice(This, bluetoothDevice, selector) \
    ((This)->lpVtbl->GetDeviceSelectorForBluetoothDevice(This, bluetoothDevice, selector))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_GetDeviceSelectorForBluetoothDeviceWithCacheMode(This, bluetoothDevice, cacheMode, selector) \
    ((This)->lpVtbl->GetDeviceSelectorForBluetoothDeviceWithCacheMode(This, bluetoothDevice, cacheMode, selector))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_GetDeviceSelectorForBluetoothDeviceAndServiceId(This, bluetoothDevice, serviceId, selector) \
    ((This)->lpVtbl->GetDeviceSelectorForBluetoothDeviceAndServiceId(This, bluetoothDevice, serviceId, selector))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode(This, bluetoothDevice, serviceId, cacheMode, selector) \
    ((This)->lpVtbl->GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode(This, bluetoothDevice, serviceId, cacheMode, selector))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServiceStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServicesResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceServicesResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommDeviceServicesResult[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServicesResult";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Error)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothError* value);
    HRESULT (STDMETHODCALLTYPE* get_Services)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult* This,
        __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService** services);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResultVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_get_Error(This, value) \
    ((This)->lpVtbl->get_Error(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_get_Services(This, services) \
    ((This)->lpVtbl->get_Services(This, services))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommServiceId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommServiceId[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceId";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Uuid)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* AsShortId)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId* This,
        UINT32* shortId);
    HRESULT (STDMETHODCALLTYPE* AsString)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId* This,
        HSTRING* id);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_get_Uuid(This, value) \
    ((This)->lpVtbl->get_Uuid(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_AsShortId(This, shortId) \
    ((This)->lpVtbl->AsShortId(This, shortId))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_AsString(This, id) \
    ((This)->lpVtbl->AsString(This, id))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceIdStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommServiceId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommServiceIdStatics[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceIdStatics";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromUuid)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics* This,
        GUID uuid,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId** serviceId);
    HRESULT (STDMETHODCALLTYPE* FromShortId)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics* This,
        UINT32 shortId,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId** serviceId);
    HRESULT (STDMETHODCALLTYPE* get_SerialPort)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId** serviceId);
    HRESULT (STDMETHODCALLTYPE* get_ObexObjectPush)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId** serviceId);
    HRESULT (STDMETHODCALLTYPE* get_ObexFileTransfer)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId** serviceId);
    HRESULT (STDMETHODCALLTYPE* get_PhoneBookAccessPce)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId** serviceId);
    HRESULT (STDMETHODCALLTYPE* get_PhoneBookAccessPse)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId** serviceId);
    HRESULT (STDMETHODCALLTYPE* get_GenericFileTransfer)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId** serviceId);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_FromUuid(This, uuid, serviceId) \
    ((This)->lpVtbl->FromUuid(This, uuid, serviceId))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_FromShortId(This, shortId, serviceId) \
    ((This)->lpVtbl->FromShortId(This, shortId, serviceId))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_get_SerialPort(This, serviceId) \
    ((This)->lpVtbl->get_SerialPort(This, serviceId))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_get_ObexObjectPush(This, serviceId) \
    ((This)->lpVtbl->get_ObexObjectPush(This, serviceId))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_get_ObexFileTransfer(This, serviceId) \
    ((This)->lpVtbl->get_ObexFileTransfer(This, serviceId))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_get_PhoneBookAccessPce(This, serviceId) \
    ((This)->lpVtbl->get_PhoneBookAccessPce(This, serviceId))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_get_PhoneBookAccessPse(This, serviceId) \
    ((This)->lpVtbl->get_PhoneBookAccessPse(This, serviceId))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_get_GenericFileTransfer(This, serviceId) \
    ((This)->lpVtbl->get_GenericFileTransfer(This, serviceId))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceIdStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommServiceProvider[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ServiceId)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId** value);
    HRESULT (STDMETHODCALLTYPE* get_SdpRawAttributes)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider* This,
        __FIMap_2_UINT32_Windows__CStorage__CStreams__CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* StartAdvertising)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener* listener);
    HRESULT (STDMETHODCALLTYPE* StopAdvertising)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider* This);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_get_ServiceId(This, value) \
    ((This)->lpVtbl->get_ServiceId(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_get_SdpRawAttributes(This, value) \
    ((This)->lpVtbl->get_SdpRawAttributes(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_StartAdvertising(This, listener) \
    ((This)->lpVtbl->StartAdvertising(This, listener))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_StopAdvertising(This) \
    ((This)->lpVtbl->StopAdvertising(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommServiceProvider2[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider2";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* StartAdvertisingWithRadioDiscoverability)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener* listener,
        boolean radioDiscoverable);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_StartAdvertisingWithRadioDiscoverability(This, listener, radioDiscoverable) \
    ((This)->lpVtbl->StartAdvertisingWithRadioDiscoverability(This, listener, radioDiscoverable))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProvider2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProviderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Rfcomm_IRfcommServiceProviderStatics[] = L"Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProviderStatics";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId* serviceId,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommServiceProvider** asyncOp);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_CreateAsync(This, serviceId, asyncOp) \
    ((This)->lpVtbl->CreateAsync(This, serviceId, asyncOp))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService ** Default Interface **
 *    Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService2
 *    Windows.Foundation.IClosable
 *    Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Rfcomm_RfcommDeviceService_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Rfcomm_RfcommDeviceService_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Rfcomm_RfcommDeviceService[] = L"Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceServicesResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServicesResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Rfcomm_RfcommDeviceServicesResult_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Rfcomm_RfcommDeviceServicesResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Rfcomm_RfcommDeviceServicesResult[] = L"Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceServicesResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Devices.Bluetooth.Rfcomm.RfcommServiceId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceIdStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceId ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Rfcomm_RfcommServiceId_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Rfcomm_RfcommServiceId_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Rfcomm_RfcommServiceId[] = L"Windows.Devices.Bluetooth.Rfcomm.RfcommServiceId";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProviderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider ** Default Interface **
 *    Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Rfcomm_RfcommServiceProvider_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Rfcomm_RfcommServiceProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Rfcomm_RfcommServiceProvider[] = L"Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider";
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
#endif // __windows2Edevices2Ebluetooth2Erfcomm_p_h__

#endif // __windows2Edevices2Ebluetooth2Erfcomm_h__
