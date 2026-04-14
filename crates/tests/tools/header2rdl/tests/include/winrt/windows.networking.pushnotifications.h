
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
#ifndef __windows2Enetworking2Epushnotifications_h__
#define __windows2Enetworking2Epushnotifications_h__
#ifndef __windows2Enetworking2Epushnotifications_p_h__
#define __windows2Enetworking2Epushnotifications_p_h__


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
#include "Windows.Storage.Streams.h"
#include "Windows.System.h"
#include "Windows.UI.Notifications.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                interface IPushNotificationChannel;
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel ABI::Windows::Networking::PushNotifications::IPushNotificationChannel

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                interface IPushNotificationChannelManagerForUser;
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser ABI::Windows::Networking::PushNotifications::IPushNotificationChannelManagerForUser

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                interface IPushNotificationChannelManagerForUser2;
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2 ABI::Windows::Networking::PushNotifications::IPushNotificationChannelManagerForUser2

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                interface IPushNotificationChannelManagerStatics;
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics ABI::Windows::Networking::PushNotifications::IPushNotificationChannelManagerStatics

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                interface IPushNotificationChannelManagerStatics2;
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2 ABI::Windows::Networking::PushNotifications::IPushNotificationChannelManagerStatics2

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                interface IPushNotificationChannelManagerStatics3;
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3 ABI::Windows::Networking::PushNotifications::IPushNotificationChannelManagerStatics3

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                interface IPushNotificationChannelManagerStatics4;
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4 ABI::Windows::Networking::PushNotifications::IPushNotificationChannelManagerStatics4

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                interface IPushNotificationChannelsRevokedEventArgs;
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs ABI::Windows::Networking::PushNotifications::IPushNotificationChannelsRevokedEventArgs

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                interface IPushNotificationReceivedEventArgs;
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs ABI::Windows::Networking::PushNotifications::IPushNotificationReceivedEventArgs

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                interface IRawNotification;
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification ABI::Windows::Networking::PushNotifications::IRawNotification

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                interface IRawNotification2;
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2 ABI::Windows::Networking::PushNotifications::IRawNotification2

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                interface IRawNotification3;
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3 ABI::Windows::Networking::PushNotifications::IRawNotification3

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                class PushNotificationChannel;
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_USE
#define DEF___FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("70945a09-331a-5e40-b854-66b7a3233bab"))
IAsyncOperation<ABI::Windows::Networking::PushNotifications::PushNotificationChannel*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::PushNotifications::PushNotificationChannel*, ABI::Windows::Networking::PushNotifications::IPushNotificationChannel*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Networking.PushNotifications.PushNotificationChannel>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Networking::PushNotifications::PushNotificationChannel*> __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_t;
#define __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cf7c902f-0f0d-5b22-90b1-85141b5816cd"))
IAsyncOperationCompletedHandler<ABI::Windows::Networking::PushNotifications::PushNotificationChannel*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::PushNotifications::PushNotificationChannel*, ABI::Windows::Networking::PushNotifications::IPushNotificationChannel*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Networking.PushNotifications.PushNotificationChannel>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Networking::PushNotifications::PushNotificationChannel*> __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_USE */

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


namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                class PushNotificationChannelsRevokedEventArgs;
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs_USE
#define DEF___FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7d4309e7-b3ca-5fde-9b37-be323adb370f"))
IEventHandler<ABI::Windows::Networking::PushNotifications::PushNotificationChannelsRevokedEventArgs*> : IEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::PushNotifications::PushNotificationChannelsRevokedEventArgs*, ABI::Windows::Networking::PushNotifications::IPushNotificationChannelsRevokedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Windows.Networking.PushNotifications.PushNotificationChannelsRevokedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<ABI::Windows::Networking::PushNotifications::PushNotificationChannelsRevokedEventArgs*> __FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs_t;
#define __FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs ABI::Windows::Foundation::__FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                class PushNotificationReceivedEventArgs;
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("55fa217d-1fc3-5863-b980-7094d4379694"))
ITypedEventHandler<ABI::Windows::Networking::PushNotifications::PushNotificationChannel*, ABI::Windows::Networking::PushNotifications::PushNotificationReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::PushNotifications::PushNotificationChannel*, ABI::Windows::Networking::PushNotifications::IPushNotificationChannel*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::PushNotifications::PushNotificationReceivedEventArgs*, ABI::Windows::Networking::PushNotifications::IPushNotificationReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.PushNotifications.PushNotificationChannel, Windows.Networking.PushNotifications.PushNotificationReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::PushNotifications::PushNotificationChannel*, ABI::Windows::Networking::PushNotifications::PushNotificationReceivedEventArgs*> __FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
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
        namespace UI {
            namespace Notifications {
                class BadgeNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IBadgeNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification ABI::Windows::UI::Notifications::IBadgeNotification

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class TileNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface ITileNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CITileNotification ABI::Windows::UI::Notifications::ITileNotification

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class ToastNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification ABI::Windows::UI::Notifications::IToastNotification

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                typedef enum PushNotificationType : int PushNotificationType;
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                class PushNotificationChannelManagerForUser;
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                class RawNotification;
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Networking.PushNotifications.PushNotificationType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                enum PushNotificationType : int
                {
                    PushNotificationType_Toast = 0,
                    PushNotificationType_Tile = 1,
                    PushNotificationType_Badge = 2,
                    PushNotificationType_Raw = 3,
                    PushNotificationType_TileFlyout = 4,
                };
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.PushNotifications.IPushNotificationChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.PushNotificationChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IPushNotificationChannel[] = L"Windows.Networking.PushNotifications.IPushNotificationChannel";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                MIDL_INTERFACE("2b28102e-ef0b-4f39-9b8a-a3c194de7081")
                IPushNotificationChannel : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExpirationTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Close(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PushNotificationReceived(
                        __FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PushNotificationReceived(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPushNotificationChannel = __uuidof(IPushNotificationChannel);
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.PushNotifications.IPushNotificationChannelManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.PushNotificationChannelManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IPushNotificationChannelManagerForUser[] = L"Windows.Networking.PushNotifications.IPushNotificationChannelManagerForUser";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                MIDL_INTERFACE("a4c45704-1182-42c7-8890-f563c4890dc4")
                IPushNotificationChannelManagerForUser : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreatePushNotificationChannelForApplicationAsync(
                        __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreatePushNotificationChannelForApplicationAsyncWithId(
                        HSTRING applicationId,
                        __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreatePushNotificationChannelForSecondaryTileAsync(
                        HSTRING tileId,
                        __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPushNotificationChannelManagerForUser = __uuidof(IPushNotificationChannelManagerForUser);
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.PushNotifications.IPushNotificationChannelManagerForUser2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.PushNotificationChannelManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IPushNotificationChannelManagerForUser2[] = L"Windows.Networking.PushNotifications.IPushNotificationChannelManagerForUser2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                MIDL_INTERFACE("c38b066a-7cc1-4dac-87fd-be6e920414a4")
                IPushNotificationChannelManagerForUser2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync(
                        ABI::Windows::Storage::Streams::IBuffer* appServerKey,
                        HSTRING channelId,
                        __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId(
                        ABI::Windows::Storage::Streams::IBuffer* appServerKey,
                        HSTRING channelId,
                        HSTRING appId,
                        __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPushNotificationChannelManagerForUser2 = __uuidof(IPushNotificationChannelManagerForUser2);
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.PushNotificationChannelManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IPushNotificationChannelManagerStatics[] = L"Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                MIDL_INTERFACE("8baf9b65-77a1-4588-bd19-861529a9dcf0")
                IPushNotificationChannelManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreatePushNotificationChannelForApplicationAsync(
                        __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreatePushNotificationChannelForApplicationAsyncWithId(
                        HSTRING applicationId,
                        __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreatePushNotificationChannelForSecondaryTileAsync(
                        HSTRING tileId,
                        __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPushNotificationChannelManagerStatics = __uuidof(IPushNotificationChannelManagerStatics);
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.PushNotificationChannelManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IPushNotificationChannelManagerStatics2[] = L"Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                MIDL_INTERFACE("b444a65d-a7e9-4b28-950e-f375a907f9df")
                IPushNotificationChannelManagerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::Networking::PushNotifications::IPushNotificationChannelManagerForUser** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPushNotificationChannelManagerStatics2 = __uuidof(IPushNotificationChannelManagerStatics2);
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.PushNotificationChannelManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IPushNotificationChannelManagerStatics3[] = L"Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics3";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                MIDL_INTERFACE("4701fefe-0ede-4a3f-ae78-bfa471496925")
                IPushNotificationChannelManagerStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::Networking::PushNotifications::IPushNotificationChannelManagerForUser** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPushNotificationChannelManagerStatics3 = __uuidof(IPushNotificationChannelManagerStatics3);
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.PushNotificationChannelManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IPushNotificationChannelManagerStatics4[] = L"Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics4";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                MIDL_INTERFACE("bc540efb-7820-5a5b-9c01-b4757f774025")
                IPushNotificationChannelManagerStatics4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_ChannelsRevoked(
                        __FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ChannelsRevoked(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPushNotificationChannelManagerStatics4 = __uuidof(IPushNotificationChannelManagerStatics4);
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Networking.PushNotifications.IPushNotificationChannelsRevokedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.PushNotificationChannelsRevokedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IPushNotificationChannelsRevokedEventArgs[] = L"Windows.Networking.PushNotifications.IPushNotificationChannelsRevokedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                MIDL_INTERFACE("20e1a24c-1a34-5beb-aae2-40c232c8c140")
                IPushNotificationChannelsRevokedEventArgs : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IPushNotificationChannelsRevokedEventArgs = __uuidof(IPushNotificationChannelsRevokedEventArgs);
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Networking.PushNotifications.IPushNotificationReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.PushNotificationReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IPushNotificationReceivedEventArgs[] = L"Windows.Networking.PushNotifications.IPushNotificationReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                MIDL_INTERFACE("d1065e0c-36cd-484c-b935-0a99b753cf00")
                IPushNotificationReceivedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Cancel(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Cancel(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NotificationType(
                        ABI::Windows::Networking::PushNotifications::PushNotificationType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ToastNotification(
                        ABI::Windows::UI::Notifications::IToastNotification** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TileNotification(
                        ABI::Windows::UI::Notifications::ITileNotification** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BadgeNotification(
                        ABI::Windows::UI::Notifications::IBadgeNotification** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RawNotification(
                        ABI::Windows::Networking::PushNotifications::IRawNotification** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPushNotificationReceivedEventArgs = __uuidof(IPushNotificationReceivedEventArgs);
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.PushNotifications.IRawNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.RawNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IRawNotification[] = L"Windows.Networking.PushNotifications.IRawNotification";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                MIDL_INTERFACE("1a227281-3b79-42ac-9963-22ab00d4f0b7")
                IRawNotification : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Content(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRawNotification = __uuidof(IRawNotification);
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.PushNotifications.IRawNotification2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.RawNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IRawNotification2[] = L"Windows.Networking.PushNotifications.IRawNotification2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                MIDL_INTERFACE("e6d0cf19-0c6f-4cdd-9424-eec5be014d26")
                IRawNotification2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Headers(
                        __FIMapView_2_HSTRING_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ChannelId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRawNotification2 = __uuidof(IRawNotification2);
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Networking.PushNotifications.IRawNotification3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.RawNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IRawNotification3[] = L"Windows.Networking.PushNotifications.IRawNotification3";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace PushNotifications {
                MIDL_INTERFACE("62737dde-8a73-424c-ab44-5635f40a96e5")
                IRawNotification3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContentBytes(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRawNotification3 = __uuidof(IRawNotification3);
            } /* PushNotifications */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Networking.PushNotifications.PushNotificationChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.PushNotifications.IPushNotificationChannel ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationChannel_DEFINED
#define RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationChannel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_PushNotifications_PushNotificationChannel[] = L"Windows.Networking.PushNotifications.PushNotificationChannel";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.PushNotifications.PushNotificationChannelManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics3 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics4 interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationChannelManager_DEFINED
#define RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationChannelManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_PushNotifications_PushNotificationChannelManager[] = L"Windows.Networking.PushNotifications.PushNotificationChannelManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.PushNotifications.PushNotificationChannelManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.PushNotifications.IPushNotificationChannelManagerForUser ** Default Interface **
 *    Windows.Networking.PushNotifications.IPushNotificationChannelManagerForUser2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationChannelManagerForUser_DEFINED
#define RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationChannelManagerForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_PushNotifications_PushNotificationChannelManagerForUser[] = L"Windows.Networking.PushNotifications.PushNotificationChannelManagerForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Networking.PushNotifications.PushNotificationChannelsRevokedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.PushNotifications.IPushNotificationChannelsRevokedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationChannelsRevokedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationChannelsRevokedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_PushNotifications_PushNotificationChannelsRevokedEventArgs[] = L"Windows.Networking.PushNotifications.PushNotificationChannelsRevokedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Networking.PushNotifications.PushNotificationReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.PushNotifications.IPushNotificationReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_PushNotifications_PushNotificationReceivedEventArgs[] = L"Windows.Networking.PushNotifications.PushNotificationReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.PushNotifications.RawNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.PushNotifications.IRawNotification ** Default Interface **
 *    Windows.Networking.PushNotifications.IRawNotification2
 *    Windows.Networking.PushNotifications.IRawNotification3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_PushNotifications_RawNotification_DEFINED
#define RUNTIMECLASS_Windows_Networking_PushNotifications_RawNotification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_PushNotifications_RawNotification[] = L"Windows.Networking.PushNotifications.RawNotification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel;

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser;

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2 __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2;

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics;

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2 __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2;

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3 __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3;

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4 __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4;

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs;

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs;

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification;

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2 __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2;

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3 __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3;

#endif // ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel;

typedef struct __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel* This,
        __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelVtbl;

interface __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel* This,
        __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_INTERFACE_DEFINED__
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs __FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs;

typedef struct __FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs* args);

    END_INTERFACE
} __FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgsVtbl;

interface __FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs
{
    CONST_VTBL struct __FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs __FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel* sender,
        __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CITileNotification __x_ABI_CWindows_CUI_CNotifications_CITileNotification;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotification __x_ABI_CWindows_CUI_CNotifications_CIToastNotification;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CNetworking_CPushNotifications_CPushNotificationType __x_ABI_CWindows_CNetworking_CPushNotifications_CPushNotificationType;

/*
 *
 * Struct Windows.Networking.PushNotifications.PushNotificationType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CPushNotifications_CPushNotificationType
{
    PushNotificationType_Toast = 0,
    PushNotificationType_Tile = 1,
    PushNotificationType_Badge = 2,
    PushNotificationType_Raw = 3,
    PushNotificationType_TileFlyout = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.PushNotifications.IPushNotificationChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.PushNotificationChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IPushNotificationChannel[] = L"Windows.Networking.PushNotifications.IPushNotificationChannel";
typedef struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ExpirationTime)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* Close)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel* This);
    HRESULT (STDMETHODCALLTYPE* add_PushNotificationReceived)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel* This,
        __FITypedEventHandler_2_Windows__CNetworking__CPushNotifications__CPushNotificationChannel_Windows__CNetworking__CPushNotifications__CPushNotificationReceivedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PushNotificationReceived)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelVtbl;

interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_get_ExpirationTime(This, value) \
    ((This)->lpVtbl->get_ExpirationTime(This, value))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_Close(This) \
    ((This)->lpVtbl->Close(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_add_PushNotificationReceived(This, handler, token) \
    ((This)->lpVtbl->add_PushNotificationReceived(This, handler, token))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_remove_PushNotificationReceived(This, token) \
    ((This)->lpVtbl->remove_PushNotificationReceived(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannel_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.PushNotifications.IPushNotificationChannelManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.PushNotificationChannelManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IPushNotificationChannelManagerForUser[] = L"Windows.Networking.PushNotifications.IPushNotificationChannelManagerForUser";
typedef struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUserVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreatePushNotificationChannelForApplicationAsync)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser* This,
        __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel** operation);
    HRESULT (STDMETHODCALLTYPE* CreatePushNotificationChannelForApplicationAsyncWithId)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser* This,
        HSTRING applicationId,
        __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel** operation);
    HRESULT (STDMETHODCALLTYPE* CreatePushNotificationChannelForSecondaryTileAsync)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser* This,
        HSTRING tileId,
        __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel** operation);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUserVtbl;

interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUserVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_CreatePushNotificationChannelForApplicationAsync(This, operation) \
    ((This)->lpVtbl->CreatePushNotificationChannelForApplicationAsync(This, operation))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_CreatePushNotificationChannelForApplicationAsyncWithId(This, applicationId, operation) \
    ((This)->lpVtbl->CreatePushNotificationChannelForApplicationAsyncWithId(This, applicationId, operation))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_CreatePushNotificationChannelForSecondaryTileAsync(This, tileId, operation) \
    ((This)->lpVtbl->CreatePushNotificationChannelForSecondaryTileAsync(This, tileId, operation))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.PushNotifications.IPushNotificationChannelManagerForUser2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.PushNotificationChannelManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IPushNotificationChannelManagerForUser2[] = L"Windows.Networking.PushNotifications.IPushNotificationChannelManagerForUser2";
typedef struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* appServerKey,
        HSTRING channelId,
        __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel** operation);
    HRESULT (STDMETHODCALLTYPE* CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* appServerKey,
        HSTRING channelId,
        HSTRING appId,
        __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2Vtbl;

interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync(This, appServerKey, channelId, operation) \
    ((This)->lpVtbl->CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync(This, appServerKey, channelId, operation))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId(This, appServerKey, channelId, appId, operation) \
    ((This)->lpVtbl->CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId(This, appServerKey, channelId, appId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.PushNotificationChannelManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IPushNotificationChannelManagerStatics[] = L"Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics";
typedef struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreatePushNotificationChannelForApplicationAsync)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics* This,
        __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel** operation);
    HRESULT (STDMETHODCALLTYPE* CreatePushNotificationChannelForApplicationAsyncWithId)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics* This,
        HSTRING applicationId,
        __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel** operation);
    HRESULT (STDMETHODCALLTYPE* CreatePushNotificationChannelForSecondaryTileAsync)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics* This,
        HSTRING tileId,
        __FIAsyncOperation_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannel** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStaticsVtbl;

interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_CreatePushNotificationChannelForApplicationAsync(This, operation) \
    ((This)->lpVtbl->CreatePushNotificationChannelForApplicationAsync(This, operation))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_CreatePushNotificationChannelForApplicationAsyncWithId(This, applicationId, operation) \
    ((This)->lpVtbl->CreatePushNotificationChannelForApplicationAsyncWithId(This, applicationId, operation))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_CreatePushNotificationChannelForSecondaryTileAsync(This, tileId, operation) \
    ((This)->lpVtbl->CreatePushNotificationChannelForSecondaryTileAsync(This, tileId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.PushNotificationChannelManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IPushNotificationChannelManagerStatics2[] = L"Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics2";
typedef struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser** result);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2Vtbl;

interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_GetForUser(This, user, result) \
    ((This)->lpVtbl->GetForUser(This, user, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.PushNotificationChannelManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IPushNotificationChannelManagerStatics3[] = L"Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics3";
typedef struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3* This,
        __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerForUser** result);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3Vtbl;

interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.PushNotificationChannelManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IPushNotificationChannelManagerStatics4[] = L"Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics4";
typedef struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_ChannelsRevoked)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4* This,
        __FIEventHandler_1_Windows__CNetworking__CPushNotifications__CPushNotificationChannelsRevokedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ChannelsRevoked)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4Vtbl;

interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_add_ChannelsRevoked(This, handler, token) \
    ((This)->lpVtbl->add_ChannelsRevoked(This, handler, token))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_remove_ChannelsRevoked(This, token) \
    ((This)->lpVtbl->remove_ChannelsRevoked(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelManagerStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Networking.PushNotifications.IPushNotificationChannelsRevokedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.PushNotificationChannelsRevokedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IPushNotificationChannelsRevokedEventArgs[] = L"Windows.Networking.PushNotifications.IPushNotificationChannelsRevokedEventArgs";
typedef struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgsVtbl;

interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationChannelsRevokedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Networking.PushNotifications.IPushNotificationReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.PushNotificationReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IPushNotificationReceivedEventArgs[] = L"Windows.Networking.PushNotifications.IPushNotificationReceivedEventArgs";
typedef struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Cancel)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Cancel)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_NotificationType)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs* This,
        enum __x_ABI_CWindows_CNetworking_CPushNotifications_CPushNotificationType* value);
    HRESULT (STDMETHODCALLTYPE* get_ToastNotification)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification** value);
    HRESULT (STDMETHODCALLTYPE* get_TileNotification)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs* This,
        __x_ABI_CWindows_CUI_CNotifications_CITileNotification** value);
    HRESULT (STDMETHODCALLTYPE* get_BadgeNotification)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs* This,
        __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification** value);
    HRESULT (STDMETHODCALLTYPE* get_RawNotification)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_put_Cancel(This, value) \
    ((This)->lpVtbl->put_Cancel(This, value))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_get_Cancel(This, value) \
    ((This)->lpVtbl->get_Cancel(This, value))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_get_NotificationType(This, value) \
    ((This)->lpVtbl->get_NotificationType(This, value))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_get_ToastNotification(This, value) \
    ((This)->lpVtbl->get_ToastNotification(This, value))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_get_TileNotification(This, value) \
    ((This)->lpVtbl->get_TileNotification(This, value))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_get_BadgeNotification(This, value) \
    ((This)->lpVtbl->get_BadgeNotification(This, value))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_get_RawNotification(This, value) \
    ((This)->lpVtbl->get_RawNotification(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIPushNotificationReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.PushNotifications.IRawNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.RawNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IRawNotification[] = L"Windows.Networking.PushNotifications.IRawNotification";
typedef struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotificationVtbl;

interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.PushNotifications.IRawNotification2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.RawNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IRawNotification2[] = L"Windows.Networking.PushNotifications.IRawNotification2";
typedef struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Headers)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2* This,
        __FIMapView_2_HSTRING_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_ChannelId)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2Vtbl;

interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_get_Headers(This, value) \
    ((This)->lpVtbl->get_Headers(This, value))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_get_ChannelId(This, value) \
    ((This)->lpVtbl->get_ChannelId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Networking.PushNotifications.IRawNotification3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Networking.PushNotifications.RawNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_PushNotifications_IRawNotification3[] = L"Windows.Networking.PushNotifications.IRawNotification3";
typedef struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContentBytes)(__x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3Vtbl;

interface __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_get_ContentBytes(This, value) \
    ((This)->lpVtbl->get_ContentBytes(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CPushNotifications_CIRawNotification3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Networking.PushNotifications.PushNotificationChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.PushNotifications.IPushNotificationChannel ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationChannel_DEFINED
#define RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationChannel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_PushNotifications_PushNotificationChannel[] = L"Windows.Networking.PushNotifications.PushNotificationChannel";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.PushNotifications.PushNotificationChannelManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics3 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics4 interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationChannelManager_DEFINED
#define RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationChannelManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_PushNotifications_PushNotificationChannelManager[] = L"Windows.Networking.PushNotifications.PushNotificationChannelManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.PushNotifications.PushNotificationChannelManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.PushNotifications.IPushNotificationChannelManagerForUser ** Default Interface **
 *    Windows.Networking.PushNotifications.IPushNotificationChannelManagerForUser2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationChannelManagerForUser_DEFINED
#define RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationChannelManagerForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_PushNotifications_PushNotificationChannelManagerForUser[] = L"Windows.Networking.PushNotifications.PushNotificationChannelManagerForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Networking.PushNotifications.PushNotificationChannelsRevokedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.PushNotifications.IPushNotificationChannelsRevokedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationChannelsRevokedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationChannelsRevokedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_PushNotifications_PushNotificationChannelsRevokedEventArgs[] = L"Windows.Networking.PushNotifications.PushNotificationChannelsRevokedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Networking.PushNotifications.PushNotificationReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.PushNotifications.IPushNotificationReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_PushNotifications_PushNotificationReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_PushNotifications_PushNotificationReceivedEventArgs[] = L"Windows.Networking.PushNotifications.PushNotificationReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.PushNotifications.RawNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.PushNotifications.IRawNotification ** Default Interface **
 *    Windows.Networking.PushNotifications.IRawNotification2
 *    Windows.Networking.PushNotifications.IRawNotification3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_PushNotifications_RawNotification_DEFINED
#define RUNTIMECLASS_Windows_Networking_PushNotifications_RawNotification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_PushNotifications_RawNotification[] = L"Windows.Networking.PushNotifications.RawNotification";
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
#endif // __windows2Enetworking2Epushnotifications_p_h__

#endif // __windows2Enetworking2Epushnotifications_h__
