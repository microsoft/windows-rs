
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
#ifndef __windows2Enetworking2Exboxlive_h__
#define __windows2Enetworking2Exboxlive_h__
#ifndef __windows2Enetworking2Exboxlive_p_h__
#define __windows2Enetworking2Exboxlive_p_h__


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

#if !defined(WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION)
#define WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Networking.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                interface IXboxLiveDeviceAddress;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                interface IXboxLiveDeviceAddressStatics;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddressStatics

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                interface IXboxLiveEndpointPair;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPair

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                interface IXboxLiveEndpointPairCreationResult;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPairCreationResult

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                interface IXboxLiveEndpointPairStateChangedEventArgs;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPairStateChangedEventArgs

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                interface IXboxLiveEndpointPairStatics;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPairStatics

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                interface IXboxLiveEndpointPairTemplate;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPairTemplate

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                interface IXboxLiveEndpointPairTemplateStatics;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPairTemplateStatics

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                interface IXboxLiveInboundEndpointPairCreatedEventArgs;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs ABI::Windows::Networking::XboxLive::IXboxLiveInboundEndpointPairCreatedEventArgs

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                interface IXboxLiveQualityOfServiceMeasurement;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement ABI::Windows::Networking::XboxLive::IXboxLiveQualityOfServiceMeasurement

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                interface IXboxLiveQualityOfServiceMeasurementStatics;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics ABI::Windows::Networking::XboxLive::IXboxLiveQualityOfServiceMeasurementStatics

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                interface IXboxLiveQualityOfServiceMetricResult;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult ABI::Windows::Networking::XboxLive::IXboxLiveQualityOfServiceMetricResult

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                interface IXboxLiveQualityOfServicePrivatePayloadResult;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult ABI::Windows::Networking::XboxLive::IXboxLiveQualityOfServicePrivatePayloadResult

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                class XboxLiveEndpointPairCreationResult;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a4b9d41e-4b2c-5bf9-a19d-ed395b23823c"))
IAsyncOperation<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairCreationResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairCreationResult*, ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPairCreationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Networking.XboxLive.XboxLiveEndpointPairCreationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairCreationResult*> __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_t;
#define __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ad57fa1d-db1b-5df5-9694-f71b9bd5625b"))
IAsyncOperationCompletedHandler<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairCreationResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairCreationResult*, ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPairCreationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Networking.XboxLive.XboxLiveEndpointPairCreationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairCreationResult*> __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                class XboxLiveDeviceAddress;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_USE
#define DEF___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("85fb7103-ef5e-5746-9341-1dfbb21d720d"))
IIterator<ABI::Windows::Networking::XboxLive::XboxLiveDeviceAddress*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveDeviceAddress*, ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.XboxLive.XboxLiveDeviceAddress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::XboxLive::XboxLiveDeviceAddress*> __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_t;
#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_USE
#define DEF___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("44df817c-b475-5fdb-862a-4a96ed1dad5c"))
IIterable<ABI::Windows::Networking::XboxLive::XboxLiveDeviceAddress*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveDeviceAddress*, ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.XboxLive.XboxLiveDeviceAddress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::XboxLive::XboxLiveDeviceAddress*> __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_t;
#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                class XboxLiveEndpointPair;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_USE
#define DEF___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e796a96b-98a0-52a2-bb54-9dba5bb64fc4"))
IIterator<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPair*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPair*, ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPair*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.XboxLive.XboxLiveEndpointPair>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPair*> __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_t;
#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_USE
#define DEF___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7fcaf666-ddc0-50dc-a76f-9bb6058be3c9"))
IIterable<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPair*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPair*, ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPair*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.XboxLive.XboxLiveEndpointPair>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPair*> __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_t;
#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                class XboxLiveEndpointPairTemplate;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_USE
#define DEF___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("515331c6-39e1-5cc7-8454-7f1c8bcba18e"))
IIterator<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairTemplate*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairTemplate*, ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPairTemplate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairTemplate*> __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_t;
#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_USE
#define DEF___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9fce55c3-5580-581a-85b5-88dd5e9d4ff7"))
IIterable<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairTemplate*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairTemplate*, ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPairTemplate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairTemplate*> __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_t;
#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                typedef enum XboxLiveQualityOfServiceMetric : int XboxLiveQualityOfServiceMetric;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_USE
#define DEF___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b281ef1a-56bc-5daa-bbe2-65e617b49925"))
IIterator<enum ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetric> : IIterator_impl<enum ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetric>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetric>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetric> __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_t;
#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_USE
#define DEF___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e202eef6-93bd-5fa2-91fc-7ca6a4edc38a"))
IIterable<enum ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetric> : IIterable_impl<enum ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetric>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetric>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetric> __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_t;
#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                class XboxLiveQualityOfServiceMetricResult;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_USE
#define DEF___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e0696826-0069-57d6-9ad8-d0b368629e2e"))
IIterator<ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetricResult*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetricResult*, ABI::Windows::Networking::XboxLive::IXboxLiveQualityOfServiceMetricResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetricResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetricResult*> __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_t;
#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_USE
#define DEF___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("65cc99ec-9779-568a-8982-aee439fa24ce"))
IIterable<ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetricResult*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetricResult*, ABI::Windows::Networking::XboxLive::IXboxLiveQualityOfServiceMetricResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetricResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetricResult*> __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_t;
#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                class XboxLiveQualityOfServicePrivatePayloadResult;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_USE
#define DEF___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("eb5bfdf4-7897-5cbf-aaa5-3582fafb4d78"))
IIterator<ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServicePrivatePayloadResult*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServicePrivatePayloadResult*, ABI::Windows::Networking::XboxLive::IXboxLiveQualityOfServicePrivatePayloadResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.XboxLive.XboxLiveQualityOfServicePrivatePayloadResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServicePrivatePayloadResult*> __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_t;
#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_USE
#define DEF___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3a061dd4-03dd-52fb-9a83-067de27c2bb1"))
IIterable<ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServicePrivatePayloadResult*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServicePrivatePayloadResult*, ABI::Windows::Networking::XboxLive::IXboxLiveQualityOfServicePrivatePayloadResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.XboxLive.XboxLiveQualityOfServicePrivatePayloadResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServicePrivatePayloadResult*> __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_t;
#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a0594a5f-664d-52f2-a4ef-53a23c0dea3d"))
IVectorView<ABI::Windows::Networking::XboxLive::XboxLiveDeviceAddress*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveDeviceAddress*, ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.XboxLive.XboxLiveDeviceAddress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::XboxLive::XboxLiveDeviceAddress*> __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_t;
#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c88caa17-ae4a-5e4c-8435-d6db6b120464"))
IVectorView<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPair*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPair*, ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPair*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.XboxLive.XboxLiveEndpointPair>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPair*> __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_t;
#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1128b50b-3ac4-543e-9d7e-d796c6226669"))
IVectorView<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairTemplate*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairTemplate*, ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPairTemplate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairTemplate*> __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_t;
#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("05b168df-9678-509d-9057-a79fd84b54bf"))
IVectorView<enum ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetric> : IVectorView_impl<enum ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetric>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetric>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetric> __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_t;
#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fd58662f-d206-5d57-a6ad-a84567187f30"))
IVectorView<ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetricResult*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetricResult*, ABI::Windows::Networking::XboxLive::IXboxLiveQualityOfServiceMetricResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetricResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetricResult*> __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_t;
#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("821b8135-7801-5cb5-baa4-fd74772fb91c"))
IVectorView<ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServicePrivatePayloadResult*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServicePrivatePayloadResult*, ABI::Windows::Networking::XboxLive::IXboxLiveQualityOfServicePrivatePayloadResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.XboxLive.XboxLiveQualityOfServicePrivatePayloadResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServicePrivatePayloadResult*> __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_t;
#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_USE
#define DEF___FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8aacb2da-c37e-5be0-ab46-ba445910b9cb"))
IVector<ABI::Windows::Networking::XboxLive::XboxLiveDeviceAddress*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveDeviceAddress*, ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Networking.XboxLive.XboxLiveDeviceAddress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Networking::XboxLive::XboxLiveDeviceAddress*> __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_t;
#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_USE
#define DEF___FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8f3109d5-dc6b-58d1-9f70-60cc47bf1ced"))
IVector<enum ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetric> : IVector_impl<enum ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetric>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetric>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<enum ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetric> __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_t;
#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7fa76199-d1b8-5494-a042-7002a416adc3"))
ITypedEventHandler<ABI::Windows::Networking::XboxLive::XboxLiveDeviceAddress*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveDeviceAddress*, ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.XboxLive.XboxLiveDeviceAddress, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::XboxLive::XboxLiveDeviceAddress*, IInspectable*> __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                class XboxLiveEndpointPairStateChangedEventArgs;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("51037260-4609-58c1-adb3-0618b89b9975"))
ITypedEventHandler<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPair*, ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairStateChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPair*, ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPair*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairStateChangedEventArgs*, ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPairStateChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.XboxLive.XboxLiveEndpointPair, Windows.Networking.XboxLive.XboxLiveEndpointPairStateChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPair*, ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairStateChangedEventArgs*> __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                class XboxLiveInboundEndpointPairCreatedEventArgs;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2aef5136-6e7a-51f8-8853-cc0ce466fef9"))
ITypedEventHandler<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairTemplate*, ABI::Windows::Networking::XboxLive::XboxLiveInboundEndpointPairCreatedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairTemplate*, ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPairTemplate*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::XboxLive::XboxLiveInboundEndpointPairCreatedEventArgs*, ABI::Windows::Networking::XboxLive::IXboxLiveInboundEndpointPairCreatedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate, Windows.Networking.XboxLive.XboxLiveInboundEndpointPairCreatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairTemplate*, ABI::Windows::Networking::XboxLive::XboxLiveInboundEndpointPairCreatedEventArgs*> __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs_USE */

#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

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
        namespace Networking {
            namespace XboxLive {
                typedef enum XboxLiveEndpointPairCreationBehaviors : unsigned int XboxLiveEndpointPairCreationBehaviors;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                typedef enum XboxLiveEndpointPairCreationStatus : int XboxLiveEndpointPairCreationStatus;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                typedef enum XboxLiveEndpointPairState : int XboxLiveEndpointPairState;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                typedef enum XboxLiveNetworkAccessKind : int XboxLiveNetworkAccessKind;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                typedef enum XboxLiveQualityOfServiceMeasurementStatus : int XboxLiveQualityOfServiceMeasurementStatus;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                typedef enum XboxLiveSocketKind : int XboxLiveSocketKind;
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Networking.XboxLive.XboxLiveEndpointPairCreationBehaviors
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                enum XboxLiveEndpointPairCreationBehaviors : unsigned int
                {
                    XboxLiveEndpointPairCreationBehaviors_None = 0,
                    XboxLiveEndpointPairCreationBehaviors_ReevaluatePath = 0x1,
                };

                DEFINE_ENUM_FLAG_OPERATORS(XboxLiveEndpointPairCreationBehaviors)
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.XboxLive.XboxLiveEndpointPairCreationStatus
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                enum XboxLiveEndpointPairCreationStatus : int
                {
                    XboxLiveEndpointPairCreationStatus_Succeeded = 0,
                    XboxLiveEndpointPairCreationStatus_NoLocalNetworks = 1,
                    XboxLiveEndpointPairCreationStatus_NoCompatibleNetworkPaths = 2,
                    XboxLiveEndpointPairCreationStatus_LocalSystemNotAuthorized = 3,
                    XboxLiveEndpointPairCreationStatus_Canceled = 4,
                    XboxLiveEndpointPairCreationStatus_TimedOut = 5,
                    XboxLiveEndpointPairCreationStatus_RemoteSystemNotAuthorized = 6,
                    XboxLiveEndpointPairCreationStatus_RefusedDueToConfiguration = 7,
                    XboxLiveEndpointPairCreationStatus_UnexpectedInternalError = 8,
                };
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.XboxLive.XboxLiveEndpointPairState
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                enum XboxLiveEndpointPairState : int
                {
                    XboxLiveEndpointPairState_Invalid = 0,
                    XboxLiveEndpointPairState_CreatingOutbound = 1,
                    XboxLiveEndpointPairState_CreatingInbound = 2,
                    XboxLiveEndpointPairState_Ready = 3,
                    XboxLiveEndpointPairState_DeletingLocally = 4,
                    XboxLiveEndpointPairState_RemoteEndpointTerminating = 5,
                    XboxLiveEndpointPairState_Deleted = 6,
                };
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.XboxLive.XboxLiveNetworkAccessKind
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                enum XboxLiveNetworkAccessKind : int
                {
                    XboxLiveNetworkAccessKind_Open = 0,
                    XboxLiveNetworkAccessKind_Moderate = 1,
                    XboxLiveNetworkAccessKind_Strict = 2,
                };
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurementStatus
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                enum XboxLiveQualityOfServiceMeasurementStatus : int
                {
                    XboxLiveQualityOfServiceMeasurementStatus_NotStarted = 0,
                    XboxLiveQualityOfServiceMeasurementStatus_InProgress = 1,
                    XboxLiveQualityOfServiceMeasurementStatus_InProgressWithProvisionalResults = 2,
                    XboxLiveQualityOfServiceMeasurementStatus_Succeeded = 3,
                    XboxLiveQualityOfServiceMeasurementStatus_NoLocalNetworks = 4,
                    XboxLiveQualityOfServiceMeasurementStatus_NoCompatibleNetworkPaths = 5,
                    XboxLiveQualityOfServiceMeasurementStatus_LocalSystemNotAuthorized = 6,
                    XboxLiveQualityOfServiceMeasurementStatus_Canceled = 7,
                    XboxLiveQualityOfServiceMeasurementStatus_TimedOut = 8,
                    XboxLiveQualityOfServiceMeasurementStatus_RemoteSystemNotAuthorized = 9,
                    XboxLiveQualityOfServiceMeasurementStatus_RefusedDueToConfiguration = 10,
                    XboxLiveQualityOfServiceMeasurementStatus_UnexpectedInternalError = 11,
                };
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetric
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                enum XboxLiveQualityOfServiceMetric : int
                {
                    XboxLiveQualityOfServiceMetric_AverageLatencyInMilliseconds = 0,
                    XboxLiveQualityOfServiceMetric_MinLatencyInMilliseconds = 1,
                    XboxLiveQualityOfServiceMetric_MaxLatencyInMilliseconds = 2,
                    XboxLiveQualityOfServiceMetric_AverageOutboundBitsPerSecond = 3,
                    XboxLiveQualityOfServiceMetric_MinOutboundBitsPerSecond = 4,
                    XboxLiveQualityOfServiceMetric_MaxOutboundBitsPerSecond = 5,
                    XboxLiveQualityOfServiceMetric_AverageInboundBitsPerSecond = 6,
                    XboxLiveQualityOfServiceMetric_MinInboundBitsPerSecond = 7,
                    XboxLiveQualityOfServiceMetric_MaxInboundBitsPerSecond = 8,
                };
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.XboxLive.XboxLiveSocketKind
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                enum XboxLiveSocketKind : int
                {
                    XboxLiveSocketKind_None = 0,
                    XboxLiveSocketKind_Datagram = 1,
                    XboxLiveSocketKind_Stream = 2,
                };
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveDeviceAddress
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveDeviceAddress
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveDeviceAddress[] = L"Windows.Networking.XboxLive.IXboxLiveDeviceAddress";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                MIDL_INTERFACE("f5bbd279-3c86-4b57-a31a-b9462408fd01")
                IXboxLiveDeviceAddress : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_SnapshotChanged(
                        __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_SnapshotChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSnapshotAsBase64(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSnapshotAsBuffer(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSnapshotAsBytes(
                        UINT32 bufferLength,
                        BYTE* buffer,
                        UINT32* bytesWritten
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Compare(
                        ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress* otherDeviceAddress,
                        INT32* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsValid(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsLocal(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkAccessKind(
                        ABI::Windows::Networking::XboxLive::XboxLiveNetworkAccessKind* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IXboxLiveDeviceAddress = __uuidof(IXboxLiveDeviceAddress);
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveDeviceAddressStatics
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveDeviceAddress
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveDeviceAddressStatics[] = L"Windows.Networking.XboxLive.IXboxLiveDeviceAddressStatics";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                MIDL_INTERFACE("5954a819-4a79-4931-827c-7f503e963263")
                IXboxLiveDeviceAddressStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromSnapshotBase64(
                        HSTRING base64,
                        ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromSnapshotBuffer(
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromSnapshotBytes(
                        UINT32 bufferLength,
                        BYTE* buffer,
                        ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetLocal(
                        ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxSnapshotBytesSize(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IXboxLiveDeviceAddressStatics = __uuidof(IXboxLiveDeviceAddressStatics);
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveEndpointPair
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveEndpointPair
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveEndpointPair[] = L"Windows.Networking.XboxLive.IXboxLiveEndpointPair";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                MIDL_INTERFACE("1e9a839b-813e-44e0-b87f-c87a093475e4")
                IXboxLiveEndpointPair : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_StateChanged(
                        __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_StateChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeleteAsync(
                        ABI::Windows::Foundation::IAsyncAction** action
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetRemoteSocketAddressBytes(
                        UINT32 socketAddressLength,
                        BYTE* socketAddress
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetLocalSocketAddressBytes(
                        UINT32 socketAddressLength,
                        BYTE* socketAddress
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Template(
                        ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPairTemplate** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteDeviceAddress(
                        ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteHostName(
                        ABI::Windows::Networking::IHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemotePort(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LocalHostName(
                        ABI::Windows::Networking::IHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LocalPort(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IXboxLiveEndpointPair = __uuidof(IXboxLiveEndpointPair);
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveEndpointPairCreationResult
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveEndpointPairCreationResult
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveEndpointPairCreationResult[] = L"Windows.Networking.XboxLive.IXboxLiveEndpointPairCreationResult";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                MIDL_INTERFACE("d9a8bb95-2aab-4d1e-9794-33ecc0dcf0fe")
                IXboxLiveEndpointPairCreationResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceAddress(
                        ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairCreationStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsExistingPathEvaluation(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EndpointPair(
                        ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPair** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IXboxLiveEndpointPairCreationResult = __uuidof(IXboxLiveEndpointPairCreationResult);
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveEndpointPairStateChangedEventArgs
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveEndpointPairStateChangedEventArgs
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveEndpointPairStateChangedEventArgs[] = L"Windows.Networking.XboxLive.IXboxLiveEndpointPairStateChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                MIDL_INTERFACE("592e3b55-de08-44e7-ac3b-b9b9a169583a")
                IXboxLiveEndpointPairStateChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_OldState(
                        ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NewState(
                        ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairState* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IXboxLiveEndpointPairStateChangedEventArgs = __uuidof(IXboxLiveEndpointPairStateChangedEventArgs);
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveEndpointPairStatics
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveEndpointPair
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveEndpointPairStatics[] = L"Windows.Networking.XboxLive.IXboxLiveEndpointPairStatics";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                MIDL_INTERFACE("64316b30-217a-4243-8ee1-6729281d27db")
                IXboxLiveEndpointPairStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FindEndpointPairBySocketAddressBytes(
                        UINT32 localSocketAddressLength,
                        BYTE* localSocketAddress,
                        UINT32 remoteSocketAddressLength,
                        BYTE* remoteSocketAddress,
                        ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPair** endpointPair
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindEndpointPairByHostNamesAndPorts(
                        ABI::Windows::Networking::IHostName* localHostName,
                        HSTRING localPort,
                        ABI::Windows::Networking::IHostName* remoteHostName,
                        HSTRING remotePort,
                        ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPair** endpointPair
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IXboxLiveEndpointPairStatics = __uuidof(IXboxLiveEndpointPairStatics);
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveEndpointPairTemplate
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveEndpointPairTemplate[] = L"Windows.Networking.XboxLive.IXboxLiveEndpointPairTemplate";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                MIDL_INTERFACE("6b286ecf-3457-40ce-b9a1-c0cfe0213ea7")
                IXboxLiveEndpointPairTemplate : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_InboundEndpointPairCreated(
                        __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_InboundEndpointPairCreated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateEndpointPairDefaultAsync(
                        ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress* deviceAddress,
                        __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateEndpointPairWithBehaviorsAsync(
                        ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress* deviceAddress,
                        ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairCreationBehaviors behaviors,
                        __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateEndpointPairForPortsDefaultAsync(
                        ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress* deviceAddress,
                        HSTRING initiatorPort,
                        HSTRING acceptorPort,
                        __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateEndpointPairForPortsWithBehaviorsAsync(
                        ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress* deviceAddress,
                        HSTRING initiatorPort,
                        HSTRING acceptorPort,
                        ABI::Windows::Networking::XboxLive::XboxLiveEndpointPairCreationBehaviors behaviors,
                        __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SocketKind(
                        ABI::Windows::Networking::XboxLive::XboxLiveSocketKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InitiatorBoundPortRangeLower(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InitiatorBoundPortRangeUpper(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AcceptorBoundPortRangeLower(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AcceptorBoundPortRangeUpper(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EndpointPairs(
                        __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IXboxLiveEndpointPairTemplate = __uuidof(IXboxLiveEndpointPairTemplate);
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveEndpointPairTemplateStatics
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveEndpointPairTemplateStatics[] = L"Windows.Networking.XboxLive.IXboxLiveEndpointPairTemplateStatics";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                MIDL_INTERFACE("1e13137b-737b-4a23-bc64-0870f75655ba")
                IXboxLiveEndpointPairTemplateStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetTemplateByName(
                        HSTRING name,
                        ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPairTemplate** namedTemplate
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Templates(
                        __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IXboxLiveEndpointPairTemplateStatics = __uuidof(IXboxLiveEndpointPairTemplateStatics);
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveInboundEndpointPairCreatedEventArgs
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveInboundEndpointPairCreatedEventArgs
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveInboundEndpointPairCreatedEventArgs[] = L"Windows.Networking.XboxLive.IXboxLiveInboundEndpointPairCreatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                MIDL_INTERFACE("dc183b62-22ba-48d2-80de-c23968bd198b")
                IXboxLiveInboundEndpointPairCreatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_EndpointPair(
                        ABI::Windows::Networking::XboxLive::IXboxLiveEndpointPair** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IXboxLiveInboundEndpointPairCreatedEventArgs = __uuidof(IXboxLiveInboundEndpointPairCreatedEventArgs);
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMeasurement
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurement
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveQualityOfServiceMeasurement[] = L"Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMeasurement";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                MIDL_INTERFACE("4d682bce-a5d6-47e6-a236-cfde5fbdf2ed")
                IXboxLiveQualityOfServiceMeasurement : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE MeasureAsync(
                        ABI::Windows::Foundation::IAsyncAction** action
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetMetricResultsForDevice(
                        ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress* deviceAddress,
                        __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetMetricResultsForMetric(
                        ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetric metric,
                        __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetMetricResult(
                        ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress* deviceAddress,
                        ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetric metric,
                        ABI::Windows::Networking::XboxLive::IXboxLiveQualityOfServiceMetricResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPrivatePayloadResult(
                        ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress* deviceAddress,
                        ABI::Windows::Networking::XboxLive::IXboxLiveQualityOfServicePrivatePayloadResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Metrics(
                        __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceAddresses(
                        __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShouldRequestPrivatePayloads(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ShouldRequestPrivatePayloads(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TimeoutInMilliseconds(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TimeoutInMilliseconds(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NumberOfProbesToAttempt(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NumberOfProbesToAttempt(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NumberOfResultsPending(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MetricResults(
                        __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PrivatePayloadResults(
                        __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IXboxLiveQualityOfServiceMeasurement = __uuidof(IXboxLiveQualityOfServiceMeasurement);
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMeasurementStatics
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurement
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveQualityOfServiceMeasurementStatics[] = L"Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMeasurementStatics";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                MIDL_INTERFACE("6e352dca-23cf-440a-b077-5e30857a8234")
                IXboxLiveQualityOfServiceMeasurementStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE PublishPrivatePayloadBytes(
                        UINT32 payloadLength,
                        BYTE* payload
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ClearPrivatePayload(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxSimultaneousProbeConnections(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MaxSimultaneousProbeConnections(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsSystemOutboundBandwidthConstrained(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsSystemOutboundBandwidthConstrained(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsSystemInboundBandwidthConstrained(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsSystemInboundBandwidthConstrained(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PublishedPrivatePayload(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PublishedPrivatePayload(
                        ABI::Windows::Storage::Streams::IBuffer* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxPrivatePayloadSize(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IXboxLiveQualityOfServiceMeasurementStatics = __uuidof(IXboxLiveQualityOfServiceMeasurementStatics);
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMetricResult
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetricResult
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveQualityOfServiceMetricResult[] = L"Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMetricResult";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                MIDL_INTERFACE("aeec53d1-3561-4782-b0cf-d3ae29d9fa87")
                IXboxLiveQualityOfServiceMetricResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMeasurementStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceAddress(
                        ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Metric(
                        ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMetric* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        UINT64* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IXboxLiveQualityOfServiceMetricResult = __uuidof(IXboxLiveQualityOfServiceMetricResult);
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveQualityOfServicePrivatePayloadResult
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveQualityOfServicePrivatePayloadResult
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveQualityOfServicePrivatePayloadResult[] = L"Windows.Networking.XboxLive.IXboxLiveQualityOfServicePrivatePayloadResult";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace XboxLive {
                MIDL_INTERFACE("5a6302ae-6f38-41c0-9fcc-ea6cb978cafc")
                IXboxLiveQualityOfServicePrivatePayloadResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Networking::XboxLive::XboxLiveQualityOfServiceMeasurementStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceAddress(
                        ABI::Windows::Networking::XboxLive::IXboxLiveDeviceAddress** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IXboxLiveQualityOfServicePrivatePayloadResult = __uuidof(IXboxLiveQualityOfServicePrivatePayloadResult);
            } /* XboxLive */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.XboxLive.XboxLiveDeviceAddress
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.XboxLive.IXboxLiveDeviceAddressStatics interface starting with version 1.0 of the Windows.Networking.XboxLive.XboxLiveSecureSocketsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.XboxLive.IXboxLiveDeviceAddress ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveDeviceAddress_DEFINED
#define RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveDeviceAddress_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_XboxLive_XboxLiveDeviceAddress[] = L"Windows.Networking.XboxLive.XboxLiveDeviceAddress";
#endif
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.XboxLive.XboxLiveEndpointPair
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.XboxLive.IXboxLiveEndpointPairStatics interface starting with version 1.0 of the Windows.Networking.XboxLive.XboxLiveSecureSocketsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.XboxLive.IXboxLiveEndpointPair ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveEndpointPair_DEFINED
#define RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveEndpointPair_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_XboxLive_XboxLiveEndpointPair[] = L"Windows.Networking.XboxLive.XboxLiveEndpointPair";
#endif
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.XboxLive.XboxLiveEndpointPairCreationResult
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.XboxLive.IXboxLiveEndpointPairCreationResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveEndpointPairCreationResult_DEFINED
#define RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveEndpointPairCreationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_XboxLive_XboxLiveEndpointPairCreationResult[] = L"Windows.Networking.XboxLive.XboxLiveEndpointPairCreationResult";
#endif
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.XboxLive.XboxLiveEndpointPairStateChangedEventArgs
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.XboxLive.IXboxLiveEndpointPairStateChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveEndpointPairStateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveEndpointPairStateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_XboxLive_XboxLiveEndpointPairStateChangedEventArgs[] = L"Windows.Networking.XboxLive.XboxLiveEndpointPairStateChangedEventArgs";
#endif
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.XboxLive.IXboxLiveEndpointPairTemplateStatics interface starting with version 1.0 of the Windows.Networking.XboxLive.XboxLiveSecureSocketsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.XboxLive.IXboxLiveEndpointPairTemplate ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveEndpointPairTemplate_DEFINED
#define RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveEndpointPairTemplate_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_XboxLive_XboxLiveEndpointPairTemplate[] = L"Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate";
#endif
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.XboxLive.XboxLiveInboundEndpointPairCreatedEventArgs
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.XboxLive.IXboxLiveInboundEndpointPairCreatedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveInboundEndpointPairCreatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveInboundEndpointPairCreatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_XboxLive_XboxLiveInboundEndpointPairCreatedEventArgs[] = L"Windows.Networking.XboxLive.XboxLiveInboundEndpointPairCreatedEventArgs";
#endif
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurement
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Networking.XboxLive.XboxLiveSecureSocketsContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMeasurementStatics interface starting with version 1.0 of the Windows.Networking.XboxLive.XboxLiveSecureSocketsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMeasurement ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveQualityOfServiceMeasurement_DEFINED
#define RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveQualityOfServiceMeasurement_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_XboxLive_XboxLiveQualityOfServiceMeasurement[] = L"Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurement";
#endif
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetricResult
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMetricResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveQualityOfServiceMetricResult_DEFINED
#define RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveQualityOfServiceMetricResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_XboxLive_XboxLiveQualityOfServiceMetricResult[] = L"Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetricResult";
#endif
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.XboxLive.XboxLiveQualityOfServicePrivatePayloadResult
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.XboxLive.IXboxLiveQualityOfServicePrivatePayloadResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveQualityOfServicePrivatePayloadResult_DEFINED
#define RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveQualityOfServicePrivatePayloadResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_XboxLive_XboxLiveQualityOfServicePrivatePayloadResult[] = L"Windows.Networking.XboxLive.XboxLiveQualityOfServicePrivatePayloadResult";
#endif
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress;

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics;

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair;

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult;

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs;

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics;

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate;

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics;

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs;

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement;

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics;

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult;

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult;

#endif // ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult;

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult;

typedef struct __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResultVtbl;

interface __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult* This,
        __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress;

typedef struct __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddressVtbl;

interface __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress;

typedef struct __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddressVtbl;

interface __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair;

typedef struct __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairVtbl;

interface __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair;

typedef struct __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairVtbl;

interface __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate;

typedef struct __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplateVtbl;

interface __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate;

typedef struct __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplateVtbl;

interface __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMetric __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMetric;

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric;

typedef struct __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMetric* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMetric* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricVtbl;

interface __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric;

typedef struct __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricVtbl;

interface __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult;

typedef struct __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResultVtbl;

interface __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult;

typedef struct __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResultVtbl;

interface __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult;

typedef struct __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResultVtbl;

interface __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult;

typedef struct __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        __FIIterator_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResultVtbl;

interface __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress;

typedef struct __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddressVtbl;

interface __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair;

typedef struct __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairVtbl;

interface __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate;

typedef struct __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplateVtbl;

interface __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric;

typedef struct __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        UINT32 index,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMetric* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMetric value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMetric* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricVtbl;

interface __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult;

typedef struct __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResultVtbl;

interface __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult;

typedef struct __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResultVtbl;

interface __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress;

typedef struct __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress** items);

    END_INTERFACE
} __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddressVtbl;

interface __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress
{
    CONST_VTBL struct __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric;

typedef struct __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        UINT32 index,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMetric* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMetric value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        UINT32 index,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMetric value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        UINT32 index,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMetric value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMetric value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMetric* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMetric* items);

    END_INTERFACE
} __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricVtbl;

interface __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric
{
    CONST_VTBL struct __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* sender,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* sender,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CIHostName __x_ABI_CWindows_CNetworking_CIHostName;

#endif // ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveEndpointPairCreationBehaviors __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveEndpointPairCreationBehaviors;

typedef enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveEndpointPairCreationStatus __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveEndpointPairCreationStatus;

typedef enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveEndpointPairState __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveEndpointPairState;

typedef enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveNetworkAccessKind __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveNetworkAccessKind;

typedef enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMeasurementStatus __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMeasurementStatus;

typedef enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveSocketKind __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveSocketKind;

/*
 *
 * Struct Windows.Networking.XboxLive.XboxLiveEndpointPairCreationBehaviors
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveEndpointPairCreationBehaviors
{
    XboxLiveEndpointPairCreationBehaviors_None = 0,
    XboxLiveEndpointPairCreationBehaviors_ReevaluatePath = 0x1,
};
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.XboxLive.XboxLiveEndpointPairCreationStatus
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveEndpointPairCreationStatus
{
    XboxLiveEndpointPairCreationStatus_Succeeded = 0,
    XboxLiveEndpointPairCreationStatus_NoLocalNetworks = 1,
    XboxLiveEndpointPairCreationStatus_NoCompatibleNetworkPaths = 2,
    XboxLiveEndpointPairCreationStatus_LocalSystemNotAuthorized = 3,
    XboxLiveEndpointPairCreationStatus_Canceled = 4,
    XboxLiveEndpointPairCreationStatus_TimedOut = 5,
    XboxLiveEndpointPairCreationStatus_RemoteSystemNotAuthorized = 6,
    XboxLiveEndpointPairCreationStatus_RefusedDueToConfiguration = 7,
    XboxLiveEndpointPairCreationStatus_UnexpectedInternalError = 8,
};
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.XboxLive.XboxLiveEndpointPairState
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveEndpointPairState
{
    XboxLiveEndpointPairState_Invalid = 0,
    XboxLiveEndpointPairState_CreatingOutbound = 1,
    XboxLiveEndpointPairState_CreatingInbound = 2,
    XboxLiveEndpointPairState_Ready = 3,
    XboxLiveEndpointPairState_DeletingLocally = 4,
    XboxLiveEndpointPairState_RemoteEndpointTerminating = 5,
    XboxLiveEndpointPairState_Deleted = 6,
};
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.XboxLive.XboxLiveNetworkAccessKind
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveNetworkAccessKind
{
    XboxLiveNetworkAccessKind_Open = 0,
    XboxLiveNetworkAccessKind_Moderate = 1,
    XboxLiveNetworkAccessKind_Strict = 2,
};
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurementStatus
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMeasurementStatus
{
    XboxLiveQualityOfServiceMeasurementStatus_NotStarted = 0,
    XboxLiveQualityOfServiceMeasurementStatus_InProgress = 1,
    XboxLiveQualityOfServiceMeasurementStatus_InProgressWithProvisionalResults = 2,
    XboxLiveQualityOfServiceMeasurementStatus_Succeeded = 3,
    XboxLiveQualityOfServiceMeasurementStatus_NoLocalNetworks = 4,
    XboxLiveQualityOfServiceMeasurementStatus_NoCompatibleNetworkPaths = 5,
    XboxLiveQualityOfServiceMeasurementStatus_LocalSystemNotAuthorized = 6,
    XboxLiveQualityOfServiceMeasurementStatus_Canceled = 7,
    XboxLiveQualityOfServiceMeasurementStatus_TimedOut = 8,
    XboxLiveQualityOfServiceMeasurementStatus_RemoteSystemNotAuthorized = 9,
    XboxLiveQualityOfServiceMeasurementStatus_RefusedDueToConfiguration = 10,
    XboxLiveQualityOfServiceMeasurementStatus_UnexpectedInternalError = 11,
};
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetric
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMetric
{
    XboxLiveQualityOfServiceMetric_AverageLatencyInMilliseconds = 0,
    XboxLiveQualityOfServiceMetric_MinLatencyInMilliseconds = 1,
    XboxLiveQualityOfServiceMetric_MaxLatencyInMilliseconds = 2,
    XboxLiveQualityOfServiceMetric_AverageOutboundBitsPerSecond = 3,
    XboxLiveQualityOfServiceMetric_MinOutboundBitsPerSecond = 4,
    XboxLiveQualityOfServiceMetric_MaxOutboundBitsPerSecond = 5,
    XboxLiveQualityOfServiceMetric_AverageInboundBitsPerSecond = 6,
    XboxLiveQualityOfServiceMetric_MinInboundBitsPerSecond = 7,
    XboxLiveQualityOfServiceMetric_MaxInboundBitsPerSecond = 8,
};
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.XboxLive.XboxLiveSocketKind
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveSocketKind
{
    XboxLiveSocketKind_None = 0,
    XboxLiveSocketKind_Datagram = 1,
    XboxLiveSocketKind_Stream = 2,
};
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveDeviceAddress
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveDeviceAddress
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveDeviceAddress[] = L"Windows.Networking.XboxLive.IXboxLiveDeviceAddress";
typedef struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_SnapshotChanged)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* This,
        __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SnapshotChanged)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* GetSnapshotAsBase64)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetSnapshotAsBuffer)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* GetSnapshotAsBytes)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* This,
        UINT32 bufferLength,
        BYTE* buffer,
        UINT32* bytesWritten);
    HRESULT (STDMETHODCALLTYPE* Compare)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* otherDeviceAddress,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* get_IsValid)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsLocal)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_NetworkAccessKind)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* This,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveNetworkAccessKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressVtbl;

interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_add_SnapshotChanged(This, handler, token) \
    ((This)->lpVtbl->add_SnapshotChanged(This, handler, token))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_remove_SnapshotChanged(This, token) \
    ((This)->lpVtbl->remove_SnapshotChanged(This, token))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_GetSnapshotAsBase64(This, value) \
    ((This)->lpVtbl->GetSnapshotAsBase64(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_GetSnapshotAsBuffer(This, value) \
    ((This)->lpVtbl->GetSnapshotAsBuffer(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_GetSnapshotAsBytes(This, bufferLength, buffer, bytesWritten) \
    ((This)->lpVtbl->GetSnapshotAsBytes(This, bufferLength, buffer, bytesWritten))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_Compare(This, otherDeviceAddress, result) \
    ((This)->lpVtbl->Compare(This, otherDeviceAddress, result))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_get_IsValid(This, value) \
    ((This)->lpVtbl->get_IsValid(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_get_IsLocal(This, value) \
    ((This)->lpVtbl->get_IsLocal(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_get_NetworkAccessKind(This, value) \
    ((This)->lpVtbl->get_NetworkAccessKind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveDeviceAddressStatics
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveDeviceAddress
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveDeviceAddressStatics[] = L"Windows.Networking.XboxLive.IXboxLiveDeviceAddressStatics";
typedef struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromSnapshotBase64)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics* This,
        HSTRING base64,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress** value);
    HRESULT (STDMETHODCALLTYPE* CreateFromSnapshotBuffer)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress** value);
    HRESULT (STDMETHODCALLTYPE* CreateFromSnapshotBytes)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics* This,
        UINT32 bufferLength,
        BYTE* buffer,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress** value);
    HRESULT (STDMETHODCALLTYPE* GetLocal)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress** value);
    HRESULT (STDMETHODCALLTYPE* get_MaxSnapshotBytesSize)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStaticsVtbl;

interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_CreateFromSnapshotBase64(This, base64, value) \
    ((This)->lpVtbl->CreateFromSnapshotBase64(This, base64, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_CreateFromSnapshotBuffer(This, buffer, value) \
    ((This)->lpVtbl->CreateFromSnapshotBuffer(This, buffer, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_CreateFromSnapshotBytes(This, bufferLength, buffer, value) \
    ((This)->lpVtbl->CreateFromSnapshotBytes(This, bufferLength, buffer, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_GetLocal(This, value) \
    ((This)->lpVtbl->GetLocal(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_get_MaxSnapshotBytesSize(This, value) \
    ((This)->lpVtbl->get_MaxSnapshotBytesSize(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddressStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveEndpointPair
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveEndpointPair
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveEndpointPair[] = L"Windows.Networking.XboxLive.IXboxLiveEndpointPair";
typedef struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_StateChanged)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* This,
        __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairStateChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StateChanged)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* DeleteAsync)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** action);
    HRESULT (STDMETHODCALLTYPE* GetRemoteSocketAddressBytes)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* This,
        UINT32 socketAddressLength,
        BYTE* socketAddress);
    HRESULT (STDMETHODCALLTYPE* GetLocalSocketAddressBytes)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* This,
        UINT32 socketAddressLength,
        BYTE* socketAddress);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* This,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveEndpointPairState* value);
    HRESULT (STDMETHODCALLTYPE* get_Template)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate** value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteDeviceAddress)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress** value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteHostName)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* get_RemotePort)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LocalHostName)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* get_LocalPort)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairVtbl;

interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_add_StateChanged(This, handler, token) \
    ((This)->lpVtbl->add_StateChanged(This, handler, token))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_remove_StateChanged(This, token) \
    ((This)->lpVtbl->remove_StateChanged(This, token))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_DeleteAsync(This, action) \
    ((This)->lpVtbl->DeleteAsync(This, action))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_GetRemoteSocketAddressBytes(This, socketAddressLength, socketAddress) \
    ((This)->lpVtbl->GetRemoteSocketAddressBytes(This, socketAddressLength, socketAddress))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_GetLocalSocketAddressBytes(This, socketAddressLength, socketAddress) \
    ((This)->lpVtbl->GetLocalSocketAddressBytes(This, socketAddressLength, socketAddress))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_get_Template(This, value) \
    ((This)->lpVtbl->get_Template(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_get_RemoteDeviceAddress(This, value) \
    ((This)->lpVtbl->get_RemoteDeviceAddress(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_get_RemoteHostName(This, value) \
    ((This)->lpVtbl->get_RemoteHostName(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_get_RemotePort(This, value) \
    ((This)->lpVtbl->get_RemotePort(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_get_LocalHostName(This, value) \
    ((This)->lpVtbl->get_LocalHostName(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_get_LocalPort(This, value) \
    ((This)->lpVtbl->get_LocalPort(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveEndpointPairCreationResult
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveEndpointPairCreationResult
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveEndpointPairCreationResult[] = L"Windows.Networking.XboxLive.IXboxLiveEndpointPairCreationResult";
typedef struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceAddress)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress** value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult* This,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveEndpointPairCreationStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_IsExistingPathEvaluation)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_EndpointPair)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResultVtbl;

interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_get_DeviceAddress(This, value) \
    ((This)->lpVtbl->get_DeviceAddress(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_get_IsExistingPathEvaluation(This, value) \
    ((This)->lpVtbl->get_IsExistingPathEvaluation(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_get_EndpointPair(This, value) \
    ((This)->lpVtbl->get_EndpointPair(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairCreationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveEndpointPairStateChangedEventArgs
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveEndpointPairStateChangedEventArgs
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveEndpointPairStateChangedEventArgs[] = L"Windows.Networking.XboxLive.IXboxLiveEndpointPairStateChangedEventArgs";
typedef struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OldState)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs* This,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveEndpointPairState* value);
    HRESULT (STDMETHODCALLTYPE* get_NewState)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs* This,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveEndpointPairState* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgsVtbl;

interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_get_OldState(This, value) \
    ((This)->lpVtbl->get_OldState(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_get_NewState(This, value) \
    ((This)->lpVtbl->get_NewState(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveEndpointPairStatics
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveEndpointPair
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveEndpointPairStatics[] = L"Windows.Networking.XboxLive.IXboxLiveEndpointPairStatics";
typedef struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindEndpointPairBySocketAddressBytes)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics* This,
        UINT32 localSocketAddressLength,
        BYTE* localSocketAddress,
        UINT32 remoteSocketAddressLength,
        BYTE* remoteSocketAddress,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair** endpointPair);
    HRESULT (STDMETHODCALLTYPE* FindEndpointPairByHostNamesAndPorts)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics* This,
        __x_ABI_CWindows_CNetworking_CIHostName* localHostName,
        HSTRING localPort,
        __x_ABI_CWindows_CNetworking_CIHostName* remoteHostName,
        HSTRING remotePort,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair** endpointPair);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStaticsVtbl;

interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_FindEndpointPairBySocketAddressBytes(This, localSocketAddressLength, localSocketAddress, remoteSocketAddressLength, remoteSocketAddress, endpointPair) \
    ((This)->lpVtbl->FindEndpointPairBySocketAddressBytes(This, localSocketAddressLength, localSocketAddress, remoteSocketAddressLength, remoteSocketAddress, endpointPair))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_FindEndpointPairByHostNamesAndPorts(This, localHostName, localPort, remoteHostName, remotePort, endpointPair) \
    ((This)->lpVtbl->FindEndpointPairByHostNamesAndPorts(This, localHostName, localPort, remoteHostName, remotePort, endpointPair))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveEndpointPairTemplate
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveEndpointPairTemplate[] = L"Windows.Networking.XboxLive.IXboxLiveEndpointPairTemplate";
typedef struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_InboundEndpointPairCreated)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This,
        __FITypedEventHandler_2_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate_Windows__CNetworking__CXboxLive__CXboxLiveInboundEndpointPairCreatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_InboundEndpointPairCreated)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* CreateEndpointPairDefaultAsync)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* deviceAddress,
        __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult** operation);
    HRESULT (STDMETHODCALLTYPE* CreateEndpointPairWithBehaviorsAsync)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* deviceAddress,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveEndpointPairCreationBehaviors behaviors,
        __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult** operation);
    HRESULT (STDMETHODCALLTYPE* CreateEndpointPairForPortsDefaultAsync)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* deviceAddress,
        HSTRING initiatorPort,
        HSTRING acceptorPort,
        __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult** operation);
    HRESULT (STDMETHODCALLTYPE* CreateEndpointPairForPortsWithBehaviorsAsync)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* deviceAddress,
        HSTRING initiatorPort,
        HSTRING acceptorPort,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveEndpointPairCreationBehaviors behaviors,
        __FIAsyncOperation_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairCreationResult** operation);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SocketKind)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveSocketKind* value);
    HRESULT (STDMETHODCALLTYPE* get_InitiatorBoundPortRangeLower)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_InitiatorBoundPortRangeUpper)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_AcceptorBoundPortRangeLower)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_AcceptorBoundPortRangeUpper)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_EndpointPairs)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate* This,
        __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPair** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateVtbl;

interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_add_InboundEndpointPairCreated(This, handler, token) \
    ((This)->lpVtbl->add_InboundEndpointPairCreated(This, handler, token))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_remove_InboundEndpointPairCreated(This, token) \
    ((This)->lpVtbl->remove_InboundEndpointPairCreated(This, token))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_CreateEndpointPairDefaultAsync(This, deviceAddress, operation) \
    ((This)->lpVtbl->CreateEndpointPairDefaultAsync(This, deviceAddress, operation))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_CreateEndpointPairWithBehaviorsAsync(This, deviceAddress, behaviors, operation) \
    ((This)->lpVtbl->CreateEndpointPairWithBehaviorsAsync(This, deviceAddress, behaviors, operation))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_CreateEndpointPairForPortsDefaultAsync(This, deviceAddress, initiatorPort, acceptorPort, operation) \
    ((This)->lpVtbl->CreateEndpointPairForPortsDefaultAsync(This, deviceAddress, initiatorPort, acceptorPort, operation))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_CreateEndpointPairForPortsWithBehaviorsAsync(This, deviceAddress, initiatorPort, acceptorPort, behaviors, operation) \
    ((This)->lpVtbl->CreateEndpointPairForPortsWithBehaviorsAsync(This, deviceAddress, initiatorPort, acceptorPort, behaviors, operation))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_get_SocketKind(This, value) \
    ((This)->lpVtbl->get_SocketKind(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_get_InitiatorBoundPortRangeLower(This, value) \
    ((This)->lpVtbl->get_InitiatorBoundPortRangeLower(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_get_InitiatorBoundPortRangeUpper(This, value) \
    ((This)->lpVtbl->get_InitiatorBoundPortRangeUpper(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_get_AcceptorBoundPortRangeLower(This, value) \
    ((This)->lpVtbl->get_AcceptorBoundPortRangeLower(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_get_AcceptorBoundPortRangeUpper(This, value) \
    ((This)->lpVtbl->get_AcceptorBoundPortRangeUpper(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_get_EndpointPairs(This, value) \
    ((This)->lpVtbl->get_EndpointPairs(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveEndpointPairTemplateStatics
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveEndpointPairTemplateStatics[] = L"Windows.Networking.XboxLive.IXboxLiveEndpointPairTemplateStatics";
typedef struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetTemplateByName)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics* This,
        HSTRING name,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplate** namedTemplate);
    HRESULT (STDMETHODCALLTYPE* get_Templates)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics* This,
        __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveEndpointPairTemplate** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStaticsVtbl;

interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_GetTemplateByName(This, name, namedTemplate) \
    ((This)->lpVtbl->GetTemplateByName(This, name, namedTemplate))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_get_Templates(This, value) \
    ((This)->lpVtbl->get_Templates(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPairTemplateStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveInboundEndpointPairCreatedEventArgs
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveInboundEndpointPairCreatedEventArgs
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveInboundEndpointPairCreatedEventArgs[] = L"Windows.Networking.XboxLive.IXboxLiveInboundEndpointPairCreatedEventArgs";
typedef struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EndpointPair)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveEndpointPair** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgsVtbl;

interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_get_EndpointPair(This, value) \
    ((This)->lpVtbl->get_EndpointPair(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveInboundEndpointPairCreatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMeasurement
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurement
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveQualityOfServiceMeasurement[] = L"Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMeasurement";
typedef struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* MeasureAsync)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** action);
    HRESULT (STDMETHODCALLTYPE* GetMetricResultsForDevice)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* deviceAddress,
        __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult** value);
    HRESULT (STDMETHODCALLTYPE* GetMetricResultsForMetric)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMetric metric,
        __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult** value);
    HRESULT (STDMETHODCALLTYPE* GetMetricResult)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* deviceAddress,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMetric metric,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult** value);
    HRESULT (STDMETHODCALLTYPE* GetPrivatePayloadResult)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress* deviceAddress,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult** value);
    HRESULT (STDMETHODCALLTYPE* get_Metrics)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetric** value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceAddresses)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        __FIVector_1_Windows__CNetworking__CXboxLive__CXboxLiveDeviceAddress** value);
    HRESULT (STDMETHODCALLTYPE* get_ShouldRequestPrivatePayloads)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ShouldRequestPrivatePayloads)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_TimeoutInMilliseconds)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TimeoutInMilliseconds)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_NumberOfProbesToAttempt)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_NumberOfProbesToAttempt)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_NumberOfResultsPending)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MetricResults)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServiceMetricResult** value);
    HRESULT (STDMETHODCALLTYPE* get_PrivatePayloadResults)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement* This,
        __FIVectorView_1_Windows__CNetworking__CXboxLive__CXboxLiveQualityOfServicePrivatePayloadResult** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementVtbl;

interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_MeasureAsync(This, action) \
    ((This)->lpVtbl->MeasureAsync(This, action))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_GetMetricResultsForDevice(This, deviceAddress, value) \
    ((This)->lpVtbl->GetMetricResultsForDevice(This, deviceAddress, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_GetMetricResultsForMetric(This, metric, value) \
    ((This)->lpVtbl->GetMetricResultsForMetric(This, metric, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_GetMetricResult(This, deviceAddress, metric, value) \
    ((This)->lpVtbl->GetMetricResult(This, deviceAddress, metric, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_GetPrivatePayloadResult(This, deviceAddress, value) \
    ((This)->lpVtbl->GetPrivatePayloadResult(This, deviceAddress, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_get_Metrics(This, value) \
    ((This)->lpVtbl->get_Metrics(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_get_DeviceAddresses(This, value) \
    ((This)->lpVtbl->get_DeviceAddresses(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_get_ShouldRequestPrivatePayloads(This, value) \
    ((This)->lpVtbl->get_ShouldRequestPrivatePayloads(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_put_ShouldRequestPrivatePayloads(This, value) \
    ((This)->lpVtbl->put_ShouldRequestPrivatePayloads(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_get_TimeoutInMilliseconds(This, value) \
    ((This)->lpVtbl->get_TimeoutInMilliseconds(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_put_TimeoutInMilliseconds(This, value) \
    ((This)->lpVtbl->put_TimeoutInMilliseconds(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_get_NumberOfProbesToAttempt(This, value) \
    ((This)->lpVtbl->get_NumberOfProbesToAttempt(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_put_NumberOfProbesToAttempt(This, value) \
    ((This)->lpVtbl->put_NumberOfProbesToAttempt(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_get_NumberOfResultsPending(This, value) \
    ((This)->lpVtbl->get_NumberOfResultsPending(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_get_MetricResults(This, value) \
    ((This)->lpVtbl->get_MetricResults(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_get_PrivatePayloadResults(This, value) \
    ((This)->lpVtbl->get_PrivatePayloadResults(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurement_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMeasurementStatics
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurement
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveQualityOfServiceMeasurementStatics[] = L"Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMeasurementStatics";
typedef struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* PublishPrivatePayloadBytes)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics* This,
        UINT32 payloadLength,
        BYTE* payload);
    HRESULT (STDMETHODCALLTYPE* ClearPrivatePayload)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics* This);
    HRESULT (STDMETHODCALLTYPE* get_MaxSimultaneousProbeConnections)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_MaxSimultaneousProbeConnections)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_IsSystemOutboundBandwidthConstrained)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsSystemOutboundBandwidthConstrained)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsSystemInboundBandwidthConstrained)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsSystemInboundBandwidthConstrained)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_PublishedPrivatePayload)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* put_PublishedPrivatePayload)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxPrivatePayloadSize)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStaticsVtbl;

interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_PublishPrivatePayloadBytes(This, payloadLength, payload) \
    ((This)->lpVtbl->PublishPrivatePayloadBytes(This, payloadLength, payload))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_ClearPrivatePayload(This) \
    ((This)->lpVtbl->ClearPrivatePayload(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_get_MaxSimultaneousProbeConnections(This, value) \
    ((This)->lpVtbl->get_MaxSimultaneousProbeConnections(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_put_MaxSimultaneousProbeConnections(This, value) \
    ((This)->lpVtbl->put_MaxSimultaneousProbeConnections(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_get_IsSystemOutboundBandwidthConstrained(This, value) \
    ((This)->lpVtbl->get_IsSystemOutboundBandwidthConstrained(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_put_IsSystemOutboundBandwidthConstrained(This, value) \
    ((This)->lpVtbl->put_IsSystemOutboundBandwidthConstrained(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_get_IsSystemInboundBandwidthConstrained(This, value) \
    ((This)->lpVtbl->get_IsSystemInboundBandwidthConstrained(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_put_IsSystemInboundBandwidthConstrained(This, value) \
    ((This)->lpVtbl->put_IsSystemInboundBandwidthConstrained(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_get_PublishedPrivatePayload(This, value) \
    ((This)->lpVtbl->get_PublishedPrivatePayload(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_put_PublishedPrivatePayload(This, value) \
    ((This)->lpVtbl->put_PublishedPrivatePayload(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_get_MaxPrivatePayloadSize(This, value) \
    ((This)->lpVtbl->get_MaxPrivatePayloadSize(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMeasurementStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMetricResult
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetricResult
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveQualityOfServiceMetricResult[] = L"Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMetricResult";
typedef struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult* This,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMeasurementStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceAddress)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress** value);
    HRESULT (STDMETHODCALLTYPE* get_Metric)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult* This,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMetric* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult* This,
        UINT64* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResultVtbl;

interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_get_DeviceAddress(This, value) \
    ((This)->lpVtbl->get_DeviceAddress(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_get_Metric(This, value) \
    ((This)->lpVtbl->get_Metric(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServiceMetricResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.XboxLive.IXboxLiveQualityOfServicePrivatePayloadResult
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.XboxLive.XboxLiveQualityOfServicePrivatePayloadResult
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_XboxLive_IXboxLiveQualityOfServicePrivatePayloadResult[] = L"Windows.Networking.XboxLive.IXboxLiveQualityOfServicePrivatePayloadResult";
typedef struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult* This,
        enum __x_ABI_CWindows_CNetworking_CXboxLive_CXboxLiveQualityOfServiceMeasurementStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceAddress)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult* This,
        __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveDeviceAddress** value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResultVtbl;

interface __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_get_DeviceAddress(This, value) \
    ((This)->lpVtbl->get_DeviceAddress(This, value))

#define __x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CXboxLive_CIXboxLiveQualityOfServicePrivatePayloadResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.XboxLive.XboxLiveDeviceAddress
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.XboxLive.IXboxLiveDeviceAddressStatics interface starting with version 1.0 of the Windows.Networking.XboxLive.XboxLiveSecureSocketsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.XboxLive.IXboxLiveDeviceAddress ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveDeviceAddress_DEFINED
#define RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveDeviceAddress_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_XboxLive_XboxLiveDeviceAddress[] = L"Windows.Networking.XboxLive.XboxLiveDeviceAddress";
#endif
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.XboxLive.XboxLiveEndpointPair
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.XboxLive.IXboxLiveEndpointPairStatics interface starting with version 1.0 of the Windows.Networking.XboxLive.XboxLiveSecureSocketsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.XboxLive.IXboxLiveEndpointPair ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveEndpointPair_DEFINED
#define RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveEndpointPair_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_XboxLive_XboxLiveEndpointPair[] = L"Windows.Networking.XboxLive.XboxLiveEndpointPair";
#endif
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.XboxLive.XboxLiveEndpointPairCreationResult
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.XboxLive.IXboxLiveEndpointPairCreationResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveEndpointPairCreationResult_DEFINED
#define RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveEndpointPairCreationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_XboxLive_XboxLiveEndpointPairCreationResult[] = L"Windows.Networking.XboxLive.XboxLiveEndpointPairCreationResult";
#endif
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.XboxLive.XboxLiveEndpointPairStateChangedEventArgs
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.XboxLive.IXboxLiveEndpointPairStateChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveEndpointPairStateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveEndpointPairStateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_XboxLive_XboxLiveEndpointPairStateChangedEventArgs[] = L"Windows.Networking.XboxLive.XboxLiveEndpointPairStateChangedEventArgs";
#endif
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.XboxLive.IXboxLiveEndpointPairTemplateStatics interface starting with version 1.0 of the Windows.Networking.XboxLive.XboxLiveSecureSocketsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.XboxLive.IXboxLiveEndpointPairTemplate ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveEndpointPairTemplate_DEFINED
#define RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveEndpointPairTemplate_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_XboxLive_XboxLiveEndpointPairTemplate[] = L"Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate";
#endif
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.XboxLive.XboxLiveInboundEndpointPairCreatedEventArgs
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.XboxLive.IXboxLiveInboundEndpointPairCreatedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveInboundEndpointPairCreatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveInboundEndpointPairCreatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_XboxLive_XboxLiveInboundEndpointPairCreatedEventArgs[] = L"Windows.Networking.XboxLive.XboxLiveInboundEndpointPairCreatedEventArgs";
#endif
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurement
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Networking.XboxLive.XboxLiveSecureSocketsContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMeasurementStatics interface starting with version 1.0 of the Windows.Networking.XboxLive.XboxLiveSecureSocketsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMeasurement ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveQualityOfServiceMeasurement_DEFINED
#define RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveQualityOfServiceMeasurement_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_XboxLive_XboxLiveQualityOfServiceMeasurement[] = L"Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurement";
#endif
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetricResult
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMetricResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveQualityOfServiceMetricResult_DEFINED
#define RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveQualityOfServiceMetricResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_XboxLive_XboxLiveQualityOfServiceMetricResult[] = L"Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetricResult";
#endif
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.XboxLive.XboxLiveQualityOfServicePrivatePayloadResult
 *
 * Introduced to Windows.Networking.XboxLive.XboxLiveSecureSocketsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.XboxLive.IXboxLiveQualityOfServicePrivatePayloadResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveQualityOfServicePrivatePayloadResult_DEFINED
#define RUNTIMECLASS_Windows_Networking_XboxLive_XboxLiveQualityOfServicePrivatePayloadResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_XboxLive_XboxLiveQualityOfServicePrivatePayloadResult[] = L"Windows.Networking.XboxLive.XboxLiveQualityOfServicePrivatePayloadResult";
#endif
#endif // WINDOWS_NETWORKING_XBOXLIVE_XBOXLIVESECURESOCKETSCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Enetworking2Exboxlive_p_h__

#endif // __windows2Enetworking2Exboxlive_h__
