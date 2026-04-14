
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
#ifndef __windows2Enetworking2Esockets_h__
#define __windows2Enetworking2Esockets_h__
#ifndef __windows2Enetworking2Esockets_p_h__
#define __windows2Enetworking2Esockets_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_BACKGROUND_BACKGROUNDALARMAPPLICATIONCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_BACKGROUND_BACKGROUNDALARMAPPLICATIONCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_BACKGROUND_BACKGROUNDALARMAPPLICATIONCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)
#define WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)
#define WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION 0x130000
#endif // defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)

#if !defined(WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION)
#define WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION)

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
#include "Windows.ApplicationModel.Background.h"
#include "Windows.Networking.h"
#include "Windows.Networking.Connectivity.h"
#include "Windows.Security.Credentials.h"
#include "Windows.Security.Cryptography.Certificates.h"
#include "Windows.Storage.Streams.h"
#include "Windows.Web.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IControlChannelTrigger;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger ABI::Windows::Networking::Sockets::IControlChannelTrigger

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IControlChannelTrigger2;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2 ABI::Windows::Networking::Sockets::IControlChannelTrigger2

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IControlChannelTriggerEventDetails;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails ABI::Windows::Networking::Sockets::IControlChannelTriggerEventDetails

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IControlChannelTriggerFactory;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory ABI::Windows::Networking::Sockets::IControlChannelTriggerFactory

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IControlChannelTriggerResetEventDetails;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails ABI::Windows::Networking::Sockets::IControlChannelTriggerResetEventDetails

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IDatagramSocket2;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2 ABI::Windows::Networking::Sockets::IDatagramSocket2

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IDatagramSocket3;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3 ABI::Windows::Networking::Sockets::IDatagramSocket3

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IDatagramSocketControl;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl ABI::Windows::Networking::Sockets::IDatagramSocketControl

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IDatagramSocketControl2;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2 ABI::Windows::Networking::Sockets::IDatagramSocketControl2

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IDatagramSocketControl3;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3 ABI::Windows::Networking::Sockets::IDatagramSocketControl3

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IDatagramSocketInformation;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation ABI::Windows::Networking::Sockets::IDatagramSocketInformation

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IDatagramSocketMessageReceivedEventArgs;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs ABI::Windows::Networking::Sockets::IDatagramSocketMessageReceivedEventArgs

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IDatagramSocketStatics;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics ABI::Windows::Networking::Sockets::IDatagramSocketStatics

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IMessageWebSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket ABI::Windows::Networking::Sockets::IMessageWebSocket

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IMessageWebSocket2;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2 ABI::Windows::Networking::Sockets::IMessageWebSocket2

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IMessageWebSocket3;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3 ABI::Windows::Networking::Sockets::IMessageWebSocket3

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IMessageWebSocketControl;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl ABI::Windows::Networking::Sockets::IMessageWebSocketControl

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IMessageWebSocketControl2;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2 ABI::Windows::Networking::Sockets::IMessageWebSocketControl2

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IMessageWebSocketMessageReceivedEventArgs;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs ABI::Windows::Networking::Sockets::IMessageWebSocketMessageReceivedEventArgs

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IMessageWebSocketMessageReceivedEventArgs2;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2 ABI::Windows::Networking::Sockets::IMessageWebSocketMessageReceivedEventArgs2

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IServerMessageWebSocketControl;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl ABI::Windows::Networking::Sockets::IServerMessageWebSocketControl

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IServerMessageWebSocketInformation;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation ABI::Windows::Networking::Sockets::IServerMessageWebSocketInformation

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IServerStreamWebSocketInformation;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation ABI::Windows::Networking::Sockets::IServerStreamWebSocketInformation

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface ISocketActivityContext;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext ABI::Windows::Networking::Sockets::ISocketActivityContext

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface ISocketActivityContextFactory;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory ABI::Windows::Networking::Sockets::ISocketActivityContextFactory

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface ISocketActivityInformation;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation ABI::Windows::Networking::Sockets::ISocketActivityInformation

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface ISocketActivityInformationStatics;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics ABI::Windows::Networking::Sockets::ISocketActivityInformationStatics

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface ISocketActivityTriggerDetails;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails ABI::Windows::Networking::Sockets::ISocketActivityTriggerDetails

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface ISocketErrorStatics;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics ABI::Windows::Networking::Sockets::ISocketErrorStatics

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocket2;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2 ABI::Windows::Networking::Sockets::IStreamSocket2

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocket3;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3 ABI::Windows::Networking::Sockets::IStreamSocket3

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocketControl;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl ABI::Windows::Networking::Sockets::IStreamSocketControl

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocketControl2;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2 ABI::Windows::Networking::Sockets::IStreamSocketControl2

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocketControl3;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3 ABI::Windows::Networking::Sockets::IStreamSocketControl3

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocketControl4;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4 ABI::Windows::Networking::Sockets::IStreamSocketControl4

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocketInformation;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation ABI::Windows::Networking::Sockets::IStreamSocketInformation

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocketInformation2;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2 ABI::Windows::Networking::Sockets::IStreamSocketInformation2

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocketListener2;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2 ABI::Windows::Networking::Sockets::IStreamSocketListener2

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocketListener3;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3 ABI::Windows::Networking::Sockets::IStreamSocketListener3

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocketListenerConnectionReceivedEventArgs;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs ABI::Windows::Networking::Sockets::IStreamSocketListenerConnectionReceivedEventArgs

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocketListenerControl;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl ABI::Windows::Networking::Sockets::IStreamSocketListenerControl

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocketListenerControl2;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2 ABI::Windows::Networking::Sockets::IStreamSocketListenerControl2

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocketListenerInformation;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation ABI::Windows::Networking::Sockets::IStreamSocketListenerInformation

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocketStatics;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics ABI::Windows::Networking::Sockets::IStreamSocketStatics

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamWebSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket ABI::Windows::Networking::Sockets::IStreamWebSocket

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamWebSocket2;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2 ABI::Windows::Networking::Sockets::IStreamWebSocket2

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamWebSocketControl;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl ABI::Windows::Networking::Sockets::IStreamWebSocketControl

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamWebSocketControl2;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2 ABI::Windows::Networking::Sockets::IStreamWebSocketControl2

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IWebSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocket ABI::Windows::Networking::Sockets::IWebSocket

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IWebSocketClosedEventArgs;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs ABI::Windows::Networking::Sockets::IWebSocketClosedEventArgs

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IWebSocketControl;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl ABI::Windows::Networking::Sockets::IWebSocketControl

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IWebSocketControl2;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2 ABI::Windows::Networking::Sockets::IWebSocketControl2

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IWebSocketErrorStatics;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics ABI::Windows::Networking::Sockets::IWebSocketErrorStatics

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IWebSocketInformation;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation ABI::Windows::Networking::Sockets::IWebSocketInformation

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IWebSocketInformation2;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2 ABI::Windows::Networking::Sockets::IWebSocketInformation2

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IWebSocketServerCustomValidationRequestedEventArgs;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs ABI::Windows::Networking::Sockets::IWebSocketServerCustomValidationRequestedEventArgs

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("afc2ff8e-e393-566a-89c4-d043e940050d"))
IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CEndpointPair*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CNetworking__CEndpointPair*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.EndpointPair>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CEndpointPair*> __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("20d6faab-3b8e-5a1f-8397-b01cb219a18d"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CEndpointPair*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CNetworking__CEndpointPair*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.EndpointPair>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CEndpointPair*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IOutputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIOutputStream ABI::Windows::Storage::Streams::IOutputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e8736833-d013-5361-977d-c5e99934680e"))
IAsyncOperation<ABI::Windows::Storage::Streams::IOutputStream*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IOutputStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IOutputStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IOutputStream*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bcb37f4f-3af4-561c-a9e3-eef1738494d7"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IOutputStream*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IOutputStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IOutputStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IOutputStream*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1e466dc5-840f-54f9-b877-5e3a9f4b6c74"))
IAsyncOperationWithProgressCompletedHandler<UINT32, UINT32> : IAsyncOperationWithProgressCompletedHandler_impl<UINT32, UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<UInt32, UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<UINT32, UINT32> __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32 ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_USE */



#ifndef DEF___FIAsyncOperationWithProgress_2_UINT32_UINT32_USE
#define DEF___FIAsyncOperationWithProgress_2_UINT32_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("eccb574a-c684-5572-a679-6b0842cfb57f"))
IAsyncOperationWithProgress<UINT32, UINT32> : IAsyncOperationWithProgress_impl<UINT32, UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<UInt32, UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<UINT32, UINT32> __FIAsyncOperationWithProgress_2_UINT32_UINT32_t;
#define __FIAsyncOperationWithProgress_2_UINT32_UINT32 ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_UINT32_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_UINT32_UINT32_USE */



#ifndef DEF___FIAsyncOperationProgressHandler_2_UINT32_UINT32_USE
#define DEF___FIAsyncOperationProgressHandler_2_UINT32_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ea0fe405-d432-5ac7-9ef8-5a65e1f97d7e"))
IAsyncOperationProgressHandler<UINT32, UINT32> : IAsyncOperationProgressHandler_impl<UINT32, UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<UInt32, UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<UINT32, UINT32> __FIAsyncOperationProgressHandler_2_UINT32_UINT32_t;
#define __FIAsyncOperationProgressHandler_2_UINT32_UINT32 ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_UINT32_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_UINT32_UINT32_USE */



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
        namespace Networking {
            namespace Sockets {
                class SocketActivityInformation;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7e4bda2c-0125-587d-8806-1285060f3b2d"))
IKeyValuePair<HSTRING, ABI::Windows::Networking::Sockets::SocketActivityInformation*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::SocketActivityInformation*, ABI::Windows::Networking::Sockets::ISocketActivityInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Networking.Sockets.SocketActivityInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::Networking::Sockets::SocketActivityInformation*> __FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f656f7a2-264b-5cfd-8288-64eb89455157"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Networking.Sockets.SocketActivityInformation>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3e43fa16-7af1-51df-a0d3-da81b321639d"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Networking.Sockets.SocketActivityInformation>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class Certificate;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificate;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate ABI::Windows::Security::Cryptography::Certificates::ICertificate

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#define DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("676fc159-f15c-58bd-91a7-28f7e795c756"))
IIterator<ABI::Windows::Security::Cryptography::Certificates::Certificate*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::Certificate*, ABI::Windows::Security::Cryptography::Certificates::ICertificate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Cryptography.Certificates.Certificate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Security::Cryptography::Certificates::Certificate*> __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t;
#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#define DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0c7d1423-e8fd-5a91-b55c-8bfbe7ac2d40"))
IIterable<ABI::Windows::Security::Cryptography::Certificates::Certificate*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::Certificate*, ABI::Windows::Security::Cryptography::Certificates::ICertificate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Cryptography.Certificates.Certificate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Security::Cryptography::Certificates::Certificate*> __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t;
#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    typedef enum ChainValidationResult : int ChainValidationResult;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE
#define DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8bcad2b7-0e3b-5eae-bf69-e1f6d9c888f8"))
IIterator<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult> : IIterator_impl<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Cryptography.Certificates.ChainValidationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult> __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_t;
#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE
#define DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2628f58f-3f02-54f2-808f-e1117709d6d0"))
IIterable<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult> : IIterable_impl<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Cryptography.Certificates.ChainValidationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult> __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_t;
#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_USE
#define DEF___FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e6ac8bee-a31c-5af2-9227-5be2f9e80763"))
IMapView<HSTRING, ABI::Windows::Networking::Sockets::SocketActivityInformation*> : IMapView_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::SocketActivityInformation*, ABI::Windows::Networking::Sockets::ISocketActivityInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.Networking.Sockets.SocketActivityInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::Networking::Sockets::SocketActivityInformation*> __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_t;
#define __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_USE */

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

#ifndef DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#define DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("963f7013-77c2-51c5-8038-b5bcef633edb"))
IVectorView<ABI::Windows::Security::Cryptography::Certificates::Certificate*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::Certificate*, ABI::Windows::Security::Cryptography::Certificates::ICertificate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Security.Cryptography.Certificates.Certificate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Security::Cryptography::Certificates::Certificate*> __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t;
#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE
#define DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cb383486-c2bc-5756-912d-6a708a07e5bd"))
IVectorView<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult> : IVectorView_impl<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Security.Cryptography.Certificates.ChainValidationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult> __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_t;
#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE */

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

#ifndef DEF___FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE
#define DEF___FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d7828cf7-4301-58d3-aab5-06e5eefcf79f"))
IVector<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult> : IVector_impl<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Security.Cryptography.Certificates.ChainValidationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult> __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_t;
#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class DatagramSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class DatagramSocketMessageReceivedEventArgs;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4482e19b-2389-5767-9b0b-8d7a8ef55743"))
ITypedEventHandler<ABI::Windows::Networking::Sockets::DatagramSocket*, ABI::Windows::Networking::Sockets::DatagramSocketMessageReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::DatagramSocket*, ABI::Windows::Networking::Sockets::IDatagramSocket*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::DatagramSocketMessageReceivedEventArgs*, ABI::Windows::Networking::Sockets::IDatagramSocketMessageReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.Sockets.DatagramSocket, Windows.Networking.Sockets.DatagramSocketMessageReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::Sockets::DatagramSocket*, ABI::Windows::Networking::Sockets::DatagramSocketMessageReceivedEventArgs*> __FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class WebSocketClosedEventArgs;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("03cf8f90-1669-5f4d-9404-2b784678e6dd"))
ITypedEventHandler<ABI::Windows::Networking::Sockets::IWebSocket*, ABI::Windows::Networking::Sockets::WebSocketClosedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Networking::Sockets::IWebSocket*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::WebSocketClosedEventArgs*, ABI::Windows::Networking::Sockets::IWebSocketClosedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.Sockets.IWebSocket, Windows.Networking.Sockets.WebSocketClosedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::Sockets::IWebSocket*, ABI::Windows::Networking::Sockets::WebSocketClosedEventArgs*> __FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class MessageWebSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class MessageWebSocketMessageReceivedEventArgs;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("de980538-6dcd-52b0-802f-4b6cf59a01ab"))
ITypedEventHandler<ABI::Windows::Networking::Sockets::MessageWebSocket*, ABI::Windows::Networking::Sockets::MessageWebSocketMessageReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::MessageWebSocket*, ABI::Windows::Networking::Sockets::IMessageWebSocket*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::MessageWebSocketMessageReceivedEventArgs*, ABI::Windows::Networking::Sockets::IMessageWebSocketMessageReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.Sockets.MessageWebSocket, Windows.Networking.Sockets.MessageWebSocketMessageReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::Sockets::MessageWebSocket*, ABI::Windows::Networking::Sockets::MessageWebSocketMessageReceivedEventArgs*> __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class WebSocketServerCustomValidationRequestedEventArgs;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2c34c585-9cf6-56c7-8dd1-5da26e322078"))
ITypedEventHandler<ABI::Windows::Networking::Sockets::MessageWebSocket*, ABI::Windows::Networking::Sockets::WebSocketServerCustomValidationRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::MessageWebSocket*, ABI::Windows::Networking::Sockets::IMessageWebSocket*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::WebSocketServerCustomValidationRequestedEventArgs*, ABI::Windows::Networking::Sockets::IWebSocketServerCustomValidationRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.Sockets.MessageWebSocket, Windows.Networking.Sockets.WebSocketServerCustomValidationRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::Sockets::MessageWebSocket*, ABI::Windows::Networking::Sockets::WebSocketServerCustomValidationRequestedEventArgs*> __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class ServerMessageWebSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6e66714b-94d1-5c08-b2b3-10b891d08747"))
ITypedEventHandler<ABI::Windows::Networking::Sockets::ServerMessageWebSocket*, ABI::Windows::Networking::Sockets::MessageWebSocketMessageReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::ServerMessageWebSocket*, ABI::Windows::Networking::Sockets::IServerMessageWebSocket*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::MessageWebSocketMessageReceivedEventArgs*, ABI::Windows::Networking::Sockets::IMessageWebSocketMessageReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.Sockets.ServerMessageWebSocket, Windows.Networking.Sockets.MessageWebSocketMessageReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::Sockets::ServerMessageWebSocket*, ABI::Windows::Networking::Sockets::MessageWebSocketMessageReceivedEventArgs*> __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cae65204-b986-5cf3-87ea-b67b00fba78d"))
ITypedEventHandler<ABI::Windows::Networking::Sockets::ServerMessageWebSocket*, ABI::Windows::Networking::Sockets::WebSocketClosedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::ServerMessageWebSocket*, ABI::Windows::Networking::Sockets::IServerMessageWebSocket*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::WebSocketClosedEventArgs*, ABI::Windows::Networking::Sockets::IWebSocketClosedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.Sockets.ServerMessageWebSocket, Windows.Networking.Sockets.WebSocketClosedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::Sockets::ServerMessageWebSocket*, ABI::Windows::Networking::Sockets::WebSocketClosedEventArgs*> __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class ServerStreamWebSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b51c5b3b-161b-559e-a553-0059336329cc"))
ITypedEventHandler<ABI::Windows::Networking::Sockets::ServerStreamWebSocket*, ABI::Windows::Networking::Sockets::WebSocketClosedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::ServerStreamWebSocket*, ABI::Windows::Networking::Sockets::IServerStreamWebSocket*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::WebSocketClosedEventArgs*, ABI::Windows::Networking::Sockets::IWebSocketClosedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.Sockets.ServerStreamWebSocket, Windows.Networking.Sockets.WebSocketClosedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::Sockets::ServerStreamWebSocket*, ABI::Windows::Networking::Sockets::WebSocketClosedEventArgs*> __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class StreamSocketListener;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class StreamSocketListenerConnectionReceivedEventArgs;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("33d00d41-c94f-5a61-9ab7-280dcefa0b08"))
ITypedEventHandler<ABI::Windows::Networking::Sockets::StreamSocketListener*, ABI::Windows::Networking::Sockets::StreamSocketListenerConnectionReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::StreamSocketListener*, ABI::Windows::Networking::Sockets::IStreamSocketListener*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::StreamSocketListenerConnectionReceivedEventArgs*, ABI::Windows::Networking::Sockets::IStreamSocketListenerConnectionReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.Sockets.StreamSocketListener, Windows.Networking.Sockets.StreamSocketListenerConnectionReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::Sockets::StreamSocketListener*, ABI::Windows::Networking::Sockets::StreamSocketListenerConnectionReceivedEventArgs*> __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class StreamWebSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("18b143d3-897e-595e-acc1-ef35614b4cec"))
ITypedEventHandler<ABI::Windows::Networking::Sockets::StreamWebSocket*, ABI::Windows::Networking::Sockets::WebSocketServerCustomValidationRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::StreamWebSocket*, ABI::Windows::Networking::Sockets::IStreamWebSocket*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Sockets::WebSocketServerCustomValidationRequestedEventArgs*, ABI::Windows::Networking::Sockets::IWebSocketServerCustomValidationRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.Sockets.StreamWebSocket, Windows.Networking.Sockets.WebSocketServerCustomValidationRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::Sockets::StreamWebSocket*, ABI::Windows::Networking::Sockets::WebSocketServerCustomValidationRequestedEventArgs*> __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTask_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTask_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Background {
                interface IBackgroundTask;
            } /* Background */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTask ABI::Windows::ApplicationModel::Background::IBackgroundTask

#endif // ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTask_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Background {
                interface IBackgroundTrigger;
            } /* Background */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger ABI::Windows::ApplicationModel::Background::IBackgroundTrigger

#endif // ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger_FWD_DEFINED__

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
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
            typedef enum HostNameSortOptions : unsigned int HostNameSortOptions;
        } /* Networking */
    } /* Windows */
} /* ABI */

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
        namespace Storage {
            namespace Streams {
                class DataReader;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIDataReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataReader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IDataReader;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader ABI::Windows::Storage::Streams::IDataReader

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIDataReader_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IInputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIInputStream ABI::Windows::Storage::Streams::IInputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Web {
            typedef enum WebErrorStatus : int WebErrorStatus;
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                typedef enum ControlChannelTriggerResetReason : int ControlChannelTriggerResetReason;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                typedef enum ControlChannelTriggerResourceType : int ControlChannelTriggerResourceType;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                typedef enum ControlChannelTriggerStatus : int ControlChannelTriggerStatus;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

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
                typedef enum SocketActivityConnectedStandbyAction : int SocketActivityConnectedStandbyAction;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                typedef enum SocketActivityKind : int SocketActivityKind;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                typedef enum SocketActivityTriggerReason : int SocketActivityTriggerReason;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                typedef enum SocketErrorStatus : int SocketErrorStatus;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

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
                typedef enum SocketQualityOfService : int SocketQualityOfService;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                typedef enum SocketSslErrorSeverity : int SocketSslErrorSeverity;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                typedef struct BandwidthStatistics BandwidthStatistics;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                typedef struct RoundTripTimeStatistics RoundTripTimeStatistics;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class ControlChannelTrigger;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class DatagramSocketControl;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class DatagramSocketInformation;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class MessageWebSocketControl;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class MessageWebSocketInformation;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class ServerMessageWebSocketControl;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class ServerMessageWebSocketInformation;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class ServerStreamWebSocketInformation;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class SocketActivityContext;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class StreamSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class StreamSocketControl;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class StreamSocketInformation;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class StreamSocketListenerControl;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class StreamSocketListenerInformation;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class StreamWebSocketControl;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class StreamWebSocketInformation;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Networking.Sockets.ControlChannelTriggerResetReason
 *
 * Introduced to Windows.Networking.Sockets.ControlChannelTriggerContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                enum ControlChannelTriggerResetReason : int
                {
                    ControlChannelTriggerResetReason_FastUserSwitched = 0,
                    ControlChannelTriggerResetReason_LowPowerExit = 1,
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
                    ControlChannelTriggerResetReason_QuietHoursExit = 2,
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
                    ControlChannelTriggerResetReason_ApplicationRestart = 3,
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
                };
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.ControlChannelTriggerResourceType
 *
 * Introduced to Windows.Networking.Sockets.ControlChannelTriggerContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                enum ControlChannelTriggerResourceType : int
                {
                    ControlChannelTriggerResourceType_RequestSoftwareSlot = 0,
                    ControlChannelTriggerResourceType_RequestHardwareSlot = 1,
                };
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.ControlChannelTriggerStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000 || \
    WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000 && WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION < 0x30000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                enum ControlChannelTriggerStatus : int
                {
                    ControlChannelTriggerStatus_HardwareSlotRequested = 0,
                    ControlChannelTriggerStatus_SoftwareSlotAllocated = 1,
                    ControlChannelTriggerStatus_HardwareSlotAllocated = 2,
                    ControlChannelTriggerStatus_PolicyError = 3,
                    ControlChannelTriggerStatus_SystemError = 4,
                    ControlChannelTriggerStatus_TransportDisconnected = 5,
                    ControlChannelTriggerStatus_ServiceUnavailable = 6,
                };
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000 || \
       // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000 && WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION < 0x30000

/*
 *
 * Struct Windows.Networking.Sockets.MessageWebSocketReceiveMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                enum MessageWebSocketReceiveMode : int
                {
                    MessageWebSocketReceiveMode_FullMessage = 0,
                    MessageWebSocketReceiveMode_PartialMessage = 1,
                };
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Networking.Sockets.SocketActivityConnectedStandbyAction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                enum SocketActivityConnectedStandbyAction : int
                {
                    SocketActivityConnectedStandbyAction_DoNotWake = 0,
                    SocketActivityConnectedStandbyAction_Wake = 1,
                };
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.SocketActivityKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                enum SocketActivityKind : int
                {
                    SocketActivityKind_None = 0,
                    SocketActivityKind_StreamSocketListener = 1,
                    SocketActivityKind_DatagramSocket = 2,
                    SocketActivityKind_StreamSocket = 3,
                };
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.SocketActivityTriggerReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                enum SocketActivityTriggerReason : int
                {
                    SocketActivityTriggerReason_None = 0,
                    SocketActivityTriggerReason_SocketActivity = 1,
                    SocketActivityTriggerReason_ConnectionAccepted = 2,
                    SocketActivityTriggerReason_KeepAliveTimerExpired = 3,
                    SocketActivityTriggerReason_SocketClosed = 4,
                };
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.SocketErrorStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                enum SocketErrorStatus : int
                {
                    SocketErrorStatus_Unknown = 0,
                    SocketErrorStatus_OperationAborted = 1,
                    SocketErrorStatus_HttpInvalidServerResponse = 2,
                    SocketErrorStatus_ConnectionTimedOut = 3,
                    SocketErrorStatus_AddressFamilyNotSupported = 4,
                    SocketErrorStatus_SocketTypeNotSupported = 5,
                    SocketErrorStatus_HostNotFound = 6,
                    SocketErrorStatus_NoDataRecordOfRequestedType = 7,
                    SocketErrorStatus_NonAuthoritativeHostNotFound = 8,
                    SocketErrorStatus_ClassTypeNotFound = 9,
                    SocketErrorStatus_AddressAlreadyInUse = 10,
                    SocketErrorStatus_CannotAssignRequestedAddress = 11,
                    SocketErrorStatus_ConnectionRefused = 12,
                    SocketErrorStatus_NetworkIsUnreachable = 13,
                    SocketErrorStatus_UnreachableHost = 14,
                    SocketErrorStatus_NetworkIsDown = 15,
                    SocketErrorStatus_NetworkDroppedConnectionOnReset = 16,
                    SocketErrorStatus_SoftwareCausedConnectionAbort = 17,
                    SocketErrorStatus_ConnectionResetByPeer = 18,
                    SocketErrorStatus_HostIsDown = 19,
                    SocketErrorStatus_NoAddressesFound = 20,
                    SocketErrorStatus_TooManyOpenFiles = 21,
                    SocketErrorStatus_MessageTooLong = 22,
                    SocketErrorStatus_CertificateExpired = 23,
                    SocketErrorStatus_CertificateUntrustedRoot = 24,
                    SocketErrorStatus_CertificateCommonNameIsIncorrect = 25,
                    SocketErrorStatus_CertificateWrongUsage = 26,
                    SocketErrorStatus_CertificateRevoked = 27,
                    SocketErrorStatus_CertificateNoRevocationCheck = 28,
                    SocketErrorStatus_CertificateRevocationServerOffline = 29,
                    SocketErrorStatus_CertificateIsInvalid = 30,
                };
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.SocketMessageType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                enum SocketMessageType : int
                {
                    SocketMessageType_Binary = 0,
                    SocketMessageType_Utf8 = 1,
                };
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.SocketProtectionLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                enum SocketProtectionLevel : int
                {
                    SocketProtectionLevel_PlainSocket = 0,
                    SocketProtectionLevel_Ssl
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("Ssl may result in insecure connections and may be altered or unavailable for releases after Windows 8.1. Instead, use one of the TLS SocketProtectionLevel values.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 1,
                    SocketProtectionLevel_SslAllowNullEncryption = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SocketProtectionLevel_BluetoothEncryptionAllowNullAuthentication = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SocketProtectionLevel_BluetoothEncryptionWithAuthentication = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SocketProtectionLevel_Ssl3AllowWeakEncryption
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("Ssl3AllowWeakEncryption may result in insecure connections and may be altered or unavailable for releases after Windows 8.1. Instead, use one of the TLS SocketProtectionLevel values.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SocketProtectionLevel_Tls10 = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SocketProtectionLevel_Tls11 = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SocketProtectionLevel_Tls12 = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                    SocketProtectionLevel_Unspecified = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
                    SocketProtectionLevel_Tls13 = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
                };
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.SocketQualityOfService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                enum SocketQualityOfService : int
                {
                    SocketQualityOfService_Normal = 0,
                    SocketQualityOfService_LowLatency = 1,
                };
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.SocketSslErrorSeverity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                enum SocketSslErrorSeverity : int
                {
                    SocketSslErrorSeverity_None = 0,
                    SocketSslErrorSeverity_Ignorable = 1,
                    SocketSslErrorSeverity_Fatal = 2,
                };
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.BandwidthStatistics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                struct BandwidthStatistics
                {
                    UINT64 OutboundBitsPerSecond;
                    UINT64 InboundBitsPerSecond;
                    UINT64 OutboundBitsPerSecondInstability;
                    UINT64 InboundBitsPerSecondInstability;
                    boolean OutboundBandwidthPeaked;
                    boolean InboundBandwidthPeaked;
                };
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.RoundTripTimeStatistics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                struct RoundTripTimeStatistics
                {
                    UINT32 Variance;
                    UINT32 Max;
                    UINT32 Min;
                    UINT32 Sum;
                };
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IControlChannelTrigger
 *
 * Introduced to Windows.Networking.Sockets.ControlChannelTriggerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.ControlChannelTrigger
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IControlChannelTrigger[] = L"Windows.Networking.Sockets.IControlChannelTrigger";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("7d1431a7-ee96-40e8-a199-8703cd969ec3")
                IControlChannelTrigger : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ControlChannelTriggerId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServerKeepAliveIntervalInMinutes(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ServerKeepAliveIntervalInMinutes(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentKeepAliveIntervalInMinutes(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransportObject(
                        IInspectable** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_KeepAliveTrigger(
                        ABI::Windows::ApplicationModel::Background::IBackgroundTrigger** trigger
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PushNotificationTrigger(
                        ABI::Windows::ApplicationModel::Background::IBackgroundTrigger** trigger
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UsingTransport(
                        IInspectable* transport
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WaitForPushEnabled(
                        ABI::Windows::Networking::Sockets::ControlChannelTriggerStatus* channelTriggerStatus
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DecreaseNetworkKeepAliveInterval(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FlushTransport(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IControlChannelTrigger = __uuidof(IControlChannelTrigger);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IControlChannelTrigger2
 *
 * Introduced to Windows.Networking.Sockets.ControlChannelTriggerContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.ControlChannelTrigger
 *
 */
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IControlChannelTrigger2[] = L"Windows.Networking.Sockets.IControlChannelTrigger2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("af00d237-51be-4514-9725-3556e1879580")
                IControlChannelTrigger2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsWakeFromLowPowerSupported(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IControlChannelTrigger2 = __uuidof(IControlChannelTrigger2);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Networking.Sockets.IControlChannelTriggerEventDetails
 *
 * Introduced to Windows.Networking.Sockets.ControlChannelTriggerContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IControlChannelTriggerEventDetails[] = L"Windows.Networking.Sockets.IControlChannelTriggerEventDetails";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("1b36e047-89bb-4236-96ac-71d012bb4869")
                IControlChannelTriggerEventDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ControlChannelTrigger(
                        ABI::Windows::Networking::Sockets::IControlChannelTrigger** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IControlChannelTriggerEventDetails = __uuidof(IControlChannelTriggerEventDetails);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IControlChannelTriggerFactory
 *
 * Introduced to Windows.Networking.Sockets.ControlChannelTriggerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.ControlChannelTrigger
 *
 */
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IControlChannelTriggerFactory[] = L"Windows.Networking.Sockets.IControlChannelTriggerFactory";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("da4b7cf0-8d71-446f-88c3-b95184a2d6cd")
                IControlChannelTriggerFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateControlChannelTrigger(
                        HSTRING channelId,
                        UINT32 serverKeepAliveIntervalInMinutes,
                        ABI::Windows::Networking::Sockets::IControlChannelTrigger** notificationChannel
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateControlChannelTriggerEx(
                        HSTRING channelId,
                        UINT32 serverKeepAliveIntervalInMinutes,
                        ABI::Windows::Networking::Sockets::ControlChannelTriggerResourceType resourceRequestType,
                        ABI::Windows::Networking::Sockets::IControlChannelTrigger** notificationChannel
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IControlChannelTriggerFactory = __uuidof(IControlChannelTriggerFactory);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IControlChannelTriggerResetEventDetails
 *
 * Introduced to Windows.Networking.Sockets.ControlChannelTriggerContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IControlChannelTriggerResetEventDetails[] = L"Windows.Networking.Sockets.IControlChannelTriggerResetEventDetails";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("6851038e-8ec4-42fe-9bb2-21e91b7bfcb1")
                IControlChannelTriggerResetEventDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ResetReason(
                        ABI::Windows::Networking::Sockets::ControlChannelTriggerResetReason* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HardwareSlotReset(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SoftwareSlotReset(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IControlChannelTriggerResetEventDetails = __uuidof(IControlChannelTriggerResetEventDetails);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IDatagramSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.DatagramSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IDatagramSocket[] = L"Windows.Networking.Sockets.IDatagramSocket";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("7fe25bbb-c3bc-4677-8446-ca28a465a3af")
                IDatagramSocket : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Control(
                        ABI::Windows::Networking::Sockets::IDatagramSocketControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Information(
                        ABI::Windows::Networking::Sockets::IDatagramSocketInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutputStream(
                        ABI::Windows::Storage::Streams::IOutputStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConnectAsync(
                        ABI::Windows::Networking::IHostName* remoteHostName,
                        HSTRING remoteServiceName,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConnectWithEndpointPairAsync(
                        ABI::Windows::Networking::IEndpointPair* endpointPair,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE BindServiceNameAsync(
                        HSTRING localServiceName,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE BindEndpointAsync(
                        ABI::Windows::Networking::IHostName* localHostName,
                        HSTRING localServiceName,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE JoinMulticastGroup(
                        ABI::Windows::Networking::IHostName* host
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetOutputStreamAsync(
                        ABI::Windows::Networking::IHostName* remoteHostName,
                        HSTRING remoteServiceName,
                        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetOutputStreamWithEndpointPairAsync(
                        ABI::Windows::Networking::IEndpointPair* endpointPair,
                        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_MessageReceived(
                        __FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_MessageReceived(
                        EventRegistrationToken eventCookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDatagramSocket = __uuidof(IDatagramSocket);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IDatagramSocket2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.DatagramSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IDatagramSocket2[] = L"Windows.Networking.Sockets.IDatagramSocket2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("d83ba354-9a9d-4185-a20a-1424c9c2a7cd")
                IDatagramSocket2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE BindServiceNameAndAdapterAsync(
                        HSTRING localServiceName,
                        ABI::Windows::Networking::Connectivity::INetworkAdapter* adapter,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDatagramSocket2 = __uuidof(IDatagramSocket2);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IDatagramSocket3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.DatagramSocket
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IDatagramSocket3[] = L"Windows.Networking.Sockets.IDatagramSocket3";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("37544f09-ab92-4306-9ac1-0c381283d9c6")
                IDatagramSocket3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CancelIOAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EnableTransferOwnership(
                        GUID taskId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EnableTransferOwnershipWithConnectedStandbyAction(
                        GUID taskId,
                        ABI::Windows::Networking::Sockets::SocketActivityConnectedStandbyAction connectedStandbyAction
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TransferOwnership(
                        HSTRING socketId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TransferOwnershipWithContext(
                        HSTRING socketId,
                        ABI::Windows::Networking::Sockets::ISocketActivityContext* data
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TransferOwnershipWithContextAndKeepAliveTime(
                        HSTRING socketId,
                        ABI::Windows::Networking::Sockets::ISocketActivityContext* data,
                        ABI::Windows::Foundation::TimeSpan keepAliveTime
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDatagramSocket3 = __uuidof(IDatagramSocket3);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IDatagramSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.DatagramSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IDatagramSocketControl[] = L"Windows.Networking.Sockets.IDatagramSocketControl";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("52ac3f2e-349a-4135-bb58-b79b2647d390")
                IDatagramSocketControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_QualityOfService(
                        ABI::Windows::Networking::Sockets::SocketQualityOfService* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_QualityOfService(
                        ABI::Windows::Networking::Sockets::SocketQualityOfService value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutboundUnicastHopLimit(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OutboundUnicastHopLimit(
                        BYTE value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDatagramSocketControl = __uuidof(IDatagramSocketControl);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IDatagramSocketControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.DatagramSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IDatagramSocketControl2[] = L"Windows.Networking.Sockets.IDatagramSocketControl2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("33ead5c2-979c-4415-82a1-3cfaf646c192")
                IDatagramSocketControl2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_InboundBufferSizeInBytes(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_InboundBufferSizeInBytes(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DontFragment(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DontFragment(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDatagramSocketControl2 = __uuidof(IDatagramSocketControl2);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IDatagramSocketControl3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.DatagramSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IDatagramSocketControl3[] = L"Windows.Networking.Sockets.IDatagramSocketControl3";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("d4eb8256-1f6d-4598-9b57-d42a001df349")
                IDatagramSocketControl3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MulticastOnly(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MulticastOnly(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDatagramSocketControl3 = __uuidof(IDatagramSocketControl3);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IDatagramSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.DatagramSocketInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IDatagramSocketInformation[] = L"Windows.Networking.Sockets.IDatagramSocketInformation";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("5f1a569a-55fb-48cd-9706-7a974f7b1585")
                IDatagramSocketInformation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LocalAddress(
                        ABI::Windows::Networking::IHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LocalPort(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteAddress(
                        ABI::Windows::Networking::IHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemotePort(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDatagramSocketInformation = __uuidof(IDatagramSocketInformation);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IDatagramSocketMessageReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.DatagramSocketMessageReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IDatagramSocketMessageReceivedEventArgs[] = L"Windows.Networking.Sockets.IDatagramSocketMessageReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("9e2ddca2-1712-4ce4-b179-8c652c6d107e")
                IDatagramSocketMessageReceivedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteAddress(
                        ABI::Windows::Networking::IHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemotePort(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LocalAddress(
                        ABI::Windows::Networking::IHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDataReader(
                        ABI::Windows::Storage::Streams::IDataReader** dataReader
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDataStream(
                        ABI::Windows::Storage::Streams::IInputStream** inputStream
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDatagramSocketMessageReceivedEventArgs = __uuidof(IDatagramSocketMessageReceivedEventArgs);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IDatagramSocketStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.DatagramSocket
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IDatagramSocketStatics[] = L"Windows.Networking.Sockets.IDatagramSocketStatics";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("e9c62aee-1494-4a21-bb7e-8589fc751d9d")
                IDatagramSocketStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetEndpointPairsAsync(
                        ABI::Windows::Networking::IHostName* remoteHostName,
                        HSTRING remoteServiceName,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetEndpointPairsWithSortOptionsAsync(
                        ABI::Windows::Networking::IHostName* remoteHostName,
                        HSTRING remoteServiceName,
                        ABI::Windows::Networking::HostNameSortOptions sortOptions,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDatagramSocketStatics = __uuidof(IDatagramSocketStatics);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IMessageWebSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.MessageWebSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Sockets.IWebSocket
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IMessageWebSocket[] = L"Windows.Networking.Sockets.IMessageWebSocket";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("33727d08-34d5-4746-ad7b-8dde5bc2ef88")
                IMessageWebSocket : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Control(
                        ABI::Windows::Networking::Sockets::IMessageWebSocketControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Information(
                        ABI::Windows::Networking::Sockets::IWebSocketInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_MessageReceived(
                        __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_MessageReceived(
                        EventRegistrationToken eventCookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMessageWebSocket = __uuidof(IMessageWebSocket);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IMessageWebSocket2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.MessageWebSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Sockets.IMessageWebSocket
 *     Windows.Networking.Sockets.IWebSocket
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IMessageWebSocket2[] = L"Windows.Networking.Sockets.IMessageWebSocket2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("bed0cee7-f9c8-440a-9ad5-737281d9742e")
                IMessageWebSocket2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_ServerCustomValidationRequested(
                        __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ServerCustomValidationRequested(
                        EventRegistrationToken eventCookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMessageWebSocket2 = __uuidof(IMessageWebSocket2);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.Sockets.IMessageWebSocket3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.MessageWebSocket
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IMessageWebSocket3[] = L"Windows.Networking.Sockets.IMessageWebSocket3";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("59d9defb-71af-4349-8487-911fcf681597")
                IMessageWebSocket3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SendNonfinalFrameAsync(
                        ABI::Windows::Storage::Streams::IBuffer* data,
                        __FIAsyncOperationWithProgress_2_UINT32_UINT32** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendFinalFrameAsync(
                        ABI::Windows::Storage::Streams::IBuffer* data,
                        __FIAsyncOperationWithProgress_2_UINT32_UINT32** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMessageWebSocket3 = __uuidof(IMessageWebSocket3);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Sockets.IMessageWebSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.MessageWebSocketControl
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Sockets.IWebSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IMessageWebSocketControl[] = L"Windows.Networking.Sockets.IMessageWebSocketControl";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("8118388a-c629-4f0a-80fb-81fc05538862")
                IMessageWebSocketControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MaxMessageSize(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MaxMessageSize(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MessageType(
                        ABI::Windows::Networking::Sockets::SocketMessageType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MessageType(
                        ABI::Windows::Networking::Sockets::SocketMessageType value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMessageWebSocketControl = __uuidof(IMessageWebSocketControl);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IMessageWebSocketControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.MessageWebSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IMessageWebSocketControl2[] = L"Windows.Networking.Sockets.IMessageWebSocketControl2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("e30fd791-080c-400a-a712-27dfa9e744d8")
                IMessageWebSocketControl2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DesiredUnsolicitedPongInterval(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DesiredUnsolicitedPongInterval(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActualUnsolicitedPongInterval(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReceiveMode(
                        ABI::Windows::Networking::Sockets::MessageWebSocketReceiveMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ReceiveMode(
                        ABI::Windows::Networking::Sockets::MessageWebSocketReceiveMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ClientCertificate(
                        ABI::Windows::Security::Cryptography::Certificates::ICertificate** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ClientCertificate(
                        ABI::Windows::Security::Cryptography::Certificates::ICertificate* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMessageWebSocketControl2 = __uuidof(IMessageWebSocketControl2);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Networking.Sockets.IMessageWebSocketMessageReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.MessageWebSocketMessageReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IMessageWebSocketMessageReceivedEventArgs[] = L"Windows.Networking.Sockets.IMessageWebSocketMessageReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("478c22ac-4c4b-42ed-9ed7-1ef9f94fa3d5")
                IMessageWebSocketMessageReceivedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MessageType(
                        ABI::Windows::Networking::Sockets::SocketMessageType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDataReader(
                        ABI::Windows::Storage::Streams::IDataReader** dataReader
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDataStream(
                        ABI::Windows::Storage::Streams::IInputStream** inputStream
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMessageWebSocketMessageReceivedEventArgs = __uuidof(IMessageWebSocketMessageReceivedEventArgs);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IMessageWebSocketMessageReceivedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.MessageWebSocketMessageReceivedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Sockets.IMessageWebSocketMessageReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IMessageWebSocketMessageReceivedEventArgs2[] = L"Windows.Networking.Sockets.IMessageWebSocketMessageReceivedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("89ce06fd-dd6f-4a07-87f9-f9eb4d89d83d")
                IMessageWebSocketMessageReceivedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsMessageComplete(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMessageWebSocketMessageReceivedEventArgs2 = __uuidof(IMessageWebSocketMessageReceivedEventArgs2);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Networking.Sockets.IServerMessageWebSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.ServerMessageWebSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IServerMessageWebSocket[] = L"Windows.Networking.Sockets.IServerMessageWebSocket";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("e3ac9240-813b-5efd-7e11-ae2305fc77f1")
                IServerMessageWebSocket : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_MessageReceived(
                        __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_MessageReceived(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Control(
                        ABI::Windows::Networking::Sockets::IServerMessageWebSocketControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Information(
                        ABI::Windows::Networking::Sockets::IServerMessageWebSocketInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutputStream(
                        ABI::Windows::Storage::Streams::IOutputStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Closed(
                        __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Closed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CloseWithStatus(
                        UINT16 code,
                        HSTRING reason
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IServerMessageWebSocket = __uuidof(IServerMessageWebSocket);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Sockets.IServerMessageWebSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.ServerMessageWebSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IServerMessageWebSocketControl[] = L"Windows.Networking.Sockets.IServerMessageWebSocketControl";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("69c2f051-1c1f-587a-4519-2181610192b7")
                IServerMessageWebSocketControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MessageType(
                        ABI::Windows::Networking::Sockets::SocketMessageType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MessageType(
                        ABI::Windows::Networking::Sockets::SocketMessageType value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IServerMessageWebSocketControl = __uuidof(IServerMessageWebSocketControl);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Sockets.IServerMessageWebSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.ServerMessageWebSocketInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IServerMessageWebSocketInformation[] = L"Windows.Networking.Sockets.IServerMessageWebSocketInformation";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("fc32b45f-4448-5505-6cc9-09afa8915f5d")
                IServerMessageWebSocketInformation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BandwidthStatistics(
                        ABI::Windows::Networking::Sockets::BandwidthStatistics* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Protocol(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LocalAddress(
                        ABI::Windows::Networking::IHostName** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IServerMessageWebSocketInformation = __uuidof(IServerMessageWebSocketInformation);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Sockets.IServerStreamWebSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.ServerStreamWebSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IServerStreamWebSocket[] = L"Windows.Networking.Sockets.IServerStreamWebSocket";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("2ced5bbf-74f6-55e4-79df-9132680dfee8")
                IServerStreamWebSocket : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Information(
                        ABI::Windows::Networking::Sockets::IServerStreamWebSocketInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InputStream(
                        ABI::Windows::Storage::Streams::IInputStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutputStream(
                        ABI::Windows::Storage::Streams::IOutputStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Closed(
                        __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Closed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CloseWithStatus(
                        UINT16 code,
                        HSTRING reason
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IServerStreamWebSocket = __uuidof(IServerStreamWebSocket);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Sockets.IServerStreamWebSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.ServerStreamWebSocketInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IServerStreamWebSocketInformation[] = L"Windows.Networking.Sockets.IServerStreamWebSocketInformation";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("fc32b45f-4448-5505-6cc9-09aba8915f5d")
                IServerStreamWebSocketInformation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BandwidthStatistics(
                        ABI::Windows::Networking::Sockets::BandwidthStatistics* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Protocol(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LocalAddress(
                        ABI::Windows::Networking::IHostName** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IServerStreamWebSocketInformation = __uuidof(IServerStreamWebSocketInformation);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Sockets.ISocketActivityContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.SocketActivityContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_ISocketActivityContext[] = L"Windows.Networking.Sockets.ISocketActivityContext";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("43b04d64-4c85-4396-a637-1d973f6ebd49")
                ISocketActivityContext : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Data(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISocketActivityContext = __uuidof(ISocketActivityContext);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.ISocketActivityContextFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.SocketActivityContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_ISocketActivityContextFactory[] = L"Windows.Networking.Sockets.ISocketActivityContextFactory";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("b99fc3c3-088c-4388-83ae-2525138e049a")
                ISocketActivityContextFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Storage::Streams::IBuffer* data,
                        ABI::Windows::Networking::Sockets::ISocketActivityContext** context
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISocketActivityContextFactory = __uuidof(ISocketActivityContextFactory);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.ISocketActivityInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.SocketActivityInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_ISocketActivityInformation[] = L"Windows.Networking.Sockets.ISocketActivityInformation";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("8d8a42e4-a87e-4b74-9968-185b2511defe")
                ISocketActivityInformation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TaskId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SocketKind(
                        ABI::Windows::Networking::Sockets::SocketActivityKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Context(
                        ABI::Windows::Networking::Sockets::ISocketActivityContext** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DatagramSocket(
                        ABI::Windows::Networking::Sockets::IDatagramSocket** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StreamSocket(
                        ABI::Windows::Networking::Sockets::IStreamSocket** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StreamSocketListener(
                        ABI::Windows::Networking::Sockets::IStreamSocketListener** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISocketActivityInformation = __uuidof(ISocketActivityInformation);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.ISocketActivityInformationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.SocketActivityInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_ISocketActivityInformationStatics[] = L"Windows.Networking.Sockets.ISocketActivityInformationStatics";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("8570b47a-7e7d-4736-8041-1327a6543c56")
                ISocketActivityInformationStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AllSockets(
                        __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation** sockets
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISocketActivityInformationStatics = __uuidof(ISocketActivityInformationStatics);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.ISocketActivityTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.SocketActivityTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_ISocketActivityTriggerDetails[] = L"Windows.Networking.Sockets.ISocketActivityTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("45f406a7-fc9f-4f81-acad-355fef51e67b")
                ISocketActivityTriggerDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Reason(
                        ABI::Windows::Networking::Sockets::SocketActivityTriggerReason* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SocketInformation(
                        ABI::Windows::Networking::Sockets::ISocketActivityInformation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISocketActivityTriggerDetails = __uuidof(ISocketActivityTriggerDetails);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.ISocketErrorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.SocketError
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_ISocketErrorStatics[] = L"Windows.Networking.Sockets.ISocketErrorStatics";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("828337f4-7d56-4d8e-b7b4-a07dd7c1bca9")
                ISocketErrorStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetStatus(
                        INT32 hresult,
                        ABI::Windows::Networking::Sockets::SocketErrorStatus* status
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISocketErrorStatics = __uuidof(ISocketErrorStatics);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocket[] = L"Windows.Networking.Sockets.IStreamSocket";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("69a22cf3-fc7b-4857-af38-f6e7de6a5b49")
                IStreamSocket : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Control(
                        ABI::Windows::Networking::Sockets::IStreamSocketControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Information(
                        ABI::Windows::Networking::Sockets::IStreamSocketInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InputStream(
                        ABI::Windows::Storage::Streams::IInputStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutputStream(
                        ABI::Windows::Storage::Streams::IOutputStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConnectWithEndpointPairAsync(
                        ABI::Windows::Networking::IEndpointPair* endpointPair,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConnectAsync(
                        ABI::Windows::Networking::IHostName* remoteHostName,
                        HSTRING remoteServiceName,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConnectWithEndpointPairAndProtectionLevelAsync(
                        ABI::Windows::Networking::IEndpointPair* endpointPair,
                        ABI::Windows::Networking::Sockets::SocketProtectionLevel protectionLevel,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConnectWithProtectionLevelAsync(
                        ABI::Windows::Networking::IHostName* remoteHostName,
                        HSTRING remoteServiceName,
                        ABI::Windows::Networking::Sockets::SocketProtectionLevel protectionLevel,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UpgradeToSslAsync(
                        ABI::Windows::Networking::Sockets::SocketProtectionLevel protectionLevel,
                        ABI::Windows::Networking::IHostName* validationHostName,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamSocket = __uuidof(IStreamSocket);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocket2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocket2[] = L"Windows.Networking.Sockets.IStreamSocket2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("29d0e575-f314-4d09-adf0-0fbd967fbd9f")
                IStreamSocket2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ConnectWithProtectionLevelAndAdapterAsync(
                        ABI::Windows::Networking::IHostName* remoteHostName,
                        HSTRING remoteServiceName,
                        ABI::Windows::Networking::Sockets::SocketProtectionLevel protectionLevel,
                        ABI::Windows::Networking::Connectivity::INetworkAdapter* adapter,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamSocket2 = __uuidof(IStreamSocket2);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocket3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocket
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocket3[] = L"Windows.Networking.Sockets.IStreamSocket3";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("3f430b00-9d28-4854-bac3-2301941ec223")
                IStreamSocket3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CancelIOAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EnableTransferOwnership(
                        GUID taskId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EnableTransferOwnershipWithConnectedStandbyAction(
                        GUID taskId,
                        ABI::Windows::Networking::Sockets::SocketActivityConnectedStandbyAction connectedStandbyAction
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TransferOwnership(
                        HSTRING socketId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TransferOwnershipWithContext(
                        HSTRING socketId,
                        ABI::Windows::Networking::Sockets::ISocketActivityContext* data
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TransferOwnershipWithContextAndKeepAliveTime(
                        HSTRING socketId,
                        ABI::Windows::Networking::Sockets::ISocketActivityContext* data,
                        ABI::Windows::Foundation::TimeSpan keepAliveTime
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamSocket3 = __uuidof(IStreamSocket3);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketControl[] = L"Windows.Networking.Sockets.IStreamSocketControl";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("fe25adf1-92ab-4af3-9992-0f4c85e36cc4")
                IStreamSocketControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NoDelay(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NoDelay(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_KeepAlive(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_KeepAlive(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutboundBufferSizeInBytes(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OutboundBufferSizeInBytes(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_QualityOfService(
                        ABI::Windows::Networking::Sockets::SocketQualityOfService* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_QualityOfService(
                        ABI::Windows::Networking::Sockets::SocketQualityOfService value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutboundUnicastHopLimit(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OutboundUnicastHopLimit(
                        BYTE value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamSocketControl = __uuidof(IStreamSocketControl);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketControl2[] = L"Windows.Networking.Sockets.IStreamSocketControl2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("c2d09a56-060f-44c1-b8e2-1fbf60bd62c5")
                IStreamSocketControl2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IgnorableServerCertificateErrors(
                        __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamSocketControl2 = __uuidof(IStreamSocketControl2);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketControl3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketControl3[] = L"Windows.Networking.Sockets.IStreamSocketControl3";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("c56a444c-4e74-403e-894c-b31cae5c7342")
                IStreamSocketControl3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SerializeConnectionAttempts(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SerializeConnectionAttempts(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ClientCertificate(
                        ABI::Windows::Security::Cryptography::Certificates::ICertificate** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ClientCertificate(
                        ABI::Windows::Security::Cryptography::Certificates::ICertificate* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamSocketControl3 = __uuidof(IStreamSocketControl3);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketControl4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketControl4[] = L"Windows.Networking.Sockets.IStreamSocketControl4";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("964e2b3d-ec27-4888-b3ce-c74b418423ad")
                IStreamSocketControl4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MinProtectionLevel(
                        ABI::Windows::Networking::Sockets::SocketProtectionLevel* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MinProtectionLevel(
                        ABI::Windows::Networking::Sockets::SocketProtectionLevel value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamSocketControl4 = __uuidof(IStreamSocketControl4);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketInformation[] = L"Windows.Networking.Sockets.IStreamSocketInformation";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("3b80ae30-5e68-4205-88f0-dc85d2e25ded")
                IStreamSocketInformation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LocalAddress(
                        ABI::Windows::Networking::IHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LocalPort(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteHostName(
                        ABI::Windows::Networking::IHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteAddress(
                        ABI::Windows::Networking::IHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteServiceName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemotePort(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RoundTripTimeStatistics(
                        ABI::Windows::Networking::Sockets::RoundTripTimeStatistics* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BandwidthStatistics(
                        ABI::Windows::Networking::Sockets::BandwidthStatistics* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProtectionLevel(
                        ABI::Windows::Networking::Sockets::SocketProtectionLevel* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SessionKey(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamSocketInformation = __uuidof(IStreamSocketInformation);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketInformation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketInformation2[] = L"Windows.Networking.Sockets.IStreamSocketInformation2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("12c28452-4bdc-4ee4-976a-cf130e9d92e3")
                IStreamSocketInformation2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ServerCertificateErrorSeverity(
                        ABI::Windows::Networking::Sockets::SocketSslErrorSeverity* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServerCertificateErrors(
                        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServerCertificate(
                        ABI::Windows::Security::Cryptography::Certificates::ICertificate** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServerIntermediateCertificates(
                        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamSocketInformation2 = __uuidof(IStreamSocketInformation2);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketListener
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketListener[] = L"Windows.Networking.Sockets.IStreamSocketListener";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("ff513437-df9f-4df0-bf82-0ec5d7b35aae")
                IStreamSocketListener : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Control(
                        ABI::Windows::Networking::Sockets::IStreamSocketListenerControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Information(
                        ABI::Windows::Networking::Sockets::IStreamSocketListenerInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE BindServiceNameAsync(
                        HSTRING localServiceName,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE BindEndpointAsync(
                        ABI::Windows::Networking::IHostName* localHostName,
                        HSTRING localServiceName,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ConnectionReceived(
                        __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ConnectionReceived(
                        EventRegistrationToken eventCookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamSocketListener = __uuidof(IStreamSocketListener);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketListener2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketListener
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketListener2[] = L"Windows.Networking.Sockets.IStreamSocketListener2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("658dc13e-bb3e-4458-b232-ed1088694b98")
                IStreamSocketListener2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE BindServiceNameWithProtectionLevelAsync(
                        HSTRING localServiceName,
                        ABI::Windows::Networking::Sockets::SocketProtectionLevel protectionLevel,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE BindServiceNameWithProtectionLevelAndAdapterAsync(
                        HSTRING localServiceName,
                        ABI::Windows::Networking::Sockets::SocketProtectionLevel protectionLevel,
                        ABI::Windows::Networking::Connectivity::INetworkAdapter* adapter,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamSocketListener2 = __uuidof(IStreamSocketListener2);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketListener3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketListener3[] = L"Windows.Networking.Sockets.IStreamSocketListener3";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("4798201c-bdf8-4919-8542-28d450e74507")
                IStreamSocketListener3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CancelIOAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EnableTransferOwnership(
                        GUID taskId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EnableTransferOwnershipWithConnectedStandbyAction(
                        GUID taskId,
                        ABI::Windows::Networking::Sockets::SocketActivityConnectedStandbyAction connectedStandbyAction
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TransferOwnership(
                        HSTRING socketId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TransferOwnershipWithContext(
                        HSTRING socketId,
                        ABI::Windows::Networking::Sockets::ISocketActivityContext* data
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamSocketListener3 = __uuidof(IStreamSocketListener3);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketListenerConnectionReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketListenerConnectionReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketListenerConnectionReceivedEventArgs[] = L"Windows.Networking.Sockets.IStreamSocketListenerConnectionReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("0c472ea9-373f-447b-85b1-ddd4548803ba")
                IStreamSocketListenerConnectionReceivedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Socket(
                        ABI::Windows::Networking::Sockets::IStreamSocket** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamSocketListenerConnectionReceivedEventArgs = __uuidof(IStreamSocketListenerConnectionReceivedEventArgs);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketListenerControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketListenerControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketListenerControl[] = L"Windows.Networking.Sockets.IStreamSocketListenerControl";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("20d8c576-8d8a-4dba-9722-a16c4d984980")
                IStreamSocketListenerControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_QualityOfService(
                        ABI::Windows::Networking::Sockets::SocketQualityOfService* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_QualityOfService(
                        ABI::Windows::Networking::Sockets::SocketQualityOfService value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamSocketListenerControl = __uuidof(IStreamSocketListenerControl);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketListenerControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketListenerControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketListenerControl2[] = L"Windows.Networking.Sockets.IStreamSocketListenerControl2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("948bb665-2c3e-404b-b8b0-8eb249a2b0a1")
                IStreamSocketListenerControl2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NoDelay(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NoDelay(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_KeepAlive(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_KeepAlive(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutboundBufferSizeInBytes(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OutboundBufferSizeInBytes(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutboundUnicastHopLimit(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OutboundUnicastHopLimit(
                        BYTE value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamSocketListenerControl2 = __uuidof(IStreamSocketListenerControl2);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketListenerInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketListenerInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketListenerInformation[] = L"Windows.Networking.Sockets.IStreamSocketListenerInformation";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("e62ba82f-a63a-430b-bf62-29e93e5633b4")
                IStreamSocketListenerInformation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LocalPort(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamSocketListenerInformation = __uuidof(IStreamSocketListenerInformation);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocket
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketStatics[] = L"Windows.Networking.Sockets.IStreamSocketStatics";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("a420bc4a-6e2e-4af5-b556-355ae0cd4f29")
                IStreamSocketStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetEndpointPairsAsync(
                        ABI::Windows::Networking::IHostName* remoteHostName,
                        HSTRING remoteServiceName,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetEndpointPairsWithSortOptionsAsync(
                        ABI::Windows::Networking::IHostName* remoteHostName,
                        HSTRING remoteServiceName,
                        ABI::Windows::Networking::HostNameSortOptions sortOptions,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamSocketStatics = __uuidof(IStreamSocketStatics);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamWebSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamWebSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Sockets.IWebSocket
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamWebSocket[] = L"Windows.Networking.Sockets.IStreamWebSocket";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("bd4a49d8-b289-45bb-97eb-c7525205a843")
                IStreamWebSocket : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Control(
                        ABI::Windows::Networking::Sockets::IStreamWebSocketControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Information(
                        ABI::Windows::Networking::Sockets::IWebSocketInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InputStream(
                        ABI::Windows::Storage::Streams::IInputStream** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamWebSocket = __uuidof(IStreamWebSocket);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamWebSocket2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamWebSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Sockets.IStreamWebSocket
 *     Windows.Networking.Sockets.IWebSocket
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamWebSocket2[] = L"Windows.Networking.Sockets.IStreamWebSocket2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("aa4d08cb-93f5-4678-8236-57cce5417ed5")
                IStreamWebSocket2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_ServerCustomValidationRequested(
                        __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ServerCustomValidationRequested(
                        EventRegistrationToken eventCookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamWebSocket2 = __uuidof(IStreamWebSocket2);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamWebSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamWebSocketControl
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Sockets.IWebSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamWebSocketControl[] = L"Windows.Networking.Sockets.IStreamWebSocketControl";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("b4f478b1-a45a-48db-953a-645b7d964c07")
                IStreamWebSocketControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NoDelay(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NoDelay(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamWebSocketControl = __uuidof(IStreamWebSocketControl);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamWebSocketControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamWebSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamWebSocketControl2[] = L"Windows.Networking.Sockets.IStreamWebSocketControl2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("215d9f7e-fa58-40da-9f11-a48dafe95037")
                IStreamWebSocketControl2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DesiredUnsolicitedPongInterval(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DesiredUnsolicitedPongInterval(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActualUnsolicitedPongInterval(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ClientCertificate(
                        ABI::Windows::Security::Cryptography::Certificates::ICertificate** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ClientCertificate(
                        ABI::Windows::Security::Cryptography::Certificates::ICertificate* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamWebSocketControl2 = __uuidof(IStreamWebSocketControl2);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Networking.Sockets.IWebSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IWebSocket[] = L"Windows.Networking.Sockets.IWebSocket";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("f877396f-99b1-4e18-bc08-850c9adf156e")
                IWebSocket : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_OutputStream(
                        ABI::Windows::Storage::Streams::IOutputStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConnectAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetRequestHeader(
                        HSTRING headerName,
                        HSTRING headerValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Closed(
                        __FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Closed(
                        EventRegistrationToken eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CloseWithStatus(
                        UINT16 code,
                        HSTRING reason
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebSocket = __uuidof(IWebSocket);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIWebSocket;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IWebSocketClosedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.WebSocketClosedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IWebSocketClosedEventArgs[] = L"Windows.Networking.Sockets.IWebSocketClosedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("ceb78d07-d0a8-4703-a091-c8c2c0915bc3")
                IWebSocketClosedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Code(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Reason(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebSocketClosedEventArgs = __uuidof(IWebSocketClosedEventArgs);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IWebSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IWebSocketControl[] = L"Windows.Networking.Sockets.IWebSocketControl";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("2ec4bdc3-d9a5-455a-9811-de24d45337e9")
                IWebSocketControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_OutboundBufferSizeInBytes(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OutboundBufferSizeInBytes(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServerCredential(
                        ABI::Windows::Security::Credentials::IPasswordCredential** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ServerCredential(
                        ABI::Windows::Security::Credentials::IPasswordCredential* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProxyCredential(
                        ABI::Windows::Security::Credentials::IPasswordCredential** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ProxyCredential(
                        ABI::Windows::Security::Credentials::IPasswordCredential* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedProtocols(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebSocketControl = __uuidof(IWebSocketControl);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IWebSocketControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Sockets.IWebSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IWebSocketControl2[] = L"Windows.Networking.Sockets.IWebSocketControl2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("79c3be03-f2ca-461e-af4e-9665bc2d0620")
                IWebSocketControl2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IgnorableServerCertificateErrors(
                        __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebSocketControl2 = __uuidof(IWebSocketControl2);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.Sockets.IWebSocketErrorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.WebSocketError
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IWebSocketErrorStatics[] = L"Windows.Networking.Sockets.IWebSocketErrorStatics";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("27cdf35b-1f61-4709-8e02-61283ada4e9d")
                IWebSocketErrorStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetStatus(
                        INT32 hresult,
                        ABI::Windows::Web::WebErrorStatus* status
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebSocketErrorStatics = __uuidof(IWebSocketErrorStatics);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IWebSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IWebSocketInformation[] = L"Windows.Networking.Sockets.IWebSocketInformation";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("5e01e316-c92a-47a5-b25f-07847639d181")
                IWebSocketInformation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LocalAddress(
                        ABI::Windows::Networking::IHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BandwidthStatistics(
                        ABI::Windows::Networking::Sockets::BandwidthStatistics* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Protocol(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebSocketInformation = __uuidof(IWebSocketInformation);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IWebSocketInformation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Sockets.IWebSocketInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IWebSocketInformation2[] = L"Windows.Networking.Sockets.IWebSocketInformation2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("ce1d39ce-a1b7-4d43-8269-8d5b981bd47a")
                IWebSocketInformation2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ServerCertificate(
                        ABI::Windows::Security::Cryptography::Certificates::ICertificate** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServerCertificateErrorSeverity(
                        ABI::Windows::Networking::Sockets::SocketSslErrorSeverity* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServerCertificateErrors(
                        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServerIntermediateCertificates(
                        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebSocketInformation2 = __uuidof(IWebSocketInformation2);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.Sockets.IWebSocketServerCustomValidationRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.WebSocketServerCustomValidationRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IWebSocketServerCustomValidationRequestedEventArgs[] = L"Windows.Networking.Sockets.IWebSocketServerCustomValidationRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                MIDL_INTERFACE("ffeffe48-022a-4ab7-8b36-e10af4640e6b")
                IWebSocketServerCustomValidationRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ServerCertificate(
                        ABI::Windows::Security::Cryptography::Certificates::ICertificate** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServerCertificateErrorSeverity(
                        ABI::Windows::Networking::Sockets::SocketSslErrorSeverity* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServerCertificateErrors(
                        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServerIntermediateCertificates(
                        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Reject(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebSocketServerCustomValidationRequestedEventArgs = __uuidof(IWebSocketServerCustomValidationRequestedEventArgs);
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Networking.Sockets.ControlChannelTrigger
 *
 * Introduced to Windows.Networking.Sockets.ControlChannelTriggerContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Sockets.IControlChannelTriggerFactory interface starting with version 1.0 of the Windows.Networking.Sockets.ControlChannelTriggerContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IControlChannelTrigger ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Networking.Sockets.IControlChannelTrigger2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_ControlChannelTrigger_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_ControlChannelTrigger_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_ControlChannelTrigger[] = L"Windows.Networking.Sockets.ControlChannelTrigger";
#endif
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.DatagramSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Sockets.IDatagramSocketStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IDatagramSocket ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Networking.Sockets.IDatagramSocket2
 *    Windows.Networking.Sockets.IDatagramSocket3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_DatagramSocket_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_DatagramSocket_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_DatagramSocket[] = L"Windows.Networking.Sockets.DatagramSocket";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.DatagramSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IDatagramSocketControl ** Default Interface **
 *    Windows.Networking.Sockets.IDatagramSocketControl2
 *    Windows.Networking.Sockets.IDatagramSocketControl3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_DatagramSocketControl_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_DatagramSocketControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_DatagramSocketControl[] = L"Windows.Networking.Sockets.DatagramSocketControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.DatagramSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IDatagramSocketInformation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_DatagramSocketInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_DatagramSocketInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_DatagramSocketInformation[] = L"Windows.Networking.Sockets.DatagramSocketInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.DatagramSocketMessageReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IDatagramSocketMessageReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_DatagramSocketMessageReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_DatagramSocketMessageReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_DatagramSocketMessageReceivedEventArgs[] = L"Windows.Networking.Sockets.DatagramSocketMessageReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.MessageWebSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IMessageWebSocket ** Default Interface **
 *    Windows.Networking.Sockets.IWebSocket
 *    Windows.Foundation.IClosable
 *    Windows.Networking.Sockets.IMessageWebSocket2
 *    Windows.Networking.Sockets.IMessageWebSocket3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_MessageWebSocket_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_MessageWebSocket_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_MessageWebSocket[] = L"Windows.Networking.Sockets.MessageWebSocket";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.MessageWebSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IMessageWebSocketControl ** Default Interface **
 *    Windows.Networking.Sockets.IWebSocketControl
 *    Windows.Networking.Sockets.IWebSocketControl2
 *    Windows.Networking.Sockets.IMessageWebSocketControl2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_MessageWebSocketControl_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_MessageWebSocketControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_MessageWebSocketControl[] = L"Windows.Networking.Sockets.MessageWebSocketControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.MessageWebSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IWebSocketInformation ** Default Interface **
 *    Windows.Networking.Sockets.IWebSocketInformation2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_MessageWebSocketInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_MessageWebSocketInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_MessageWebSocketInformation[] = L"Windows.Networking.Sockets.MessageWebSocketInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.MessageWebSocketMessageReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IMessageWebSocketMessageReceivedEventArgs ** Default Interface **
 *    Windows.Networking.Sockets.IMessageWebSocketMessageReceivedEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_MessageWebSocketMessageReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_MessageWebSocketMessageReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_MessageWebSocketMessageReceivedEventArgs[] = L"Windows.Networking.Sockets.MessageWebSocketMessageReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.ServerMessageWebSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IServerMessageWebSocket ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_ServerMessageWebSocket_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_ServerMessageWebSocket_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_ServerMessageWebSocket[] = L"Windows.Networking.Sockets.ServerMessageWebSocket";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Networking.Sockets.ServerMessageWebSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IServerMessageWebSocketControl ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_ServerMessageWebSocketControl_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_ServerMessageWebSocketControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_ServerMessageWebSocketControl[] = L"Windows.Networking.Sockets.ServerMessageWebSocketControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Networking.Sockets.ServerMessageWebSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IServerMessageWebSocketInformation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_ServerMessageWebSocketInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_ServerMessageWebSocketInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_ServerMessageWebSocketInformation[] = L"Windows.Networking.Sockets.ServerMessageWebSocketInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Networking.Sockets.ServerStreamWebSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IServerStreamWebSocket ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_ServerStreamWebSocket_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_ServerStreamWebSocket_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_ServerStreamWebSocket[] = L"Windows.Networking.Sockets.ServerStreamWebSocket";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Networking.Sockets.ServerStreamWebSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IServerStreamWebSocketInformation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_ServerStreamWebSocketInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_ServerStreamWebSocketInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_ServerStreamWebSocketInformation[] = L"Windows.Networking.Sockets.ServerStreamWebSocketInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Networking.Sockets.SocketActivityContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Sockets.ISocketActivityContextFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.ISocketActivityContext ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_SocketActivityContext_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_SocketActivityContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_SocketActivityContext[] = L"Windows.Networking.Sockets.SocketActivityContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.SocketActivityInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Sockets.ISocketActivityInformationStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.ISocketActivityInformation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_SocketActivityInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_SocketActivityInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_SocketActivityInformation[] = L"Windows.Networking.Sockets.SocketActivityInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.SocketActivityTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.ISocketActivityTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_SocketActivityTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_SocketActivityTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_SocketActivityTriggerDetails[] = L"Windows.Networking.Sockets.SocketActivityTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.SocketError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Sockets.ISocketErrorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_SocketError_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_SocketError_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_SocketError[] = L"Windows.Networking.Sockets.SocketError";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Sockets.IStreamSocketStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IStreamSocket ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Networking.Sockets.IStreamSocket2
 *    Windows.Networking.Sockets.IStreamSocket3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamSocket_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamSocket_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamSocket[] = L"Windows.Networking.Sockets.StreamSocket";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IStreamSocketControl ** Default Interface **
 *    Windows.Networking.Sockets.IStreamSocketControl2
 *    Windows.Networking.Sockets.IStreamSocketControl3
 *    Windows.Networking.Sockets.IStreamSocketControl4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketControl_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamSocketControl[] = L"Windows.Networking.Sockets.StreamSocketControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IStreamSocketInformation ** Default Interface **
 *    Windows.Networking.Sockets.IStreamSocketInformation2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamSocketInformation[] = L"Windows.Networking.Sockets.StreamSocketInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamSocketListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IStreamSocketListener ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Networking.Sockets.IStreamSocketListener2
 *    Windows.Networking.Sockets.IStreamSocketListener3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketListener_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketListener_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamSocketListener[] = L"Windows.Networking.Sockets.StreamSocketListener";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamSocketListenerConnectionReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IStreamSocketListenerConnectionReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketListenerConnectionReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketListenerConnectionReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamSocketListenerConnectionReceivedEventArgs[] = L"Windows.Networking.Sockets.StreamSocketListenerConnectionReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamSocketListenerControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IStreamSocketListenerControl ** Default Interface **
 *    Windows.Networking.Sockets.IStreamSocketListenerControl2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketListenerControl_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketListenerControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamSocketListenerControl[] = L"Windows.Networking.Sockets.StreamSocketListenerControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamSocketListenerInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IStreamSocketListenerInformation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketListenerInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketListenerInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamSocketListenerInformation[] = L"Windows.Networking.Sockets.StreamSocketListenerInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamWebSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IStreamWebSocket ** Default Interface **
 *    Windows.Networking.Sockets.IWebSocket
 *    Windows.Foundation.IClosable
 *    Windows.Networking.Sockets.IStreamWebSocket2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamWebSocket_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamWebSocket_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamWebSocket[] = L"Windows.Networking.Sockets.StreamWebSocket";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamWebSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IStreamWebSocketControl ** Default Interface **
 *    Windows.Networking.Sockets.IWebSocketControl
 *    Windows.Networking.Sockets.IWebSocketControl2
 *    Windows.Networking.Sockets.IStreamWebSocketControl2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamWebSocketControl_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamWebSocketControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamWebSocketControl[] = L"Windows.Networking.Sockets.StreamWebSocketControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamWebSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IWebSocketInformation ** Default Interface **
 *    Windows.Networking.Sockets.IWebSocketInformation2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamWebSocketInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamWebSocketInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamWebSocketInformation[] = L"Windows.Networking.Sockets.StreamWebSocketInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.WebSocketClosedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IWebSocketClosedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_WebSocketClosedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_WebSocketClosedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_WebSocketClosedEventArgs[] = L"Windows.Networking.Sockets.WebSocketClosedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.WebSocketError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Sockets.IWebSocketErrorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_WebSocketError_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_WebSocketError_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_WebSocketError[] = L"Windows.Networking.Sockets.WebSocketError";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.WebSocketKeepAlive
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Background.IBackgroundTask ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_WebSocketKeepAlive_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_WebSocketKeepAlive_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_WebSocketKeepAlive[] = L"Windows.Networking.Sockets.WebSocketKeepAlive";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.WebSocketServerCustomValidationRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IWebSocketServerCustomValidationRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_WebSocketServerCustomValidationRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_WebSocketServerCustomValidationRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_WebSocketServerCustomValidationRequestedEventArgs[] = L"Windows.Networking.Sockets.WebSocketServerCustomValidationRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2 __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2 __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3 __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2 __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3 __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2 __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3 __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2 __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2 __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics __x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2 __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3 __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2 __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3 __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4 __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2 __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2 __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3 __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2 __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2 __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2 __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIWebSocket __x_ABI_CWindows_CNetworking_CSockets_CIWebSocket;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2 __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2 __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

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

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPairVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair* This,
        __FIVectorView_1_Windows__CNetworking__CEndpointPair** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPairVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPairVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPairVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPairVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPairVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CEndpointPair_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIOutputStream __x_ABI_CWindows_CStorage_CStreams_CIOutputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStreamVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStreamVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIOutputStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationProgressHandler_2_UINT32_UINT32 __FIAsyncOperationProgressHandler_2_UINT32_UINT32;

typedef interface __FIAsyncOperationWithProgress_2_UINT32_UINT32 __FIAsyncOperationWithProgress_2_UINT32_UINT32;

#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32 __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32* This,
        __FIAsyncOperationWithProgress_2_UINT32_UINT32* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32Vtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationWithProgress_2_UINT32_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_UINT32_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_UINT32_UINT32 __FIAsyncOperationWithProgress_2_UINT32_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_UINT32_UINT32;

typedef struct __FIAsyncOperationWithProgress_2_UINT32_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        __FIAsyncOperationProgressHandler_2_UINT32_UINT32* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        __FIAsyncOperationProgressHandler_2_UINT32_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        UINT32* result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_UINT32_UINT32Vtbl;

interface __FIAsyncOperationWithProgress_2_UINT32_UINT32
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_UINT32_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_UINT32_UINT32_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationProgressHandler_2_UINT32_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_UINT32_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_UINT32_UINT32 __FIAsyncOperationProgressHandler_2_UINT32_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_UINT32_UINT32;

typedef struct __FIAsyncOperationProgressHandler_2_UINT32_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_UINT32_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_UINT32_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_UINT32_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_UINT32_UINT32* This,
        __FIAsyncOperationWithProgress_2_UINT32_UINT32* asyncInfo,
        UINT32 progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_UINT32_UINT32Vtbl;

interface __FIAsyncOperationProgressHandler_2_UINT32_UINT32
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_UINT32_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_UINT32_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_UINT32_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_UINT32_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_UINT32_UINT32_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_UINT32_UINT32_INTERFACE_DEFINED__

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation __FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformationVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        __FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformationVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformationVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

typedef struct __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl;

interface __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

typedef struct __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl;

interface __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult;

typedef struct __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl;

interface __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult;

typedef struct __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl;

interface __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation;

typedef struct __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        HSTRING key,
        __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation* This,
        __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation** first,
        __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformationVtbl;

interface __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation_INTERFACE_DEFINED__
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
#if !defined(____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

typedef struct __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl;

interface __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult;

typedef struct __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32 index,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl;

interface __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__
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
#if !defined(____FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult;

typedef struct __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32 index,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32 index,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32 index,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult* items);

    END_INTERFACE
} __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl;

interface __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult
{
    CONST_VTBL struct __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs __FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* sender,
        __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs __FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIWebSocket* sender,
        __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket* sender,
        __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket* sender,
        __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket* sender,
        __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket* sender,
        __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket* sender,
        __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener* sender,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket* sender,
        __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTask_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTask_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTask __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTask;

#endif // ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTask_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger;

#endif // ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CIHostName __x_ABI_CWindows_CNetworking_CIHostName;

#endif // ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CNetworking_CHostNameSortOptions __x_ABI_CWindows_CNetworking_CHostNameSortOptions;

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIDataReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataReader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIDataReader __x_ABI_CWindows_CStorage_CStreams_CIDataReader;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIDataReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CWeb_CWebErrorStatus __x_ABI_CWindows_CWeb_CWebErrorStatus;

typedef enum __x_ABI_CWindows_CNetworking_CSockets_CControlChannelTriggerResetReason __x_ABI_CWindows_CNetworking_CSockets_CControlChannelTriggerResetReason;

typedef enum __x_ABI_CWindows_CNetworking_CSockets_CControlChannelTriggerResourceType __x_ABI_CWindows_CNetworking_CSockets_CControlChannelTriggerResourceType;

typedef enum __x_ABI_CWindows_CNetworking_CSockets_CControlChannelTriggerStatus __x_ABI_CWindows_CNetworking_CSockets_CControlChannelTriggerStatus;

typedef enum __x_ABI_CWindows_CNetworking_CSockets_CMessageWebSocketReceiveMode __x_ABI_CWindows_CNetworking_CSockets_CMessageWebSocketReceiveMode;

typedef enum __x_ABI_CWindows_CNetworking_CSockets_CSocketActivityConnectedStandbyAction __x_ABI_CWindows_CNetworking_CSockets_CSocketActivityConnectedStandbyAction;

typedef enum __x_ABI_CWindows_CNetworking_CSockets_CSocketActivityKind __x_ABI_CWindows_CNetworking_CSockets_CSocketActivityKind;

typedef enum __x_ABI_CWindows_CNetworking_CSockets_CSocketActivityTriggerReason __x_ABI_CWindows_CNetworking_CSockets_CSocketActivityTriggerReason;

typedef enum __x_ABI_CWindows_CNetworking_CSockets_CSocketErrorStatus __x_ABI_CWindows_CNetworking_CSockets_CSocketErrorStatus;

typedef enum __x_ABI_CWindows_CNetworking_CSockets_CSocketMessageType __x_ABI_CWindows_CNetworking_CSockets_CSocketMessageType;

typedef enum __x_ABI_CWindows_CNetworking_CSockets_CSocketProtectionLevel __x_ABI_CWindows_CNetworking_CSockets_CSocketProtectionLevel;

typedef enum __x_ABI_CWindows_CNetworking_CSockets_CSocketQualityOfService __x_ABI_CWindows_CNetworking_CSockets_CSocketQualityOfService;

typedef enum __x_ABI_CWindows_CNetworking_CSockets_CSocketSslErrorSeverity __x_ABI_CWindows_CNetworking_CSockets_CSocketSslErrorSeverity;

typedef struct __x_ABI_CWindows_CNetworking_CSockets_CBandwidthStatistics __x_ABI_CWindows_CNetworking_CSockets_CBandwidthStatistics;

typedef struct __x_ABI_CWindows_CNetworking_CSockets_CRoundTripTimeStatistics __x_ABI_CWindows_CNetworking_CSockets_CRoundTripTimeStatistics;

/*
 *
 * Struct Windows.Networking.Sockets.ControlChannelTriggerResetReason
 *
 * Introduced to Windows.Networking.Sockets.ControlChannelTriggerContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CSockets_CControlChannelTriggerResetReason
{
    ControlChannelTriggerResetReason_FastUserSwitched = 0,
    ControlChannelTriggerResetReason_LowPowerExit = 1,
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
    ControlChannelTriggerResetReason_QuietHoursExit = 2,
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
    ControlChannelTriggerResetReason_ApplicationRestart = 3,
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.ControlChannelTriggerResourceType
 *
 * Introduced to Windows.Networking.Sockets.ControlChannelTriggerContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CSockets_CControlChannelTriggerResourceType
{
    ControlChannelTriggerResourceType_RequestSoftwareSlot = 0,
    ControlChannelTriggerResourceType_RequestHardwareSlot = 1,
};
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.ControlChannelTriggerStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000 || \
    WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000 && WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION < 0x30000
enum __x_ABI_CWindows_CNetworking_CSockets_CControlChannelTriggerStatus
{
    ControlChannelTriggerStatus_HardwareSlotRequested = 0,
    ControlChannelTriggerStatus_SoftwareSlotAllocated = 1,
    ControlChannelTriggerStatus_HardwareSlotAllocated = 2,
    ControlChannelTriggerStatus_PolicyError = 3,
    ControlChannelTriggerStatus_SystemError = 4,
    ControlChannelTriggerStatus_TransportDisconnected = 5,
    ControlChannelTriggerStatus_ServiceUnavailable = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000 || \
       // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000 && WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION < 0x30000

/*
 *
 * Struct Windows.Networking.Sockets.MessageWebSocketReceiveMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CNetworking_CSockets_CMessageWebSocketReceiveMode
{
    MessageWebSocketReceiveMode_FullMessage = 0,
    MessageWebSocketReceiveMode_PartialMessage = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Networking.Sockets.SocketActivityConnectedStandbyAction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CSockets_CSocketActivityConnectedStandbyAction
{
    SocketActivityConnectedStandbyAction_DoNotWake = 0,
    SocketActivityConnectedStandbyAction_Wake = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.SocketActivityKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CSockets_CSocketActivityKind
{
    SocketActivityKind_None = 0,
    SocketActivityKind_StreamSocketListener = 1,
    SocketActivityKind_DatagramSocket = 2,
    SocketActivityKind_StreamSocket = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.SocketActivityTriggerReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CSockets_CSocketActivityTriggerReason
{
    SocketActivityTriggerReason_None = 0,
    SocketActivityTriggerReason_SocketActivity = 1,
    SocketActivityTriggerReason_ConnectionAccepted = 2,
    SocketActivityTriggerReason_KeepAliveTimerExpired = 3,
    SocketActivityTriggerReason_SocketClosed = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.SocketErrorStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CSockets_CSocketErrorStatus
{
    SocketErrorStatus_Unknown = 0,
    SocketErrorStatus_OperationAborted = 1,
    SocketErrorStatus_HttpInvalidServerResponse = 2,
    SocketErrorStatus_ConnectionTimedOut = 3,
    SocketErrorStatus_AddressFamilyNotSupported = 4,
    SocketErrorStatus_SocketTypeNotSupported = 5,
    SocketErrorStatus_HostNotFound = 6,
    SocketErrorStatus_NoDataRecordOfRequestedType = 7,
    SocketErrorStatus_NonAuthoritativeHostNotFound = 8,
    SocketErrorStatus_ClassTypeNotFound = 9,
    SocketErrorStatus_AddressAlreadyInUse = 10,
    SocketErrorStatus_CannotAssignRequestedAddress = 11,
    SocketErrorStatus_ConnectionRefused = 12,
    SocketErrorStatus_NetworkIsUnreachable = 13,
    SocketErrorStatus_UnreachableHost = 14,
    SocketErrorStatus_NetworkIsDown = 15,
    SocketErrorStatus_NetworkDroppedConnectionOnReset = 16,
    SocketErrorStatus_SoftwareCausedConnectionAbort = 17,
    SocketErrorStatus_ConnectionResetByPeer = 18,
    SocketErrorStatus_HostIsDown = 19,
    SocketErrorStatus_NoAddressesFound = 20,
    SocketErrorStatus_TooManyOpenFiles = 21,
    SocketErrorStatus_MessageTooLong = 22,
    SocketErrorStatus_CertificateExpired = 23,
    SocketErrorStatus_CertificateUntrustedRoot = 24,
    SocketErrorStatus_CertificateCommonNameIsIncorrect = 25,
    SocketErrorStatus_CertificateWrongUsage = 26,
    SocketErrorStatus_CertificateRevoked = 27,
    SocketErrorStatus_CertificateNoRevocationCheck = 28,
    SocketErrorStatus_CertificateRevocationServerOffline = 29,
    SocketErrorStatus_CertificateIsInvalid = 30,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.SocketMessageType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CSockets_CSocketMessageType
{
    SocketMessageType_Binary = 0,
    SocketMessageType_Utf8 = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.SocketProtectionLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CSockets_CSocketProtectionLevel
{
    SocketProtectionLevel_PlainSocket = 0,
    SocketProtectionLevel_Ssl
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("Ssl may result in insecure connections and may be altered or unavailable for releases after Windows 8.1. Instead, use one of the TLS SocketProtectionLevel values.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 1,
    SocketProtectionLevel_SslAllowNullEncryption = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SocketProtectionLevel_BluetoothEncryptionAllowNullAuthentication = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SocketProtectionLevel_BluetoothEncryptionWithAuthentication = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SocketProtectionLevel_Ssl3AllowWeakEncryption
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("Ssl3AllowWeakEncryption may result in insecure connections and may be altered or unavailable for releases after Windows 8.1. Instead, use one of the TLS SocketProtectionLevel values.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SocketProtectionLevel_Tls10 = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SocketProtectionLevel_Tls11 = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SocketProtectionLevel_Tls12 = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    SocketProtectionLevel_Unspecified = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
    SocketProtectionLevel_Tls13 = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.SocketQualityOfService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CSockets_CSocketQualityOfService
{
    SocketQualityOfService_Normal = 0,
    SocketQualityOfService_LowLatency = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.SocketSslErrorSeverity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CSockets_CSocketSslErrorSeverity
{
    SocketSslErrorSeverity_None = 0,
    SocketSslErrorSeverity_Ignorable = 1,
    SocketSslErrorSeverity_Fatal = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.BandwidthStatistics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CNetworking_CSockets_CBandwidthStatistics
{
    UINT64 OutboundBitsPerSecond;
    UINT64 InboundBitsPerSecond;
    UINT64 OutboundBitsPerSecondInstability;
    UINT64 InboundBitsPerSecondInstability;
    boolean OutboundBandwidthPeaked;
    boolean InboundBandwidthPeaked;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Sockets.RoundTripTimeStatistics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CNetworking_CSockets_CRoundTripTimeStatistics
{
    UINT32 Variance;
    UINT32 Max;
    UINT32 Min;
    UINT32 Sum;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IControlChannelTrigger
 *
 * Introduced to Windows.Networking.Sockets.ControlChannelTriggerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.ControlChannelTrigger
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IControlChannelTrigger[] = L"Windows.Networking.Sockets.IControlChannelTrigger";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ControlChannelTriggerId)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ServerKeepAliveIntervalInMinutes)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ServerKeepAliveIntervalInMinutes)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentKeepAliveIntervalInMinutes)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_TransportObject)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_KeepAliveTrigger)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger* This,
        __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger** trigger);
    HRESULT (STDMETHODCALLTYPE* get_PushNotificationTrigger)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger* This,
        __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger** trigger);
    HRESULT (STDMETHODCALLTYPE* UsingTransport)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger* This,
        IInspectable* transport);
    HRESULT (STDMETHODCALLTYPE* WaitForPushEnabled)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CControlChannelTriggerStatus* channelTriggerStatus);
    HRESULT (STDMETHODCALLTYPE* DecreaseNetworkKeepAliveInterval)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger* This);
    HRESULT (STDMETHODCALLTYPE* FlushTransport)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger* This);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_get_ControlChannelTriggerId(This, value) \
    ((This)->lpVtbl->get_ControlChannelTriggerId(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_get_ServerKeepAliveIntervalInMinutes(This, value) \
    ((This)->lpVtbl->get_ServerKeepAliveIntervalInMinutes(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_put_ServerKeepAliveIntervalInMinutes(This, value) \
    ((This)->lpVtbl->put_ServerKeepAliveIntervalInMinutes(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_get_CurrentKeepAliveIntervalInMinutes(This, value) \
    ((This)->lpVtbl->get_CurrentKeepAliveIntervalInMinutes(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_get_TransportObject(This, value) \
    ((This)->lpVtbl->get_TransportObject(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_get_KeepAliveTrigger(This, trigger) \
    ((This)->lpVtbl->get_KeepAliveTrigger(This, trigger))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_get_PushNotificationTrigger(This, trigger) \
    ((This)->lpVtbl->get_PushNotificationTrigger(This, trigger))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_UsingTransport(This, transport) \
    ((This)->lpVtbl->UsingTransport(This, transport))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_WaitForPushEnabled(This, channelTriggerStatus) \
    ((This)->lpVtbl->WaitForPushEnabled(This, channelTriggerStatus))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_DecreaseNetworkKeepAliveInterval(This) \
    ((This)->lpVtbl->DecreaseNetworkKeepAliveInterval(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_FlushTransport(This) \
    ((This)->lpVtbl->FlushTransport(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IControlChannelTrigger2
 *
 * Introduced to Windows.Networking.Sockets.ControlChannelTriggerContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.ControlChannelTrigger
 *
 */
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IControlChannelTrigger2[] = L"Windows.Networking.Sockets.IControlChannelTrigger2";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsWakeFromLowPowerSupported)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_get_IsWakeFromLowPowerSupported(This, value) \
    ((This)->lpVtbl->get_IsWakeFromLowPowerSupported(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger2_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Networking.Sockets.IControlChannelTriggerEventDetails
 *
 * Introduced to Windows.Networking.Sockets.ControlChannelTriggerContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IControlChannelTriggerEventDetails[] = L"Windows.Networking.Sockets.IControlChannelTriggerEventDetails";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ControlChannelTrigger)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetailsVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_get_ControlChannelTrigger(This, value) \
    ((This)->lpVtbl->get_ControlChannelTrigger(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerEventDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IControlChannelTriggerFactory
 *
 * Introduced to Windows.Networking.Sockets.ControlChannelTriggerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.ControlChannelTrigger
 *
 */
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IControlChannelTriggerFactory[] = L"Windows.Networking.Sockets.IControlChannelTriggerFactory";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateControlChannelTrigger)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory* This,
        HSTRING channelId,
        UINT32 serverKeepAliveIntervalInMinutes,
        __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger** notificationChannel);
    HRESULT (STDMETHODCALLTYPE* CreateControlChannelTriggerEx)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory* This,
        HSTRING channelId,
        UINT32 serverKeepAliveIntervalInMinutes,
        enum __x_ABI_CWindows_CNetworking_CSockets_CControlChannelTriggerResourceType resourceRequestType,
        __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTrigger** notificationChannel);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactoryVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_CreateControlChannelTrigger(This, channelId, serverKeepAliveIntervalInMinutes, notificationChannel) \
    ((This)->lpVtbl->CreateControlChannelTrigger(This, channelId, serverKeepAliveIntervalInMinutes, notificationChannel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_CreateControlChannelTriggerEx(This, channelId, serverKeepAliveIntervalInMinutes, resourceRequestType, notificationChannel) \
    ((This)->lpVtbl->CreateControlChannelTriggerEx(This, channelId, serverKeepAliveIntervalInMinutes, resourceRequestType, notificationChannel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IControlChannelTriggerResetEventDetails
 *
 * Introduced to Windows.Networking.Sockets.ControlChannelTriggerContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IControlChannelTriggerResetEventDetails[] = L"Windows.Networking.Sockets.IControlChannelTriggerResetEventDetails";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResetReason)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CControlChannelTriggerResetReason* value);
    HRESULT (STDMETHODCALLTYPE* get_HardwareSlotReset)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SoftwareSlotReset)(__x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetailsVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_get_ResetReason(This, value) \
    ((This)->lpVtbl->get_ResetReason(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_get_HardwareSlotReset(This, value) \
    ((This)->lpVtbl->get_HardwareSlotReset(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_get_SoftwareSlotReset(This, value) \
    ((This)->lpVtbl->get_SoftwareSlotReset(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIControlChannelTriggerResetEventDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IDatagramSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.DatagramSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IDatagramSocket[] = L"Windows.Networking.Sockets.IDatagramSocket";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Control)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl** value);
    HRESULT (STDMETHODCALLTYPE* get_Information)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation** value);
    HRESULT (STDMETHODCALLTYPE* get_OutputStream)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream** value);
    HRESULT (STDMETHODCALLTYPE* ConnectAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* This,
        __x_ABI_CWindows_CNetworking_CIHostName* remoteHostName,
        HSTRING remoteServiceName,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ConnectWithEndpointPairAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* This,
        __x_ABI_CWindows_CNetworking_CIEndpointPair* endpointPair,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* BindServiceNameAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* This,
        HSTRING localServiceName,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* BindEndpointAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* This,
        __x_ABI_CWindows_CNetworking_CIHostName* localHostName,
        HSTRING localServiceName,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* JoinMulticastGroup)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* This,
        __x_ABI_CWindows_CNetworking_CIHostName* host);
    HRESULT (STDMETHODCALLTYPE* GetOutputStreamAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* This,
        __x_ABI_CWindows_CNetworking_CIHostName* remoteHostName,
        HSTRING remoteServiceName,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream** value);
    HRESULT (STDMETHODCALLTYPE* GetOutputStreamWithEndpointPairAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* This,
        __x_ABI_CWindows_CNetworking_CIEndpointPair* endpointPair,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIOutputStream** value);
    HRESULT (STDMETHODCALLTYPE* add_MessageReceived)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* This,
        __FITypedEventHandler_2_Windows__CNetworking__CSockets__CDatagramSocket_Windows__CNetworking__CSockets__CDatagramSocketMessageReceivedEventArgs* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_MessageReceived)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_get_Control(This, value) \
    ((This)->lpVtbl->get_Control(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_get_Information(This, value) \
    ((This)->lpVtbl->get_Information(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_get_OutputStream(This, value) \
    ((This)->lpVtbl->get_OutputStream(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_ConnectAsync(This, remoteHostName, remoteServiceName, operation) \
    ((This)->lpVtbl->ConnectAsync(This, remoteHostName, remoteServiceName, operation))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_ConnectWithEndpointPairAsync(This, endpointPair, operation) \
    ((This)->lpVtbl->ConnectWithEndpointPairAsync(This, endpointPair, operation))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_BindServiceNameAsync(This, localServiceName, operation) \
    ((This)->lpVtbl->BindServiceNameAsync(This, localServiceName, operation))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_BindEndpointAsync(This, localHostName, localServiceName, operation) \
    ((This)->lpVtbl->BindEndpointAsync(This, localHostName, localServiceName, operation))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_JoinMulticastGroup(This, host) \
    ((This)->lpVtbl->JoinMulticastGroup(This, host))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_GetOutputStreamAsync(This, remoteHostName, remoteServiceName, value) \
    ((This)->lpVtbl->GetOutputStreamAsync(This, remoteHostName, remoteServiceName, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_GetOutputStreamWithEndpointPairAsync(This, endpointPair, value) \
    ((This)->lpVtbl->GetOutputStreamWithEndpointPairAsync(This, endpointPair, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_add_MessageReceived(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_MessageReceived(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_remove_MessageReceived(This, eventCookie) \
    ((This)->lpVtbl->remove_MessageReceived(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IDatagramSocket2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.DatagramSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IDatagramSocket2[] = L"Windows.Networking.Sockets.IDatagramSocket2";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* BindServiceNameAndAdapterAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2* This,
        HSTRING localServiceName,
        __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter* adapter,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_BindServiceNameAndAdapterAsync(This, localServiceName, adapter, operation) \
    ((This)->lpVtbl->BindServiceNameAndAdapterAsync(This, localServiceName, adapter, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IDatagramSocket3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.DatagramSocket
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IDatagramSocket3[] = L"Windows.Networking.Sockets.IDatagramSocket3";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CancelIOAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* EnableTransferOwnership)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3* This,
        GUID taskId);
    HRESULT (STDMETHODCALLTYPE* EnableTransferOwnershipWithConnectedStandbyAction)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3* This,
        GUID taskId,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketActivityConnectedStandbyAction connectedStandbyAction);
    HRESULT (STDMETHODCALLTYPE* TransferOwnership)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3* This,
        HSTRING socketId);
    HRESULT (STDMETHODCALLTYPE* TransferOwnershipWithContext)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3* This,
        HSTRING socketId,
        __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext* data);
    HRESULT (STDMETHODCALLTYPE* TransferOwnershipWithContextAndKeepAliveTime)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3* This,
        HSTRING socketId,
        __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext* data,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan keepAliveTime);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_CancelIOAsync(This, operation) \
    ((This)->lpVtbl->CancelIOAsync(This, operation))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_EnableTransferOwnership(This, taskId) \
    ((This)->lpVtbl->EnableTransferOwnership(This, taskId))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_EnableTransferOwnershipWithConnectedStandbyAction(This, taskId, connectedStandbyAction) \
    ((This)->lpVtbl->EnableTransferOwnershipWithConnectedStandbyAction(This, taskId, connectedStandbyAction))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_TransferOwnership(This, socketId) \
    ((This)->lpVtbl->TransferOwnership(This, socketId))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_TransferOwnershipWithContext(This, socketId, data) \
    ((This)->lpVtbl->TransferOwnershipWithContext(This, socketId, data))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_TransferOwnershipWithContextAndKeepAliveTime(This, socketId, data, keepAliveTime) \
    ((This)->lpVtbl->TransferOwnershipWithContextAndKeepAliveTime(This, socketId, data, keepAliveTime))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IDatagramSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.DatagramSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IDatagramSocketControl[] = L"Windows.Networking.Sockets.IDatagramSocketControl";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_QualityOfService)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketQualityOfService* value);
    HRESULT (STDMETHODCALLTYPE* put_QualityOfService)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketQualityOfService value);
    HRESULT (STDMETHODCALLTYPE* get_OutboundUnicastHopLimit)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_OutboundUnicastHopLimit)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl* This,
        BYTE value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControlVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_get_QualityOfService(This, value) \
    ((This)->lpVtbl->get_QualityOfService(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_put_QualityOfService(This, value) \
    ((This)->lpVtbl->put_QualityOfService(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_get_OutboundUnicastHopLimit(This, value) \
    ((This)->lpVtbl->get_OutboundUnicastHopLimit(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_put_OutboundUnicastHopLimit(This, value) \
    ((This)->lpVtbl->put_OutboundUnicastHopLimit(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IDatagramSocketControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.DatagramSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IDatagramSocketControl2[] = L"Windows.Networking.Sockets.IDatagramSocketControl2";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InboundBufferSizeInBytes)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_InboundBufferSizeInBytes)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_DontFragment)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_DontFragment)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_get_InboundBufferSizeInBytes(This, value) \
    ((This)->lpVtbl->get_InboundBufferSizeInBytes(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_put_InboundBufferSizeInBytes(This, value) \
    ((This)->lpVtbl->put_InboundBufferSizeInBytes(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_get_DontFragment(This, value) \
    ((This)->lpVtbl->get_DontFragment(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_put_DontFragment(This, value) \
    ((This)->lpVtbl->put_DontFragment(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IDatagramSocketControl3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.DatagramSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IDatagramSocketControl3[] = L"Windows.Networking.Sockets.IDatagramSocketControl3";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MulticastOnly)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_MulticastOnly)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_get_MulticastOnly(This, value) \
    ((This)->lpVtbl->get_MulticastOnly(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_put_MulticastOnly(This, value) \
    ((This)->lpVtbl->put_MulticastOnly(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketControl3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IDatagramSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.DatagramSocketInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IDatagramSocketInformation[] = L"Windows.Networking.Sockets.IDatagramSocketInformation";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LocalAddress)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* get_LocalPort)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteAddress)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* get_RemotePort)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformationVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_get_LocalAddress(This, value) \
    ((This)->lpVtbl->get_LocalAddress(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_get_LocalPort(This, value) \
    ((This)->lpVtbl->get_LocalPort(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_get_RemoteAddress(This, value) \
    ((This)->lpVtbl->get_RemoteAddress(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_get_RemotePort(This, value) \
    ((This)->lpVtbl->get_RemotePort(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IDatagramSocketMessageReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.DatagramSocketMessageReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IDatagramSocketMessageReceivedEventArgs[] = L"Windows.Networking.Sockets.IDatagramSocketMessageReceivedEventArgs";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RemoteAddress)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* get_RemotePort)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LocalAddress)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* GetDataReader)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs* This,
        __x_ABI_CWindows_CStorage_CStreams_CIDataReader** dataReader);
    HRESULT (STDMETHODCALLTYPE* GetDataStream)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** inputStream);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_get_RemoteAddress(This, value) \
    ((This)->lpVtbl->get_RemoteAddress(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_get_RemotePort(This, value) \
    ((This)->lpVtbl->get_RemotePort(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_get_LocalAddress(This, value) \
    ((This)->lpVtbl->get_LocalAddress(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_GetDataReader(This, dataReader) \
    ((This)->lpVtbl->GetDataReader(This, dataReader))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_GetDataStream(This, inputStream) \
    ((This)->lpVtbl->GetDataStream(This, inputStream))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketMessageReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IDatagramSocketStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.DatagramSocket
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IDatagramSocketStatics[] = L"Windows.Networking.Sockets.IDatagramSocketStatics";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetEndpointPairsAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics* This,
        __x_ABI_CWindows_CNetworking_CIHostName* remoteHostName,
        HSTRING remoteServiceName,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair** operation);
    HRESULT (STDMETHODCALLTYPE* GetEndpointPairsWithSortOptionsAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics* This,
        __x_ABI_CWindows_CNetworking_CIHostName* remoteHostName,
        HSTRING remoteServiceName,
        enum __x_ABI_CWindows_CNetworking_CHostNameSortOptions sortOptions,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStaticsVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_GetEndpointPairsAsync(This, remoteHostName, remoteServiceName, operation) \
    ((This)->lpVtbl->GetEndpointPairsAsync(This, remoteHostName, remoteServiceName, operation))

#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_GetEndpointPairsWithSortOptionsAsync(This, remoteHostName, remoteServiceName, sortOptions, operation) \
    ((This)->lpVtbl->GetEndpointPairsWithSortOptionsAsync(This, remoteHostName, remoteServiceName, sortOptions, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocketStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IMessageWebSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.MessageWebSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Sockets.IWebSocket
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IMessageWebSocket[] = L"Windows.Networking.Sockets.IMessageWebSocket";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Control)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl** value);
    HRESULT (STDMETHODCALLTYPE* get_Information)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation** value);
    HRESULT (STDMETHODCALLTYPE* add_MessageReceived)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket* This,
        __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_MessageReceived)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_get_Control(This, value) \
    ((This)->lpVtbl->get_Control(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_get_Information(This, value) \
    ((This)->lpVtbl->get_Information(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_add_MessageReceived(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_MessageReceived(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_remove_MessageReceived(This, eventCookie) \
    ((This)->lpVtbl->remove_MessageReceived(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IMessageWebSocket2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.MessageWebSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Sockets.IMessageWebSocket
 *     Windows.Networking.Sockets.IWebSocket
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IMessageWebSocket2[] = L"Windows.Networking.Sockets.IMessageWebSocket2";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_ServerCustomValidationRequested)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2* This,
        __FITypedEventHandler_2_Windows__CNetworking__CSockets__CMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_ServerCustomValidationRequested)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_add_ServerCustomValidationRequested(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_ServerCustomValidationRequested(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_remove_ServerCustomValidationRequested(This, eventCookie) \
    ((This)->lpVtbl->remove_ServerCustomValidationRequested(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.Sockets.IMessageWebSocket3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.MessageWebSocket
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IMessageWebSocket3[] = L"Windows.Networking.Sockets.IMessageWebSocket3";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SendNonfinalFrameAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __FIAsyncOperationWithProgress_2_UINT32_UINT32** operation);
    HRESULT (STDMETHODCALLTYPE* SendFinalFrameAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __FIAsyncOperationWithProgress_2_UINT32_UINT32** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_SendNonfinalFrameAsync(This, data, operation) \
    ((This)->lpVtbl->SendNonfinalFrameAsync(This, data, operation))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_SendFinalFrameAsync(This, data, operation) \
    ((This)->lpVtbl->SendFinalFrameAsync(This, data, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocket3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Sockets.IMessageWebSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.MessageWebSocketControl
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Sockets.IWebSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IMessageWebSocketControl[] = L"Windows.Networking.Sockets.IMessageWebSocketControl";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaxMessageSize)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_MaxMessageSize)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_MessageType)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketMessageType* value);
    HRESULT (STDMETHODCALLTYPE* put_MessageType)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketMessageType value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControlVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_get_MaxMessageSize(This, value) \
    ((This)->lpVtbl->get_MaxMessageSize(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_put_MaxMessageSize(This, value) \
    ((This)->lpVtbl->put_MaxMessageSize(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_get_MessageType(This, value) \
    ((This)->lpVtbl->get_MessageType(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_put_MessageType(This, value) \
    ((This)->lpVtbl->put_MessageType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IMessageWebSocketControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.MessageWebSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IMessageWebSocketControl2[] = L"Windows.Networking.Sockets.IMessageWebSocketControl2";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DesiredUnsolicitedPongInterval)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_DesiredUnsolicitedPongInterval)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_ActualUnsolicitedPongInterval)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_ReceiveMode)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CMessageWebSocketReceiveMode* value);
    HRESULT (STDMETHODCALLTYPE* put_ReceiveMode)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CMessageWebSocketReceiveMode value);
    HRESULT (STDMETHODCALLTYPE* get_ClientCertificate)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** value);
    HRESULT (STDMETHODCALLTYPE* put_ClientCertificate)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_get_DesiredUnsolicitedPongInterval(This, value) \
    ((This)->lpVtbl->get_DesiredUnsolicitedPongInterval(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_put_DesiredUnsolicitedPongInterval(This, value) \
    ((This)->lpVtbl->put_DesiredUnsolicitedPongInterval(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_get_ActualUnsolicitedPongInterval(This, value) \
    ((This)->lpVtbl->get_ActualUnsolicitedPongInterval(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_get_ReceiveMode(This, value) \
    ((This)->lpVtbl->get_ReceiveMode(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_put_ReceiveMode(This, value) \
    ((This)->lpVtbl->put_ReceiveMode(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_get_ClientCertificate(This, value) \
    ((This)->lpVtbl->get_ClientCertificate(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_put_ClientCertificate(This, value) \
    ((This)->lpVtbl->put_ClientCertificate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Networking.Sockets.IMessageWebSocketMessageReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.MessageWebSocketMessageReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IMessageWebSocketMessageReceivedEventArgs[] = L"Windows.Networking.Sockets.IMessageWebSocketMessageReceivedEventArgs";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MessageType)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketMessageType* value);
    HRESULT (STDMETHODCALLTYPE* GetDataReader)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs* This,
        __x_ABI_CWindows_CStorage_CStreams_CIDataReader** dataReader);
    HRESULT (STDMETHODCALLTYPE* GetDataStream)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** inputStream);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_get_MessageType(This, value) \
    ((This)->lpVtbl->get_MessageType(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_GetDataReader(This, dataReader) \
    ((This)->lpVtbl->GetDataReader(This, dataReader))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_GetDataStream(This, inputStream) \
    ((This)->lpVtbl->GetDataStream(This, inputStream))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IMessageWebSocketMessageReceivedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.MessageWebSocketMessageReceivedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Sockets.IMessageWebSocketMessageReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IMessageWebSocketMessageReceivedEventArgs2[] = L"Windows.Networking.Sockets.IMessageWebSocketMessageReceivedEventArgs2";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsMessageComplete)(__x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_get_IsMessageComplete(This, value) \
    ((This)->lpVtbl->get_IsMessageComplete(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIMessageWebSocketMessageReceivedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Networking.Sockets.IServerMessageWebSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.ServerMessageWebSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IServerMessageWebSocket[] = L"Windows.Networking.Sockets.IServerMessageWebSocket";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_MessageReceived)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket* This,
        __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CMessageWebSocketMessageReceivedEventArgs* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_MessageReceived)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_Control)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl** value);
    HRESULT (STDMETHODCALLTYPE* get_Information)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation** value);
    HRESULT (STDMETHODCALLTYPE* get_OutputStream)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream** value);
    HRESULT (STDMETHODCALLTYPE* add_Closed)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket* This,
        __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerMessageWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Closed)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* CloseWithStatus)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket* This,
        UINT16 code,
        HSTRING reason);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_add_MessageReceived(This, value, token) \
    ((This)->lpVtbl->add_MessageReceived(This, value, token))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_remove_MessageReceived(This, token) \
    ((This)->lpVtbl->remove_MessageReceived(This, token))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_get_Control(This, value) \
    ((This)->lpVtbl->get_Control(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_get_Information(This, value) \
    ((This)->lpVtbl->get_Information(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_get_OutputStream(This, value) \
    ((This)->lpVtbl->get_OutputStream(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_add_Closed(This, value, token) \
    ((This)->lpVtbl->add_Closed(This, value, token))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_remove_Closed(This, token) \
    ((This)->lpVtbl->remove_Closed(This, token))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_CloseWithStatus(This, code, reason) \
    ((This)->lpVtbl->CloseWithStatus(This, code, reason))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Sockets.IServerMessageWebSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.ServerMessageWebSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IServerMessageWebSocketControl[] = L"Windows.Networking.Sockets.IServerMessageWebSocketControl";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MessageType)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketMessageType* value);
    HRESULT (STDMETHODCALLTYPE* put_MessageType)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketMessageType value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControlVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_get_MessageType(This, value) \
    ((This)->lpVtbl->get_MessageType(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_put_MessageType(This, value) \
    ((This)->lpVtbl->put_MessageType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Sockets.IServerMessageWebSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.ServerMessageWebSocketInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IServerMessageWebSocketInformation[] = L"Windows.Networking.Sockets.IServerMessageWebSocketInformation";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BandwidthStatistics)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation* This,
        struct __x_ABI_CWindows_CNetworking_CSockets_CBandwidthStatistics* value);
    HRESULT (STDMETHODCALLTYPE* get_Protocol)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LocalAddress)(__x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformationVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_get_BandwidthStatistics(This, value) \
    ((This)->lpVtbl->get_BandwidthStatistics(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_get_Protocol(This, value) \
    ((This)->lpVtbl->get_Protocol(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_get_LocalAddress(This, value) \
    ((This)->lpVtbl->get_LocalAddress(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerMessageWebSocketInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Sockets.IServerStreamWebSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.ServerStreamWebSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IServerStreamWebSocket[] = L"Windows.Networking.Sockets.IServerStreamWebSocket";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Information)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation** value);
    HRESULT (STDMETHODCALLTYPE* get_InputStream)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** value);
    HRESULT (STDMETHODCALLTYPE* get_OutputStream)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream** value);
    HRESULT (STDMETHODCALLTYPE* add_Closed)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket* This,
        __FITypedEventHandler_2_Windows__CNetworking__CSockets__CServerStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Closed)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* CloseWithStatus)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket* This,
        UINT16 code,
        HSTRING reason);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_get_Information(This, value) \
    ((This)->lpVtbl->get_Information(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_get_InputStream(This, value) \
    ((This)->lpVtbl->get_InputStream(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_get_OutputStream(This, value) \
    ((This)->lpVtbl->get_OutputStream(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_add_Closed(This, value, token) \
    ((This)->lpVtbl->add_Closed(This, value, token))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_remove_Closed(This, token) \
    ((This)->lpVtbl->remove_Closed(This, token))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_CloseWithStatus(This, code, reason) \
    ((This)->lpVtbl->CloseWithStatus(This, code, reason))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Sockets.IServerStreamWebSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.ServerStreamWebSocketInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IServerStreamWebSocketInformation[] = L"Windows.Networking.Sockets.IServerStreamWebSocketInformation";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BandwidthStatistics)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation* This,
        struct __x_ABI_CWindows_CNetworking_CSockets_CBandwidthStatistics* value);
    HRESULT (STDMETHODCALLTYPE* get_Protocol)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LocalAddress)(__x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformationVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_get_BandwidthStatistics(This, value) \
    ((This)->lpVtbl->get_BandwidthStatistics(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_get_Protocol(This, value) \
    ((This)->lpVtbl->get_Protocol(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_get_LocalAddress(This, value) \
    ((This)->lpVtbl->get_LocalAddress(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIServerStreamWebSocketInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Sockets.ISocketActivityContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.SocketActivityContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_ISocketActivityContext[] = L"Windows.Networking.Sockets.ISocketActivityContext";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.ISocketActivityContextFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.SocketActivityContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_ISocketActivityContextFactory[] = L"Windows.Networking.Sockets.ISocketActivityContextFactory";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext** context);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactoryVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_Create(This, data, context) \
    ((This)->lpVtbl->Create(This, data, context))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContextFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.ISocketActivityInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.SocketActivityInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_ISocketActivityInformation[] = L"Windows.Networking.Sockets.ISocketActivityInformation";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TaskId)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SocketKind)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketActivityKind* value);
    HRESULT (STDMETHODCALLTYPE* get_Context)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation* This,
        __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext** value);
    HRESULT (STDMETHODCALLTYPE* get_DatagramSocket)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket** value);
    HRESULT (STDMETHODCALLTYPE* get_StreamSocket)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket** value);
    HRESULT (STDMETHODCALLTYPE* get_StreamSocketListener)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_get_TaskId(This, value) \
    ((This)->lpVtbl->get_TaskId(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_get_SocketKind(This, value) \
    ((This)->lpVtbl->get_SocketKind(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_get_Context(This, value) \
    ((This)->lpVtbl->get_Context(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_get_DatagramSocket(This, value) \
    ((This)->lpVtbl->get_DatagramSocket(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_get_StreamSocket(This, value) \
    ((This)->lpVtbl->get_StreamSocket(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_get_StreamSocketListener(This, value) \
    ((This)->lpVtbl->get_StreamSocketListener(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.ISocketActivityInformationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.SocketActivityInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_ISocketActivityInformationStatics[] = L"Windows.Networking.Sockets.ISocketActivityInformationStatics";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AllSockets)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics* This,
        __FIMapView_2_HSTRING_Windows__CNetworking__CSockets__CSocketActivityInformation** sockets);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStaticsVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_get_AllSockets(This, sockets) \
    ((This)->lpVtbl->get_AllSockets(This, sockets))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.ISocketActivityTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.SocketActivityTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_ISocketActivityTriggerDetails[] = L"Windows.Networking.Sockets.ISocketActivityTriggerDetails";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Reason)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketActivityTriggerReason* value);
    HRESULT (STDMETHODCALLTYPE* get_SocketInformation)(__x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails* This,
        __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityInformation** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetailsVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_get_Reason(This, value) \
    ((This)->lpVtbl->get_Reason(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_get_SocketInformation(This, value) \
    ((This)->lpVtbl->get_SocketInformation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketActivityTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.ISocketErrorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.SocketError
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_ISocketErrorStatics[] = L"Windows.Networking.Sockets.ISocketErrorStatics";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetStatus)(__x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics* This,
        INT32 hresult,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketErrorStatus* status);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStaticsVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_GetStatus(This, hresult, status) \
    ((This)->lpVtbl->GetStatus(This, hresult, status))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CISocketErrorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocket[] = L"Windows.Networking.Sockets.IStreamSocket";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Control)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl** value);
    HRESULT (STDMETHODCALLTYPE* get_Information)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation** value);
    HRESULT (STDMETHODCALLTYPE* get_InputStream)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** value);
    HRESULT (STDMETHODCALLTYPE* get_OutputStream)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream** value);
    HRESULT (STDMETHODCALLTYPE* ConnectWithEndpointPairAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket* This,
        __x_ABI_CWindows_CNetworking_CIEndpointPair* endpointPair,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ConnectAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket* This,
        __x_ABI_CWindows_CNetworking_CIHostName* remoteHostName,
        HSTRING remoteServiceName,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ConnectWithEndpointPairAndProtectionLevelAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket* This,
        __x_ABI_CWindows_CNetworking_CIEndpointPair* endpointPair,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketProtectionLevel protectionLevel,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ConnectWithProtectionLevelAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket* This,
        __x_ABI_CWindows_CNetworking_CIHostName* remoteHostName,
        HSTRING remoteServiceName,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketProtectionLevel protectionLevel,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* UpgradeToSslAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketProtectionLevel protectionLevel,
        __x_ABI_CWindows_CNetworking_CIHostName* validationHostName,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_get_Control(This, value) \
    ((This)->lpVtbl->get_Control(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_get_Information(This, value) \
    ((This)->lpVtbl->get_Information(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_get_InputStream(This, value) \
    ((This)->lpVtbl->get_InputStream(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_get_OutputStream(This, value) \
    ((This)->lpVtbl->get_OutputStream(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_ConnectWithEndpointPairAsync(This, endpointPair, operation) \
    ((This)->lpVtbl->ConnectWithEndpointPairAsync(This, endpointPair, operation))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_ConnectAsync(This, remoteHostName, remoteServiceName, operation) \
    ((This)->lpVtbl->ConnectAsync(This, remoteHostName, remoteServiceName, operation))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_ConnectWithEndpointPairAndProtectionLevelAsync(This, endpointPair, protectionLevel, operation) \
    ((This)->lpVtbl->ConnectWithEndpointPairAndProtectionLevelAsync(This, endpointPair, protectionLevel, operation))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_ConnectWithProtectionLevelAsync(This, remoteHostName, remoteServiceName, protectionLevel, operation) \
    ((This)->lpVtbl->ConnectWithProtectionLevelAsync(This, remoteHostName, remoteServiceName, protectionLevel, operation))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_UpgradeToSslAsync(This, protectionLevel, validationHostName, operation) \
    ((This)->lpVtbl->UpgradeToSslAsync(This, protectionLevel, validationHostName, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocket2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocket2[] = L"Windows.Networking.Sockets.IStreamSocket2";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ConnectWithProtectionLevelAndAdapterAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2* This,
        __x_ABI_CWindows_CNetworking_CIHostName* remoteHostName,
        HSTRING remoteServiceName,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketProtectionLevel protectionLevel,
        __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter* adapter,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_ConnectWithProtectionLevelAndAdapterAsync(This, remoteHostName, remoteServiceName, protectionLevel, adapter, operation) \
    ((This)->lpVtbl->ConnectWithProtectionLevelAndAdapterAsync(This, remoteHostName, remoteServiceName, protectionLevel, adapter, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocket3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocket
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocket3[] = L"Windows.Networking.Sockets.IStreamSocket3";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CancelIOAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* EnableTransferOwnership)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3* This,
        GUID taskId);
    HRESULT (STDMETHODCALLTYPE* EnableTransferOwnershipWithConnectedStandbyAction)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3* This,
        GUID taskId,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketActivityConnectedStandbyAction connectedStandbyAction);
    HRESULT (STDMETHODCALLTYPE* TransferOwnership)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3* This,
        HSTRING socketId);
    HRESULT (STDMETHODCALLTYPE* TransferOwnershipWithContext)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3* This,
        HSTRING socketId,
        __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext* data);
    HRESULT (STDMETHODCALLTYPE* TransferOwnershipWithContextAndKeepAliveTime)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3* This,
        HSTRING socketId,
        __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext* data,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan keepAliveTime);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_CancelIOAsync(This, operation) \
    ((This)->lpVtbl->CancelIOAsync(This, operation))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_EnableTransferOwnership(This, taskId) \
    ((This)->lpVtbl->EnableTransferOwnership(This, taskId))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_EnableTransferOwnershipWithConnectedStandbyAction(This, taskId, connectedStandbyAction) \
    ((This)->lpVtbl->EnableTransferOwnershipWithConnectedStandbyAction(This, taskId, connectedStandbyAction))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_TransferOwnership(This, socketId) \
    ((This)->lpVtbl->TransferOwnership(This, socketId))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_TransferOwnershipWithContext(This, socketId, data) \
    ((This)->lpVtbl->TransferOwnershipWithContext(This, socketId, data))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_TransferOwnershipWithContextAndKeepAliveTime(This, socketId, data, keepAliveTime) \
    ((This)->lpVtbl->TransferOwnershipWithContextAndKeepAliveTime(This, socketId, data, keepAliveTime))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketControl[] = L"Windows.Networking.Sockets.IStreamSocketControl";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NoDelay)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_NoDelay)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_KeepAlive)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_KeepAlive)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_OutboundBufferSizeInBytes)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_OutboundBufferSizeInBytes)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_QualityOfService)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketQualityOfService* value);
    HRESULT (STDMETHODCALLTYPE* put_QualityOfService)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketQualityOfService value);
    HRESULT (STDMETHODCALLTYPE* get_OutboundUnicastHopLimit)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_OutboundUnicastHopLimit)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl* This,
        BYTE value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControlVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_get_NoDelay(This, value) \
    ((This)->lpVtbl->get_NoDelay(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_put_NoDelay(This, value) \
    ((This)->lpVtbl->put_NoDelay(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_get_KeepAlive(This, value) \
    ((This)->lpVtbl->get_KeepAlive(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_put_KeepAlive(This, value) \
    ((This)->lpVtbl->put_KeepAlive(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_get_OutboundBufferSizeInBytes(This, value) \
    ((This)->lpVtbl->get_OutboundBufferSizeInBytes(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_put_OutboundBufferSizeInBytes(This, value) \
    ((This)->lpVtbl->put_OutboundBufferSizeInBytes(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_get_QualityOfService(This, value) \
    ((This)->lpVtbl->get_QualityOfService(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_put_QualityOfService(This, value) \
    ((This)->lpVtbl->put_QualityOfService(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_get_OutboundUnicastHopLimit(This, value) \
    ((This)->lpVtbl->get_OutboundUnicastHopLimit(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_put_OutboundUnicastHopLimit(This, value) \
    ((This)->lpVtbl->put_OutboundUnicastHopLimit(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketControl2[] = L"Windows.Networking.Sockets.IStreamSocketControl2";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IgnorableServerCertificateErrors)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2* This,
        __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_get_IgnorableServerCertificateErrors(This, value) \
    ((This)->lpVtbl->get_IgnorableServerCertificateErrors(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketControl3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketControl3[] = L"Windows.Networking.Sockets.IStreamSocketControl3";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SerializeConnectionAttempts)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_SerializeConnectionAttempts)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ClientCertificate)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** value);
    HRESULT (STDMETHODCALLTYPE* put_ClientCertificate)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_get_SerializeConnectionAttempts(This, value) \
    ((This)->lpVtbl->get_SerializeConnectionAttempts(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_put_SerializeConnectionAttempts(This, value) \
    ((This)->lpVtbl->put_SerializeConnectionAttempts(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_get_ClientCertificate(This, value) \
    ((This)->lpVtbl->get_ClientCertificate(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_put_ClientCertificate(This, value) \
    ((This)->lpVtbl->put_ClientCertificate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketControl4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketControl4[] = L"Windows.Networking.Sockets.IStreamSocketControl4";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MinProtectionLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketProtectionLevel* value);
    HRESULT (STDMETHODCALLTYPE* put_MinProtectionLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketProtectionLevel value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_get_MinProtectionLevel(This, value) \
    ((This)->lpVtbl->get_MinProtectionLevel(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_put_MinProtectionLevel(This, value) \
    ((This)->lpVtbl->put_MinProtectionLevel(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketControl4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketInformation[] = L"Windows.Networking.Sockets.IStreamSocketInformation";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LocalAddress)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* get_LocalPort)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteHostName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteAddress)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteServiceName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RemotePort)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RoundTripTimeStatistics)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation* This,
        struct __x_ABI_CWindows_CNetworking_CSockets_CRoundTripTimeStatistics* value);
    HRESULT (STDMETHODCALLTYPE* get_BandwidthStatistics)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation* This,
        struct __x_ABI_CWindows_CNetworking_CSockets_CBandwidthStatistics* value);
    HRESULT (STDMETHODCALLTYPE* get_ProtectionLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketProtectionLevel* value);
    HRESULT (STDMETHODCALLTYPE* get_SessionKey)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformationVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_get_LocalAddress(This, value) \
    ((This)->lpVtbl->get_LocalAddress(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_get_LocalPort(This, value) \
    ((This)->lpVtbl->get_LocalPort(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_get_RemoteHostName(This, value) \
    ((This)->lpVtbl->get_RemoteHostName(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_get_RemoteAddress(This, value) \
    ((This)->lpVtbl->get_RemoteAddress(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_get_RemoteServiceName(This, value) \
    ((This)->lpVtbl->get_RemoteServiceName(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_get_RemotePort(This, value) \
    ((This)->lpVtbl->get_RemotePort(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_get_RoundTripTimeStatistics(This, value) \
    ((This)->lpVtbl->get_RoundTripTimeStatistics(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_get_BandwidthStatistics(This, value) \
    ((This)->lpVtbl->get_BandwidthStatistics(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_get_ProtectionLevel(This, value) \
    ((This)->lpVtbl->get_ProtectionLevel(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_get_SessionKey(This, value) \
    ((This)->lpVtbl->get_SessionKey(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketInformation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketInformation2[] = L"Windows.Networking.Sockets.IStreamSocketInformation2";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ServerCertificateErrorSeverity)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketSslErrorSeverity* value);
    HRESULT (STDMETHODCALLTYPE* get_ServerCertificateErrors)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** value);
    HRESULT (STDMETHODCALLTYPE* get_ServerCertificate)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** value);
    HRESULT (STDMETHODCALLTYPE* get_ServerIntermediateCertificates)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_get_ServerCertificateErrorSeverity(This, value) \
    ((This)->lpVtbl->get_ServerCertificateErrorSeverity(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_get_ServerCertificateErrors(This, value) \
    ((This)->lpVtbl->get_ServerCertificateErrors(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_get_ServerCertificate(This, value) \
    ((This)->lpVtbl->get_ServerCertificate(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_get_ServerIntermediateCertificates(This, value) \
    ((This)->lpVtbl->get_ServerIntermediateCertificates(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketInformation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketListener
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketListener[] = L"Windows.Networking.Sockets.IStreamSocketListener";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Control)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl** value);
    HRESULT (STDMETHODCALLTYPE* get_Information)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation** value);
    HRESULT (STDMETHODCALLTYPE* BindServiceNameAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener* This,
        HSTRING localServiceName,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* BindEndpointAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener* This,
        __x_ABI_CWindows_CNetworking_CIHostName* localHostName,
        HSTRING localServiceName,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* add_ConnectionReceived)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener* This,
        __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamSocketListener_Windows__CNetworking__CSockets__CStreamSocketListenerConnectionReceivedEventArgs* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_ConnectionReceived)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_get_Control(This, value) \
    ((This)->lpVtbl->get_Control(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_get_Information(This, value) \
    ((This)->lpVtbl->get_Information(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_BindServiceNameAsync(This, localServiceName, operation) \
    ((This)->lpVtbl->BindServiceNameAsync(This, localServiceName, operation))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_BindEndpointAsync(This, localHostName, localServiceName, operation) \
    ((This)->lpVtbl->BindEndpointAsync(This, localHostName, localServiceName, operation))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_add_ConnectionReceived(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_ConnectionReceived(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_remove_ConnectionReceived(This, eventCookie) \
    ((This)->lpVtbl->remove_ConnectionReceived(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketListener2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketListener
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketListener2[] = L"Windows.Networking.Sockets.IStreamSocketListener2";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* BindServiceNameWithProtectionLevelAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2* This,
        HSTRING localServiceName,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketProtectionLevel protectionLevel,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* BindServiceNameWithProtectionLevelAndAdapterAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2* This,
        HSTRING localServiceName,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketProtectionLevel protectionLevel,
        __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter* adapter,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_BindServiceNameWithProtectionLevelAsync(This, localServiceName, protectionLevel, operation) \
    ((This)->lpVtbl->BindServiceNameWithProtectionLevelAsync(This, localServiceName, protectionLevel, operation))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_BindServiceNameWithProtectionLevelAndAdapterAsync(This, localServiceName, protectionLevel, adapter, operation) \
    ((This)->lpVtbl->BindServiceNameWithProtectionLevelAndAdapterAsync(This, localServiceName, protectionLevel, adapter, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketListener3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketListener3[] = L"Windows.Networking.Sockets.IStreamSocketListener3";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CancelIOAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* EnableTransferOwnership)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3* This,
        GUID taskId);
    HRESULT (STDMETHODCALLTYPE* EnableTransferOwnershipWithConnectedStandbyAction)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3* This,
        GUID taskId,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketActivityConnectedStandbyAction connectedStandbyAction);
    HRESULT (STDMETHODCALLTYPE* TransferOwnership)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3* This,
        HSTRING socketId);
    HRESULT (STDMETHODCALLTYPE* TransferOwnershipWithContext)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3* This,
        HSTRING socketId,
        __x_ABI_CWindows_CNetworking_CSockets_CISocketActivityContext* data);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_CancelIOAsync(This, operation) \
    ((This)->lpVtbl->CancelIOAsync(This, operation))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_EnableTransferOwnership(This, taskId) \
    ((This)->lpVtbl->EnableTransferOwnership(This, taskId))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_EnableTransferOwnershipWithConnectedStandbyAction(This, taskId, connectedStandbyAction) \
    ((This)->lpVtbl->EnableTransferOwnershipWithConnectedStandbyAction(This, taskId, connectedStandbyAction))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_TransferOwnership(This, socketId) \
    ((This)->lpVtbl->TransferOwnership(This, socketId))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_TransferOwnershipWithContext(This, socketId, data) \
    ((This)->lpVtbl->TransferOwnershipWithContext(This, socketId, data))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketListenerConnectionReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketListenerConnectionReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketListenerConnectionReceivedEventArgs[] = L"Windows.Networking.Sockets.IStreamSocketListenerConnectionReceivedEventArgs";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Socket)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_get_Socket(This, value) \
    ((This)->lpVtbl->get_Socket(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerConnectionReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketListenerControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketListenerControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketListenerControl[] = L"Windows.Networking.Sockets.IStreamSocketListenerControl";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_QualityOfService)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketQualityOfService* value);
    HRESULT (STDMETHODCALLTYPE* put_QualityOfService)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketQualityOfService value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControlVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_get_QualityOfService(This, value) \
    ((This)->lpVtbl->get_QualityOfService(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_put_QualityOfService(This, value) \
    ((This)->lpVtbl->put_QualityOfService(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketListenerControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketListenerControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketListenerControl2[] = L"Windows.Networking.Sockets.IStreamSocketListenerControl2";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NoDelay)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_NoDelay)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_KeepAlive)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_KeepAlive)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_OutboundBufferSizeInBytes)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_OutboundBufferSizeInBytes)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_OutboundUnicastHopLimit)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_OutboundUnicastHopLimit)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2* This,
        BYTE value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_get_NoDelay(This, value) \
    ((This)->lpVtbl->get_NoDelay(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_put_NoDelay(This, value) \
    ((This)->lpVtbl->put_NoDelay(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_get_KeepAlive(This, value) \
    ((This)->lpVtbl->get_KeepAlive(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_put_KeepAlive(This, value) \
    ((This)->lpVtbl->put_KeepAlive(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_get_OutboundBufferSizeInBytes(This, value) \
    ((This)->lpVtbl->get_OutboundBufferSizeInBytes(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_put_OutboundBufferSizeInBytes(This, value) \
    ((This)->lpVtbl->put_OutboundBufferSizeInBytes(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_get_OutboundUnicastHopLimit(This, value) \
    ((This)->lpVtbl->get_OutboundUnicastHopLimit(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_put_OutboundUnicastHopLimit(This, value) \
    ((This)->lpVtbl->put_OutboundUnicastHopLimit(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketListenerInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocketListenerInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketListenerInformation[] = L"Windows.Networking.Sockets.IStreamSocketListenerInformation";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LocalPort)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformationVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_get_LocalPort(This, value) \
    ((This)->lpVtbl->get_LocalPort(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListenerInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamSocketStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamSocket
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamSocketStatics[] = L"Windows.Networking.Sockets.IStreamSocketStatics";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetEndpointPairsAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics* This,
        __x_ABI_CWindows_CNetworking_CIHostName* remoteHostName,
        HSTRING remoteServiceName,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair** operation);
    HRESULT (STDMETHODCALLTYPE* GetEndpointPairsWithSortOptionsAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics* This,
        __x_ABI_CWindows_CNetworking_CIHostName* remoteHostName,
        HSTRING remoteServiceName,
        enum __x_ABI_CWindows_CNetworking_CHostNameSortOptions sortOptions,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CEndpointPair** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStaticsVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_GetEndpointPairsAsync(This, remoteHostName, remoteServiceName, operation) \
    ((This)->lpVtbl->GetEndpointPairsAsync(This, remoteHostName, remoteServiceName, operation))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_GetEndpointPairsWithSortOptionsAsync(This, remoteHostName, remoteServiceName, sortOptions, operation) \
    ((This)->lpVtbl->GetEndpointPairsWithSortOptionsAsync(This, remoteHostName, remoteServiceName, sortOptions, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamWebSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamWebSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Sockets.IWebSocket
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamWebSocket[] = L"Windows.Networking.Sockets.IStreamWebSocket";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Control)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl** value);
    HRESULT (STDMETHODCALLTYPE* get_Information)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation** value);
    HRESULT (STDMETHODCALLTYPE* get_InputStream)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_get_Control(This, value) \
    ((This)->lpVtbl->get_Control(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_get_Information(This, value) \
    ((This)->lpVtbl->get_Information(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_get_InputStream(This, value) \
    ((This)->lpVtbl->get_InputStream(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamWebSocket2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamWebSocket
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Sockets.IStreamWebSocket
 *     Windows.Networking.Sockets.IWebSocket
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamWebSocket2[] = L"Windows.Networking.Sockets.IStreamWebSocket2";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_ServerCustomValidationRequested)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2* This,
        __FITypedEventHandler_2_Windows__CNetworking__CSockets__CStreamWebSocket_Windows__CNetworking__CSockets__CWebSocketServerCustomValidationRequestedEventArgs* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_ServerCustomValidationRequested)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_add_ServerCustomValidationRequested(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_ServerCustomValidationRequested(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_remove_ServerCustomValidationRequested(This, eventCookie) \
    ((This)->lpVtbl->remove_ServerCustomValidationRequested(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocket2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamWebSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamWebSocketControl
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Sockets.IWebSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamWebSocketControl[] = L"Windows.Networking.Sockets.IStreamWebSocketControl";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NoDelay)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_NoDelay)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControlVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_get_NoDelay(This, value) \
    ((This)->lpVtbl->get_NoDelay(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_put_NoDelay(This, value) \
    ((This)->lpVtbl->put_NoDelay(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IStreamWebSocketControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.StreamWebSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IStreamWebSocketControl2[] = L"Windows.Networking.Sockets.IStreamWebSocketControl2";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DesiredUnsolicitedPongInterval)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_DesiredUnsolicitedPongInterval)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_ActualUnsolicitedPongInterval)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_ClientCertificate)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** value);
    HRESULT (STDMETHODCALLTYPE* put_ClientCertificate)(__x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_get_DesiredUnsolicitedPongInterval(This, value) \
    ((This)->lpVtbl->get_DesiredUnsolicitedPongInterval(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_put_DesiredUnsolicitedPongInterval(This, value) \
    ((This)->lpVtbl->put_DesiredUnsolicitedPongInterval(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_get_ActualUnsolicitedPongInterval(This, value) \
    ((This)->lpVtbl->get_ActualUnsolicitedPongInterval(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_get_ClientCertificate(This, value) \
    ((This)->lpVtbl->get_ClientCertificate(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_put_ClientCertificate(This, value) \
    ((This)->lpVtbl->put_ClientCertificate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIStreamWebSocketControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Networking.Sockets.IWebSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IWebSocket[] = L"Windows.Networking.Sockets.IWebSocket";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocket* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocket* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocket* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocket* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocket* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocket* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OutputStream)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocket* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream** value);
    HRESULT (STDMETHODCALLTYPE* ConnectAsync)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocket* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* SetRequestHeader)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocket* This,
        HSTRING headerName,
        HSTRING headerValue);
    HRESULT (STDMETHODCALLTYPE* add_Closed)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocket* This,
        __FITypedEventHandler_2_Windows__CNetworking__CSockets__CIWebSocket_Windows__CNetworking__CSockets__CWebSocketClosedEventArgs* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_Closed)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocket* This,
        EventRegistrationToken eventCookie);
    HRESULT (STDMETHODCALLTYPE* CloseWithStatus)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocket* This,
        UINT16 code,
        HSTRING reason);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIWebSocket
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_get_OutputStream(This, value) \
    ((This)->lpVtbl->get_OutputStream(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_ConnectAsync(This, uri, operation) \
    ((This)->lpVtbl->ConnectAsync(This, uri, operation))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_SetRequestHeader(This, headerName, headerValue) \
    ((This)->lpVtbl->SetRequestHeader(This, headerName, headerValue))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_add_Closed(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_Closed(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_remove_Closed(This, eventCookie) \
    ((This)->lpVtbl->remove_Closed(This, eventCookie))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_CloseWithStatus(This, code, reason) \
    ((This)->lpVtbl->CloseWithStatus(This, code, reason))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIWebSocket;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IWebSocketClosedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.WebSocketClosedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IWebSocketClosedEventArgs[] = L"Windows.Networking.Sockets.IWebSocketClosedEventArgs";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Code)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Reason)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgsVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_get_Code(This, value) \
    ((This)->lpVtbl->get_Code(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_get_Reason(This, value) \
    ((This)->lpVtbl->get_Reason(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketClosedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IWebSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IWebSocketControl[] = L"Windows.Networking.Sockets.IWebSocketControl";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OutboundBufferSizeInBytes)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_OutboundBufferSizeInBytes)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_ServerCredential)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** value);
    HRESULT (STDMETHODCALLTYPE* put_ServerCredential)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* value);
    HRESULT (STDMETHODCALLTYPE* get_ProxyCredential)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** value);
    HRESULT (STDMETHODCALLTYPE* put_ProxyCredential)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedProtocols)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl* This,
        __FIVector_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControlVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_get_OutboundBufferSizeInBytes(This, value) \
    ((This)->lpVtbl->get_OutboundBufferSizeInBytes(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_put_OutboundBufferSizeInBytes(This, value) \
    ((This)->lpVtbl->put_OutboundBufferSizeInBytes(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_get_ServerCredential(This, value) \
    ((This)->lpVtbl->get_ServerCredential(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_put_ServerCredential(This, value) \
    ((This)->lpVtbl->put_ServerCredential(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_get_ProxyCredential(This, value) \
    ((This)->lpVtbl->get_ProxyCredential(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_put_ProxyCredential(This, value) \
    ((This)->lpVtbl->put_ProxyCredential(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_get_SupportedProtocols(This, value) \
    ((This)->lpVtbl->get_SupportedProtocols(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IWebSocketControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Sockets.IWebSocketControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IWebSocketControl2[] = L"Windows.Networking.Sockets.IWebSocketControl2";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IgnorableServerCertificateErrors)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2* This,
        __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_get_IgnorableServerCertificateErrors(This, value) \
    ((This)->lpVtbl->get_IgnorableServerCertificateErrors(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.Sockets.IWebSocketErrorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.WebSocketError
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IWebSocketErrorStatics[] = L"Windows.Networking.Sockets.IWebSocketErrorStatics";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetStatus)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics* This,
        INT32 hresult,
        enum __x_ABI_CWindows_CWeb_CWebErrorStatus* status);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStaticsVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_GetStatus(This, hresult, status) \
    ((This)->lpVtbl->GetStatus(This, hresult, status))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketErrorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IWebSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IWebSocketInformation[] = L"Windows.Networking.Sockets.IWebSocketInformation";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LocalAddress)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* get_BandwidthStatistics)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation* This,
        struct __x_ABI_CWindows_CNetworking_CSockets_CBandwidthStatistics* value);
    HRESULT (STDMETHODCALLTYPE* get_Protocol)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformationVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_get_LocalAddress(This, value) \
    ((This)->lpVtbl->get_LocalAddress(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_get_BandwidthStatistics(This, value) \
    ((This)->lpVtbl->get_BandwidthStatistics(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_get_Protocol(This, value) \
    ((This)->lpVtbl->get_Protocol(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Sockets.IWebSocketInformation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Sockets.IWebSocketInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IWebSocketInformation2[] = L"Windows.Networking.Sockets.IWebSocketInformation2";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ServerCertificate)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** value);
    HRESULT (STDMETHODCALLTYPE* get_ServerCertificateErrorSeverity)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketSslErrorSeverity* value);
    HRESULT (STDMETHODCALLTYPE* get_ServerCertificateErrors)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** value);
    HRESULT (STDMETHODCALLTYPE* get_ServerIntermediateCertificates)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2Vtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_get_ServerCertificate(This, value) \
    ((This)->lpVtbl->get_ServerCertificate(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_get_ServerCertificateErrorSeverity(This, value) \
    ((This)->lpVtbl->get_ServerCertificateErrorSeverity(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_get_ServerCertificateErrors(This, value) \
    ((This)->lpVtbl->get_ServerCertificateErrors(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_get_ServerIntermediateCertificates(This, value) \
    ((This)->lpVtbl->get_ServerIntermediateCertificates(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketInformation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.Sockets.IWebSocketServerCustomValidationRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Sockets.WebSocketServerCustomValidationRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Sockets_IWebSocketServerCustomValidationRequestedEventArgs[] = L"Windows.Networking.Sockets.IWebSocketServerCustomValidationRequestedEventArgs";
typedef struct __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ServerCertificate)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** value);
    HRESULT (STDMETHODCALLTYPE* get_ServerCertificateErrorSeverity)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketSslErrorSeverity* value);
    HRESULT (STDMETHODCALLTYPE* get_ServerCertificateErrors)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** value);
    HRESULT (STDMETHODCALLTYPE* get_ServerIntermediateCertificates)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value);
    HRESULT (STDMETHODCALLTYPE* Reject)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_get_ServerCertificate(This, value) \
    ((This)->lpVtbl->get_ServerCertificate(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_get_ServerCertificateErrorSeverity(This, value) \
    ((This)->lpVtbl->get_ServerCertificateErrorSeverity(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_get_ServerCertificateErrors(This, value) \
    ((This)->lpVtbl->get_ServerCertificateErrors(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_get_ServerIntermediateCertificates(This, value) \
    ((This)->lpVtbl->get_ServerIntermediateCertificates(This, value))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_Reject(This) \
    ((This)->lpVtbl->Reject(This))

#define __x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CSockets_CIWebSocketServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Networking.Sockets.ControlChannelTrigger
 *
 * Introduced to Windows.Networking.Sockets.ControlChannelTriggerContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Sockets.IControlChannelTriggerFactory interface starting with version 1.0 of the Windows.Networking.Sockets.ControlChannelTriggerContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IControlChannelTrigger ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Networking.Sockets.IControlChannelTrigger2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_ControlChannelTrigger_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_ControlChannelTrigger_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_ControlChannelTrigger[] = L"Windows.Networking.Sockets.ControlChannelTrigger";
#endif
#endif // WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.DatagramSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Sockets.IDatagramSocketStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IDatagramSocket ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Networking.Sockets.IDatagramSocket2
 *    Windows.Networking.Sockets.IDatagramSocket3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_DatagramSocket_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_DatagramSocket_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_DatagramSocket[] = L"Windows.Networking.Sockets.DatagramSocket";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.DatagramSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IDatagramSocketControl ** Default Interface **
 *    Windows.Networking.Sockets.IDatagramSocketControl2
 *    Windows.Networking.Sockets.IDatagramSocketControl3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_DatagramSocketControl_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_DatagramSocketControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_DatagramSocketControl[] = L"Windows.Networking.Sockets.DatagramSocketControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.DatagramSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IDatagramSocketInformation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_DatagramSocketInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_DatagramSocketInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_DatagramSocketInformation[] = L"Windows.Networking.Sockets.DatagramSocketInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.DatagramSocketMessageReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IDatagramSocketMessageReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_DatagramSocketMessageReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_DatagramSocketMessageReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_DatagramSocketMessageReceivedEventArgs[] = L"Windows.Networking.Sockets.DatagramSocketMessageReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.MessageWebSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IMessageWebSocket ** Default Interface **
 *    Windows.Networking.Sockets.IWebSocket
 *    Windows.Foundation.IClosable
 *    Windows.Networking.Sockets.IMessageWebSocket2
 *    Windows.Networking.Sockets.IMessageWebSocket3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_MessageWebSocket_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_MessageWebSocket_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_MessageWebSocket[] = L"Windows.Networking.Sockets.MessageWebSocket";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.MessageWebSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IMessageWebSocketControl ** Default Interface **
 *    Windows.Networking.Sockets.IWebSocketControl
 *    Windows.Networking.Sockets.IWebSocketControl2
 *    Windows.Networking.Sockets.IMessageWebSocketControl2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_MessageWebSocketControl_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_MessageWebSocketControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_MessageWebSocketControl[] = L"Windows.Networking.Sockets.MessageWebSocketControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.MessageWebSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IWebSocketInformation ** Default Interface **
 *    Windows.Networking.Sockets.IWebSocketInformation2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_MessageWebSocketInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_MessageWebSocketInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_MessageWebSocketInformation[] = L"Windows.Networking.Sockets.MessageWebSocketInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.MessageWebSocketMessageReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IMessageWebSocketMessageReceivedEventArgs ** Default Interface **
 *    Windows.Networking.Sockets.IMessageWebSocketMessageReceivedEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_MessageWebSocketMessageReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_MessageWebSocketMessageReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_MessageWebSocketMessageReceivedEventArgs[] = L"Windows.Networking.Sockets.MessageWebSocketMessageReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.ServerMessageWebSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IServerMessageWebSocket ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_ServerMessageWebSocket_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_ServerMessageWebSocket_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_ServerMessageWebSocket[] = L"Windows.Networking.Sockets.ServerMessageWebSocket";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Networking.Sockets.ServerMessageWebSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IServerMessageWebSocketControl ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_ServerMessageWebSocketControl_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_ServerMessageWebSocketControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_ServerMessageWebSocketControl[] = L"Windows.Networking.Sockets.ServerMessageWebSocketControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Networking.Sockets.ServerMessageWebSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IServerMessageWebSocketInformation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_ServerMessageWebSocketInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_ServerMessageWebSocketInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_ServerMessageWebSocketInformation[] = L"Windows.Networking.Sockets.ServerMessageWebSocketInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Networking.Sockets.ServerStreamWebSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IServerStreamWebSocket ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_ServerStreamWebSocket_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_ServerStreamWebSocket_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_ServerStreamWebSocket[] = L"Windows.Networking.Sockets.ServerStreamWebSocket";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Networking.Sockets.ServerStreamWebSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IServerStreamWebSocketInformation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_ServerStreamWebSocketInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_ServerStreamWebSocketInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_ServerStreamWebSocketInformation[] = L"Windows.Networking.Sockets.ServerStreamWebSocketInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Networking.Sockets.SocketActivityContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Sockets.ISocketActivityContextFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.ISocketActivityContext ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_SocketActivityContext_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_SocketActivityContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_SocketActivityContext[] = L"Windows.Networking.Sockets.SocketActivityContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.SocketActivityInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Sockets.ISocketActivityInformationStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.ISocketActivityInformation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_SocketActivityInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_SocketActivityInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_SocketActivityInformation[] = L"Windows.Networking.Sockets.SocketActivityInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.SocketActivityTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.ISocketActivityTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_SocketActivityTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_SocketActivityTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_SocketActivityTriggerDetails[] = L"Windows.Networking.Sockets.SocketActivityTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.SocketError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Sockets.ISocketErrorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_SocketError_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_SocketError_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_SocketError[] = L"Windows.Networking.Sockets.SocketError";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Sockets.IStreamSocketStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IStreamSocket ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Networking.Sockets.IStreamSocket2
 *    Windows.Networking.Sockets.IStreamSocket3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamSocket_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamSocket_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamSocket[] = L"Windows.Networking.Sockets.StreamSocket";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IStreamSocketControl ** Default Interface **
 *    Windows.Networking.Sockets.IStreamSocketControl2
 *    Windows.Networking.Sockets.IStreamSocketControl3
 *    Windows.Networking.Sockets.IStreamSocketControl4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketControl_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamSocketControl[] = L"Windows.Networking.Sockets.StreamSocketControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IStreamSocketInformation ** Default Interface **
 *    Windows.Networking.Sockets.IStreamSocketInformation2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamSocketInformation[] = L"Windows.Networking.Sockets.StreamSocketInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamSocketListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IStreamSocketListener ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Networking.Sockets.IStreamSocketListener2
 *    Windows.Networking.Sockets.IStreamSocketListener3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketListener_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketListener_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamSocketListener[] = L"Windows.Networking.Sockets.StreamSocketListener";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamSocketListenerConnectionReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IStreamSocketListenerConnectionReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketListenerConnectionReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketListenerConnectionReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamSocketListenerConnectionReceivedEventArgs[] = L"Windows.Networking.Sockets.StreamSocketListenerConnectionReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamSocketListenerControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IStreamSocketListenerControl ** Default Interface **
 *    Windows.Networking.Sockets.IStreamSocketListenerControl2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketListenerControl_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketListenerControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamSocketListenerControl[] = L"Windows.Networking.Sockets.StreamSocketListenerControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamSocketListenerInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IStreamSocketListenerInformation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketListenerInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamSocketListenerInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamSocketListenerInformation[] = L"Windows.Networking.Sockets.StreamSocketListenerInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamWebSocket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IStreamWebSocket ** Default Interface **
 *    Windows.Networking.Sockets.IWebSocket
 *    Windows.Foundation.IClosable
 *    Windows.Networking.Sockets.IStreamWebSocket2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamWebSocket_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamWebSocket_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamWebSocket[] = L"Windows.Networking.Sockets.StreamWebSocket";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamWebSocketControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IStreamWebSocketControl ** Default Interface **
 *    Windows.Networking.Sockets.IWebSocketControl
 *    Windows.Networking.Sockets.IWebSocketControl2
 *    Windows.Networking.Sockets.IStreamWebSocketControl2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamWebSocketControl_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamWebSocketControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamWebSocketControl[] = L"Windows.Networking.Sockets.StreamWebSocketControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.StreamWebSocketInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IWebSocketInformation ** Default Interface **
 *    Windows.Networking.Sockets.IWebSocketInformation2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_StreamWebSocketInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_StreamWebSocketInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_StreamWebSocketInformation[] = L"Windows.Networking.Sockets.StreamWebSocketInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.WebSocketClosedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IWebSocketClosedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_WebSocketClosedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_WebSocketClosedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_WebSocketClosedEventArgs[] = L"Windows.Networking.Sockets.WebSocketClosedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.WebSocketError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Sockets.IWebSocketErrorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_WebSocketError_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_WebSocketError_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_WebSocketError[] = L"Windows.Networking.Sockets.WebSocketError";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.WebSocketKeepAlive
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Background.IBackgroundTask ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_WebSocketKeepAlive_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_WebSocketKeepAlive_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_WebSocketKeepAlive[] = L"Windows.Networking.Sockets.WebSocketKeepAlive";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Sockets.WebSocketServerCustomValidationRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Sockets.IWebSocketServerCustomValidationRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Networking_Sockets_WebSocketServerCustomValidationRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Sockets_WebSocketServerCustomValidationRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Sockets_WebSocketServerCustomValidationRequestedEventArgs[] = L"Windows.Networking.Sockets.WebSocketServerCustomValidationRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Enetworking2Esockets_p_h__

#endif // __windows2Enetworking2Esockets_h__
