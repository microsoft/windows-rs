
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
#ifndef __windows2Enetworking2Eproximity_h__
#define __windows2Enetworking2Eproximity_h__
#ifndef __windows2Enetworking2Eproximity_p_h__
#define __windows2Enetworking2Eproximity_p_h__


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
#include "Windows.Networking.h"
#include "Windows.Networking.Sockets.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                interface IDeviceArrivedEventHandler;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler ABI::Windows::Networking::Proximity::IDeviceArrivedEventHandler

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                interface IDeviceDepartedEventHandler;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler ABI::Windows::Networking::Proximity::IDeviceDepartedEventHandler

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                interface IMessageReceivedHandler;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler ABI::Windows::Networking::Proximity::IMessageReceivedHandler

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                interface IMessageTransmittedHandler;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler ABI::Windows::Networking::Proximity::IMessageTransmittedHandler

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                interface IConnectionRequestedEventArgs;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs ABI::Windows::Networking::Proximity::IConnectionRequestedEventArgs

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                interface IPeerFinderStatics;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics ABI::Windows::Networking::Proximity::IPeerFinderStatics

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                interface IPeerFinderStatics2;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2 ABI::Windows::Networking::Proximity::IPeerFinderStatics2

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                interface IPeerInformation;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation ABI::Windows::Networking::Proximity::IPeerInformation

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                interface IPeerInformation3;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3 ABI::Windows::Networking::Proximity::IPeerInformation3

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                interface IPeerInformationWithHostAndService;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService ABI::Windows::Networking::Proximity::IPeerInformationWithHostAndService

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                interface IPeerWatcher;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher ABI::Windows::Networking::Proximity::IPeerWatcher

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                interface IProximityDevice;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice ABI::Windows::Networking::Proximity::IProximityDevice

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                interface IProximityDeviceStatics;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics ABI::Windows::Networking::Proximity::IProximityDeviceStatics

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                interface IProximityMessage;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage ABI::Windows::Networking::Proximity::IProximityMessage

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                interface ITriggeredConnectionStateChangedEventArgs;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs ABI::Windows::Networking::Proximity::ITriggeredConnectionStateChangedEventArgs

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                class PeerInformation;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation_USE
#define DEF___FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("eb6fe2c2-d6cd-5df0-a295-74c56ec58aab"))
IIterator<ABI::Windows::Networking::Proximity::PeerInformation*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Proximity::PeerInformation*, ABI::Windows::Networking::Proximity::IPeerInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.Proximity.PeerInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::Proximity::PeerInformation*> __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation_t;
#define __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation_USE
#define DEF___FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("917e1105-0647-5793-9d50-befe225f0f2f"))
IIterable<ABI::Windows::Networking::Proximity::PeerInformation*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Proximity::PeerInformation*, ABI::Windows::Networking::Proximity::IPeerInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.Proximity.PeerInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::Proximity::PeerInformation*> __FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation_t;
#define __FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("948f92bc-2a05-5c80-95f2-96fe345852cc"))
IVectorView<ABI::Windows::Networking::Proximity::PeerInformation*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Proximity::PeerInformation*, ABI::Windows::Networking::Proximity::IPeerInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Proximity.PeerInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::Proximity::PeerInformation*> __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_t;
#define __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a36ec4bc-607a-5180-a785-4042f8795c8b"))
IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Proximity.PeerInformation>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation*> __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ecf90f2c-e3f4-5b62-a066-8b9c818fd41a"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Proximity.PeerInformation>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class StreamSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket ABI::Windows::Networking::Sockets::IStreamSocket

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket_USE
#define DEF___FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fb3e3d3c-6fe5-5e27-a132-902247e2a93e"))
IAsyncOperation<ABI::Windows::Networking::Sockets::StreamSocket*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::StreamSocket*, ABI::Windows::Networking::Sockets::IStreamSocket*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Networking.Sockets.StreamSocket>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Networking::Sockets::StreamSocket*> __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket_t;
#define __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("71b5d99e-3854-5e9a-b4dc-d1b58bf198fc"))
IAsyncOperationCompletedHandler<ABI::Windows::Networking::Sockets::StreamSocket*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::StreamSocket*, ABI::Windows::Networking::Sockets::IStreamSocket*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Networking.Sockets.StreamSocket>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Networking::Sockets::StreamSocket*> __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("60310303-49c5-52e6-abc6-a9b36eccc716"))
IKeyValuePair<HSTRING, HSTRING> : IKeyValuePair_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, HSTRING> __FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("05eb86f1-7140-5517-b88d-cbaebe57e6b1"))
IIterator<__FIKeyValuePair_2_HSTRING_HSTRING*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_HSTRING*> __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e9bdaaf0-cbf6-5c72-be90-29cbf3a1319b"))
IIterable<__FIKeyValuePair_2_HSTRING_HSTRING*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_HSTRING*> __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIMapView_2_HSTRING_HSTRING_USE
#define DEF___FIMapView_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ac7f26f2-feb7-5b2a-8ac4-345bc62caede"))
IMapView<HSTRING, HSTRING> : IMapView_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, HSTRING> __FIMapView_2_HSTRING_HSTRING_t;
#define __FIMapView_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIMap_2_HSTRING_HSTRING_USE
#define DEF___FIMap_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f6d1f700-49c2-52ae-8154-826f9908773c"))
IMap<HSTRING, HSTRING> : IMap_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, HSTRING> __FIMap_2_HSTRING_HSTRING_t;
#define __FIMap_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                class ConnectionRequestedEventArgs;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("512c383c-8b29-5079-953e-8dee8f8a8224"))
ITypedEventHandler<IInspectable*, ABI::Windows::Networking::Proximity::ConnectionRequestedEventArgs*> : ITypedEventHandler_impl<IInspectable*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Proximity::ConnectionRequestedEventArgs*, ABI::Windows::Networking::Proximity::IConnectionRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Object, Windows.Networking.Proximity.ConnectionRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<IInspectable*, ABI::Windows::Networking::Proximity::ConnectionRequestedEventArgs*> __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs_t;
#define __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                class TriggeredConnectionStateChangedEventArgs;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2e5ad6fa-3ca6-5518-bd4d-fefc4535580e"))
ITypedEventHandler<IInspectable*, ABI::Windows::Networking::Proximity::TriggeredConnectionStateChangedEventArgs*> : ITypedEventHandler_impl<IInspectable*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Proximity::TriggeredConnectionStateChangedEventArgs*, ABI::Windows::Networking::Proximity::ITriggeredConnectionStateChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Object, Windows.Networking.Proximity.TriggeredConnectionStateChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<IInspectable*, ABI::Windows::Networking::Proximity::TriggeredConnectionStateChangedEventArgs*> __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs_t;
#define __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                class PeerWatcher;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f4979ea1-7e06-50a8-88dc-3f29524e4fdb"))
ITypedEventHandler<ABI::Windows::Networking::Proximity::PeerWatcher*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Proximity::PeerWatcher*, ABI::Windows::Networking::Proximity::IPeerWatcher*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.Proximity.PeerWatcher, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::Proximity::PeerWatcher*, IInspectable*> __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ad674bbf-6281-5943-9772-e0fd7664d4e1"))
ITypedEventHandler<ABI::Windows::Networking::Proximity::PeerWatcher*, ABI::Windows::Networking::Proximity::PeerInformation*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Proximity::PeerWatcher*, ABI::Windows::Networking::Proximity::IPeerWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Proximity::PeerInformation*, ABI::Windows::Networking::Proximity::IPeerInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.Proximity.PeerWatcher, Windows.Networking.Proximity.PeerInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::Proximity::PeerWatcher*, ABI::Windows::Networking::Proximity::PeerInformation*> __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class Uri;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IUriRuntimeClass;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass ABI::Windows::Foundation::IUriRuntimeClass

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

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
            namespace Proximity {
                typedef enum PeerDiscoveryTypes : unsigned int PeerDiscoveryTypes;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                typedef enum PeerRole : int PeerRole;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                typedef enum PeerWatcherStatus : int PeerWatcherStatus;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                typedef enum TriggeredConnectState : int TriggeredConnectState;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                class ProximityDevice;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                class ProximityMessage;
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Networking.Proximity.PeerDiscoveryTypes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                enum PeerDiscoveryTypes : unsigned int
                {
                    PeerDiscoveryTypes_None = 0,
                    PeerDiscoveryTypes_Browse = 0x1,
                    PeerDiscoveryTypes_Triggered = 0x2,
                };

                DEFINE_ENUM_FLAG_OPERATORS(PeerDiscoveryTypes)
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Proximity.PeerRole
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                enum PeerRole : int
                {
                    PeerRole_Peer = 0,
                    PeerRole_Host = 1,
                    PeerRole_Client = 2,
                };
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Proximity.PeerWatcherStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                enum PeerWatcherStatus : int
                {
                    PeerWatcherStatus_Created = 0,
                    PeerWatcherStatus_Started = 1,
                    PeerWatcherStatus_EnumerationCompleted = 2,
                    PeerWatcherStatus_Stopping = 3,
                    PeerWatcherStatus_Stopped = 4,
                    PeerWatcherStatus_Aborted = 5,
                };
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Proximity.TriggeredConnectState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                enum TriggeredConnectState : int
                {
                    TriggeredConnectState_PeerFound = 0,
                    TriggeredConnectState_Listening = 1,
                    TriggeredConnectState_Connecting = 2,
                    TriggeredConnectState_Completed = 3,
                    TriggeredConnectState_Canceled = 4,
                    TriggeredConnectState_Failed = 5,
                };
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Networking.Proximity.DeviceArrivedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                MIDL_INTERFACE("efa9da69-f6e1-49c9-a49e-8e0fc58fb911")
                IDeviceArrivedEventHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::Networking::Proximity::IProximityDevice* sender
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceArrivedEventHandler = __uuidof(IDeviceArrivedEventHandler);
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Networking.Proximity.DeviceDepartedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                MIDL_INTERFACE("efa9da69-f6e2-49c9-a49e-8e0fc58fb911")
                IDeviceDepartedEventHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::Networking::Proximity::IProximityDevice* sender
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceDepartedEventHandler = __uuidof(IDeviceDepartedEventHandler);
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Networking.Proximity.MessageReceivedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                MIDL_INTERFACE("efab0782-f6e2-4675-a045-d8e320c24808")
                IMessageReceivedHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::Networking::Proximity::IProximityDevice* sender,
                        ABI::Windows::Networking::Proximity::IProximityMessage* message
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMessageReceivedHandler = __uuidof(IMessageReceivedHandler);
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Networking.Proximity.MessageTransmittedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                MIDL_INTERFACE("efaa0b4a-f6e2-4d7d-856c-78fc8efc021e")
                IMessageTransmittedHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::Networking::Proximity::IProximityDevice* sender,
                        INT64 messageId
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMessageTransmittedHandler = __uuidof(IMessageTransmittedHandler);
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IConnectionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.ConnectionRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IConnectionRequestedEventArgs[] = L"Windows.Networking.Proximity.IConnectionRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                MIDL_INTERFACE("eb6891ae-4f1e-4c66-bd0d-46924a942e08")
                IConnectionRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PeerInformation(
                        ABI::Windows::Networking::Proximity::IPeerInformation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConnectionRequestedEventArgs = __uuidof(IConnectionRequestedEventArgs);
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IPeerFinderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.PeerFinder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IPeerFinderStatics[] = L"Windows.Networking.Proximity.IPeerFinderStatics";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                MIDL_INTERFACE("914b3b61-f6e1-47c4-a14c-148a1903d0c6")
                IPeerFinderStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AllowBluetooth(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AllowBluetooth(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AllowInfrastructure(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AllowInfrastructure(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AllowWiFiDirect(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AllowWiFiDirect(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedDiscoveryTypes(
                        ABI::Windows::Networking::Proximity::PeerDiscoveryTypes* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AlternateIdentities(
                        __FIMap_2_HSTRING_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartWithMessage(
                        HSTRING peerMessage
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_TriggeredConnectionStateChanged(
                        __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_TriggeredConnectionStateChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ConnectionRequested(
                        __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ConnectionRequested(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllPeersAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation** asyncOp
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConnectAsync(
                        ABI::Windows::Networking::Proximity::IPeerInformation* peerInformation,
                        __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket** asyncOp
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPeerFinderStatics = __uuidof(IPeerFinderStatics);
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IPeerFinderStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.PeerFinder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IPeerFinderStatics2[] = L"Windows.Networking.Proximity.IPeerFinderStatics2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                MIDL_INTERFACE("d6e73c65-fdd0-4b0b-9312-866408935d82")
                IPeerFinderStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Role(
                        ABI::Windows::Networking::Proximity::PeerRole* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Role(
                        ABI::Windows::Networking::Proximity::PeerRole value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DiscoveryData(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DiscoveryData(
                        ABI::Windows::Storage::Streams::IBuffer* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWatcher(
                        ABI::Windows::Networking::Proximity::IPeerWatcher** watcher
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPeerFinderStatics2 = __uuidof(IPeerFinderStatics2);
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IPeerInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.PeerInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IPeerInformation[] = L"Windows.Networking.Proximity.IPeerInformation";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                MIDL_INTERFACE("20024f08-9fff-45f4-b6e9-408b2ebef373")
                IPeerInformation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPeerInformation = __uuidof(IPeerInformation);
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IPeerInformation3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.PeerInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IPeerInformation3[] = L"Windows.Networking.Proximity.IPeerInformation3";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                MIDL_INTERFACE("b20f612a-dbd0-40f8-95bd-2d4209c7836f")
                IPeerInformation3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DiscoveryData(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPeerInformation3 = __uuidof(IPeerInformation3);
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IPeerInformationWithHostAndService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.PeerInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IPeerInformationWithHostAndService[] = L"Windows.Networking.Proximity.IPeerInformationWithHostAndService";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                MIDL_INTERFACE("ecc7ccad-1b70-4e8b-92db-bbe781419308")
                IPeerInformationWithHostAndService : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_HostName(
                        ABI::Windows::Networking::IHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceName(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPeerInformationWithHostAndService = __uuidof(IPeerInformationWithHostAndService);
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IPeerWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.PeerWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IPeerWatcher[] = L"Windows.Networking.Proximity.IPeerWatcher";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                MIDL_INTERFACE("3cee21f8-2fa6-4679-9691-03c94a420f34")
                IPeerWatcher : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_Added(
                        __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Added(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Removed(
                        __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Removed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Updated(
                        __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Updated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_EnumerationCompleted(
                        __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_EnumerationCompleted(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Stopped(
                        __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Stopped(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Networking::Proximity::PeerWatcherStatus* status
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IPeerWatcher = __uuidof(IPeerWatcher);
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IProximityDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.ProximityDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IProximityDevice[] = L"Windows.Networking.Proximity.IProximityDevice";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                MIDL_INTERFACE("efa8a552-f6e1-4329-a0fc-ab6b0fd28262")
                IProximityDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SubscribeForMessage(
                        HSTRING messageType,
                        ABI::Windows::Networking::Proximity::IMessageReceivedHandler* messageReceivedHandler,
                        INT64* subscriptionId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PublishMessage(
                        HSTRING messageType,
                        HSTRING message,
                        INT64* messageId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PublishMessageWithCallback(
                        HSTRING messageType,
                        HSTRING message,
                        ABI::Windows::Networking::Proximity::IMessageTransmittedHandler* messageTransmittedHandler,
                        INT64* messageId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PublishBinaryMessage(
                        HSTRING messageType,
                        ABI::Windows::Storage::Streams::IBuffer* message,
                        INT64* messageId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PublishBinaryMessageWithCallback(
                        HSTRING messageType,
                        ABI::Windows::Storage::Streams::IBuffer* message,
                        ABI::Windows::Networking::Proximity::IMessageTransmittedHandler* messageTransmittedHandler,
                        INT64* messageId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PublishUriMessage(
                        ABI::Windows::Foundation::IUriRuntimeClass* message,
                        INT64* messageId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PublishUriMessageWithCallback(
                        ABI::Windows::Foundation::IUriRuntimeClass* message,
                        ABI::Windows::Networking::Proximity::IMessageTransmittedHandler* messageTransmittedHandler,
                        INT64* messageId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StopSubscribingForMessage(
                        INT64 subscriptionId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StopPublishingMessage(
                        INT64 messageId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_DeviceArrived(
                        ABI::Windows::Networking::Proximity::IDeviceArrivedEventHandler* arrivedHandler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DeviceArrived(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_DeviceDeparted(
                        ABI::Windows::Networking::Proximity::IDeviceDepartedEventHandler* departedHandler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DeviceDeparted(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxMessageBytes(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BitsPerSecond(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProximityDevice = __uuidof(IProximityDevice);
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IProximityDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.ProximityDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IProximityDeviceStatics[] = L"Windows.Networking.Proximity.IProximityDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                MIDL_INTERFACE("914ba01d-f6e1-47c4-a14c-148a1903d0c6")
                IProximityDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* selector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::Networking::Proximity::IProximityDevice** proximityDevice
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromId(
                        HSTRING deviceId,
                        ABI::Windows::Networking::Proximity::IProximityDevice** proximityDevice
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProximityDeviceStatics = __uuidof(IProximityDeviceStatics);
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IProximityMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.ProximityMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IProximityMessage[] = L"Windows.Networking.Proximity.IProximityMessage";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                MIDL_INTERFACE("efab0782-f6e1-4675-a045-d8e320c24808")
                IProximityMessage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MessageType(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SubscriptionId(
                        INT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Data(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DataAsString(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProximityMessage = __uuidof(IProximityMessage);
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.ITriggeredConnectionStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.TriggeredConnectionStateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_ITriggeredConnectionStateChangedEventArgs[] = L"Windows.Networking.Proximity.ITriggeredConnectionStateChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Proximity {
                MIDL_INTERFACE("c6a780ad-f6e1-4d54-96e2-33f620bca88a")
                ITriggeredConnectionStateChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::Networking::Proximity::TriggeredConnectState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Socket(
                        ABI::Windows::Networking::Sockets::IStreamSocket** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITriggeredConnectionStateChangedEventArgs = __uuidof(ITriggeredConnectionStateChangedEventArgs);
            } /* Proximity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Proximity.ConnectionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Proximity.IConnectionRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Proximity_ConnectionRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Proximity_ConnectionRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Proximity_ConnectionRequestedEventArgs[] = L"Windows.Networking.Proximity.ConnectionRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Proximity.PeerFinder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Proximity.IPeerFinderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.Proximity.IPeerFinderStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Proximity_PeerFinder_DEFINED
#define RUNTIMECLASS_Windows_Networking_Proximity_PeerFinder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Proximity_PeerFinder[] = L"Windows.Networking.Proximity.PeerFinder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Proximity.PeerInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Proximity.IPeerInformation ** Default Interface **
 *    Windows.Networking.Proximity.IPeerInformation3
 *    Windows.Networking.Proximity.IPeerInformationWithHostAndService
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Proximity_PeerInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Proximity_PeerInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Proximity_PeerInformation[] = L"Windows.Networking.Proximity.PeerInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Proximity.PeerWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Proximity.IPeerWatcher ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Proximity_PeerWatcher_DEFINED
#define RUNTIMECLASS_Windows_Networking_Proximity_PeerWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Proximity_PeerWatcher[] = L"Windows.Networking.Proximity.PeerWatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Proximity.ProximityDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Proximity.IProximityDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Proximity.IProximityDevice ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Proximity_ProximityDevice_DEFINED
#define RUNTIMECLASS_Windows_Networking_Proximity_ProximityDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Proximity_ProximityDevice[] = L"Windows.Networking.Proximity.ProximityDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Proximity.ProximityMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Proximity.IProximityMessage ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Proximity_ProximityMessage_DEFINED
#define RUNTIMECLASS_Windows_Networking_Proximity_ProximityMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Proximity_ProximityMessage[] = L"Windows.Networking.Proximity.ProximityMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Proximity.TriggeredConnectionStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Proximity.ITriggeredConnectionStateChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Proximity_TriggeredConnectionStateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Proximity_TriggeredConnectionStateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Proximity_TriggeredConnectionStateChangedEventArgs[] = L"Windows.Networking.Proximity.TriggeredConnectionStateChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler __x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler;

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler __x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler;

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler __x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler;

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler __x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler;

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs __x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs;

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics;

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2 __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2;

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation;

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3 __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3;

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService;

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher;

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice;

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics;

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage __x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage;

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs __x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs;

#endif // ____x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation;

typedef struct __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformationVtbl;

interface __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation __FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation;

typedef struct __FIIterable_1_Windows__CNetworking__CProximity__CPeerInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        __FIIterator_1_Windows__CNetworking__CProximity__CPeerInformation** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CProximity__CPeerInformationVtbl;

interface __FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CProximity__CPeerInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CProximity__CPeerInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation;

typedef struct __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformationVtbl;

interface __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        __FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformationVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformationVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket;

typedef struct __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocketVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocketVtbl;

interface __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocketVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocketVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket* This,
        __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocketVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocketVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CSockets__CStreamSocket_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_HSTRING __FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_HSTRING_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_HSTRING_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        __FIKeyValuePair_2_HSTRING_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_HSTRING** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_HSTRING __FIMapView_2_HSTRING_HSTRING;

#if !defined(____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_HSTRING __FIMapView_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_HSTRING;

typedef struct __FIMapView_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_HSTRING* This,
        __FIMapView_2_HSTRING_HSTRING** first,
        __FIMapView_2_HSTRING_HSTRING** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_HSTRINGVtbl;

interface __FIMapView_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIMapView_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_HSTRING_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_HSTRING __FIMap_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_HSTRING;

typedef struct __FIMap_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_HSTRING* This,
        __FIMapView_2_HSTRING_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_HSTRING* This);

    END_INTERFACE
} __FIMap_2_HSTRING_HSTRINGVtbl;

interface __FIMap_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIMap_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_HSTRING_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_HSTRING_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs;

typedef struct __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs;

typedef struct __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgsVtbl;

interface __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable* This,
        __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation* This,
        __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* sender,
        __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformationVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CIHostName __x_ABI_CWindows_CNetworking_CIHostName;

#endif // ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CNetworking_CProximity_CPeerDiscoveryTypes __x_ABI_CWindows_CNetworking_CProximity_CPeerDiscoveryTypes;

typedef enum __x_ABI_CWindows_CNetworking_CProximity_CPeerRole __x_ABI_CWindows_CNetworking_CProximity_CPeerRole;

typedef enum __x_ABI_CWindows_CNetworking_CProximity_CPeerWatcherStatus __x_ABI_CWindows_CNetworking_CProximity_CPeerWatcherStatus;

typedef enum __x_ABI_CWindows_CNetworking_CProximity_CTriggeredConnectState __x_ABI_CWindows_CNetworking_CProximity_CTriggeredConnectState;

/*
 *
 * Struct Windows.Networking.Proximity.PeerDiscoveryTypes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CProximity_CPeerDiscoveryTypes
{
    PeerDiscoveryTypes_None = 0,
    PeerDiscoveryTypes_Browse = 0x1,
    PeerDiscoveryTypes_Triggered = 0x2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Proximity.PeerRole
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CProximity_CPeerRole
{
    PeerRole_Peer = 0,
    PeerRole_Host = 1,
    PeerRole_Client = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Proximity.PeerWatcherStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CProximity_CPeerWatcherStatus
{
    PeerWatcherStatus_Created = 0,
    PeerWatcherStatus_Started = 1,
    PeerWatcherStatus_EnumerationCompleted = 2,
    PeerWatcherStatus_Stopping = 3,
    PeerWatcherStatus_Stopped = 4,
    PeerWatcherStatus_Aborted = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Proximity.TriggeredConnectState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CProximity_CTriggeredConnectState
{
    TriggeredConnectState_PeerFound = 0,
    TriggeredConnectState_Listening = 1,
    TriggeredConnectState_Connecting = 2,
    TriggeredConnectState_Completed = 3,
    TriggeredConnectState_Canceled = 4,
    TriggeredConnectState_Failed = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Networking.Proximity.DeviceArrivedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler* This,
        __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* sender);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandlerVtbl;

interface __x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler_Invoke(This, sender) \
    ((This)->lpVtbl->Invoke(This, sender))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Networking.Proximity.DeviceDepartedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler* This,
        __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* sender);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandlerVtbl;

interface __x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler_Invoke(This, sender) \
    ((This)->lpVtbl->Invoke(This, sender))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Networking.Proximity.MessageReceivedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler* This,
        __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* sender,
        __x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage* message);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandlerVtbl;

interface __x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler_Invoke(This, sender, message) \
    ((This)->lpVtbl->Invoke(This, sender, message))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Networking.Proximity.MessageTransmittedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler* This,
        __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* sender,
        INT64 messageId);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandlerVtbl;

interface __x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler_Invoke(This, sender, messageId) \
    ((This)->lpVtbl->Invoke(This, sender, messageId))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IConnectionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.ConnectionRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IConnectionRequestedEventArgs[] = L"Windows.Networking.Proximity.IConnectionRequestedEventArgs";
typedef struct __x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PeerInformation)(__x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_get_PeerInformation(This, value) \
    ((This)->lpVtbl->get_PeerInformation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIConnectionRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IPeerFinderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.PeerFinder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IPeerFinderStatics[] = L"Windows.Networking.Proximity.IPeerFinderStatics";
typedef struct __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AllowBluetooth)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllowBluetooth)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AllowInfrastructure)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllowInfrastructure)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AllowWiFiDirect)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllowWiFiDirect)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedDiscoveryTypes)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        enum __x_ABI_CWindows_CNetworking_CProximity_CPeerDiscoveryTypes* value);
    HRESULT (STDMETHODCALLTYPE* get_AlternateIdentities)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        __FIMap_2_HSTRING_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This);
    HRESULT (STDMETHODCALLTYPE* StartWithMessage)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        HSTRING peerMessage);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This);
    HRESULT (STDMETHODCALLTYPE* add_TriggeredConnectionStateChanged)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CTriggeredConnectionStateChangedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_TriggeredConnectionStateChanged)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_ConnectionRequested)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        __FITypedEventHandler_2_IInspectable_Windows__CNetworking__CProximity__CConnectionRequestedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_ConnectionRequested)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* FindAllPeersAsync)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CProximity__CPeerInformation** asyncOp);
    HRESULT (STDMETHODCALLTYPE* ConnectAsync)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics* This,
        __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation* peerInformation,
        __FIAsyncOperation_1_Windows__CNetworking__CSockets__CStreamSocket** asyncOp);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStaticsVtbl;

interface __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_get_AllowBluetooth(This, value) \
    ((This)->lpVtbl->get_AllowBluetooth(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_put_AllowBluetooth(This, value) \
    ((This)->lpVtbl->put_AllowBluetooth(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_get_AllowInfrastructure(This, value) \
    ((This)->lpVtbl->get_AllowInfrastructure(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_put_AllowInfrastructure(This, value) \
    ((This)->lpVtbl->put_AllowInfrastructure(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_get_AllowWiFiDirect(This, value) \
    ((This)->lpVtbl->get_AllowWiFiDirect(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_put_AllowWiFiDirect(This, value) \
    ((This)->lpVtbl->put_AllowWiFiDirect(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_get_SupportedDiscoveryTypes(This, value) \
    ((This)->lpVtbl->get_SupportedDiscoveryTypes(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_get_AlternateIdentities(This, value) \
    ((This)->lpVtbl->get_AlternateIdentities(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_StartWithMessage(This, peerMessage) \
    ((This)->lpVtbl->StartWithMessage(This, peerMessage))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_add_TriggeredConnectionStateChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_TriggeredConnectionStateChanged(This, handler, cookie))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_remove_TriggeredConnectionStateChanged(This, cookie) \
    ((This)->lpVtbl->remove_TriggeredConnectionStateChanged(This, cookie))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_add_ConnectionRequested(This, handler, cookie) \
    ((This)->lpVtbl->add_ConnectionRequested(This, handler, cookie))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_remove_ConnectionRequested(This, cookie) \
    ((This)->lpVtbl->remove_ConnectionRequested(This, cookie))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_FindAllPeersAsync(This, asyncOp) \
    ((This)->lpVtbl->FindAllPeersAsync(This, asyncOp))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_ConnectAsync(This, peerInformation, asyncOp) \
    ((This)->lpVtbl->ConnectAsync(This, peerInformation, asyncOp))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IPeerFinderStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.PeerFinder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IPeerFinderStatics2[] = L"Windows.Networking.Proximity.IPeerFinderStatics2";
typedef struct __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Role)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2* This,
        enum __x_ABI_CWindows_CNetworking_CProximity_CPeerRole* value);
    HRESULT (STDMETHODCALLTYPE* put_Role)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2* This,
        enum __x_ABI_CWindows_CNetworking_CProximity_CPeerRole value);
    HRESULT (STDMETHODCALLTYPE* get_DiscoveryData)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* put_DiscoveryData)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* value);
    HRESULT (STDMETHODCALLTYPE* CreateWatcher)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2* This,
        __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher** watcher);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2Vtbl;

interface __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_get_Role(This, value) \
    ((This)->lpVtbl->get_Role(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_put_Role(This, value) \
    ((This)->lpVtbl->put_Role(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_get_DiscoveryData(This, value) \
    ((This)->lpVtbl->get_DiscoveryData(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_put_DiscoveryData(This, value) \
    ((This)->lpVtbl->put_DiscoveryData(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_CreateWatcher(This, watcher) \
    ((This)->lpVtbl->CreateWatcher(This, watcher))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerFinderStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IPeerInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.PeerInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IPeerInformation[] = L"Windows.Networking.Proximity.IPeerInformation";
typedef struct __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationVtbl;

interface __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IPeerInformation3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.PeerInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IPeerInformation3[] = L"Windows.Networking.Proximity.IPeerInformation3";
typedef struct __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DiscoveryData)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3Vtbl;

interface __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_get_DiscoveryData(This, value) \
    ((This)->lpVtbl->get_DiscoveryData(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformation3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IPeerInformationWithHostAndService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.PeerInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IPeerInformationWithHostAndService[] = L"Windows.Networking.Proximity.IPeerInformationWithHostAndService";
typedef struct __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndServiceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HostName)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceName)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndServiceVtbl;

interface __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndServiceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_get_HostName(This, value) \
    ((This)->lpVtbl->get_HostName(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_get_ServiceName(This, value) \
    ((This)->lpVtbl->get_ServiceName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerInformationWithHostAndService_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IPeerWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.PeerWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IPeerWatcher[] = L"Windows.Networking.Proximity.IPeerWatcher";
typedef struct __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcherVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_Added)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This,
        __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Added)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Removed)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This,
        __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Removed)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Updated)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This,
        __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_Windows__CNetworking__CProximity__CPeerInformation* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Updated)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_EnumerationCompleted)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This,
        __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_EnumerationCompleted)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Stopped)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This,
        __FITypedEventHandler_2_Windows__CNetworking__CProximity__CPeerWatcher_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Stopped)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This,
        enum __x_ABI_CWindows_CNetworking_CProximity_CPeerWatcherStatus* status);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher* This);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcherVtbl;

interface __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcherVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_add_Added(This, handler, token) \
    ((This)->lpVtbl->add_Added(This, handler, token))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_remove_Added(This, token) \
    ((This)->lpVtbl->remove_Added(This, token))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_add_Removed(This, handler, token) \
    ((This)->lpVtbl->add_Removed(This, handler, token))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_remove_Removed(This, token) \
    ((This)->lpVtbl->remove_Removed(This, token))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_add_Updated(This, handler, token) \
    ((This)->lpVtbl->add_Updated(This, handler, token))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_remove_Updated(This, token) \
    ((This)->lpVtbl->remove_Updated(This, token))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_add_EnumerationCompleted(This, handler, token) \
    ((This)->lpVtbl->add_EnumerationCompleted(This, handler, token))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_remove_EnumerationCompleted(This, token) \
    ((This)->lpVtbl->remove_EnumerationCompleted(This, token))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_add_Stopped(This, handler, token) \
    ((This)->lpVtbl->add_Stopped(This, handler, token))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_remove_Stopped(This, token) \
    ((This)->lpVtbl->remove_Stopped(This, token))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_get_Status(This, status) \
    ((This)->lpVtbl->get_Status(This, status))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIPeerWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IProximityDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.ProximityDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IProximityDevice[] = L"Windows.Networking.Proximity.IProximityDevice";
typedef struct __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SubscribeForMessage)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        HSTRING messageType,
        __x_ABI_CWindows_CNetworking_CProximity_CIMessageReceivedHandler* messageReceivedHandler,
        INT64* subscriptionId);
    HRESULT (STDMETHODCALLTYPE* PublishMessage)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        HSTRING messageType,
        HSTRING message,
        INT64* messageId);
    HRESULT (STDMETHODCALLTYPE* PublishMessageWithCallback)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        HSTRING messageType,
        HSTRING message,
        __x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler* messageTransmittedHandler,
        INT64* messageId);
    HRESULT (STDMETHODCALLTYPE* PublishBinaryMessage)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        HSTRING messageType,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* message,
        INT64* messageId);
    HRESULT (STDMETHODCALLTYPE* PublishBinaryMessageWithCallback)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        HSTRING messageType,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* message,
        __x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler* messageTransmittedHandler,
        INT64* messageId);
    HRESULT (STDMETHODCALLTYPE* PublishUriMessage)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* message,
        INT64* messageId);
    HRESULT (STDMETHODCALLTYPE* PublishUriMessageWithCallback)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* message,
        __x_ABI_CWindows_CNetworking_CProximity_CIMessageTransmittedHandler* messageTransmittedHandler,
        INT64* messageId);
    HRESULT (STDMETHODCALLTYPE* StopSubscribingForMessage)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        INT64 subscriptionId);
    HRESULT (STDMETHODCALLTYPE* StopPublishingMessage)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        INT64 messageId);
    HRESULT (STDMETHODCALLTYPE* add_DeviceArrived)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        __x_ABI_CWindows_CNetworking_CProximity_CIDeviceArrivedEventHandler* arrivedHandler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_DeviceArrived)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_DeviceDeparted)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        __x_ABI_CWindows_CNetworking_CProximity_CIDeviceDepartedEventHandler* departedHandler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_DeviceDeparted)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* get_MaxMessageBytes)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_BitsPerSecond)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceVtbl;

interface __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_SubscribeForMessage(This, messageType, messageReceivedHandler, subscriptionId) \
    ((This)->lpVtbl->SubscribeForMessage(This, messageType, messageReceivedHandler, subscriptionId))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_PublishMessage(This, messageType, message, messageId) \
    ((This)->lpVtbl->PublishMessage(This, messageType, message, messageId))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_PublishMessageWithCallback(This, messageType, message, messageTransmittedHandler, messageId) \
    ((This)->lpVtbl->PublishMessageWithCallback(This, messageType, message, messageTransmittedHandler, messageId))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_PublishBinaryMessage(This, messageType, message, messageId) \
    ((This)->lpVtbl->PublishBinaryMessage(This, messageType, message, messageId))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_PublishBinaryMessageWithCallback(This, messageType, message, messageTransmittedHandler, messageId) \
    ((This)->lpVtbl->PublishBinaryMessageWithCallback(This, messageType, message, messageTransmittedHandler, messageId))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_PublishUriMessage(This, message, messageId) \
    ((This)->lpVtbl->PublishUriMessage(This, message, messageId))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_PublishUriMessageWithCallback(This, message, messageTransmittedHandler, messageId) \
    ((This)->lpVtbl->PublishUriMessageWithCallback(This, message, messageTransmittedHandler, messageId))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_StopSubscribingForMessage(This, subscriptionId) \
    ((This)->lpVtbl->StopSubscribingForMessage(This, subscriptionId))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_StopPublishingMessage(This, messageId) \
    ((This)->lpVtbl->StopPublishingMessage(This, messageId))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_add_DeviceArrived(This, arrivedHandler, cookie) \
    ((This)->lpVtbl->add_DeviceArrived(This, arrivedHandler, cookie))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_remove_DeviceArrived(This, cookie) \
    ((This)->lpVtbl->remove_DeviceArrived(This, cookie))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_add_DeviceDeparted(This, departedHandler, cookie) \
    ((This)->lpVtbl->add_DeviceDeparted(This, departedHandler, cookie))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_remove_DeviceDeparted(This, cookie) \
    ((This)->lpVtbl->remove_DeviceDeparted(This, cookie))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_get_MaxMessageBytes(This, value) \
    ((This)->lpVtbl->get_MaxMessageBytes(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_get_BitsPerSecond(This, value) \
    ((This)->lpVtbl->get_BitsPerSecond(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IProximityDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.ProximityDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IProximityDeviceStatics[] = L"Windows.Networking.Proximity.IProximityDeviceStatics";
typedef struct __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics* This,
        HSTRING* selector);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics* This,
        __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice** proximityDevice);
    HRESULT (STDMETHODCALLTYPE* FromId)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics* This,
        HSTRING deviceId,
        __x_ABI_CWindows_CNetworking_CProximity_CIProximityDevice** proximityDevice);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStaticsVtbl;

interface __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_GetDeviceSelector(This, selector) \
    ((This)->lpVtbl->GetDeviceSelector(This, selector))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_GetDefault(This, proximityDevice) \
    ((This)->lpVtbl->GetDefault(This, proximityDevice))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_FromId(This, deviceId, proximityDevice) \
    ((This)->lpVtbl->FromId(This, deviceId, proximityDevice))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIProximityDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.IProximityMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.ProximityMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_IProximityMessage[] = L"Windows.Networking.Proximity.IProximityMessage";
typedef struct __x_ABI_CWindows_CNetworking_CProximity_CIProximityMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MessageType)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SubscriptionId)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage* This,
        INT64* value);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* get_DataAsString)(__x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CProximity_CIProximityMessageVtbl;

interface __x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CProximity_CIProximityMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_get_MessageType(This, value) \
    ((This)->lpVtbl->get_MessageType(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_get_SubscriptionId(This, value) \
    ((This)->lpVtbl->get_SubscriptionId(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_get_DataAsString(This, value) \
    ((This)->lpVtbl->get_DataAsString(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CIProximityMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Proximity.ITriggeredConnectionStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Proximity.TriggeredConnectionStateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Proximity_ITriggeredConnectionStateChangedEventArgs[] = L"Windows.Networking.Proximity.ITriggeredConnectionStateChangedEventArgs";
typedef struct __x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs* This,
        enum __x_ABI_CWindows_CNetworking_CProximity_CTriggeredConnectState* value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Socket)(__x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgsVtbl;

interface __x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_get_Socket(This, value) \
    ((This)->lpVtbl->get_Socket(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CProximity_CITriggeredConnectionStateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Proximity.ConnectionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Proximity.IConnectionRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Proximity_ConnectionRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Proximity_ConnectionRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Proximity_ConnectionRequestedEventArgs[] = L"Windows.Networking.Proximity.ConnectionRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Proximity.PeerFinder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Proximity.IPeerFinderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.Proximity.IPeerFinderStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Proximity_PeerFinder_DEFINED
#define RUNTIMECLASS_Windows_Networking_Proximity_PeerFinder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Proximity_PeerFinder[] = L"Windows.Networking.Proximity.PeerFinder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Proximity.PeerInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Proximity.IPeerInformation ** Default Interface **
 *    Windows.Networking.Proximity.IPeerInformation3
 *    Windows.Networking.Proximity.IPeerInformationWithHostAndService
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Proximity_PeerInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Proximity_PeerInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Proximity_PeerInformation[] = L"Windows.Networking.Proximity.PeerInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Proximity.PeerWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Proximity.IPeerWatcher ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Proximity_PeerWatcher_DEFINED
#define RUNTIMECLASS_Windows_Networking_Proximity_PeerWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Proximity_PeerWatcher[] = L"Windows.Networking.Proximity.PeerWatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Proximity.ProximityDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Proximity.IProximityDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Proximity.IProximityDevice ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Proximity_ProximityDevice_DEFINED
#define RUNTIMECLASS_Windows_Networking_Proximity_ProximityDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Proximity_ProximityDevice[] = L"Windows.Networking.Proximity.ProximityDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Proximity.ProximityMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Proximity.IProximityMessage ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Proximity_ProximityMessage_DEFINED
#define RUNTIMECLASS_Windows_Networking_Proximity_ProximityMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Proximity_ProximityMessage[] = L"Windows.Networking.Proximity.ProximityMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Proximity.TriggeredConnectionStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Proximity.ITriggeredConnectionStateChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Proximity_TriggeredConnectionStateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Proximity_TriggeredConnectionStateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Proximity_TriggeredConnectionStateChangedEventArgs[] = L"Windows.Networking.Proximity.TriggeredConnectionStateChangedEventArgs";
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
#endif // __windows2Enetworking2Eproximity_p_h__

#endif // __windows2Enetworking2Eproximity_h__
