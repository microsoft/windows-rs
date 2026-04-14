
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
#ifndef __windows2Edevices2Ehumaninterfacedevice_h__
#define __windows2Edevices2Ehumaninterfacedevice_h__
#ifndef __windows2Edevices2Ehumaninterfacedevice_p_h__
#define __windows2Edevices2Ehumaninterfacedevice_p_h__


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
#include "Windows.Storage.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                interface IHidBooleanControl;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControl

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                interface IHidBooleanControlDescription;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControlDescription

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                interface IHidBooleanControlDescription2;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2 ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControlDescription2

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                interface IHidCollection;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection ABI::Windows::Devices::HumanInterfaceDevice::IHidCollection

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                interface IHidDevice;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice ABI::Windows::Devices::HumanInterfaceDevice::IHidDevice

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                interface IHidDeviceStatics;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics ABI::Windows::Devices::HumanInterfaceDevice::IHidDeviceStatics

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                interface IHidFeatureReport;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport ABI::Windows::Devices::HumanInterfaceDevice::IHidFeatureReport

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                interface IHidInputReport;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport ABI::Windows::Devices::HumanInterfaceDevice::IHidInputReport

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                interface IHidInputReportReceivedEventArgs;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs ABI::Windows::Devices::HumanInterfaceDevice::IHidInputReportReceivedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                interface IHidNumericControl;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControl

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                interface IHidNumericControlDescription;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControlDescription

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                interface IHidOutputReport;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport ABI::Windows::Devices::HumanInterfaceDevice::IHidOutputReport

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIAsyncOperation_1_UINT32_USE
#define DEF___FIAsyncOperation_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ef60385f-be78-584b-aaef-7829ada2b0de"))
IAsyncOperation<UINT32> : IAsyncOperation_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<UINT32> __FIAsyncOperation_1_UINT32_t;
#define __FIAsyncOperation_1_UINT32 ABI::Windows::Foundation::__FIAsyncOperation_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_UINT32_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE
#define DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9343b6e7-e3d2-5e4a-ab2d-2bce4919a6a4"))
IAsyncOperationCompletedHandler<UINT32> : IAsyncOperationCompletedHandler_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<UINT32> __FIAsyncOperationCompletedHandler_1_UINT32_t;
#define __FIAsyncOperationCompletedHandler_1_UINT32 ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE */


namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                class HidDevice;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a76a4fbf-5177-5256-84a8-b31a8dcf1048"))
IAsyncOperation<ABI::Windows::Devices::HumanInterfaceDevice::HidDevice*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidDevice*, ABI::Windows::Devices::HumanInterfaceDevice::IHidDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.HumanInterfaceDevice.HidDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::HumanInterfaceDevice::HidDevice*> __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_t;
#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b0e8e149-0cb6-55a7-bcc1-d996324d65c4"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::HumanInterfaceDevice::HidDevice*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidDevice*, ABI::Windows::Devices::HumanInterfaceDevice::IHidDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.HumanInterfaceDevice.HidDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::HumanInterfaceDevice::HidDevice*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                class HidFeatureReport;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d72fb6f9-42f6-5f45-bfe3-29af247c2e85"))
IAsyncOperation<ABI::Windows::Devices::HumanInterfaceDevice::HidFeatureReport*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidFeatureReport*, ABI::Windows::Devices::HumanInterfaceDevice::IHidFeatureReport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.HumanInterfaceDevice.HidFeatureReport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::HumanInterfaceDevice::HidFeatureReport*> __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_t;
#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("db643555-3d16-57fe-b7ef-2bdbd719fdbf"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::HumanInterfaceDevice::HidFeatureReport*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidFeatureReport*, ABI::Windows::Devices::HumanInterfaceDevice::IHidFeatureReport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.HumanInterfaceDevice.HidFeatureReport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::HumanInterfaceDevice::HidFeatureReport*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                class HidInputReport;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b3e28917-cd48-57b3-a0b1-321432e85bd6"))
IAsyncOperation<ABI::Windows::Devices::HumanInterfaceDevice::HidInputReport*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidInputReport*, ABI::Windows::Devices::HumanInterfaceDevice::IHidInputReport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.HumanInterfaceDevice.HidInputReport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::HumanInterfaceDevice::HidInputReport*> __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_t;
#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("01c83770-03ab-5576-98b4-8d75ce1a9885"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::HumanInterfaceDevice::HidInputReport*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidInputReport*, ABI::Windows::Devices::HumanInterfaceDevice::IHidInputReport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.HumanInterfaceDevice.HidInputReport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::HumanInterfaceDevice::HidInputReport*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                class HidBooleanControl;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_USE
#define DEF___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5cde3c23-d054-53d6-abf1-41e73379b472"))
IIterator<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControl*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControl*, ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControl*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.HumanInterfaceDevice.HidBooleanControl>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControl*> __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_t;
#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_USE
#define DEF___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1111e585-5ab0-5d2b-8aed-b6d6186d1c3f"))
IIterable<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControl*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControl*, ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControl*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.HumanInterfaceDevice.HidBooleanControl>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControl*> __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_t;
#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                class HidBooleanControlDescription;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_USE
#define DEF___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("203203b0-b7f4-542d-b0d0-9caa1fb55d7f"))
IIterator<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControlDescription*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControlDescription*, ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControlDescription*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControlDescription*> __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_t;
#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_USE
#define DEF___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d0ff0fed-a156-58bf-9411-5777df9d57bf"))
IIterable<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControlDescription*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControlDescription*, ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControlDescription*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControlDescription*> __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_t;
#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                class HidCollection;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_USE
#define DEF___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cefcee70-c7ff-57c1-a675-a0df8976a988"))
IIterator<ABI::Windows::Devices::HumanInterfaceDevice::HidCollection*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidCollection*, ABI::Windows::Devices::HumanInterfaceDevice::IHidCollection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.HumanInterfaceDevice.HidCollection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::HumanInterfaceDevice::HidCollection*> __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_t;
#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_USE
#define DEF___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bbeada0f-708f-5b5e-a017-5c64ffb96b69"))
IIterable<ABI::Windows::Devices::HumanInterfaceDevice::HidCollection*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidCollection*, ABI::Windows::Devices::HumanInterfaceDevice::IHidCollection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.HumanInterfaceDevice.HidCollection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::HumanInterfaceDevice::HidCollection*> __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_t;
#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                class HidNumericControlDescription;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_USE
#define DEF___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("52b9c36e-7d95-5d1c-acab-23c19ea76f01"))
IIterator<ABI::Windows::Devices::HumanInterfaceDevice::HidNumericControlDescription*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidNumericControlDescription*, ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControlDescription*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::HumanInterfaceDevice::HidNumericControlDescription*> __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_t;
#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_USE
#define DEF___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("868f060d-e0d4-571b-b2f7-431d6984a513"))
IIterable<ABI::Windows::Devices::HumanInterfaceDevice::HidNumericControlDescription*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidNumericControlDescription*, ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControlDescription*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::HumanInterfaceDevice::HidNumericControlDescription*> __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_t;
#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_USE
#define DEF___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0e417dac-591a-5de0-afd6-0b2c04c304e7"))
IVectorView<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControl*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControl*, ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControl*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.HumanInterfaceDevice.HidBooleanControl>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControl*> __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_t;
#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_USE
#define DEF___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("aab72786-ec34-536f-a7c5-27394753df2c"))
IVectorView<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControlDescription*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControlDescription*, ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControlDescription*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControlDescription*> __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_t;
#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_USE
#define DEF___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("96f1faac-348f-5b8f-a71d-2d650e0b11f2"))
IVectorView<ABI::Windows::Devices::HumanInterfaceDevice::HidCollection*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidCollection*, ABI::Windows::Devices::HumanInterfaceDevice::IHidCollection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.HumanInterfaceDevice.HidCollection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::HumanInterfaceDevice::HidCollection*> __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_t;
#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_USE
#define DEF___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e02ca66c-610a-51b4-aef9-3707b697b985"))
IVectorView<ABI::Windows::Devices::HumanInterfaceDevice::HidNumericControlDescription*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidNumericControlDescription*, ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControlDescription*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::HumanInterfaceDevice::HidNumericControlDescription*> __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_t;
#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                class HidInputReportReceivedEventArgs;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("31e757c8-8f6a-540b-938b-aba79b6f03ec"))
ITypedEventHandler<ABI::Windows::Devices::HumanInterfaceDevice::HidDevice*, ABI::Windows::Devices::HumanInterfaceDevice::HidInputReportReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidDevice*, ABI::Windows::Devices::HumanInterfaceDevice::IHidDevice*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidInputReportReceivedEventArgs*, ABI::Windows::Devices::HumanInterfaceDevice::IHidInputReportReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.HumanInterfaceDevice.HidDevice, Windows.Devices.HumanInterfaceDevice.HidInputReportReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::HumanInterfaceDevice::HidDevice*, ABI::Windows::Devices::HumanInterfaceDevice::HidInputReportReceivedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
        namespace Storage {
            typedef enum FileAccessMode : int FileAccessMode;
        } /* Storage */
    } /* Windows */
} /* ABI */

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
            namespace HumanInterfaceDevice {
                typedef enum HidCollectionType : int HidCollectionType;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                typedef enum HidReportType : int HidReportType;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                class HidNumericControl;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                class HidOutputReport;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.HumanInterfaceDevice.HidCollectionType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                enum HidCollectionType : int
                {
                    HidCollectionType_Physical = 0,
                    HidCollectionType_Application = 1,
                    HidCollectionType_Logical = 2,
                    HidCollectionType_Report = 3,
                    HidCollectionType_NamedArray = 4,
                    HidCollectionType_UsageSwitch = 5,
                    HidCollectionType_UsageModifier = 6,
                    HidCollectionType_Other = 7,
                };
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.HumanInterfaceDevice.HidReportType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                enum HidReportType : int
                {
                    HidReportType_Input = 0,
                    HidReportType_Output = 1,
                    HidReportType_Feature = 2,
                };
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidBooleanControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidBooleanControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidBooleanControl[] = L"Windows.Devices.HumanInterfaceDevice.IHidBooleanControl";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                MIDL_INTERFACE("524df48a-3695-408c-bba2-e2eb5abfbc20")
                IHidBooleanControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UsagePage(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UsageId(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsActive(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsActive(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ControlDescription(
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControlDescription** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHidBooleanControl = __uuidof(IHidBooleanControl);
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidBooleanControlDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidBooleanControlDescription[] = L"Windows.Devices.HumanInterfaceDevice.IHidBooleanControlDescription";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                MIDL_INTERFACE("6196e543-29d8-4a2a-8683-849e207bbe31")
                IHidBooleanControlDescription : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReportId(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReportType(
                        ABI::Windows::Devices::HumanInterfaceDevice::HidReportType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UsagePage(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UsageId(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ParentCollections(
                        __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHidBooleanControlDescription = __uuidof(IHidBooleanControlDescription);
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidBooleanControlDescription2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidBooleanControlDescription2[] = L"Windows.Devices.HumanInterfaceDevice.IHidBooleanControlDescription2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                MIDL_INTERFACE("c8eed2ea-8a77-4c36-aa00-5ff0449d3e73")
                IHidBooleanControlDescription2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsAbsolute(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHidBooleanControlDescription2 = __uuidof(IHidBooleanControlDescription2);
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidCollection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidCollection[] = L"Windows.Devices.HumanInterfaceDevice.IHidCollection";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                MIDL_INTERFACE("7189f5a3-32f1-46e3-befd-44d2663b7e6a")
                IHidCollection : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Type(
                        ABI::Windows::Devices::HumanInterfaceDevice::HidCollectionType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UsagePage(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UsageId(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHidCollection = __uuidof(IHidCollection);
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidDevice
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidDevice[] = L"Windows.Devices.HumanInterfaceDevice.IHidDevice";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                MIDL_INTERFACE("5f8a14e7-2200-432e-95da-d09b87d574a8")
                IHidDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_VendorId(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProductId(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Version(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UsagePage(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UsageId(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetInputReportAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetInputReportByIdAsync(
                        UINT16 reportId,
                        __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetFeatureReportAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetFeatureReportByIdAsync(
                        UINT16 reportId,
                        __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateOutputReport(
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidOutputReport** outputReport
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateOutputReportById(
                        UINT16 reportId,
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidOutputReport** outputReport
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFeatureReport(
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidFeatureReport** featureReport
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFeatureReportById(
                        UINT16 reportId,
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidFeatureReport** featureReport
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendOutputReportAsync(
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidOutputReport* outputReport,
                        __FIAsyncOperation_1_UINT32** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendFeatureReportAsync(
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidFeatureReport* featureReport,
                        __FIAsyncOperation_1_UINT32** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetBooleanControlDescriptions(
                        ABI::Windows::Devices::HumanInterfaceDevice::HidReportType reportType,
                        UINT16 usagePage,
                        UINT16 usageId,
                        __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetNumericControlDescriptions(
                        ABI::Windows::Devices::HumanInterfaceDevice::HidReportType reportType,
                        UINT16 usagePage,
                        UINT16 usageId,
                        __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_InputReportReceived(
                        __FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs* reportHandler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_InputReportReceived(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHidDevice = __uuidof(IHidDevice);
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidDeviceStatics[] = L"Windows.Devices.HumanInterfaceDevice.IHidDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                MIDL_INTERFACE("9e5981e4-9856-418c-9f73-77de0cd85754")
                IHidDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        UINT16 usagePage,
                        UINT16 usageId,
                        HSTRING* selector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorVidPid(
                        UINT16 usagePage,
                        UINT16 usageId,
                        UINT16 vendorId,
                        UINT16 productId,
                        HSTRING* selector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING deviceId,
                        ABI::Windows::Storage::FileAccessMode accessMode,
                        __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice** hidDevice
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHidDeviceStatics = __uuidof(IHidDeviceStatics);
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidFeatureReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidFeatureReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidFeatureReport[] = L"Windows.Devices.HumanInterfaceDevice.IHidFeatureReport";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                MIDL_INTERFACE("841d9b79-5ae5-46e3-82ef-1fec5c8942f4")
                IHidFeatureReport : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Data(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Data(
                        ABI::Windows::Storage::Streams::IBuffer* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetBooleanControl(
                        UINT16 usagePage,
                        UINT16 usageId,
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetBooleanControlByDescription(
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControlDescription* controlDescription,
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetNumericControl(
                        UINT16 usagePage,
                        UINT16 usageId,
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetNumericControlByDescription(
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControlDescription* controlDescription,
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControl** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHidFeatureReport = __uuidof(IHidFeatureReport);
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidInputReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidInputReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidInputReport[] = L"Windows.Devices.HumanInterfaceDevice.IHidInputReport";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                MIDL_INTERFACE("c35d0e50-f7e7-4e8d-b23e-cabbe56b90e9")
                IHidInputReport : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Data(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActivatedBooleanControls(
                        __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransitionedBooleanControls(
                        __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetBooleanControl(
                        UINT16 usagePage,
                        UINT16 usageId,
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetBooleanControlByDescription(
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControlDescription* controlDescription,
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetNumericControl(
                        UINT16 usagePage,
                        UINT16 usageId,
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetNumericControlByDescription(
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControlDescription* controlDescription,
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControl** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHidInputReport = __uuidof(IHidInputReport);
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidInputReportReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidInputReportReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidInputReportReceivedEventArgs[] = L"Windows.Devices.HumanInterfaceDevice.IHidInputReportReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                MIDL_INTERFACE("7059c5cb-59b2-4dc2-985c-0adc6136fa2d")
                IHidInputReportReceivedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Report(
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidInputReport** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHidInputReportReceivedEventArgs = __uuidof(IHidInputReportReceivedEventArgs);
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidNumericControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidNumericControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidNumericControl[] = L"Windows.Devices.HumanInterfaceDevice.IHidNumericControl";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                MIDL_INTERFACE("e38a12a5-35a7-4b75-89c8-fb1f28b10823")
                IHidNumericControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsGrouped(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UsagePage(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UsageId(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        INT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Value(
                        INT64 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ScaledValue(
                        INT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ScaledValue(
                        INT64 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ControlDescription(
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControlDescription** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHidNumericControl = __uuidof(IHidNumericControl);
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidNumericControlDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidNumericControlDescription[] = L"Windows.Devices.HumanInterfaceDevice.IHidNumericControlDescription";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                MIDL_INTERFACE("638d5e86-1d97-4c75-927f-5ff58ba05e32")
                IHidNumericControlDescription : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReportId(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReportType(
                        ABI::Windows::Devices::HumanInterfaceDevice::HidReportType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReportSize(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReportCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UsagePage(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UsageId(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LogicalMinimum(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LogicalMaximum(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PhysicalMinimum(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PhysicalMaximum(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UnitExponent(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Unit(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsAbsolute(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasNull(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ParentCollections(
                        __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHidNumericControlDescription = __uuidof(IHidNumericControlDescription);
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidOutputReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidOutputReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidOutputReport[] = L"Windows.Devices.HumanInterfaceDevice.IHidOutputReport";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                MIDL_INTERFACE("62cb2544-c896-4463-93c1-df9db053c450")
                IHidOutputReport : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Data(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Data(
                        ABI::Windows::Storage::Streams::IBuffer* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetBooleanControl(
                        UINT16 usagePage,
                        UINT16 usageId,
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetBooleanControlByDescription(
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControlDescription* controlDescription,
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetNumericControl(
                        UINT16 usagePage,
                        UINT16 usageId,
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetNumericControlByDescription(
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControlDescription* controlDescription,
                        ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControl** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHidOutputReport = __uuidof(IHidOutputReport);
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidBooleanControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidBooleanControl ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidBooleanControl_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidBooleanControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidBooleanControl[] = L"Windows.Devices.HumanInterfaceDevice.HidBooleanControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidBooleanControlDescription ** Default Interface **
 *    Windows.Devices.HumanInterfaceDevice.IHidBooleanControlDescription2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidBooleanControlDescription_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidBooleanControlDescription_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidBooleanControlDescription[] = L"Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidCollection ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidCollection_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidCollection[] = L"Windows.Devices.HumanInterfaceDevice.HidCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.HumanInterfaceDevice.IHidDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidDevice ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidDevice[] = L"Windows.Devices.HumanInterfaceDevice.HidDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidFeatureReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidFeatureReport ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidFeatureReport_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidFeatureReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidFeatureReport[] = L"Windows.Devices.HumanInterfaceDevice.HidFeatureReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidInputReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidInputReport ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidInputReport_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidInputReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidInputReport[] = L"Windows.Devices.HumanInterfaceDevice.HidInputReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidInputReportReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidInputReportReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidInputReportReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidInputReportReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidInputReportReceivedEventArgs[] = L"Windows.Devices.HumanInterfaceDevice.HidInputReportReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidNumericControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidNumericControl ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidNumericControl_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidNumericControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidNumericControl[] = L"Windows.Devices.HumanInterfaceDevice.HidNumericControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidNumericControlDescription ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidNumericControlDescription_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidNumericControlDescription_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidNumericControlDescription[] = L"Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidOutputReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidOutputReport ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidOutputReport_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidOutputReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidOutputReport[] = L"Windows.Devices.HumanInterfaceDevice.HidOutputReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl;

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription;

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2 __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2;

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection;

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice;

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics;

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport;

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport;

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl;

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription;

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport;

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_UINT32 __FIAsyncOperationCompletedHandler_1_UINT32;

#if !defined(____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_UINT32 __FIAsyncOperation_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_UINT32;

typedef struct __FIAsyncOperation_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_UINT32* This,
        __FIAsyncOperationCompletedHandler_1_UINT32* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_UINT32* This,
        __FIAsyncOperationCompletedHandler_1_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_UINT32* This,
        UINT32* result);

    END_INTERFACE
} __FIAsyncOperation_1_UINT32Vtbl;

interface __FIAsyncOperation_1_UINT32
{
    CONST_VTBL struct __FIAsyncOperation_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_UINT32_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_UINT32_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_UINT32_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_UINT32 __FIAsyncOperationCompletedHandler_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_UINT32;

typedef struct __FIAsyncOperationCompletedHandler_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_UINT32* This,
        __FIAsyncOperation_1_UINT32* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_UINT32Vtbl;

interface __FIAsyncOperationCompletedHandler_1_UINT32
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_UINT32_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDeviceVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice* This,
        __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDeviceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReportVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport* This,
        __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReportVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport* This,
        __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl;

typedef struct __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlVtbl;

interface __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl;

typedef struct __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlVtbl;

interface __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription;

typedef struct __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescriptionVtbl;

interface __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription;

typedef struct __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescriptionVtbl;

interface __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection;

typedef struct __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollectionVtbl;

interface __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection;

typedef struct __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollectionVtbl;

interface __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription;

typedef struct __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescriptionVtbl;

interface __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription;

typedef struct __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescriptionVtbl;

interface __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl;

typedef struct __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlVtbl;

interface __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription;

typedef struct __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescriptionVtbl;

interface __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection;

typedef struct __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollectionVtbl;

interface __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription;

typedef struct __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescriptionVtbl;

interface __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs __FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* sender,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CStorage_CFileAccessMode __x_ABI_CWindows_CStorage_CFileAccessMode;

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CHidCollectionType __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CHidCollectionType;

typedef enum __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CHidReportType __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CHidReportType;

/*
 *
 * Struct Windows.Devices.HumanInterfaceDevice.HidCollectionType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CHidCollectionType
{
    HidCollectionType_Physical = 0,
    HidCollectionType_Application = 1,
    HidCollectionType_Logical = 2,
    HidCollectionType_Report = 3,
    HidCollectionType_NamedArray = 4,
    HidCollectionType_UsageSwitch = 5,
    HidCollectionType_UsageModifier = 6,
    HidCollectionType_Other = 7,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.HumanInterfaceDevice.HidReportType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CHidReportType
{
    HidReportType_Input = 0,
    HidReportType_Output = 1,
    HidReportType_Feature = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidBooleanControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidBooleanControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidBooleanControl[] = L"Windows.Devices.HumanInterfaceDevice.IHidBooleanControl";
typedef struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_UsagePage)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_UsageId)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_IsActive)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsActive)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ControlDescription)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlVtbl;

interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_get_UsagePage(This, value) \
    ((This)->lpVtbl->get_UsagePage(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_get_UsageId(This, value) \
    ((This)->lpVtbl->get_UsageId(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_get_IsActive(This, value) \
    ((This)->lpVtbl->get_IsActive(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_put_IsActive(This, value) \
    ((This)->lpVtbl->put_IsActive(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_get_ControlDescription(This, value) \
    ((This)->lpVtbl->get_ControlDescription(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidBooleanControlDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidBooleanControlDescription[] = L"Windows.Devices.HumanInterfaceDevice.IHidBooleanControlDescription";
typedef struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ReportId)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_ReportType)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription* This,
        enum __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CHidReportType* value);
    HRESULT (STDMETHODCALLTYPE* get_UsagePage)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_UsageId)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_ParentCollections)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription* This,
        __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescriptionVtbl;

interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_get_ReportId(This, value) \
    ((This)->lpVtbl->get_ReportId(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_get_ReportType(This, value) \
    ((This)->lpVtbl->get_ReportType(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_get_UsagePage(This, value) \
    ((This)->lpVtbl->get_UsagePage(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_get_UsageId(This, value) \
    ((This)->lpVtbl->get_UsageId(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_get_ParentCollections(This, value) \
    ((This)->lpVtbl->get_ParentCollections(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidBooleanControlDescription2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidBooleanControlDescription2[] = L"Windows.Devices.HumanInterfaceDevice.IHidBooleanControlDescription2";
typedef struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsAbsolute)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2Vtbl;

interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_get_IsAbsolute(This, value) \
    ((This)->lpVtbl->get_IsAbsolute(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidCollection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidCollection[] = L"Windows.Devices.HumanInterfaceDevice.IHidCollection";
typedef struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection* This,
        enum __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CHidCollectionType* value);
    HRESULT (STDMETHODCALLTYPE* get_UsagePage)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_UsageId)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollectionVtbl;

interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_get_UsagePage(This, value) \
    ((This)->lpVtbl->get_UsagePage(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_get_UsageId(This, value) \
    ((This)->lpVtbl->get_UsageId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidCollection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidDevice
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidDevice[] = L"Windows.Devices.HumanInterfaceDevice.IHidDevice";
typedef struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_VendorId)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_ProductId)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Version)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_UsagePage)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_UsageId)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* GetInputReportAsync)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport** value);
    HRESULT (STDMETHODCALLTYPE* GetInputReportByIdAsync)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        UINT16 reportId,
        __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidInputReport** value);
    HRESULT (STDMETHODCALLTYPE* GetFeatureReportAsync)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport** value);
    HRESULT (STDMETHODCALLTYPE* GetFeatureReportByIdAsync)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        UINT16 reportId,
        __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidFeatureReport** value);
    HRESULT (STDMETHODCALLTYPE* CreateOutputReport)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport** outputReport);
    HRESULT (STDMETHODCALLTYPE* CreateOutputReportById)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        UINT16 reportId,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport** outputReport);
    HRESULT (STDMETHODCALLTYPE* CreateFeatureReport)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport** featureReport);
    HRESULT (STDMETHODCALLTYPE* CreateFeatureReportById)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        UINT16 reportId,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport** featureReport);
    HRESULT (STDMETHODCALLTYPE* SendOutputReportAsync)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport* outputReport,
        __FIAsyncOperation_1_UINT32** operation);
    HRESULT (STDMETHODCALLTYPE* SendFeatureReportAsync)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport* featureReport,
        __FIAsyncOperation_1_UINT32** operation);
    HRESULT (STDMETHODCALLTYPE* GetBooleanControlDescriptions)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        enum __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CHidReportType reportType,
        UINT16 usagePage,
        UINT16 usageId,
        __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription** value);
    HRESULT (STDMETHODCALLTYPE* GetNumericControlDescriptions)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        enum __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CHidReportType reportType,
        UINT16 usagePage,
        UINT16 usageId,
        __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription** value);
    HRESULT (STDMETHODCALLTYPE* add_InputReportReceived)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        __FITypedEventHandler_2_Windows__CDevices__CHumanInterfaceDevice__CHidDevice_Windows__CDevices__CHumanInterfaceDevice__CHidInputReportReceivedEventArgs* reportHandler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_InputReportReceived)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceVtbl;

interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_get_VendorId(This, value) \
    ((This)->lpVtbl->get_VendorId(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_get_ProductId(This, value) \
    ((This)->lpVtbl->get_ProductId(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_get_Version(This, value) \
    ((This)->lpVtbl->get_Version(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_get_UsagePage(This, value) \
    ((This)->lpVtbl->get_UsagePage(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_get_UsageId(This, value) \
    ((This)->lpVtbl->get_UsageId(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_GetInputReportAsync(This, value) \
    ((This)->lpVtbl->GetInputReportAsync(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_GetInputReportByIdAsync(This, reportId, value) \
    ((This)->lpVtbl->GetInputReportByIdAsync(This, reportId, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_GetFeatureReportAsync(This, value) \
    ((This)->lpVtbl->GetFeatureReportAsync(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_GetFeatureReportByIdAsync(This, reportId, value) \
    ((This)->lpVtbl->GetFeatureReportByIdAsync(This, reportId, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_CreateOutputReport(This, outputReport) \
    ((This)->lpVtbl->CreateOutputReport(This, outputReport))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_CreateOutputReportById(This, reportId, outputReport) \
    ((This)->lpVtbl->CreateOutputReportById(This, reportId, outputReport))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_CreateFeatureReport(This, featureReport) \
    ((This)->lpVtbl->CreateFeatureReport(This, featureReport))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_CreateFeatureReportById(This, reportId, featureReport) \
    ((This)->lpVtbl->CreateFeatureReportById(This, reportId, featureReport))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_SendOutputReportAsync(This, outputReport, operation) \
    ((This)->lpVtbl->SendOutputReportAsync(This, outputReport, operation))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_SendFeatureReportAsync(This, featureReport, operation) \
    ((This)->lpVtbl->SendFeatureReportAsync(This, featureReport, operation))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_GetBooleanControlDescriptions(This, reportType, usagePage, usageId, value) \
    ((This)->lpVtbl->GetBooleanControlDescriptions(This, reportType, usagePage, usageId, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_GetNumericControlDescriptions(This, reportType, usagePage, usageId, value) \
    ((This)->lpVtbl->GetNumericControlDescriptions(This, reportType, usagePage, usageId, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_add_InputReportReceived(This, reportHandler, token) \
    ((This)->lpVtbl->add_InputReportReceived(This, reportHandler, token))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_remove_InputReportReceived(This, token) \
    ((This)->lpVtbl->remove_InputReportReceived(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidDeviceStatics[] = L"Windows.Devices.HumanInterfaceDevice.IHidDeviceStatics";
typedef struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics* This,
        UINT16 usagePage,
        UINT16 usageId,
        HSTRING* selector);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorVidPid)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics* This,
        UINT16 usagePage,
        UINT16 usageId,
        UINT16 vendorId,
        UINT16 productId,
        HSTRING* selector);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics* This,
        HSTRING deviceId,
        enum __x_ABI_CWindows_CStorage_CFileAccessMode accessMode,
        __FIAsyncOperation_1_Windows__CDevices__CHumanInterfaceDevice__CHidDevice** hidDevice);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_GetDeviceSelector(This, usagePage, usageId, selector) \
    ((This)->lpVtbl->GetDeviceSelector(This, usagePage, usageId, selector))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_GetDeviceSelectorVidPid(This, usagePage, usageId, vendorId, productId, selector) \
    ((This)->lpVtbl->GetDeviceSelectorVidPid(This, usagePage, usageId, vendorId, productId, selector))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_FromIdAsync(This, deviceId, accessMode, hidDevice) \
    ((This)->lpVtbl->FromIdAsync(This, deviceId, accessMode, hidDevice))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidFeatureReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidFeatureReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidFeatureReport[] = L"Windows.Devices.HumanInterfaceDevice.IHidFeatureReport";
typedef struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* put_Data)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* value);
    HRESULT (STDMETHODCALLTYPE* GetBooleanControl)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport* This,
        UINT16 usagePage,
        UINT16 usageId,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl** value);
    HRESULT (STDMETHODCALLTYPE* GetBooleanControlByDescription)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription* controlDescription,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl** value);
    HRESULT (STDMETHODCALLTYPE* GetNumericControl)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport* This,
        UINT16 usagePage,
        UINT16 usageId,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl** value);
    HRESULT (STDMETHODCALLTYPE* GetNumericControlByDescription)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* controlDescription,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReportVtbl;

interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_put_Data(This, value) \
    ((This)->lpVtbl->put_Data(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_GetBooleanControl(This, usagePage, usageId, value) \
    ((This)->lpVtbl->GetBooleanControl(This, usagePage, usageId, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_GetBooleanControlByDescription(This, controlDescription, value) \
    ((This)->lpVtbl->GetBooleanControlByDescription(This, controlDescription, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_GetNumericControl(This, usagePage, usageId, value) \
    ((This)->lpVtbl->GetNumericControl(This, usagePage, usageId, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_GetNumericControlByDescription(This, controlDescription, value) \
    ((This)->lpVtbl->GetNumericControlByDescription(This, controlDescription, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidFeatureReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidInputReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidInputReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidInputReport[] = L"Windows.Devices.HumanInterfaceDevice.IHidInputReport";
typedef struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* get_ActivatedBooleanControls)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport* This,
        __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl** value);
    HRESULT (STDMETHODCALLTYPE* get_TransitionedBooleanControls)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport* This,
        __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControl** value);
    HRESULT (STDMETHODCALLTYPE* GetBooleanControl)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport* This,
        UINT16 usagePage,
        UINT16 usageId,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl** value);
    HRESULT (STDMETHODCALLTYPE* GetBooleanControlByDescription)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription* controlDescription,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl** value);
    HRESULT (STDMETHODCALLTYPE* GetNumericControl)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport* This,
        UINT16 usagePage,
        UINT16 usageId,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl** value);
    HRESULT (STDMETHODCALLTYPE* GetNumericControlByDescription)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* controlDescription,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportVtbl;

interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_get_ActivatedBooleanControls(This, value) \
    ((This)->lpVtbl->get_ActivatedBooleanControls(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_get_TransitionedBooleanControls(This, value) \
    ((This)->lpVtbl->get_TransitionedBooleanControls(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_GetBooleanControl(This, usagePage, usageId, value) \
    ((This)->lpVtbl->GetBooleanControl(This, usagePage, usageId, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_GetBooleanControlByDescription(This, controlDescription, value) \
    ((This)->lpVtbl->GetBooleanControlByDescription(This, controlDescription, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_GetNumericControl(This, usagePage, usageId, value) \
    ((This)->lpVtbl->GetNumericControl(This, usagePage, usageId, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_GetNumericControlByDescription(This, controlDescription, value) \
    ((This)->lpVtbl->GetNumericControlByDescription(This, controlDescription, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidInputReportReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidInputReportReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidInputReportReceivedEventArgs[] = L"Windows.Devices.HumanInterfaceDevice.IHidInputReportReceivedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Report)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_get_Report(This, value) \
    ((This)->lpVtbl->get_Report(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReportReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidNumericControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidNumericControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidNumericControl[] = L"Windows.Devices.HumanInterfaceDevice.IHidNumericControl";
typedef struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_IsGrouped)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_UsagePage)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_UsageId)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl* This,
        INT64* value);
    HRESULT (STDMETHODCALLTYPE* put_Value)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl* This,
        INT64 value);
    HRESULT (STDMETHODCALLTYPE* get_ScaledValue)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl* This,
        INT64* value);
    HRESULT (STDMETHODCALLTYPE* put_ScaledValue)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl* This,
        INT64 value);
    HRESULT (STDMETHODCALLTYPE* get_ControlDescription)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlVtbl;

interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_get_IsGrouped(This, value) \
    ((This)->lpVtbl->get_IsGrouped(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_get_UsagePage(This, value) \
    ((This)->lpVtbl->get_UsagePage(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_get_UsageId(This, value) \
    ((This)->lpVtbl->get_UsageId(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_put_Value(This, value) \
    ((This)->lpVtbl->put_Value(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_get_ScaledValue(This, value) \
    ((This)->lpVtbl->get_ScaledValue(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_put_ScaledValue(This, value) \
    ((This)->lpVtbl->put_ScaledValue(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_get_ControlDescription(This, value) \
    ((This)->lpVtbl->get_ControlDescription(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidNumericControlDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidNumericControlDescription[] = L"Windows.Devices.HumanInterfaceDevice.IHidNumericControlDescription";
typedef struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ReportId)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_ReportType)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        enum __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CHidReportType* value);
    HRESULT (STDMETHODCALLTYPE* get_ReportSize)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ReportCount)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_UsagePage)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_UsageId)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_LogicalMinimum)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_LogicalMaximum)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PhysicalMinimum)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PhysicalMaximum)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_UnitExponent)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Unit)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_IsAbsolute)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_HasNull)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ParentCollections)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* This,
        __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidCollection** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescriptionVtbl;

interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_get_ReportId(This, value) \
    ((This)->lpVtbl->get_ReportId(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_get_ReportType(This, value) \
    ((This)->lpVtbl->get_ReportType(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_get_ReportSize(This, value) \
    ((This)->lpVtbl->get_ReportSize(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_get_ReportCount(This, value) \
    ((This)->lpVtbl->get_ReportCount(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_get_UsagePage(This, value) \
    ((This)->lpVtbl->get_UsagePage(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_get_UsageId(This, value) \
    ((This)->lpVtbl->get_UsageId(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_get_LogicalMinimum(This, value) \
    ((This)->lpVtbl->get_LogicalMinimum(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_get_LogicalMaximum(This, value) \
    ((This)->lpVtbl->get_LogicalMaximum(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_get_PhysicalMinimum(This, value) \
    ((This)->lpVtbl->get_PhysicalMinimum(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_get_PhysicalMaximum(This, value) \
    ((This)->lpVtbl->get_PhysicalMaximum(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_get_UnitExponent(This, value) \
    ((This)->lpVtbl->get_UnitExponent(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_get_Unit(This, value) \
    ((This)->lpVtbl->get_Unit(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_get_IsAbsolute(This, value) \
    ((This)->lpVtbl->get_IsAbsolute(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_get_HasNull(This, value) \
    ((This)->lpVtbl->get_HasNull(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_get_ParentCollections(This, value) \
    ((This)->lpVtbl->get_ParentCollections(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.HumanInterfaceDevice.IHidOutputReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.HumanInterfaceDevice.HidOutputReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_HumanInterfaceDevice_IHidOutputReport[] = L"Windows.Devices.HumanInterfaceDevice.IHidOutputReport";
typedef struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* put_Data)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* value);
    HRESULT (STDMETHODCALLTYPE* GetBooleanControl)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport* This,
        UINT16 usagePage,
        UINT16 usageId,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl** value);
    HRESULT (STDMETHODCALLTYPE* GetBooleanControlByDescription)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription* controlDescription,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControl** value);
    HRESULT (STDMETHODCALLTYPE* GetNumericControl)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport* This,
        UINT16 usagePage,
        UINT16 usageId,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl** value);
    HRESULT (STDMETHODCALLTYPE* GetNumericControlByDescription)(__x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* controlDescription,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControl** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReportVtbl;

interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_put_Data(This, value) \
    ((This)->lpVtbl->put_Data(This, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_GetBooleanControl(This, usagePage, usageId, value) \
    ((This)->lpVtbl->GetBooleanControl(This, usagePage, usageId, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_GetBooleanControlByDescription(This, controlDescription, value) \
    ((This)->lpVtbl->GetBooleanControlByDescription(This, controlDescription, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_GetNumericControl(This, usagePage, usageId, value) \
    ((This)->lpVtbl->GetNumericControl(This, usagePage, usageId, value))

#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_GetNumericControlByDescription(This, controlDescription, value) \
    ((This)->lpVtbl->GetNumericControlByDescription(This, controlDescription, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidOutputReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidBooleanControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidBooleanControl ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidBooleanControl_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidBooleanControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidBooleanControl[] = L"Windows.Devices.HumanInterfaceDevice.HidBooleanControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidBooleanControlDescription ** Default Interface **
 *    Windows.Devices.HumanInterfaceDevice.IHidBooleanControlDescription2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidBooleanControlDescription_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidBooleanControlDescription_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidBooleanControlDescription[] = L"Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidCollection ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidCollection_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidCollection[] = L"Windows.Devices.HumanInterfaceDevice.HidCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.HumanInterfaceDevice.IHidDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidDevice ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidDevice[] = L"Windows.Devices.HumanInterfaceDevice.HidDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidFeatureReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidFeatureReport ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidFeatureReport_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidFeatureReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidFeatureReport[] = L"Windows.Devices.HumanInterfaceDevice.HidFeatureReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidInputReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidInputReport ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidInputReport_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidInputReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidInputReport[] = L"Windows.Devices.HumanInterfaceDevice.HidInputReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidInputReportReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidInputReportReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidInputReportReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidInputReportReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidInputReportReceivedEventArgs[] = L"Windows.Devices.HumanInterfaceDevice.HidInputReportReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidNumericControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidNumericControl ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidNumericControl_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidNumericControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidNumericControl[] = L"Windows.Devices.HumanInterfaceDevice.HidNumericControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidNumericControlDescription ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidNumericControlDescription_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidNumericControlDescription_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidNumericControlDescription[] = L"Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.HumanInterfaceDevice.HidOutputReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.HumanInterfaceDevice.IHidOutputReport ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidOutputReport_DEFINED
#define RUNTIMECLASS_Windows_Devices_HumanInterfaceDevice_HidOutputReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_HumanInterfaceDevice_HidOutputReport[] = L"Windows.Devices.HumanInterfaceDevice.HidOutputReport";
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
#endif // __windows2Edevices2Ehumaninterfacedevice_p_h__

#endif // __windows2Edevices2Ehumaninterfacedevice_h__
