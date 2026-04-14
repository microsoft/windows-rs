
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
#ifndef __windows2Esystem2Ediagnostics2Edeviceportal_h__
#define __windows2Esystem2Ediagnostics2Edeviceportal_h__
#ifndef __windows2Esystem2Ediagnostics2Edeviceportal_p_h__
#define __windows2Esystem2Ediagnostics2Edeviceportal_p_h__


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
#include "Windows.ApplicationModel.AppService.h"
#include "Windows.Networking.Sockets.h"
#include "Windows.Web.Http.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace DevicePortal {
                    interface IDevicePortalConnection;
                } /* DevicePortal */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection ABI::Windows::System::Diagnostics::DevicePortal::IDevicePortalConnection

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace DevicePortal {
                    interface IDevicePortalConnectionClosedEventArgs;
                } /* DevicePortal */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs ABI::Windows::System::Diagnostics::DevicePortal::IDevicePortalConnectionClosedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace DevicePortal {
                    interface IDevicePortalConnectionRequestReceivedEventArgs;
                } /* DevicePortal */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs ABI::Windows::System::Diagnostics::DevicePortal::IDevicePortalConnectionRequestReceivedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace DevicePortal {
                    interface IDevicePortalConnectionStatics;
                } /* DevicePortal */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics ABI::Windows::System::Diagnostics::DevicePortal::IDevicePortalConnectionStatics

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace DevicePortal {
                    interface IDevicePortalWebSocketConnection;
                } /* DevicePortal */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection ABI::Windows::System::Diagnostics::DevicePortal::IDevicePortalWebSocketConnection

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace DevicePortal {
                    interface IDevicePortalWebSocketConnectionRequestReceivedEventArgs;
                } /* DevicePortal */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs ABI::Windows::System::Diagnostics::DevicePortal::IDevicePortalWebSocketConnectionRequestReceivedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

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


namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace DevicePortal {
                    class DevicePortalConnection;
                } /* DevicePortal */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace DevicePortal {
                    class DevicePortalConnectionClosedEventArgs;
                } /* DevicePortal */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2aad93a8-52fa-54b3-9556-15d651208b3f"))
ITypedEventHandler<ABI::Windows::System::Diagnostics::DevicePortal::DevicePortalConnection*, ABI::Windows::System::Diagnostics::DevicePortal::DevicePortalConnectionClosedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Diagnostics::DevicePortal::DevicePortalConnection*, ABI::Windows::System::Diagnostics::DevicePortal::IDevicePortalConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Diagnostics::DevicePortal::DevicePortalConnectionClosedEventArgs*, ABI::Windows::System::Diagnostics::DevicePortal::IDevicePortalConnectionClosedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.Diagnostics.DevicePortal.DevicePortalConnection, Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::Diagnostics::DevicePortal::DevicePortalConnection*, ABI::Windows::System::Diagnostics::DevicePortal::DevicePortalConnectionClosedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace DevicePortal {
                    class DevicePortalConnectionRequestReceivedEventArgs;
                } /* DevicePortal */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d8e33ff8-8ac4-5fd9-b184-8ae87d828eb9"))
ITypedEventHandler<ABI::Windows::System::Diagnostics::DevicePortal::DevicePortalConnection*, ABI::Windows::System::Diagnostics::DevicePortal::DevicePortalConnectionRequestReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Diagnostics::DevicePortal::DevicePortalConnection*, ABI::Windows::System::Diagnostics::DevicePortal::IDevicePortalConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Diagnostics::DevicePortal::DevicePortalConnectionRequestReceivedEventArgs*, ABI::Windows::System::Diagnostics::DevicePortal::IDevicePortalConnectionRequestReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.Diagnostics.DevicePortal.DevicePortalConnection, Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionRequestReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::Diagnostics::DevicePortal::DevicePortalConnection*, ABI::Windows::System::Diagnostics::DevicePortal::DevicePortalConnectionRequestReceivedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                class AppServiceConnection;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                interface IAppServiceConnection;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection ABI::Windows::ApplicationModel::AppService::IAppServiceConnection

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class Deferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IDeferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIDeferral ABI::Windows::Foundation::IDeferral

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                typedef enum MessageWebSocketReceiveMode : int MessageWebSocketReceiveMode;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class ServerMessageWebSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IServerMessageWebSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket ABI::Windows::Networking::Sockets::IServerMessageWebSocket

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class ServerStreamWebSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IServerStreamWebSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket ABI::Windows::Networking::Sockets::IServerStreamWebSocket

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                typedef enum SocketMessageType : int SocketMessageType;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpRequestMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpRequestMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage ABI::Windows::Web::Http::IHttpRequestMessage

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpResponseMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpResponseMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage ABI::Windows::Web::Http::IHttpResponseMessage

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace DevicePortal {
                    typedef enum DevicePortalConnectionClosedReason : int DevicePortalConnectionClosedReason;
                } /* DevicePortal */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace DevicePortal {
                    enum DevicePortalConnectionClosedReason : int
                    {
                        DevicePortalConnectionClosedReason_Unknown = 0,
                        DevicePortalConnectionClosedReason_ResourceLimitsExceeded = 1,
                        DevicePortalConnectionClosedReason_ProtocolError = 2,
                        DevicePortalConnectionClosedReason_NotAuthorized = 3,
                        DevicePortalConnectionClosedReason_UserNotPresent = 4,
                        DevicePortalConnectionClosedReason_ServiceTerminated = 5,
                    };
                } /* DevicePortal */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.DevicePortal.IDevicePortalConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DevicePortal.DevicePortalConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_DevicePortal_IDevicePortalConnection[] = L"Windows.System.Diagnostics.DevicePortal.IDevicePortalConnection";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace DevicePortal {
                    MIDL_INTERFACE("0f447f51-1198-4da1-8d54-bdef393e09b6")
                    IDevicePortalConnection : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_Closed(
                            __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Closed(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_RequestReceived(
                            __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_RequestReceived(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDevicePortalConnection = __uuidof(IDevicePortalConnection);
                } /* DevicePortal */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionClosedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_DevicePortal_IDevicePortalConnectionClosedEventArgs[] = L"Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionClosedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace DevicePortal {
                    MIDL_INTERFACE("fcf70e38-7032-428c-9f50-945c15a9f0cb")
                    IDevicePortalConnectionClosedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Reason(
                            ABI::Windows::System::Diagnostics::DevicePortal::DevicePortalConnectionClosedReason* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDevicePortalConnectionClosedEventArgs = __uuidof(IDevicePortalConnectionClosedEventArgs);
                } /* DevicePortal */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionRequestReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionRequestReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_DevicePortal_IDevicePortalConnectionRequestReceivedEventArgs[] = L"Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionRequestReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace DevicePortal {
                    MIDL_INTERFACE("64dae045-6fda-4459-9ebd-ecce22e38559")
                    IDevicePortalConnectionRequestReceivedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_RequestMessage(
                            ABI::Windows::Web::Http::IHttpRequestMessage** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResponseMessage(
                            ABI::Windows::Web::Http::IHttpResponseMessage** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDevicePortalConnectionRequestReceivedEventArgs = __uuidof(IDevicePortalConnectionRequestReceivedEventArgs);
                } /* DevicePortal */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DevicePortal.DevicePortalConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_DevicePortal_IDevicePortalConnectionStatics[] = L"Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace DevicePortal {
                    MIDL_INTERFACE("4bbe31e7-e9b9-4645-8fed-a53eea0edbd6")
                    IDevicePortalConnectionStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetForAppServiceConnection(
                            ABI::Windows::ApplicationModel::AppService::IAppServiceConnection* appServiceConnection,
                            ABI::Windows::System::Diagnostics::DevicePortal::IDevicePortalConnection** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDevicePortalConnectionStatics = __uuidof(IDevicePortalConnectionStatics);
                } /* DevicePortal */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.DevicePortal.IDevicePortalWebSocketConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DevicePortal.DevicePortalConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_DevicePortal_IDevicePortalWebSocketConnection[] = L"Windows.System.Diagnostics.DevicePortal.IDevicePortalWebSocketConnection";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace DevicePortal {
                    MIDL_INTERFACE("67657920-d65a-42f0-aef4-787808098b7b")
                    IDevicePortalWebSocketConnection : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetServerMessageWebSocketForRequest(
                            ABI::Windows::Web::Http::IHttpRequestMessage* request,
                            ABI::Windows::Networking::Sockets::IServerMessageWebSocket** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetServerMessageWebSocketForRequest2(
                            ABI::Windows::Web::Http::IHttpRequestMessage* request,
                            ABI::Windows::Networking::Sockets::SocketMessageType messageType,
                            HSTRING protocol,
                            ABI::Windows::Networking::Sockets::IServerMessageWebSocket** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetServerMessageWebSocketForRequest3(
                            ABI::Windows::Web::Http::IHttpRequestMessage* request,
                            ABI::Windows::Networking::Sockets::SocketMessageType messageType,
                            HSTRING protocol,
                            UINT32 outboundBufferSizeInBytes,
                            UINT32 maxMessageSize,
                            ABI::Windows::Networking::Sockets::MessageWebSocketReceiveMode receiveMode,
                            ABI::Windows::Networking::Sockets::IServerMessageWebSocket** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetServerStreamWebSocketForRequest(
                            ABI::Windows::Web::Http::IHttpRequestMessage* request,
                            ABI::Windows::Networking::Sockets::IServerStreamWebSocket** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetServerStreamWebSocketForRequest2(
                            ABI::Windows::Web::Http::IHttpRequestMessage* request,
                            HSTRING protocol,
                            UINT32 outboundBufferSizeInBytes,
                            boolean noDelay,
                            ABI::Windows::Networking::Sockets::IServerStreamWebSocket** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDevicePortalWebSocketConnection = __uuidof(IDevicePortalWebSocketConnection);
                } /* DevicePortal */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.Diagnostics.DevicePortal.IDevicePortalWebSocketConnectionRequestReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionRequestReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_DevicePortal_IDevicePortalWebSocketConnectionRequestReceivedEventArgs[] = L"Windows.System.Diagnostics.DevicePortal.IDevicePortalWebSocketConnectionRequestReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace DevicePortal {
                    MIDL_INTERFACE("79fdcaba-175c-4739-9f74-dda797c35b3f")
                    IDevicePortalWebSocketConnectionRequestReceivedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsWebSocketUpgradeRequest(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WebSocketProtocolsRequested(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDevicePortalWebSocketConnectionRequestReceivedEventArgs = __uuidof(IDevicePortalWebSocketConnectionRequestReceivedEventArgs);
                } /* DevicePortal */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.System.Diagnostics.DevicePortal.DevicePortalConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.DevicePortal.IDevicePortalConnection ** Default Interface **
 *    Windows.System.Diagnostics.DevicePortal.IDevicePortalWebSocketConnection
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_DevicePortal_DevicePortalConnection_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_DevicePortal_DevicePortalConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_DevicePortal_DevicePortalConnection[] = L"Windows.System.Diagnostics.DevicePortal.DevicePortalConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionClosedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_DevicePortal_DevicePortalConnectionClosedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_DevicePortal_DevicePortalConnectionClosedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_DevicePortal_DevicePortalConnectionClosedEventArgs[] = L"Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionRequestReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionRequestReceivedEventArgs ** Default Interface **
 *    Windows.System.Diagnostics.DevicePortal.IDevicePortalWebSocketConnectionRequestReceivedEventArgs
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_DevicePortal_DevicePortalConnectionRequestReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_DevicePortal_DevicePortalConnectionRequestReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_DevicePortal_DevicePortalConnectionRequestReceivedEventArgs[] = L"Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionRequestReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection* sender,
        __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection* sender,
        __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CNetworking_CSockets_CMessageWebSocketReceiveMode __x_ABI_CWindows_CNetworking_CSockets_CMessageWebSocketReceiveMode;

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CNetworking_CSockets_CSocketMessageType __x_ABI_CWindows_CNetworking_CSockets_CSocketMessageType;

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CDevicePortalConnectionClosedReason __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CDevicePortalConnectionClosedReason;

/*
 *
 * Struct Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CDevicePortalConnectionClosedReason
{
    DevicePortalConnectionClosedReason_Unknown = 0,
    DevicePortalConnectionClosedReason_ResourceLimitsExceeded = 1,
    DevicePortalConnectionClosedReason_ProtocolError = 2,
    DevicePortalConnectionClosedReason_NotAuthorized = 3,
    DevicePortalConnectionClosedReason_UserNotPresent = 4,
    DevicePortalConnectionClosedReason_ServiceTerminated = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.DevicePortal.IDevicePortalConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DevicePortal.DevicePortalConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_DevicePortal_IDevicePortalConnection[] = L"Windows.System.Diagnostics.DevicePortal.IDevicePortalConnection";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_Closed)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection* This,
        __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionClosedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Closed)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_RequestReceived)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection* This,
        __FITypedEventHandler_2_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnection_Windows__CSystem__CDiagnostics__CDevicePortal__CDevicePortalConnectionRequestReceivedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RequestReceived)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_add_Closed(This, handler, token) \
    ((This)->lpVtbl->add_Closed(This, handler, token))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_remove_Closed(This, token) \
    ((This)->lpVtbl->remove_Closed(This, token))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_add_RequestReceived(This, handler, token) \
    ((This)->lpVtbl->add_RequestReceived(This, handler, token))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_remove_RequestReceived(This, token) \
    ((This)->lpVtbl->remove_RequestReceived(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionClosedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_DevicePortal_IDevicePortalConnectionClosedEventArgs[] = L"Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionClosedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Reason)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs* This,
        enum __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CDevicePortalConnectionClosedReason* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_get_Reason(This, value) \
    ((This)->lpVtbl->get_Reason(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionClosedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionRequestReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionRequestReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_DevicePortal_IDevicePortalConnectionRequestReceivedEventArgs[] = L"Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionRequestReceivedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RequestMessage)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage** value);
    HRESULT (STDMETHODCALLTYPE* get_ResponseMessage)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_get_RequestMessage(This, value) \
    ((This)->lpVtbl->get_RequestMessage(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_get_ResponseMessage(This, value) \
    ((This)->lpVtbl->get_ResponseMessage(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionRequestReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DevicePortal.DevicePortalConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_DevicePortal_IDevicePortalConnectionStatics[] = L"Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionStatics";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForAppServiceConnection)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics* This,
        __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* appServiceConnection,
        __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnection** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_GetForAppServiceConnection(This, appServiceConnection, value) \
    ((This)->lpVtbl->GetForAppServiceConnection(This, appServiceConnection, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalConnectionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.DevicePortal.IDevicePortalWebSocketConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DevicePortal.DevicePortalConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_DevicePortal_IDevicePortalWebSocketConnection[] = L"Windows.System.Diagnostics.DevicePortal.IDevicePortalWebSocketConnection";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetServerMessageWebSocketForRequest)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* request,
        __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket** result);
    HRESULT (STDMETHODCALLTYPE* GetServerMessageWebSocketForRequest2)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* request,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketMessageType messageType,
        HSTRING protocol,
        __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket** result);
    HRESULT (STDMETHODCALLTYPE* GetServerMessageWebSocketForRequest3)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* request,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketMessageType messageType,
        HSTRING protocol,
        UINT32 outboundBufferSizeInBytes,
        UINT32 maxMessageSize,
        enum __x_ABI_CWindows_CNetworking_CSockets_CMessageWebSocketReceiveMode receiveMode,
        __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket** result);
    HRESULT (STDMETHODCALLTYPE* GetServerStreamWebSocketForRequest)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* request,
        __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket** result);
    HRESULT (STDMETHODCALLTYPE* GetServerStreamWebSocketForRequest2)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* request,
        HSTRING protocol,
        UINT32 outboundBufferSizeInBytes,
        boolean noDelay,
        __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_GetServerMessageWebSocketForRequest(This, request, result) \
    ((This)->lpVtbl->GetServerMessageWebSocketForRequest(This, request, result))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_GetServerMessageWebSocketForRequest2(This, request, messageType, protocol, result) \
    ((This)->lpVtbl->GetServerMessageWebSocketForRequest2(This, request, messageType, protocol, result))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_GetServerMessageWebSocketForRequest3(This, request, messageType, protocol, outboundBufferSizeInBytes, maxMessageSize, receiveMode, result) \
    ((This)->lpVtbl->GetServerMessageWebSocketForRequest3(This, request, messageType, protocol, outboundBufferSizeInBytes, maxMessageSize, receiveMode, result))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_GetServerStreamWebSocketForRequest(This, request, result) \
    ((This)->lpVtbl->GetServerStreamWebSocketForRequest(This, request, result))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_GetServerStreamWebSocketForRequest2(This, request, protocol, outboundBufferSizeInBytes, noDelay, result) \
    ((This)->lpVtbl->GetServerStreamWebSocketForRequest2(This, request, protocol, outboundBufferSizeInBytes, noDelay, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.Diagnostics.DevicePortal.IDevicePortalWebSocketConnectionRequestReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionRequestReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_DevicePortal_IDevicePortalWebSocketConnectionRequestReceivedEventArgs[] = L"Windows.System.Diagnostics.DevicePortal.IDevicePortalWebSocketConnectionRequestReceivedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsWebSocketUpgradeRequest)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_WebSocketProtocolsRequested)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_get_IsWebSocketUpgradeRequest(This, value) \
    ((This)->lpVtbl->get_IsWebSocketUpgradeRequest(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_get_WebSocketProtocolsRequested(This, value) \
    ((This)->lpVtbl->get_WebSocketProtocolsRequested(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CDevicePortal_CIDevicePortalWebSocketConnectionRequestReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.System.Diagnostics.DevicePortal.DevicePortalConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.DevicePortal.IDevicePortalConnection ** Default Interface **
 *    Windows.System.Diagnostics.DevicePortal.IDevicePortalWebSocketConnection
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_DevicePortal_DevicePortalConnection_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_DevicePortal_DevicePortalConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_DevicePortal_DevicePortalConnection[] = L"Windows.System.Diagnostics.DevicePortal.DevicePortalConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionClosedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_DevicePortal_DevicePortalConnectionClosedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_DevicePortal_DevicePortalConnectionClosedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_DevicePortal_DevicePortalConnectionClosedEventArgs[] = L"Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionRequestReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionRequestReceivedEventArgs ** Default Interface **
 *    Windows.System.Diagnostics.DevicePortal.IDevicePortalWebSocketConnectionRequestReceivedEventArgs
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_DevicePortal_DevicePortalConnectionRequestReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_DevicePortal_DevicePortalConnectionRequestReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_DevicePortal_DevicePortalConnectionRequestReceivedEventArgs[] = L"Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionRequestReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Esystem2Ediagnostics2Edeviceportal_p_h__

#endif // __windows2Esystem2Ediagnostics2Edeviceportal_h__
