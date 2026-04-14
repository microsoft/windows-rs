
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
#ifndef __windows2Enetworking2Econnectivity_h__
#define __windows2Enetworking2Econnectivity_h__
#ifndef __windows2Enetworking2Econnectivity_p_h__
#define __windows2Enetworking2Econnectivity_p_h__


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
#include "Windows.Networking.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface INetworkStatusChangedEventHandler;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler ABI::Windows::Networking::Connectivity::INetworkStatusChangedEventHandler

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IAttributedNetworkUsage;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage ABI::Windows::Networking::Connectivity::IAttributedNetworkUsage

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface ICellularApnContext;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext ABI::Windows::Networking::Connectivity::ICellularApnContext

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface ICellularApnContext2;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2 ABI::Windows::Networking::Connectivity::ICellularApnContext2

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IConnectionCost;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost ABI::Windows::Networking::Connectivity::IConnectionCost

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IConnectionCost2;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2 ABI::Windows::Networking::Connectivity::IConnectionCost2

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IConnectionProfile;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile ABI::Windows::Networking::Connectivity::IConnectionProfile

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IConnectionProfile2;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2 ABI::Windows::Networking::Connectivity::IConnectionProfile2

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IConnectionProfile3;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3 ABI::Windows::Networking::Connectivity::IConnectionProfile3

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IConnectionProfile4;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4 ABI::Windows::Networking::Connectivity::IConnectionProfile4

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IConnectionProfile5;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5 ABI::Windows::Networking::Connectivity::IConnectionProfile5

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IConnectionProfile6;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6 ABI::Windows::Networking::Connectivity::IConnectionProfile6

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IConnectionProfileFilter;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter ABI::Windows::Networking::Connectivity::IConnectionProfileFilter

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IConnectionProfileFilter2;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2 ABI::Windows::Networking::Connectivity::IConnectionProfileFilter2

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IConnectionProfileFilter3;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3 ABI::Windows::Networking::Connectivity::IConnectionProfileFilter3

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IConnectionSession;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession ABI::Windows::Networking::Connectivity::IConnectionSession

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IConnectivityInterval;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval ABI::Windows::Networking::Connectivity::IConnectivityInterval

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IConnectivityManagerStatics;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics ABI::Windows::Networking::Connectivity::IConnectivityManagerStatics

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IDataPlanStatus;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus ABI::Windows::Networking::Connectivity::IDataPlanStatus

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IDataPlanUsage;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage ABI::Windows::Networking::Connectivity::IDataPlanUsage

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IDataUsage;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage ABI::Windows::Networking::Connectivity::IDataUsage

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IIPInformation;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation ABI::Windows::Networking::Connectivity::IIPInformation

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface ILanIdentifier;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier ABI::Windows::Networking::Connectivity::ILanIdentifier

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface ILanIdentifierData;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData ABI::Windows::Networking::Connectivity::ILanIdentifierData

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface INetworkInformationStatics;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics ABI::Windows::Networking::Connectivity::INetworkInformationStatics

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface INetworkInformationStatics2;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2 ABI::Windows::Networking::Connectivity::INetworkInformationStatics2

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface INetworkItem;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem ABI::Windows::Networking::Connectivity::INetworkItem

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface INetworkStateChangeEventDetails;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails ABI::Windows::Networking::Connectivity::INetworkStateChangeEventDetails

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface INetworkStateChangeEventDetails2;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2 ABI::Windows::Networking::Connectivity::INetworkStateChangeEventDetails2

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface INetworkUsage;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage ABI::Windows::Networking::Connectivity::INetworkUsage

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IProviderNetworkUsage;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage ABI::Windows::Networking::Connectivity::IProviderNetworkUsage

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IProxyConfiguration;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration ABI::Windows::Networking::Connectivity::IProxyConfiguration

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IRoutePolicy;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy ABI::Windows::Networking::Connectivity::IRoutePolicy

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IRoutePolicyFactory;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory ABI::Windows::Networking::Connectivity::IRoutePolicyFactory

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IWlanConnectionProfileDetails;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails ABI::Windows::Networking::Connectivity::IWlanConnectionProfileDetails

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IWwanConnectionProfileDetails;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails ABI::Windows::Networking::Connectivity::IWwanConnectionProfileDetails

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IWwanConnectionProfileDetails2;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2 ABI::Windows::Networking::Connectivity::IWwanConnectionProfileDetails2

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class AttributedNetworkUsage;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_USE
#define DEF___FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4070c40f-ab2f-56f2-b54c-8232ae86aacd"))
IIterator<ABI::Windows::Networking::Connectivity::AttributedNetworkUsage*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::AttributedNetworkUsage*, ABI::Windows::Networking::Connectivity::IAttributedNetworkUsage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.Connectivity.AttributedNetworkUsage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::Connectivity::AttributedNetworkUsage*> __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_t;
#define __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_USE
#define DEF___FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d061dcb9-6854-5ef9-8e03-008a7a704c48"))
IIterable<ABI::Windows::Networking::Connectivity::AttributedNetworkUsage*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::AttributedNetworkUsage*, ABI::Windows::Networking::Connectivity::IAttributedNetworkUsage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.Connectivity.AttributedNetworkUsage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::Connectivity::AttributedNetworkUsage*> __FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_t;
#define __FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6e7c44ad-7753-5437-9f79-970d391ff7c4"))
IVectorView<ABI::Windows::Networking::Connectivity::AttributedNetworkUsage*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::AttributedNetworkUsage*, ABI::Windows::Networking::Connectivity::IAttributedNetworkUsage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Connectivity.AttributedNetworkUsage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::Connectivity::AttributedNetworkUsage*> __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_t;
#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9d8a4113-e7f3-552d-9a8c-1c25e2137253"))
IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Connectivity.AttributedNetworkUsage>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage*> __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("96af15cc-f060-5667-9223-e054d14239ec"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Connectivity.AttributedNetworkUsage>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class ConnectionProfile;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE
#define DEF___FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("89913732-a08b-5cb2-af16-bbbb2223839e"))
IIterator<ABI::Windows::Networking::Connectivity::ConnectionProfile*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::ConnectionProfile*, ABI::Windows::Networking::Connectivity::IConnectionProfile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.Connectivity.ConnectionProfile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::Connectivity::ConnectionProfile*> __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile_t;
#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE
#define DEF___FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("34dabef9-87d0-5b1c-a7ac-9d290adeb0c8"))
IIterable<ABI::Windows::Networking::Connectivity::ConnectionProfile*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::ConnectionProfile*, ABI::Windows::Networking::Connectivity::IConnectionProfile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.Connectivity.ConnectionProfile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::Connectivity::ConnectionProfile*> __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile_t;
#define __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6db1b492-3852-5df8-a29d-6944002f58d4"))
IVectorView<ABI::Windows::Networking::Connectivity::ConnectionProfile*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::ConnectionProfile*, ABI::Windows::Networking::Connectivity::IConnectionProfile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Connectivity.ConnectionProfile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::Connectivity::ConnectionProfile*> __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_t;
#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c0023294-c2cb-52f0-a9f4-21916032f69d"))
IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Connectivity.ConnectionProfile>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile*> __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c523d9dd-4ea6-5115-80e9-4e7ad4769798"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Connectivity.ConnectionProfile>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class ConnectivityInterval;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_USE
#define DEF___FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("741cea48-651c-5fd9-931e-4f91b521e182"))
IIterator<ABI::Windows::Networking::Connectivity::ConnectivityInterval*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::ConnectivityInterval*, ABI::Windows::Networking::Connectivity::IConnectivityInterval*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.Connectivity.ConnectivityInterval>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::Connectivity::ConnectivityInterval*> __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_t;
#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_USE
#define DEF___FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("58051a8b-b259-5414-9b9a-caa0789e833e"))
IIterable<ABI::Windows::Networking::Connectivity::ConnectivityInterval*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::ConnectivityInterval*, ABI::Windows::Networking::Connectivity::IConnectivityInterval*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.Connectivity.ConnectivityInterval>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::Connectivity::ConnectivityInterval*> __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_t;
#define __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a3d0d117-9e21-5919-b7a0-c8190bd55ac5"))
IVectorView<ABI::Windows::Networking::Connectivity::ConnectivityInterval*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::ConnectivityInterval*, ABI::Windows::Networking::Connectivity::IConnectivityInterval*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Connectivity.ConnectivityInterval>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::Connectivity::ConnectivityInterval*> __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_t;
#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("af96d70b-41c7-5dc6-9895-ea043a885d8d"))
IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Connectivity.ConnectivityInterval>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval*> __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b475014c-95f1-5310-b5d1-c2309d944440"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Connectivity.ConnectivityInterval>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class NetworkUsage;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage_USE
#define DEF___FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5fafb57b-9c82-50a1-9970-69f9cb069695"))
IIterator<ABI::Windows::Networking::Connectivity::NetworkUsage*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::NetworkUsage*, ABI::Windows::Networking::Connectivity::INetworkUsage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.Connectivity.NetworkUsage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::Connectivity::NetworkUsage*> __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage_t;
#define __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage_USE
#define DEF___FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("dd2656b1-8360-5772-b272-c47f7f0fc7a6"))
IIterable<ABI::Windows::Networking::Connectivity::NetworkUsage*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::NetworkUsage*, ABI::Windows::Networking::Connectivity::INetworkUsage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.Connectivity.NetworkUsage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::Connectivity::NetworkUsage*> __FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage_t;
#define __FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b3853391-40b6-5cf5-8f46-4882691d1ff7"))
IVectorView<ABI::Windows::Networking::Connectivity::NetworkUsage*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::NetworkUsage*, ABI::Windows::Networking::Connectivity::INetworkUsage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Connectivity.NetworkUsage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::Connectivity::NetworkUsage*> __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_t;
#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("05c9e081-6229-5049-8eea-a498407c00d5"))
IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Connectivity.NetworkUsage>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage*> __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e31d7e7e-4173-5c71-b04b-a09658002590"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Connectivity.NetworkUsage>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class ProviderNetworkUsage;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_USE
#define DEF___FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d7090752-ab5f-506f-8f15-56b37552fbea"))
IIterator<ABI::Windows::Networking::Connectivity::ProviderNetworkUsage*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::ProviderNetworkUsage*, ABI::Windows::Networking::Connectivity::IProviderNetworkUsage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.Connectivity.ProviderNetworkUsage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::Connectivity::ProviderNetworkUsage*> __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_t;
#define __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_USE
#define DEF___FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f79bc7ba-01df-51ec-bfaf-fd883f698e07"))
IIterable<ABI::Windows::Networking::Connectivity::ProviderNetworkUsage*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::ProviderNetworkUsage*, ABI::Windows::Networking::Connectivity::IProviderNetworkUsage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.Connectivity.ProviderNetworkUsage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::Connectivity::ProviderNetworkUsage*> __FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_t;
#define __FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e66ad09c-eb37-54c7-9b2d-734e0e939305"))
IVectorView<ABI::Windows::Networking::Connectivity::ProviderNetworkUsage*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::ProviderNetworkUsage*, ABI::Windows::Networking::Connectivity::IProviderNetworkUsage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Connectivity.ProviderNetworkUsage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::Connectivity::ProviderNetworkUsage*> __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_t;
#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7eba5a8f-e4fd-5201-a4f4-9567596f213c"))
IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Connectivity.ProviderNetworkUsage>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage*> __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c310276b-3932-5da9-9a3b-c5c423586b42"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Connectivity.ProviderNetworkUsage>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE
#define DEF___FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5bf519ca-8adb-5ab5-abb8-ff1bbe5d2de8"))
IAsyncOperation<ABI::Windows::Networking::Connectivity::ConnectionProfile*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::ConnectionProfile*, ABI::Windows::Networking::Connectivity::IConnectionProfile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Networking.Connectivity.ConnectionProfile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Networking::Connectivity::ConnectionProfile*> __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile_t;
#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e4f0c96a-0571-59f4-a9a9-afac3e61caa0"))
IAsyncOperationCompletedHandler<ABI::Windows::Networking::Connectivity::ConnectionProfile*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::ConnectionProfile*, ABI::Windows::Networking::Connectivity::IConnectionProfile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Networking.Connectivity.ConnectionProfile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Networking::Connectivity::ConnectionProfile*> __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                typedef enum ConnectionProfileDeleteStatus : int ConnectionProfileDeleteStatus;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2ffebc9e-4bed-5f3d-8e55-5abc2598e144"))
IAsyncOperation<enum ABI::Windows::Networking::Connectivity::ConnectionProfileDeleteStatus> : IAsyncOperation_impl<enum ABI::Windows::Networking::Connectivity::ConnectionProfileDeleteStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Networking.Connectivity.ConnectionProfileDeleteStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Networking::Connectivity::ConnectionProfileDeleteStatus> __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_t;
#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3a0fb210-36c4-5df2-9707-5f325369a9b0"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Networking::Connectivity::ConnectionProfileDeleteStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Networking::Connectivity::ConnectionProfileDeleteStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Networking.Connectivity.ConnectionProfileDeleteStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Networking::Connectivity::ConnectionProfileDeleteStatus> __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class ConnectionSession;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession_USE
#define DEF___FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("94fc6211-4702-5d24-81bf-170ca7818995"))
IAsyncOperation<ABI::Windows::Networking::Connectivity::ConnectionSession*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::ConnectionSession*, ABI::Windows::Networking::Connectivity::IConnectionSession*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Networking.Connectivity.ConnectionSession>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Networking::Connectivity::ConnectionSession*> __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession_t;
#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3bc680d8-9e83-5086-8f49-7a29bfb1c7e1"))
IAsyncOperationCompletedHandler<ABI::Windows::Networking::Connectivity::ConnectionSession*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::ConnectionSession*, ABI::Windows::Networking::Connectivity::IConnectionSession*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Networking.Connectivity.ConnectionSession>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Networking::Connectivity::ConnectionSession*> __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class ProxyConfiguration;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_USE
#define DEF___FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1e7651f6-6562-59c7-9af3-8756636eeee2"))
IAsyncOperation<ABI::Windows::Networking::Connectivity::ProxyConfiguration*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::ProxyConfiguration*, ABI::Windows::Networking::Connectivity::IProxyConfiguration*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Networking.Connectivity.ProxyConfiguration>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Networking::Connectivity::ProxyConfiguration*> __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_t;
#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("035b2567-efb9-5bc3-b609-f9a8c20b7001"))
IAsyncOperationCompletedHandler<ABI::Windows::Networking::Connectivity::ProxyConfiguration*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::ProxyConfiguration*, ABI::Windows::Networking::Connectivity::IProxyConfiguration*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Networking.Connectivity.ProxyConfiguration>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Networking::Connectivity::ProxyConfiguration*> __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIIterator_1_GUID_USE
#define DEF___FIIterator_1_GUID_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d3d64048-82b3-53c7-9285-b0be18368482"))
IIterator<GUID> : IIterator_impl<GUID>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Guid>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<GUID> __FIIterator_1_GUID_t;
#define __FIIterator_1_GUID ABI::Windows::Foundation::Collections::__FIIterator_1_GUID_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_GUID_USE */



#ifndef DEF___FIIterable_1_GUID_USE
#define DEF___FIIterable_1_GUID_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f4ca3045-5dd7-54be-982e-d88d8ca0876e"))
IIterable<GUID> : IIterable_impl<GUID>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Guid>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<GUID> __FIIterable_1_GUID_t;
#define __FIIterable_1_GUID ABI::Windows::Foundation::Collections::__FIIterable_1_GUID_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_GUID_USE */



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



#ifndef DEF___FIIterator_1_byte_USE
#define DEF___FIIterator_1_byte_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("40556131-a2a1-5fab-aaee-5f35268ca26b"))
IIterator<::byte> : IIterator_impl<::byte>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<UInt8>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<::byte> __FIIterator_1_byte_t;
#define __FIIterator_1_byte ABI::Windows::Foundation::Collections::__FIIterator_1_byte_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_byte_USE */



#ifndef DEF___FIIterable_1_byte_USE
#define DEF___FIIterable_1_byte_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("88318266-f3fd-50fc-8f08-b823a41b60c1"))
IIterable<::byte> : IIterable_impl<::byte>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<UInt8>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<::byte> __FIIterable_1_byte_t;
#define __FIIterable_1_byte ABI::Windows::Foundation::Collections::__FIIterable_1_byte_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_byte_USE */


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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CFoundation__CUri_USE
#define DEF___FIIterator_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1c157d0f-5efe-5cec-bbd6-0c6ce9af07a5"))
IIterator<ABI::Windows::Foundation::Uri*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Foundation::Uri*> __FIIterator_1_Windows__CFoundation__CUri_t;
#define __FIIterator_1_Windows__CFoundation__CUri ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CFoundation__CUri_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CFoundation__CUri_USE
#define DEF___FIIterable_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b0d63b78-78ad-5e31-b6d8-e32a0e16c447"))
IIterable<ABI::Windows::Foundation::Uri*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Foundation::Uri*> __FIIterable_1_Windows__CFoundation__CUri_t;
#define __FIIterable_1_Windows__CFoundation__CUri ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CFoundation__CUri_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class LanIdentifier;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier_USE
#define DEF___FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2c5d2f7e-ce9c-5253-a0f4-01e5bdc11988"))
IIterator<ABI::Windows::Networking::Connectivity::LanIdentifier*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::LanIdentifier*, ABI::Windows::Networking::Connectivity::ILanIdentifier*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.Connectivity.LanIdentifier>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::Connectivity::LanIdentifier*> __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier_t;
#define __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier_USE
#define DEF___FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("accef3cd-5d92-5c01-8ac4-79fe74cd733e"))
IIterable<ABI::Windows::Networking::Connectivity::LanIdentifier*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::LanIdentifier*, ABI::Windows::Networking::Connectivity::ILanIdentifier*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.Connectivity.LanIdentifier>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::Connectivity::LanIdentifier*> __FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier_t;
#define __FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier_USE */

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CHostName_USE
#define DEF___FIIterator_1_Windows__CNetworking__CHostName_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("557bf83c-a428-5dbd-a0fe-05f6ee543d45"))
IIterator<ABI::Windows::Networking::HostName*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::HostName*, ABI::Windows::Networking::IHostName*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.HostName>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::HostName*> __FIIterator_1_Windows__CNetworking__CHostName_t;
#define __FIIterator_1_Windows__CNetworking__CHostName ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CHostName_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CHostName_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CHostName_USE
#define DEF___FIIterable_1_Windows__CNetworking__CHostName_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9e5f3ed0-cf1c-5d38-832c-acea6164bf5c"))
IIterable<ABI::Windows::Networking::HostName*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::HostName*, ABI::Windows::Networking::IHostName*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.HostName>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::HostName*> __FIIterable_1_Windows__CNetworking__CHostName_t;
#define __FIIterable_1_Windows__CNetworking__CHostName ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CHostName_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CHostName_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIVectorView_1_GUID_USE
#define DEF___FIVectorView_1_GUID_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9520e64b-15b2-52a6-98ed-3191fa6cf68a"))
IVectorView<GUID> : IVectorView_impl<GUID>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Guid>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<GUID> __FIVectorView_1_GUID_t;
#define __FIVectorView_1_GUID ABI::Windows::Foundation::Collections::__FIVectorView_1_GUID_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_GUID_USE */



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



#ifndef DEF___FIVectorView_1_byte_USE
#define DEF___FIVectorView_1_byte_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6d05fb29-7885-544e-9382-a1ad391a3fa4"))
IVectorView<::byte> : IVectorView_impl<::byte>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<UInt8>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<::byte> __FIVectorView_1_byte_t;
#define __FIVectorView_1_byte ABI::Windows::Foundation::Collections::__FIVectorView_1_byte_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_byte_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CFoundation__CUri_USE
#define DEF___FIVectorView_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4b8385bd-a2cd-5ff1-bf74-7ea580423e50"))
IVectorView<ABI::Windows::Foundation::Uri*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Foundation::Uri*> __FIVectorView_1_Windows__CFoundation__CUri_t;
#define __FIVectorView_1_Windows__CFoundation__CUri ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CFoundation__CUri_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("41286159-b91d-5736-ad8b-e16fcf8aced0"))
IVectorView<ABI::Windows::Networking::Connectivity::LanIdentifier*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Connectivity::LanIdentifier*, ABI::Windows::Networking::Connectivity::ILanIdentifier*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Connectivity.LanIdentifier>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::Connectivity::LanIdentifier*> __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier_t;
#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier_USE */

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

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CHostName_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CHostName_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f4706ab1-55a3-5270-afb2-732988fe8227"))
IVectorView<ABI::Windows::Networking::HostName*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::HostName*, ABI::Windows::Networking::IHostName*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.HostName>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::HostName*> __FIVectorView_1_Windows__CNetworking__CHostName_t;
#define __FIVectorView_1_Windows__CNetworking__CHostName ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CHostName_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CHostName_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIReference_1_boolean_USE
#define DEF___FIReference_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3c00fd60-2950-5939-a21a-2d12c5a01b8a"))
IReference<bool> : IReference_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<bool> __FIReference_1_boolean_t;
#define __FIReference_1_boolean ABI::Windows::Foundation::__FIReference_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_boolean_USE */



#ifndef DEF___FIReference_1_GUID_USE
#define DEF___FIReference_1_GUID_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7d50f649-632c-51f9-849a-ee49428933ea"))
IReference<GUID> : IReference_impl<GUID>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Guid>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<GUID> __FIReference_1_GUID_t;
#define __FIReference_1_GUID ABI::Windows::Foundation::__FIReference_1_GUID_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_GUID_USE */



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



#ifndef DEF___FIReference_1_UINT64_USE
#define DEF___FIReference_1_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6755e376-53bb-568b-a11d-17239868309e"))
IReference<UINT64> : IReference_impl<UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<UINT64> __FIReference_1_UINT64_t;
#define __FIReference_1_UINT64 ABI::Windows::Foundation::__FIReference_1_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_UINT64_USE */



#ifndef DEF___FIReference_1_byte_USE
#define DEF___FIReference_1_byte_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e5198cc8-2873-55f5-b0a1-84ff9e4aad62"))
IReference<::byte> : IReference_impl<::byte>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<UInt8>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<::byte> __FIReference_1_byte_t;
#define __FIReference_1_byte ABI::Windows::Foundation::__FIReference_1_byte_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_byte_USE */


namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CFoundation__CDateTime_USE
#define DEF___FIReference_1_Windows__CFoundation__CDateTime_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5541d8a7-497c-5aa4-86fc-7713adbf2a2c"))
IReference<struct ABI::Windows::Foundation::DateTime> : IReference_impl<struct ABI::Windows::Foundation::DateTime>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Foundation.DateTime>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Foundation::DateTime> __FIReference_1_Windows__CFoundation__CDateTime_t;
#define __FIReference_1_Windows__CFoundation__CDateTime ABI::Windows::Foundation::__FIReference_1_Windows__CFoundation__CDateTime_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CFoundation__CDateTime_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

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
            typedef enum DomainNameType : int DomainNameType;
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            typedef enum HostNameSortOptions : unsigned int HostNameSortOptions;
        } /* Networking */
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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStreamReference;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference ABI::Windows::Storage::Streams::IRandomAccessStreamReference

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                typedef enum CellularApnAuthenticationType : int CellularApnAuthenticationType;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                typedef enum DataUsageGranularity : int DataUsageGranularity;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                typedef enum DomainAuthenticationKind : int DomainAuthenticationKind;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                typedef enum DomainConnectivityLevel : int DomainConnectivityLevel;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                typedef enum NetworkAuthenticationType : int NetworkAuthenticationType;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                typedef enum NetworkConnectivityLevel : int NetworkConnectivityLevel;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                typedef enum NetworkCostType : int NetworkCostType;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                typedef enum NetworkEncryptionType : int NetworkEncryptionType;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                typedef enum NetworkTypes : unsigned int NetworkTypes;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                typedef enum RoamingStates : unsigned int RoamingStates;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                typedef enum TriStates : int TriStates;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                typedef enum WwanDataClass : unsigned int WwanDataClass;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                typedef enum WwanNetworkIPKind : int WwanNetworkIPKind;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                typedef enum WwanNetworkRegistrationState : int WwanNetworkRegistrationState;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                typedef struct NetworkUsageStates NetworkUsageStates;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class CellularApnContext;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class ConnectionCost;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class ConnectionProfileFilter;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class DataPlanStatus;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class DataPlanUsage;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class DataUsage;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class LanIdentifierData;
            } /* Connectivity */
        } /* Networking */
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

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class NetworkItem;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class NetworkSecuritySettings;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class RoutePolicy;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class WlanConnectionProfileDetails;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class WwanConnectionProfileDetails;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Networking.Connectivity.CellularApnAuthenticationType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                enum CellularApnAuthenticationType : int
                {
                    CellularApnAuthenticationType_None = 0,
                    CellularApnAuthenticationType_Pap = 1,
                    CellularApnAuthenticationType_Chap = 2,
                    CellularApnAuthenticationType_Mschapv2 = 3,
                };
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.ConnectionProfileDeleteStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                enum ConnectionProfileDeleteStatus : int
                {
                    ConnectionProfileDeleteStatus_Success = 0,
                    ConnectionProfileDeleteStatus_DeniedByUser = 1,
                    ConnectionProfileDeleteStatus_DeniedBySystem = 2,
                    ConnectionProfileDeleteStatus_UnknownError = 3,
                };
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Networking.Connectivity.DataUsageGranularity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                enum DataUsageGranularity : int
                {
                    DataUsageGranularity_PerMinute = 0,
                    DataUsageGranularity_PerHour = 1,
                    DataUsageGranularity_PerDay = 2,
                    DataUsageGranularity_Total = 3,
                };
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.DomainAuthenticationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                enum DomainAuthenticationKind : int
                {
                    DomainAuthenticationKind_None = 0,
                    DomainAuthenticationKind_Ldap = 1,
                    DomainAuthenticationKind_Tls = 2,
                };
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.Networking.Connectivity.DomainConnectivityLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                enum DomainConnectivityLevel : int
                {
                    DomainConnectivityLevel_None = 0,
                    DomainConnectivityLevel_Unauthenticated = 1,
                    DomainConnectivityLevel_Authenticated = 2,
                };
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.NetworkAuthenticationType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                enum NetworkAuthenticationType : int
                {
                    NetworkAuthenticationType_None = 0,
                    NetworkAuthenticationType_Unknown = 1,
                    NetworkAuthenticationType_Open80211 = 2,
                    NetworkAuthenticationType_SharedKey80211 = 3,
                    NetworkAuthenticationType_Wpa = 4,
                    NetworkAuthenticationType_WpaPsk = 5,
                    NetworkAuthenticationType_WpaNone = 6,
                    NetworkAuthenticationType_Rsna = 7,
                    NetworkAuthenticationType_RsnaPsk = 8,
                    NetworkAuthenticationType_Ihv = 9,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
                    NetworkAuthenticationType_Wpa3 = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
                    NetworkAuthenticationType_Wpa3Enterprise192Bits = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
                    NetworkAuthenticationType_Wpa3Sae = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    NetworkAuthenticationType_Owe = 12,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
                    NetworkAuthenticationType_Wpa3Enterprise = 13,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
                };
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.NetworkConnectivityLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                enum NetworkConnectivityLevel : int
                {
                    NetworkConnectivityLevel_None = 0,
                    NetworkConnectivityLevel_LocalAccess = 1,
                    NetworkConnectivityLevel_ConstrainedInternetAccess = 2,
                    NetworkConnectivityLevel_InternetAccess = 3,
                };
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.NetworkCostType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                enum NetworkCostType : int
                {
                    NetworkCostType_Unknown = 0,
                    NetworkCostType_Unrestricted = 1,
                    NetworkCostType_Fixed = 2,
                    NetworkCostType_Variable = 3,
                };
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.NetworkEncryptionType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                enum NetworkEncryptionType : int
                {
                    NetworkEncryptionType_None = 0,
                    NetworkEncryptionType_Unknown = 1,
                    NetworkEncryptionType_Wep = 2,
                    NetworkEncryptionType_Wep40 = 3,
                    NetworkEncryptionType_Wep104 = 4,
                    NetworkEncryptionType_Tkip = 5,
                    NetworkEncryptionType_Ccmp = 6,
                    NetworkEncryptionType_WpaUseGroup = 7,
                    NetworkEncryptionType_RsnUseGroup = 8,
                    NetworkEncryptionType_Ihv = 9,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
                    NetworkEncryptionType_Gcmp = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
                    NetworkEncryptionType_Gcmp256 = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
                };
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.NetworkTypes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                enum NetworkTypes : unsigned int
                {
                    NetworkTypes_None = 0,
                    NetworkTypes_Internet = 0x1,
                    NetworkTypes_PrivateNetwork = 0x2,
                };

                DEFINE_ENUM_FLAG_OPERATORS(NetworkTypes)
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.RoamingStates
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                enum RoamingStates : unsigned int
                {
                    RoamingStates_None = 0,
                    RoamingStates_NotRoaming = 0x1,
                    RoamingStates_Roaming = 0x2,
                };

                DEFINE_ENUM_FLAG_OPERATORS(RoamingStates)
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.TriStates
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                enum TriStates : int
                {
                    TriStates_DoNotCare = 0,
                    TriStates_No = 1,
                    TriStates_Yes = 2,
                };
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.WwanDataClass
 *
 * Introduced to Windows.Networking.Connectivity.WwanContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                enum WwanDataClass : unsigned int
                {
                    WwanDataClass_None = 0,
                    WwanDataClass_Gprs = 0x1,
                    WwanDataClass_Edge = 0x2,
                    WwanDataClass_Umts = 0x4,
                    WwanDataClass_Hsdpa = 0x8,
                    WwanDataClass_Hsupa = 0x10,
                    WwanDataClass_LteAdvanced = 0x20,
#if WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x30000
                    WwanDataClass_NewRadioNonStandalone = 0x40,
#endif // WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x30000
#if WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x30000
                    WwanDataClass_NewRadioStandalone = 0x80,
#endif // WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x30000
                    WwanDataClass_Cdma1xRtt = 0x10000,
                    WwanDataClass_Cdma1xEvdo = 0x20000,
                    WwanDataClass_Cdma1xEvdoRevA = 0x40000,
                    WwanDataClass_Cdma1xEvdv = 0x80000,
                    WwanDataClass_Cdma3xRtt = 0x100000,
                    WwanDataClass_Cdma1xEvdoRevB = 0x200000,
                    WwanDataClass_CdmaUmb = 0x400000,
                    WwanDataClass_Custom = 0x80000000,
                };

                DEFINE_ENUM_FLAG_OPERATORS(WwanDataClass)
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.WwanNetworkIPKind
 *
 * Introduced to Windows.Networking.Connectivity.WwanContract in version 2.0
 *
 */
#if WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                enum WwanNetworkIPKind : int
                {
                    WwanNetworkIPKind_None = 0,
                    WwanNetworkIPKind_Ipv4 = 1,
                    WwanNetworkIPKind_Ipv6 = 2,
                    WwanNetworkIPKind_Ipv4v6 = 3,
                    WwanNetworkIPKind_Ipv4v6v4Xlat = 4,
                };
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Networking.Connectivity.WwanNetworkRegistrationState
 *
 * Introduced to Windows.Networking.Connectivity.WwanContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                enum WwanNetworkRegistrationState : int
                {
                    WwanNetworkRegistrationState_None = 0,
                    WwanNetworkRegistrationState_Deregistered = 1,
                    WwanNetworkRegistrationState_Searching = 2,
                    WwanNetworkRegistrationState_Home = 3,
                    WwanNetworkRegistrationState_Roaming = 4,
                    WwanNetworkRegistrationState_Partner = 5,
                    WwanNetworkRegistrationState_Denied = 6,
                };
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.NetworkUsageStates
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                struct NetworkUsageStates
                {
                    ABI::Windows::Networking::Connectivity::TriStates Roaming;
                    ABI::Windows::Networking::Connectivity::TriStates Shared;
                };
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Networking.Connectivity.NetworkStatusChangedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("71ba143f-598e-49d0-84eb-8febaedcc195")
                INetworkStatusChangedEventHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        IInspectable* sender
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INetworkStatusChangedEventHandler = __uuidof(INetworkStatusChangedEventHandler);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IAttributedNetworkUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.AttributedNetworkUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IAttributedNetworkUsage[] = L"Windows.Networking.Connectivity.IAttributedNetworkUsage";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("f769b039-eca2-45eb-ade1-b0368b756c49")
                IAttributedNetworkUsage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BytesSent(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BytesReceived(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AttributionId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AttributionName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AttributionThumbnail(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAttributedNetworkUsage = __uuidof(IAttributedNetworkUsage);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.ICellularApnContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.CellularApnContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_ICellularApnContext[] = L"Windows.Networking.Connectivity.ICellularApnContext";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("6fa529f4-effd-4542-9ab2-705bbf94943a")
                ICellularApnContext : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProviderId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ProviderId(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AccessPointName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AccessPointName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_UserName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Password(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Password(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsCompressionEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsCompressionEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AuthenticationType(
                        ABI::Windows::Networking::Connectivity::CellularApnAuthenticationType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AuthenticationType(
                        ABI::Windows::Networking::Connectivity::CellularApnAuthenticationType value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICellularApnContext = __uuidof(ICellularApnContext);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.ICellularApnContext2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.CellularApnContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_ICellularApnContext2[] = L"Windows.Networking.Connectivity.ICellularApnContext2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("76b0eb1a-ac49-4350-b1e5-dc4763bc69c7")
                ICellularApnContext2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProfileName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ProfileName(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICellularApnContext2 = __uuidof(ICellularApnContext2);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionCost
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionCost
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionCost[] = L"Windows.Networking.Connectivity.IConnectionCost";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("bad7d829-3416-4b10-a202-bac0b075bdae")
                IConnectionCost : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkCostType(
                        ABI::Windows::Networking::Connectivity::NetworkCostType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Roaming(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OverDataLimit(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ApproachingDataLimit(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConnectionCost = __uuidof(IConnectionCost);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionCost2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionCost
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionCost2[] = L"Windows.Networking.Connectivity.IConnectionCost2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("8e113a05-e209-4549-bb25-5e0db691cb05")
                IConnectionCost2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BackgroundDataUsageRestricted(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConnectionCost2 = __uuidof(IConnectionCost2);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionProfile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionProfile[] = L"Windows.Networking.Connectivity.IConnectionProfile";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("71ba143c-598e-49d0-84eb-8febaedcc195")
                IConnectionProfile : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProfileName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetNetworkConnectivityLevel(
                        ABI::Windows::Networking::Connectivity::NetworkConnectivityLevel* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetNetworkNames(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetConnectionCost(
                        ABI::Windows::Networking::Connectivity::IConnectionCost** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDataPlanStatus(
                        ABI::Windows::Networking::Connectivity::IDataPlanStatus** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkAdapter(
                        ABI::Windows::Networking::Connectivity::INetworkAdapter** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("GetLocalUsage may be altered or unavailable for releases after Windows 8.1. Instead, use GetNetworkUsageAsync.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE GetLocalUsage(
                        ABI::Windows::Foundation::DateTime StartTime,
                        ABI::Windows::Foundation::DateTime EndTime,
                        ABI::Windows::Networking::Connectivity::IDataUsage** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("GetLocalUsage may be altered or unavailable for releases after Windows 8.1. Instead, use GetNetworkUsageAsync.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE GetLocalUsagePerRoamingStates(
                        ABI::Windows::Foundation::DateTime StartTime,
                        ABI::Windows::Foundation::DateTime EndTime,
                        ABI::Windows::Networking::Connectivity::RoamingStates States,
                        ABI::Windows::Networking::Connectivity::IDataUsage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkSecuritySettings(
                        ABI::Windows::Networking::Connectivity::INetworkSecuritySettings** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConnectionProfile = __uuidof(IConnectionProfile);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionProfile2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionProfile2[] = L"Windows.Networking.Connectivity.IConnectionProfile2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("e2045145-4c9f-400c-9150-7ec7d6e2888a")
                IConnectionProfile2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsWwanConnectionProfile(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsWlanConnectionProfile(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WwanConnectionProfileDetails(
                        ABI::Windows::Networking::Connectivity::IWwanConnectionProfileDetails** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WlanConnectionProfileDetails(
                        ABI::Windows::Networking::Connectivity::IWlanConnectionProfileDetails** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceProviderGuid(
                        __FIReference_1_GUID** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSignalBars(
                        __FIReference_1_byte** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDomainConnectivityLevel(
                        ABI::Windows::Networking::Connectivity::DomainConnectivityLevel* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetNetworkUsageAsync(
                        ABI::Windows::Foundation::DateTime startTime,
                        ABI::Windows::Foundation::DateTime endTime,
                        ABI::Windows::Networking::Connectivity::DataUsageGranularity granularity,
                        ABI::Windows::Networking::Connectivity::NetworkUsageStates states,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetConnectivityIntervalsAsync(
                        ABI::Windows::Foundation::DateTime startTime,
                        ABI::Windows::Foundation::DateTime endTime,
                        ABI::Windows::Networking::Connectivity::NetworkUsageStates states,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConnectionProfile2 = __uuidof(IConnectionProfile2);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionProfile3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionProfile3[] = L"Windows.Networking.Connectivity.IConnectionProfile3";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("578c2528-4cd9-4161-8045-201cfd5b115c")
                IConnectionProfile3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAttributedNetworkUsageAsync(
                        ABI::Windows::Foundation::DateTime startTime,
                        ABI::Windows::Foundation::DateTime endTime,
                        ABI::Windows::Networking::Connectivity::NetworkUsageStates states,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConnectionProfile3 = __uuidof(IConnectionProfile3);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionProfile4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionProfile4[] = L"Windows.Networking.Connectivity.IConnectionProfile4";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("7a2d42cd-81e0-4ae6-abed-ab9ca13eb714")
                IConnectionProfile4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetProviderNetworkUsageAsync(
                        ABI::Windows::Foundation::DateTime startTime,
                        ABI::Windows::Foundation::DateTime endTime,
                        ABI::Windows::Networking::Connectivity::NetworkUsageStates states,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConnectionProfile4 = __uuidof(IConnectionProfile4);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionProfile5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionProfile5[] = L"Windows.Networking.Connectivity.IConnectionProfile5";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("85361ec7-9c73-4be0-8f14-578eec71ee0e")
                IConnectionProfile5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CanDelete(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryDeleteAsync(
                        __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConnectionProfile5 = __uuidof(IConnectionProfile5);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionProfile6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionProfile6[] = L"Windows.Networking.Connectivity.IConnectionProfile6";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("dc27dfe2-7a6f-5d0e-9589-2fe2e5b6f9aa")
                IConnectionProfile6 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsDomainAuthenticatedBy(
                        ABI::Windows::Networking::Connectivity::DomainAuthenticationKind kind,
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConnectionProfile6 = __uuidof(IConnectionProfile6);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionProfileFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionProfileFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionProfileFilter[] = L"Windows.Networking.Connectivity.IConnectionProfileFilter";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("204c7cc8-bd2d-4e8d-a4b3-455ec337388a")
                IConnectionProfileFilter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_IsConnected(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsConnected(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsWwanConnectionProfile(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsWwanConnectionProfile(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsWlanConnectionProfile(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsWlanConnectionProfile(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NetworkCostType(
                        ABI::Windows::Networking::Connectivity::NetworkCostType value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkCostType(
                        ABI::Windows::Networking::Connectivity::NetworkCostType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ServiceProviderGuid(
                        __FIReference_1_GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceProviderGuid(
                        __FIReference_1_GUID** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConnectionProfileFilter = __uuidof(IConnectionProfileFilter);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionProfileFilter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionProfileFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionProfileFilter2[] = L"Windows.Networking.Connectivity.IConnectionProfileFilter2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("cd068ee1-c3fc-4fad-9ddc-593faa4b7885")
                IConnectionProfileFilter2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_IsRoaming(
                        __FIReference_1_boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsRoaming(
                        __FIReference_1_boolean** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsOverDataLimit(
                        __FIReference_1_boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsOverDataLimit(
                        __FIReference_1_boolean** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsBackgroundDataUsageRestricted(
                        __FIReference_1_boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsBackgroundDataUsageRestricted(
                        __FIReference_1_boolean** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RawData(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConnectionProfileFilter2 = __uuidof(IConnectionProfileFilter2);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionProfileFilter3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionProfileFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionProfileFilter3[] = L"Windows.Networking.Connectivity.IConnectionProfileFilter3";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("0aaa09c0-5014-447c-8809-aee4cb0af94a")
                IConnectionProfileFilter3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_PurposeGuid(
                        __FIReference_1_GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PurposeGuid(
                        __FIReference_1_GUID** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConnectionProfileFilter3 = __uuidof(IConnectionProfileFilter3);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionSession
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionSession[] = L"Windows.Networking.Connectivity.IConnectionSession";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("ff905d4c-f83b-41b0-8a0c-1462d9c56b73")
                IConnectionSession : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ConnectionProfile(
                        ABI::Windows::Networking::Connectivity::IConnectionProfile** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConnectionSession = __uuidof(IConnectionSession);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectivityInterval
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectivityInterval
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectivityInterval[] = L"Windows.Networking.Connectivity.IConnectivityInterval";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("4faa3fff-6746-4824-a964-eed8e87f8709")
                IConnectivityInterval : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_StartTime(
                        ABI::Windows::Foundation::DateTime* startTime
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ConnectionDuration(
                        ABI::Windows::Foundation::TimeSpan* duration
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConnectivityInterval = __uuidof(IConnectivityInterval);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectivityManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectivityManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectivityManagerStatics[] = L"Windows.Networking.Connectivity.IConnectivityManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("5120d4b1-4fb1-48b0-afc9-42e0092a8164")
                IConnectivityManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE AcquireConnectionAsync(
                        ABI::Windows::Networking::Connectivity::ICellularApnContext* cellularApnContext,
                        __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddHttpRoutePolicy(
                        ABI::Windows::Networking::Connectivity::IRoutePolicy* routePolicy
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveHttpRoutePolicy(
                        ABI::Windows::Networking::Connectivity::IRoutePolicy* routePolicy
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConnectivityManagerStatics = __uuidof(IConnectivityManagerStatics);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IDataPlanStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.DataPlanStatus
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IDataPlanStatus[] = L"Windows.Networking.Connectivity.IDataPlanStatus";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("977a8b8c-3885-40f3-8851-42cd2bd568bb")
                IDataPlanStatus : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DataPlanUsage(
                        ABI::Windows::Networking::Connectivity::IDataPlanUsage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DataLimitInMegabytes(
                        __FIReference_1_UINT32** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InboundBitsPerSecond(
                        __FIReference_1_UINT64** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutboundBitsPerSecond(
                        __FIReference_1_UINT64** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NextBillingCycle(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxTransferSizeInMegabytes(
                        __FIReference_1_UINT32** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPlanStatus = __uuidof(IDataPlanStatus);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IDataPlanUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.DataPlanUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IDataPlanUsage[] = L"Windows.Networking.Connectivity.IDataPlanUsage";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("b921492d-3b44-47ff-b361-be59e69ed1b0")
                IDataPlanUsage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MegabytesUsed(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LastSyncTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPlanUsage = __uuidof(IDataPlanUsage);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IDataUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.DataUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IDataUsage[] = L"Windows.Networking.Connectivity.IDataUsage";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("c1431dd3-b146-4d39-b959-0c69b096c512")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("IDataUsage may be altered or unavailable for releases after Windows 8.1. Instead, use INetworkUsage.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IDataUsage : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("IDataUsage may be altered or unavailable for releases after Windows 8.1. Instead, use INetworkUsage.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_BytesSent(
                        UINT64* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("IDataUsage may be altered or unavailable for releases after Windows 8.1. Instead, use INetworkUsage.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_BytesReceived(
                        UINT64* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataUsage = __uuidof(IDataUsage);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IIPInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.IPInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IIPInformation[] = L"Windows.Networking.Connectivity.IIPInformation";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("d85145e0-138f-47d7-9b3a-36bb488cef33")
                IIPInformation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkAdapter(
                        ABI::Windows::Networking::Connectivity::INetworkAdapter** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PrefixLength(
                        __FIReference_1_byte** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIPInformation = __uuidof(IIPInformation);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.ILanIdentifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.LanIdentifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_ILanIdentifier[] = L"Windows.Networking.Connectivity.ILanIdentifier";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("48aa53aa-1108-4546-a6cb-9a74da4b7ba0")
                ILanIdentifier : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_InfrastructureId(
                        ABI::Windows::Networking::Connectivity::ILanIdentifierData** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PortId(
                        ABI::Windows::Networking::Connectivity::ILanIdentifierData** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkAdapterId(
                        GUID* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILanIdentifier = __uuidof(ILanIdentifier);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.ILanIdentifierData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.LanIdentifierData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_ILanIdentifierData[] = L"Windows.Networking.Connectivity.ILanIdentifierData";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("a74e83c3-d639-45be-a36a-c4e4aeaf6d9b")
                ILanIdentifierData : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Type(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        __FIVectorView_1_byte** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILanIdentifierData = __uuidof(ILanIdentifierData);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.INetworkAdapter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.NetworkAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_INetworkAdapter[] = L"Windows.Networking.Connectivity.INetworkAdapter";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("3b542e03-5388-496c-a8a3-affd39aec2e6")
                INetworkAdapter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_OutboundMaxBitsPerSecond(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InboundMaxBitsPerSecond(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IanaInterfaceType(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkItem(
                        ABI::Windows::Networking::Connectivity::INetworkItem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkAdapterId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetConnectedProfileAsync(
                        __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INetworkAdapter = __uuidof(INetworkAdapter);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.INetworkInformationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.NetworkInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_INetworkInformationStatics[] = L"Windows.Networking.Connectivity.INetworkInformationStatics";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("5074f851-950d-4165-9c15-365619481eea")
                INetworkInformationStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetConnectionProfiles(
                        __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetInternetConnectionProfile(
                        ABI::Windows::Networking::Connectivity::IConnectionProfile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetLanIdentifiers(
                        __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetHostNames(
                        __FIVectorView_1_Windows__CNetworking__CHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetProxyConfigurationAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSortedEndpointPairs(
                        __FIIterable_1_Windows__CNetworking__CEndpointPair* destinationList,
                        ABI::Windows::Networking::HostNameSortOptions sortOptions,
                        __FIVectorView_1_Windows__CNetworking__CEndpointPair** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_NetworkStatusChanged(
                        ABI::Windows::Networking::Connectivity::INetworkStatusChangedEventHandler* networkStatusHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_NetworkStatusChanged(
                        EventRegistrationToken eventCookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INetworkInformationStatics = __uuidof(INetworkInformationStatics);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.INetworkInformationStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.NetworkInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_INetworkInformationStatics2[] = L"Windows.Networking.Connectivity.INetworkInformationStatics2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("459ced14-2832-49b6-ba6e-e265f04786a8")
                INetworkInformationStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FindConnectionProfilesAsync(
                        ABI::Windows::Networking::Connectivity::IConnectionProfileFilter* pProfileFilter,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INetworkInformationStatics2 = __uuidof(INetworkInformationStatics2);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.INetworkItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.NetworkItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_INetworkItem[] = L"Windows.Networking.Connectivity.INetworkItem";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("01bc4d39-f5e0-4567-a28c-42080c831b2b")
                INetworkItem : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetNetworkTypes(
                        ABI::Windows::Networking::Connectivity::NetworkTypes* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INetworkItem = __uuidof(INetworkItem);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.INetworkSecuritySettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.NetworkSecuritySettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_INetworkSecuritySettings[] = L"Windows.Networking.Connectivity.INetworkSecuritySettings";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("7ca07e8d-917b-4b5f-b84d-28f7a5ac5402")
                INetworkSecuritySettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkAuthenticationType(
                        ABI::Windows::Networking::Connectivity::NetworkAuthenticationType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkEncryptionType(
                        ABI::Windows::Networking::Connectivity::NetworkEncryptionType* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INetworkSecuritySettings = __uuidof(INetworkSecuritySettings);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.INetworkStateChangeEventDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.NetworkStateChangeEventDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_INetworkStateChangeEventDetails[] = L"Windows.Networking.Connectivity.INetworkStateChangeEventDetails";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("1f0cf333-d7a6-44dd-a4e9-687c476b903d")
                INetworkStateChangeEventDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_HasNewInternetConnectionProfile(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasNewConnectionCost(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasNewNetworkConnectivityLevel(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasNewDomainConnectivityLevel(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasNewHostNameList(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasNewWwanRegistrationState(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INetworkStateChangeEventDetails = __uuidof(INetworkStateChangeEventDetails);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.INetworkStateChangeEventDetails2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.NetworkStateChangeEventDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_INetworkStateChangeEventDetails2[] = L"Windows.Networking.Connectivity.INetworkStateChangeEventDetails2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("d643c0e8-30d3-4f6a-ad47-6a1873ceb3c1")
                INetworkStateChangeEventDetails2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_HasNewTetheringOperationalState(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasNewTetheringClientCount(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INetworkStateChangeEventDetails2 = __uuidof(INetworkStateChangeEventDetails2);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.INetworkUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.NetworkUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_INetworkUsage[] = L"Windows.Networking.Connectivity.INetworkUsage";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("49da8fce-9985-4927-bf5b-072b5c65f8d9")
                INetworkUsage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BytesSent(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BytesReceived(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ConnectionDuration(
                        ABI::Windows::Foundation::TimeSpan* duration
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INetworkUsage = __uuidof(INetworkUsage);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IProviderNetworkUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ProviderNetworkUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IProviderNetworkUsage[] = L"Windows.Networking.Connectivity.IProviderNetworkUsage";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("5ec69e04-7931-48c8-b8f3-46300fa42728")
                IProviderNetworkUsage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BytesSent(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BytesReceived(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProviderId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProviderNetworkUsage = __uuidof(IProviderNetworkUsage);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Networking.Connectivity.IProxyConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ProxyConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IProxyConfiguration[] = L"Windows.Networking.Connectivity.IProxyConfiguration";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("ef3a60b4-9004-4dd6-b7d8-b3e502f4aad0")
                IProxyConfiguration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProxyUris(
                        __FIVectorView_1_Windows__CFoundation__CUri** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CanConnectDirectly(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProxyConfiguration = __uuidof(IProxyConfiguration);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IRoutePolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.RoutePolicy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IRoutePolicy[] = L"Windows.Networking.Connectivity.IRoutePolicy";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("11abc4ac-0fc7-42e4-8742-569923b1ca11")
                IRoutePolicy : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ConnectionProfile(
                        ABI::Windows::Networking::Connectivity::IConnectionProfile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HostName(
                        ABI::Windows::Networking::IHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HostNameType(
                        ABI::Windows::Networking::DomainNameType* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRoutePolicy = __uuidof(IRoutePolicy);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IRoutePolicyFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.RoutePolicy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IRoutePolicyFactory[] = L"Windows.Networking.Connectivity.IRoutePolicyFactory";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("36027933-a18e-4db5-a697-f58fa7364e44")
                IRoutePolicyFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateRoutePolicy(
                        ABI::Windows::Networking::Connectivity::IConnectionProfile* connectionProfile,
                        ABI::Windows::Networking::IHostName* hostName,
                        ABI::Windows::Networking::DomainNameType type,
                        ABI::Windows::Networking::Connectivity::IRoutePolicy** routePolicy
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRoutePolicyFactory = __uuidof(IRoutePolicyFactory);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IWlanConnectionProfileDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.WlanConnectionProfileDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IWlanConnectionProfileDetails[] = L"Windows.Networking.Connectivity.IWlanConnectionProfileDetails";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("562098cb-b35a-4bf1-a884-b7557e88ff86")
                IWlanConnectionProfileDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetConnectedSsid(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWlanConnectionProfileDetails = __uuidof(IWlanConnectionProfileDetails);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IWwanConnectionProfileDetails
 *
 * Introduced to Windows.Networking.Connectivity.WwanContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.WwanConnectionProfileDetails
 *
 */
#if WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IWwanConnectionProfileDetails[] = L"Windows.Networking.Connectivity.IWwanConnectionProfileDetails";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("0e4da8fe-835f-4df3-82fd-df556ebc09ef")
                IWwanConnectionProfileDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_HomeProviderId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AccessPointName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetNetworkRegistrationState(
                        ABI::Windows::Networking::Connectivity::WwanNetworkRegistrationState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentDataClass(
                        ABI::Windows::Networking::Connectivity::WwanDataClass* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWwanConnectionProfileDetails = __uuidof(IWwanConnectionProfileDetails);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IWwanConnectionProfileDetails2
 *
 * Introduced to Windows.Networking.Connectivity.WwanContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.WwanConnectionProfileDetails
 *
 */
#if WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IWwanConnectionProfileDetails2[] = L"Windows.Networking.Connectivity.IWwanConnectionProfileDetails2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                MIDL_INTERFACE("7a754ede-a1ed-48b2-8e92-b460033d52e2")
                IWwanConnectionProfileDetails2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IPKind(
                        ABI::Windows::Networking::Connectivity::WwanNetworkIPKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PurposeGuids(
                        __FIVectorView_1_GUID** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWwanConnectionProfileDetails2 = __uuidof(IWwanConnectionProfileDetails2);
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Networking.Connectivity.AttributedNetworkUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IAttributedNetworkUsage ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_AttributedNetworkUsage_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_AttributedNetworkUsage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_AttributedNetworkUsage[] = L"Windows.Networking.Connectivity.AttributedNetworkUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.CellularApnContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.ICellularApnContext ** Default Interface **
 *    Windows.Networking.Connectivity.ICellularApnContext2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_CellularApnContext_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_CellularApnContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_CellularApnContext[] = L"Windows.Networking.Connectivity.CellularApnContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.ConnectionCost
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IConnectionCost ** Default Interface **
 *    Windows.Networking.Connectivity.IConnectionCost2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_ConnectionCost_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_ConnectionCost_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_ConnectionCost[] = L"Windows.Networking.Connectivity.ConnectionCost";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.ConnectionProfile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IConnectionProfile ** Default Interface **
 *    Windows.Networking.Connectivity.IConnectionProfile2
 *    Windows.Networking.Connectivity.IConnectionProfile3
 *    Windows.Networking.Connectivity.IConnectionProfile4
 *    Windows.Networking.Connectivity.IConnectionProfile5
 *    Windows.Networking.Connectivity.IConnectionProfile6
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_ConnectionProfile_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_ConnectionProfile_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_ConnectionProfile[] = L"Windows.Networking.Connectivity.ConnectionProfile";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.ConnectionProfileFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IConnectionProfileFilter ** Default Interface **
 *    Windows.Networking.Connectivity.IConnectionProfileFilter2
 *    Windows.Networking.Connectivity.IConnectionProfileFilter3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_ConnectionProfileFilter_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_ConnectionProfileFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_ConnectionProfileFilter[] = L"Windows.Networking.Connectivity.ConnectionProfileFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.ConnectionSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IConnectionSession ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_ConnectionSession_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_ConnectionSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_ConnectionSession[] = L"Windows.Networking.Connectivity.ConnectionSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.ConnectivityInterval
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IConnectivityInterval ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_ConnectivityInterval_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_ConnectivityInterval_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_ConnectivityInterval[] = L"Windows.Networking.Connectivity.ConnectivityInterval";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.ConnectivityManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Connectivity.IConnectivityManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_ConnectivityManager_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_ConnectivityManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_ConnectivityManager[] = L"Windows.Networking.Connectivity.ConnectivityManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.DataPlanStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IDataPlanStatus ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_DataPlanStatus_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_DataPlanStatus_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_DataPlanStatus[] = L"Windows.Networking.Connectivity.DataPlanStatus";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.DataPlanUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IDataPlanUsage ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_DataPlanUsage_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_DataPlanUsage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_DataPlanUsage[] = L"Windows.Networking.Connectivity.DataPlanUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.DataUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IDataUsage ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_DataUsage_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_DataUsage_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("DataUsage may be altered or unavailable for releases after Windows 8.1. Instead, use NetworkUsage.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_DataUsage[] = L"Windows.Networking.Connectivity.DataUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.IPInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IIPInformation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_IPInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_IPInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_IPInformation[] = L"Windows.Networking.Connectivity.IPInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.LanIdentifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.ILanIdentifier ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_LanIdentifier_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_LanIdentifier_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_LanIdentifier[] = L"Windows.Networking.Connectivity.LanIdentifier";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.LanIdentifierData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.ILanIdentifierData ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_LanIdentifierData_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_LanIdentifierData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_LanIdentifierData[] = L"Windows.Networking.Connectivity.LanIdentifierData";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.NetworkAdapter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.INetworkAdapter ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_NetworkAdapter_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_NetworkAdapter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_NetworkAdapter[] = L"Windows.Networking.Connectivity.NetworkAdapter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.NetworkInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Connectivity.INetworkInformationStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.Connectivity.INetworkInformationStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_NetworkInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_NetworkInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_NetworkInformation[] = L"Windows.Networking.Connectivity.NetworkInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.NetworkItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.INetworkItem ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_NetworkItem_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_NetworkItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_NetworkItem[] = L"Windows.Networking.Connectivity.NetworkItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.NetworkSecuritySettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.INetworkSecuritySettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_NetworkSecuritySettings_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_NetworkSecuritySettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_NetworkSecuritySettings[] = L"Windows.Networking.Connectivity.NetworkSecuritySettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.NetworkStateChangeEventDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.INetworkStateChangeEventDetails ** Default Interface **
 *    Windows.Networking.Connectivity.INetworkStateChangeEventDetails2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_NetworkStateChangeEventDetails_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_NetworkStateChangeEventDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_NetworkStateChangeEventDetails[] = L"Windows.Networking.Connectivity.NetworkStateChangeEventDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.NetworkUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.INetworkUsage ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_NetworkUsage_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_NetworkUsage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_NetworkUsage[] = L"Windows.Networking.Connectivity.NetworkUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.ProviderNetworkUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IProviderNetworkUsage ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_ProviderNetworkUsage_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_ProviderNetworkUsage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_ProviderNetworkUsage[] = L"Windows.Networking.Connectivity.ProviderNetworkUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Networking.Connectivity.ProxyConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IProxyConfiguration ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_ProxyConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_ProxyConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_ProxyConfiguration[] = L"Windows.Networking.Connectivity.ProxyConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.RoutePolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Connectivity.IRoutePolicyFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IRoutePolicy ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_RoutePolicy_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_RoutePolicy_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_RoutePolicy[] = L"Windows.Networking.Connectivity.RoutePolicy";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.WlanConnectionProfileDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IWlanConnectionProfileDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_WlanConnectionProfileDetails_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_WlanConnectionProfileDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_WlanConnectionProfileDetails[] = L"Windows.Networking.Connectivity.WlanConnectionProfileDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.WwanConnectionProfileDetails
 *
 * Introduced to Windows.Networking.Connectivity.WwanContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IWwanConnectionProfileDetails ** Default Interface **
 *    Windows.Networking.Connectivity.IWwanConnectionProfileDetails2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_WwanConnectionProfileDetails_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_WwanConnectionProfileDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_WwanConnectionProfileDetails[] = L"Windows.Networking.Connectivity.WwanConnectionProfileDetails";
#endif
#endif // WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2 __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2 __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2 __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3 __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4 __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5 __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6 __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2 __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3 __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage __x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2 __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2 __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration __x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails __x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2 __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage;

typedef struct __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsageVtbl;

interface __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage __FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage;

typedef struct __FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        __FIIterator_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsageVtbl;

interface __FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage;

typedef struct __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsageVtbl;

interface __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        __FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsageVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsageVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile;

typedef struct __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl;

interface __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile;

typedef struct __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectionProfile** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl;

interface __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile;

typedef struct __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl;

interface __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval;

typedef struct __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityIntervalVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityIntervalVtbl;

interface __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityIntervalVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval;

typedef struct __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityIntervalVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        __FIIterator_1_Windows__CNetworking__CConnectivity__CConnectivityInterval** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityIntervalVtbl;

interface __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityIntervalVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval;

typedef struct __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityIntervalVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityIntervalVtbl;

interface __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityIntervalVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityIntervalVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityIntervalVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityIntervalVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityIntervalVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityIntervalVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityIntervalVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage;

typedef struct __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsageVtbl;

interface __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage __FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage;

typedef struct __FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        __FIIterator_1_Windows__CNetworking__CConnectivity__CNetworkUsage** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsageVtbl;

interface __FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CConnectivity__CNetworkUsage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage;

typedef struct __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsageVtbl;

interface __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        __FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsageVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsageVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage;

typedef struct __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsageVtbl;

interface __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage __FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage;

typedef struct __FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        __FIIterator_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsageVtbl;

interface __FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage;

typedef struct __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsageVtbl;

interface __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        __FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsageVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsageVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile;

typedef struct __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl;

interface __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile* This,
        __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CNetworking_CConnectivity_CConnectionProfileDeleteStatus __x_ABI_CWindows_CNetworking_CConnectivity_CConnectionProfileDeleteStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus;

typedef struct __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus* This,
        enum __x_ABI_CWindows_CNetworking_CConnectivity_CConnectionProfileDeleteStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatusVtbl;

interface __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus* This,
        __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession;

typedef struct __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSessionVtbl;

interface __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession* This,
        __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSessionVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CConnectionSession_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration;

typedef struct __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfigurationVtbl;

interface __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration* This,
        __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfigurationVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CConnectivity__CProxyConfiguration_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIIterator_1_GUID_INTERFACE_DEFINED__)
#define ____FIIterator_1_GUID_INTERFACE_DEFINED__

typedef interface __FIIterator_1_GUID __FIIterator_1_GUID;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_GUID;

typedef struct __FIIterator_1_GUIDVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_GUID* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_GUID* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_GUID* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_GUID* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_GUID* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_GUID* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_GUID* This,
        GUID* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_GUID* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_GUID* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_GUID* This,
        UINT32 itemsLength,
        GUID* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_GUIDVtbl;

interface __FIIterator_1_GUID
{
    CONST_VTBL struct __FIIterator_1_GUIDVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_GUID_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_GUID_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_GUID_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_GUID_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_GUID_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_GUID_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_GUID_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_GUID_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_GUID_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_GUID_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_GUID_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_GUID_INTERFACE_DEFINED__)
#define ____FIIterable_1_GUID_INTERFACE_DEFINED__

typedef interface __FIIterable_1_GUID __FIIterable_1_GUID;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_GUID;

typedef struct __FIIterable_1_GUIDVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_GUID* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_GUID* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_GUID* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_GUID* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_GUID* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_GUID* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_GUID* This,
        __FIIterator_1_GUID** result);

    END_INTERFACE
} __FIIterable_1_GUIDVtbl;

interface __FIIterable_1_GUID
{
    CONST_VTBL struct __FIIterable_1_GUIDVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_GUID_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_GUID_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_GUID_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_GUID_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_GUID_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_GUID_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_GUID_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_GUID_INTERFACE_DEFINED__

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

#if !defined(____FIIterator_1_byte_INTERFACE_DEFINED__)
#define ____FIIterator_1_byte_INTERFACE_DEFINED__

typedef interface __FIIterator_1_byte __FIIterator_1_byte;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_byte;

typedef struct __FIIterator_1_byteVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_byte* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_byte* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_byte* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_byte* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_byte* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_byte* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_byte* This,
        BYTE* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_byte* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_byte* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_byte* This,
        UINT32 itemsLength,
        BYTE* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_byteVtbl;

interface __FIIterator_1_byte
{
    CONST_VTBL struct __FIIterator_1_byteVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_byte_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_byte_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_byte_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_byte_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_byte_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_byte_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_byte_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_byte_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_byte_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_byte_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_byte_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_byte_INTERFACE_DEFINED__)
#define ____FIIterable_1_byte_INTERFACE_DEFINED__

typedef interface __FIIterable_1_byte __FIIterable_1_byte;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_byte;

typedef struct __FIIterable_1_byteVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_byte* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_byte* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_byte* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_byte* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_byte* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_byte* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_byte* This,
        __FIIterator_1_byte** result);

    END_INTERFACE
} __FIIterable_1_byteVtbl;

interface __FIIterable_1_byte
{
    CONST_VTBL struct __FIIterable_1_byteVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_byte_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_byte_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_byte_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_byte_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_byte_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_byte_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_byte_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_byte_INTERFACE_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CFoundation__CUri __FIIterator_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CFoundation__CUri;

typedef struct __FIIterator_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CFoundation__CUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CFoundation__CUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CFoundation__CUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CFoundation__CUri* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CFoundation__CUri* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CFoundation__CUri* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CFoundation__CUri* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CFoundation__CUriVtbl;

interface __FIIterator_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIIterator_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CFoundation__CUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CFoundation__CUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CFoundation__CUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CFoundation__CUri_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CFoundation__CUri_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CFoundation__CUri_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CFoundation__CUri_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CFoundation__CUri __FIIterable_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CFoundation__CUri;

typedef struct __FIIterable_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CFoundation__CUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CFoundation__CUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CFoundation__CUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CFoundation__CUri* This,
        __FIIterator_1_Windows__CFoundation__CUri** result);

    END_INTERFACE
} __FIIterable_1_Windows__CFoundation__CUriVtbl;

interface __FIIterable_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIIterable_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CFoundation__CUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CFoundation__CUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CFoundation__CUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CFoundation__CUri_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier;

typedef struct __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifierVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifierVtbl;

interface __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifierVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier __FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier;

typedef struct __FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifierVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        __FIIterator_1_Windows__CNetworking__CConnectivity__CLanIdentifier** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifierVtbl;

interface __FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifierVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CConnectivity__CLanIdentifier_INTERFACE_DEFINED__
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

#ifndef ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CIHostName __x_ABI_CWindows_CNetworking_CIHostName;

#endif // ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CHostName __FIIterator_1_Windows__CNetworking__CHostName;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CHostName;

typedef struct __FIIterator_1_Windows__CNetworking__CHostNameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CHostName* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CHostName* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CHostName* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CHostName* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CHostName* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CHostName* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CHostName* This,
        __x_ABI_CWindows_CNetworking_CIHostName** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CHostName* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CHostName* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CHostName* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CIHostName** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CHostNameVtbl;

interface __FIIterator_1_Windows__CNetworking__CHostName
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CHostNameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CHostName_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CHostName_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CHostName_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CHostName_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CHostName_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CHostName_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CHostName_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CHostName_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CHostName_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CHostName_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CHostName __FIIterable_1_Windows__CNetworking__CHostName;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CHostName;

typedef struct __FIIterable_1_Windows__CNetworking__CHostNameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CHostName* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CHostName* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CHostName* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CHostName* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CHostName* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CHostName* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CHostName* This,
        __FIIterator_1_Windows__CNetworking__CHostName** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CHostNameVtbl;

interface __FIIterable_1_Windows__CNetworking__CHostName
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CHostNameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CHostName_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CHostName_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CHostName_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CHostName_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CHostName_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CHostName_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CHostName_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIVectorView_1_GUID_INTERFACE_DEFINED__)
#define ____FIVectorView_1_GUID_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_GUID __FIVectorView_1_GUID;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_GUID;

typedef struct __FIVectorView_1_GUIDVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_GUID* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_GUID* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_GUID* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_GUID* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_GUID* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_GUID* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_GUID* This,
        UINT32 index,
        GUID* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_GUID* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_GUID* This,
        GUID value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_GUID* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        GUID* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_GUIDVtbl;

interface __FIVectorView_1_GUID
{
    CONST_VTBL struct __FIVectorView_1_GUIDVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_GUID_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_GUID_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_GUID_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_GUID_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_GUID_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_GUID_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_GUID_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_GUID_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_GUID_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_GUID_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_GUID_INTERFACE_DEFINED__

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

#if !defined(____FIVectorView_1_byte_INTERFACE_DEFINED__)
#define ____FIVectorView_1_byte_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_byte __FIVectorView_1_byte;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_byte;

typedef struct __FIVectorView_1_byteVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_byte* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_byte* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_byte* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_byte* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_byte* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_byte* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_byte* This,
        UINT32 index,
        BYTE* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_byte* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_byte* This,
        BYTE value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_byte* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        BYTE* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_byteVtbl;

interface __FIVectorView_1_byte
{
    CONST_VTBL struct __FIVectorView_1_byteVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_byte_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_byte_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_byte_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_byte_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_byte_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_byte_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_byte_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_byte_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_byte_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_byte_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_byte_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CFoundation__CUri __FIVectorView_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CFoundation__CUri;

typedef struct __FIVectorView_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        UINT32 index,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CFoundation__CUriVtbl;

interface __FIVectorView_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIVectorView_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CFoundation__CUri_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CFoundation__CUri_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier;

typedef struct __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifierVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifierVtbl;

interface __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifierVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier_INTERFACE_DEFINED__
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CHostName __FIVectorView_1_Windows__CNetworking__CHostName;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CHostName;

typedef struct __FIVectorView_1_Windows__CNetworking__CHostNameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CHostName* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CHostName* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CHostName* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CHostName* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CHostName* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CHostName* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CHostName* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CIHostName** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CHostName* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CHostName* This,
        __x_ABI_CWindows_CNetworking_CIHostName* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CHostName* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CIHostName** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CHostNameVtbl;

interface __FIVectorView_1_Windows__CNetworking__CHostName
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CHostNameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CHostName_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CHostName_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CHostName_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CHostName_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CHostName_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CHostName_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CHostName_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CHostName_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CHostName_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CHostName_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIReference_1_boolean_INTERFACE_DEFINED__)
#define ____FIReference_1_boolean_INTERFACE_DEFINED__

typedef interface __FIReference_1_boolean __FIReference_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_boolean;

typedef struct __FIReference_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_boolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_boolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_boolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_boolean* This,
        boolean* result);

    END_INTERFACE
} __FIReference_1_booleanVtbl;

interface __FIReference_1_boolean
{
    CONST_VTBL struct __FIReference_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_boolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_boolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_boolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_boolean_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_boolean_INTERFACE_DEFINED__

#if !defined(____FIReference_1_GUID_INTERFACE_DEFINED__)
#define ____FIReference_1_GUID_INTERFACE_DEFINED__

typedef interface __FIReference_1_GUID __FIReference_1_GUID;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_GUID;

typedef struct __FIReference_1_GUIDVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_GUID* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_GUID* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_GUID* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_GUID* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_GUID* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_GUID* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_GUID* This,
        GUID* result);

    END_INTERFACE
} __FIReference_1_GUIDVtbl;

interface __FIReference_1_GUID
{
    CONST_VTBL struct __FIReference_1_GUIDVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_GUID_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_GUID_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_GUID_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_GUID_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_GUID_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_GUID_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_GUID_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_GUID_INTERFACE_DEFINED__

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

#if !defined(____FIReference_1_UINT64_INTERFACE_DEFINED__)
#define ____FIReference_1_UINT64_INTERFACE_DEFINED__

typedef interface __FIReference_1_UINT64 __FIReference_1_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_UINT64;

typedef struct __FIReference_1_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_UINT64* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_UINT64* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_UINT64* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_UINT64* This,
        UINT64* result);

    END_INTERFACE
} __FIReference_1_UINT64Vtbl;

interface __FIReference_1_UINT64
{
    CONST_VTBL struct __FIReference_1_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_UINT64_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_UINT64_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_UINT64_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_UINT64_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_UINT64_INTERFACE_DEFINED__

#if !defined(____FIReference_1_byte_INTERFACE_DEFINED__)
#define ____FIReference_1_byte_INTERFACE_DEFINED__

typedef interface __FIReference_1_byte __FIReference_1_byte;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_byte;

typedef struct __FIReference_1_byteVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_byte* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_byte* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_byte* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_byte* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_byte* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_byte* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_byte* This,
        BYTE* result);

    END_INTERFACE
} __FIReference_1_byteVtbl;

interface __FIReference_1_byte
{
    CONST_VTBL struct __FIReference_1_byteVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_byte_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_byte_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_byte_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_byte_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_byte_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_byte_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_byte_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_byte_INTERFACE_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CFoundation__CDateTime __FIReference_1_Windows__CFoundation__CDateTime;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CFoundation__CDateTime;

typedef struct __FIReference_1_Windows__CFoundation__CDateTimeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CFoundation__CDateTime* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CFoundation__CDateTime* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* result);

    END_INTERFACE
} __FIReference_1_Windows__CFoundation__CDateTimeVtbl;

interface __FIReference_1_Windows__CFoundation__CDateTime
{
    CONST_VTBL struct __FIReference_1_Windows__CFoundation__CDateTimeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CFoundation__CDateTime_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CFoundation__CDateTime_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CFoundation__CDateTime_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CFoundation__CDateTime_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef enum __x_ABI_CWindows_CNetworking_CDomainNameType __x_ABI_CWindows_CNetworking_CDomainNameType;

typedef enum __x_ABI_CWindows_CNetworking_CHostNameSortOptions __x_ABI_CWindows_CNetworking_CHostNameSortOptions;

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CNetworking_CConnectivity_CCellularApnAuthenticationType __x_ABI_CWindows_CNetworking_CConnectivity_CCellularApnAuthenticationType;

typedef enum __x_ABI_CWindows_CNetworking_CConnectivity_CDataUsageGranularity __x_ABI_CWindows_CNetworking_CConnectivity_CDataUsageGranularity;

typedef enum __x_ABI_CWindows_CNetworking_CConnectivity_CDomainAuthenticationKind __x_ABI_CWindows_CNetworking_CConnectivity_CDomainAuthenticationKind;

typedef enum __x_ABI_CWindows_CNetworking_CConnectivity_CDomainConnectivityLevel __x_ABI_CWindows_CNetworking_CConnectivity_CDomainConnectivityLevel;

typedef enum __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkAuthenticationType __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkAuthenticationType;

typedef enum __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkConnectivityLevel __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkConnectivityLevel;

typedef enum __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkCostType __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkCostType;

typedef enum __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkEncryptionType __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkEncryptionType;

typedef enum __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkTypes __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkTypes;

typedef enum __x_ABI_CWindows_CNetworking_CConnectivity_CRoamingStates __x_ABI_CWindows_CNetworking_CConnectivity_CRoamingStates;

typedef enum __x_ABI_CWindows_CNetworking_CConnectivity_CTriStates __x_ABI_CWindows_CNetworking_CConnectivity_CTriStates;

typedef enum __x_ABI_CWindows_CNetworking_CConnectivity_CWwanDataClass __x_ABI_CWindows_CNetworking_CConnectivity_CWwanDataClass;

typedef enum __x_ABI_CWindows_CNetworking_CConnectivity_CWwanNetworkIPKind __x_ABI_CWindows_CNetworking_CConnectivity_CWwanNetworkIPKind;

typedef enum __x_ABI_CWindows_CNetworking_CConnectivity_CWwanNetworkRegistrationState __x_ABI_CWindows_CNetworking_CConnectivity_CWwanNetworkRegistrationState;

typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkUsageStates __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkUsageStates;

/*
 *
 * Struct Windows.Networking.Connectivity.CellularApnAuthenticationType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CConnectivity_CCellularApnAuthenticationType
{
    CellularApnAuthenticationType_None = 0,
    CellularApnAuthenticationType_Pap = 1,
    CellularApnAuthenticationType_Chap = 2,
    CellularApnAuthenticationType_Mschapv2 = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.ConnectionProfileDeleteStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CNetworking_CConnectivity_CConnectionProfileDeleteStatus
{
    ConnectionProfileDeleteStatus_Success = 0,
    ConnectionProfileDeleteStatus_DeniedByUser = 1,
    ConnectionProfileDeleteStatus_DeniedBySystem = 2,
    ConnectionProfileDeleteStatus_UnknownError = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Networking.Connectivity.DataUsageGranularity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CConnectivity_CDataUsageGranularity
{
    DataUsageGranularity_PerMinute = 0,
    DataUsageGranularity_PerHour = 1,
    DataUsageGranularity_PerDay = 2,
    DataUsageGranularity_Total = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.DomainAuthenticationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CNetworking_CConnectivity_CDomainAuthenticationKind
{
    DomainAuthenticationKind_None = 0,
    DomainAuthenticationKind_Ldap = 1,
    DomainAuthenticationKind_Tls = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.Networking.Connectivity.DomainConnectivityLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CConnectivity_CDomainConnectivityLevel
{
    DomainConnectivityLevel_None = 0,
    DomainConnectivityLevel_Unauthenticated = 1,
    DomainConnectivityLevel_Authenticated = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.NetworkAuthenticationType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkAuthenticationType
{
    NetworkAuthenticationType_None = 0,
    NetworkAuthenticationType_Unknown = 1,
    NetworkAuthenticationType_Open80211 = 2,
    NetworkAuthenticationType_SharedKey80211 = 3,
    NetworkAuthenticationType_Wpa = 4,
    NetworkAuthenticationType_WpaPsk = 5,
    NetworkAuthenticationType_WpaNone = 6,
    NetworkAuthenticationType_Rsna = 7,
    NetworkAuthenticationType_RsnaPsk = 8,
    NetworkAuthenticationType_Ihv = 9,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
    NetworkAuthenticationType_Wpa3 = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
    NetworkAuthenticationType_Wpa3Enterprise192Bits = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
    NetworkAuthenticationType_Wpa3Sae = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    NetworkAuthenticationType_Owe = 12,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
    NetworkAuthenticationType_Wpa3Enterprise = 13,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.NetworkConnectivityLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkConnectivityLevel
{
    NetworkConnectivityLevel_None = 0,
    NetworkConnectivityLevel_LocalAccess = 1,
    NetworkConnectivityLevel_ConstrainedInternetAccess = 2,
    NetworkConnectivityLevel_InternetAccess = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.NetworkCostType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkCostType
{
    NetworkCostType_Unknown = 0,
    NetworkCostType_Unrestricted = 1,
    NetworkCostType_Fixed = 2,
    NetworkCostType_Variable = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.NetworkEncryptionType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkEncryptionType
{
    NetworkEncryptionType_None = 0,
    NetworkEncryptionType_Unknown = 1,
    NetworkEncryptionType_Wep = 2,
    NetworkEncryptionType_Wep40 = 3,
    NetworkEncryptionType_Wep104 = 4,
    NetworkEncryptionType_Tkip = 5,
    NetworkEncryptionType_Ccmp = 6,
    NetworkEncryptionType_WpaUseGroup = 7,
    NetworkEncryptionType_RsnUseGroup = 8,
    NetworkEncryptionType_Ihv = 9,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
    NetworkEncryptionType_Gcmp = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
    NetworkEncryptionType_Gcmp256 = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.NetworkTypes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkTypes
{
    NetworkTypes_None = 0,
    NetworkTypes_Internet = 0x1,
    NetworkTypes_PrivateNetwork = 0x2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.RoamingStates
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CConnectivity_CRoamingStates
{
    RoamingStates_None = 0,
    RoamingStates_NotRoaming = 0x1,
    RoamingStates_Roaming = 0x2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.TriStates
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CConnectivity_CTriStates
{
    TriStates_DoNotCare = 0,
    TriStates_No = 1,
    TriStates_Yes = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.WwanDataClass
 *
 * Introduced to Windows.Networking.Connectivity.WwanContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CConnectivity_CWwanDataClass
{
    WwanDataClass_None = 0,
    WwanDataClass_Gprs = 0x1,
    WwanDataClass_Edge = 0x2,
    WwanDataClass_Umts = 0x4,
    WwanDataClass_Hsdpa = 0x8,
    WwanDataClass_Hsupa = 0x10,
    WwanDataClass_LteAdvanced = 0x20,
#if WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x30000
    WwanDataClass_NewRadioNonStandalone = 0x40,
#endif // WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x30000
#if WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x30000
    WwanDataClass_NewRadioStandalone = 0x80,
#endif // WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x30000
    WwanDataClass_Cdma1xRtt = 0x10000,
    WwanDataClass_Cdma1xEvdo = 0x20000,
    WwanDataClass_Cdma1xEvdoRevA = 0x40000,
    WwanDataClass_Cdma1xEvdv = 0x80000,
    WwanDataClass_Cdma3xRtt = 0x100000,
    WwanDataClass_Cdma1xEvdoRevB = 0x200000,
    WwanDataClass_CdmaUmb = 0x400000,
    WwanDataClass_Custom = 0x80000000,
};
#endif // WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.WwanNetworkIPKind
 *
 * Introduced to Windows.Networking.Connectivity.WwanContract in version 2.0
 *
 */
#if WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CNetworking_CConnectivity_CWwanNetworkIPKind
{
    WwanNetworkIPKind_None = 0,
    WwanNetworkIPKind_Ipv4 = 1,
    WwanNetworkIPKind_Ipv6 = 2,
    WwanNetworkIPKind_Ipv4v6 = 3,
    WwanNetworkIPKind_Ipv4v6v4Xlat = 4,
};
#endif // WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Networking.Connectivity.WwanNetworkRegistrationState
 *
 * Introduced to Windows.Networking.Connectivity.WwanContract in version 1.0
 *
 */
#if WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CConnectivity_CWwanNetworkRegistrationState
{
    WwanNetworkRegistrationState_None = 0,
    WwanNetworkRegistrationState_Deregistered = 1,
    WwanNetworkRegistrationState_Searching = 2,
    WwanNetworkRegistrationState_Home = 3,
    WwanNetworkRegistrationState_Roaming = 4,
    WwanNetworkRegistrationState_Partner = 5,
    WwanNetworkRegistrationState_Denied = 6,
};
#endif // WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Connectivity.NetworkUsageStates
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkUsageStates
{
    enum __x_ABI_CWindows_CNetworking_CConnectivity_CTriStates Roaming;
    enum __x_ABI_CWindows_CNetworking_CConnectivity_CTriStates Shared;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Networking.Connectivity.NetworkStatusChangedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler* This,
        IInspectable* sender);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandlerVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler_Invoke(This, sender) \
    ((This)->lpVtbl->Invoke(This, sender))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IAttributedNetworkUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.AttributedNetworkUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IAttributedNetworkUsage[] = L"Windows.Networking.Connectivity.IAttributedNetworkUsage";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BytesSent)(__x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_BytesReceived)(__x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_AttributionId)(__x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AttributionName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AttributionThumbnail)(__x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsageVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_get_BytesSent(This, value) \
    ((This)->lpVtbl->get_BytesSent(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_get_BytesReceived(This, value) \
    ((This)->lpVtbl->get_BytesReceived(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_get_AttributionId(This, value) \
    ((This)->lpVtbl->get_AttributionId(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_get_AttributionName(This, value) \
    ((This)->lpVtbl->get_AttributionName(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_get_AttributionThumbnail(This, value) \
    ((This)->lpVtbl->get_AttributionThumbnail(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIAttributedNetworkUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.ICellularApnContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.CellularApnContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_ICellularApnContext[] = L"Windows.Networking.Connectivity.ICellularApnContext";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProviderId)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ProviderId)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_AccessPointName)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_AccessPointName)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_UserName)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_UserName)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Password)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Password)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_IsCompressionEnabled)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsCompressionEnabled)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AuthenticationType)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* This,
        enum __x_ABI_CWindows_CNetworking_CConnectivity_CCellularApnAuthenticationType* value);
    HRESULT (STDMETHODCALLTYPE* put_AuthenticationType)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* This,
        enum __x_ABI_CWindows_CNetworking_CConnectivity_CCellularApnAuthenticationType value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContextVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_get_ProviderId(This, value) \
    ((This)->lpVtbl->get_ProviderId(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_put_ProviderId(This, value) \
    ((This)->lpVtbl->put_ProviderId(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_get_AccessPointName(This, value) \
    ((This)->lpVtbl->get_AccessPointName(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_put_AccessPointName(This, value) \
    ((This)->lpVtbl->put_AccessPointName(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_get_UserName(This, value) \
    ((This)->lpVtbl->get_UserName(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_put_UserName(This, value) \
    ((This)->lpVtbl->put_UserName(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_get_Password(This, value) \
    ((This)->lpVtbl->get_Password(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_put_Password(This, value) \
    ((This)->lpVtbl->put_Password(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_get_IsCompressionEnabled(This, value) \
    ((This)->lpVtbl->get_IsCompressionEnabled(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_put_IsCompressionEnabled(This, value) \
    ((This)->lpVtbl->put_IsCompressionEnabled(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_get_AuthenticationType(This, value) \
    ((This)->lpVtbl->get_AuthenticationType(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_put_AuthenticationType(This, value) \
    ((This)->lpVtbl->put_AuthenticationType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.ICellularApnContext2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.CellularApnContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_ICellularApnContext2[] = L"Windows.Networking.Connectivity.ICellularApnContext2";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProfileName)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ProfileName)(__x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2Vtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_get_ProfileName(This, value) \
    ((This)->lpVtbl->get_ProfileName(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_put_ProfileName(This, value) \
    ((This)->lpVtbl->put_ProfileName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionCost
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionCost
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionCost[] = L"Windows.Networking.Connectivity.IConnectionCost";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCostVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NetworkCostType)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost* This,
        enum __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkCostType* value);
    HRESULT (STDMETHODCALLTYPE* get_Roaming)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_OverDataLimit)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ApproachingDataLimit)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCostVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCostVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_get_NetworkCostType(This, value) \
    ((This)->lpVtbl->get_NetworkCostType(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_get_Roaming(This, value) \
    ((This)->lpVtbl->get_Roaming(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_get_OverDataLimit(This, value) \
    ((This)->lpVtbl->get_OverDataLimit(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_get_ApproachingDataLimit(This, value) \
    ((This)->lpVtbl->get_ApproachingDataLimit(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionCost2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionCost
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionCost2[] = L"Windows.Networking.Connectivity.IConnectionCost2";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BackgroundDataUsageRestricted)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2Vtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_get_BackgroundDataUsageRestricted(This, value) \
    ((This)->lpVtbl->get_BackgroundDataUsageRestricted(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionProfile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionProfile[] = L"Windows.Networking.Connectivity.IConnectionProfile";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProfileName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetNetworkConnectivityLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile* This,
        enum __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkConnectivityLevel* value);
    HRESULT (STDMETHODCALLTYPE* GetNetworkNames)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* GetConnectionCost)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionCost** value);
    HRESULT (STDMETHODCALLTYPE* GetDataPlanStatus)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus** value);
    HRESULT (STDMETHODCALLTYPE* get_NetworkAdapter)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("GetLocalUsage may be altered or unavailable for releases after Windows 8.1. Instead, use GetNetworkUsageAsync.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* GetLocalUsage)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime StartTime,
        struct __x_ABI_CWindows_CFoundation_CDateTime EndTime,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("GetLocalUsage may be altered or unavailable for releases after Windows 8.1. Instead, use GetNetworkUsageAsync.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* GetLocalUsagePerRoamingStates)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime StartTime,
        struct __x_ABI_CWindows_CFoundation_CDateTime EndTime,
        enum __x_ABI_CWindows_CNetworking_CConnectivity_CRoamingStates States,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage** value);
    HRESULT (STDMETHODCALLTYPE* get_NetworkSecuritySettings)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_get_ProfileName(This, value) \
    ((This)->lpVtbl->get_ProfileName(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_GetNetworkConnectivityLevel(This, value) \
    ((This)->lpVtbl->GetNetworkConnectivityLevel(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_GetNetworkNames(This, value) \
    ((This)->lpVtbl->GetNetworkNames(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_GetConnectionCost(This, value) \
    ((This)->lpVtbl->GetConnectionCost(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_GetDataPlanStatus(This, value) \
    ((This)->lpVtbl->GetDataPlanStatus(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_get_NetworkAdapter(This, value) \
    ((This)->lpVtbl->get_NetworkAdapter(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("GetLocalUsage may be altered or unavailable for releases after Windows 8.1. Instead, use GetNetworkUsageAsync.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_GetLocalUsage(This, StartTime, EndTime, value) \
    ((This)->lpVtbl->GetLocalUsage(This, StartTime, EndTime, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("GetLocalUsage may be altered or unavailable for releases after Windows 8.1. Instead, use GetNetworkUsageAsync.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_GetLocalUsagePerRoamingStates(This, StartTime, EndTime, States, value) \
    ((This)->lpVtbl->GetLocalUsagePerRoamingStates(This, StartTime, EndTime, States, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_get_NetworkSecuritySettings(This, value) \
    ((This)->lpVtbl->get_NetworkSecuritySettings(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionProfile2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionProfile2[] = L"Windows.Networking.Connectivity.IConnectionProfile2";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsWwanConnectionProfile)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsWlanConnectionProfile)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_WwanConnectionProfileDetails)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails** value);
    HRESULT (STDMETHODCALLTYPE* get_WlanConnectionProfileDetails)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails** value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceProviderGuid)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2* This,
        __FIReference_1_GUID** value);
    HRESULT (STDMETHODCALLTYPE* GetSignalBars)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2* This,
        __FIReference_1_byte** value);
    HRESULT (STDMETHODCALLTYPE* GetDomainConnectivityLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2* This,
        enum __x_ABI_CWindows_CNetworking_CConnectivity_CDomainConnectivityLevel* value);
    HRESULT (STDMETHODCALLTYPE* GetNetworkUsageAsync)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime startTime,
        struct __x_ABI_CWindows_CFoundation_CDateTime endTime,
        enum __x_ABI_CWindows_CNetworking_CConnectivity_CDataUsageGranularity granularity,
        struct __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkUsageStates states,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CNetworkUsage** value);
    HRESULT (STDMETHODCALLTYPE* GetConnectivityIntervalsAsync)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime startTime,
        struct __x_ABI_CWindows_CFoundation_CDateTime endTime,
        struct __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkUsageStates states,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectivityInterval** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2Vtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_get_IsWwanConnectionProfile(This, value) \
    ((This)->lpVtbl->get_IsWwanConnectionProfile(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_get_IsWlanConnectionProfile(This, value) \
    ((This)->lpVtbl->get_IsWlanConnectionProfile(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_get_WwanConnectionProfileDetails(This, value) \
    ((This)->lpVtbl->get_WwanConnectionProfileDetails(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_get_WlanConnectionProfileDetails(This, value) \
    ((This)->lpVtbl->get_WlanConnectionProfileDetails(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_get_ServiceProviderGuid(This, value) \
    ((This)->lpVtbl->get_ServiceProviderGuid(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_GetSignalBars(This, value) \
    ((This)->lpVtbl->GetSignalBars(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_GetDomainConnectivityLevel(This, value) \
    ((This)->lpVtbl->GetDomainConnectivityLevel(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_GetNetworkUsageAsync(This, startTime, endTime, granularity, states, value) \
    ((This)->lpVtbl->GetNetworkUsageAsync(This, startTime, endTime, granularity, states, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_GetConnectivityIntervalsAsync(This, startTime, endTime, states, value) \
    ((This)->lpVtbl->GetConnectivityIntervalsAsync(This, startTime, endTime, states, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionProfile3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionProfile3[] = L"Windows.Networking.Connectivity.IConnectionProfile3";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAttributedNetworkUsageAsync)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime startTime,
        struct __x_ABI_CWindows_CFoundation_CDateTime endTime,
        struct __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkUsageStates states,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CAttributedNetworkUsage** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3Vtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_GetAttributedNetworkUsageAsync(This, startTime, endTime, states, value) \
    ((This)->lpVtbl->GetAttributedNetworkUsageAsync(This, startTime, endTime, states, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionProfile4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionProfile4[] = L"Windows.Networking.Connectivity.IConnectionProfile4";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetProviderNetworkUsageAsync)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime startTime,
        struct __x_ABI_CWindows_CFoundation_CDateTime endTime,
        struct __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkUsageStates states,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CProviderNetworkUsage** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4Vtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_GetProviderNetworkUsageAsync(This, startTime, endTime, states, value) \
    ((This)->lpVtbl->GetProviderNetworkUsageAsync(This, startTime, endTime, states, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionProfile5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionProfile5[] = L"Windows.Networking.Connectivity.IConnectionProfile5";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CanDelete)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* TryDeleteAsync)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5* This,
        __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfileDeleteStatus** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5Vtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_get_CanDelete(This, value) \
    ((This)->lpVtbl->get_CanDelete(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_TryDeleteAsync(This, operation) \
    ((This)->lpVtbl->TryDeleteAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionProfile6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionProfile6[] = L"Windows.Networking.Connectivity.IConnectionProfile6";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsDomainAuthenticatedBy)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6* This,
        enum __x_ABI_CWindows_CNetworking_CConnectivity_CDomainAuthenticationKind kind,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6Vtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_IsDomainAuthenticatedBy(This, kind, result) \
    ((This)->lpVtbl->IsDomainAuthenticatedBy(This, kind, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionProfileFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionProfileFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionProfileFilter[] = L"Windows.Networking.Connectivity.IConnectionProfileFilter";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_IsConnected)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsConnected)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsWwanConnectionProfile)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsWwanConnectionProfile)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsWlanConnectionProfile)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsWlanConnectionProfile)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_NetworkCostType)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter* This,
        enum __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkCostType value);
    HRESULT (STDMETHODCALLTYPE* get_NetworkCostType)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter* This,
        enum __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkCostType* value);
    HRESULT (STDMETHODCALLTYPE* put_ServiceProviderGuid)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter* This,
        __FIReference_1_GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceProviderGuid)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter* This,
        __FIReference_1_GUID** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilterVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_put_IsConnected(This, value) \
    ((This)->lpVtbl->put_IsConnected(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_get_IsConnected(This, value) \
    ((This)->lpVtbl->get_IsConnected(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_put_IsWwanConnectionProfile(This, value) \
    ((This)->lpVtbl->put_IsWwanConnectionProfile(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_get_IsWwanConnectionProfile(This, value) \
    ((This)->lpVtbl->get_IsWwanConnectionProfile(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_put_IsWlanConnectionProfile(This, value) \
    ((This)->lpVtbl->put_IsWlanConnectionProfile(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_get_IsWlanConnectionProfile(This, value) \
    ((This)->lpVtbl->get_IsWlanConnectionProfile(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_put_NetworkCostType(This, value) \
    ((This)->lpVtbl->put_NetworkCostType(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_get_NetworkCostType(This, value) \
    ((This)->lpVtbl->get_NetworkCostType(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_put_ServiceProviderGuid(This, value) \
    ((This)->lpVtbl->put_ServiceProviderGuid(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_get_ServiceProviderGuid(This, value) \
    ((This)->lpVtbl->get_ServiceProviderGuid(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionProfileFilter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionProfileFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionProfileFilter2[] = L"Windows.Networking.Connectivity.IConnectionProfileFilter2";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_IsRoaming)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2* This,
        __FIReference_1_boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsRoaming)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2* This,
        __FIReference_1_boolean** value);
    HRESULT (STDMETHODCALLTYPE* put_IsOverDataLimit)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2* This,
        __FIReference_1_boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsOverDataLimit)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2* This,
        __FIReference_1_boolean** value);
    HRESULT (STDMETHODCALLTYPE* put_IsBackgroundDataUsageRestricted)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2* This,
        __FIReference_1_boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsBackgroundDataUsageRestricted)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2* This,
        __FIReference_1_boolean** value);
    HRESULT (STDMETHODCALLTYPE* get_RawData)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2Vtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_put_IsRoaming(This, value) \
    ((This)->lpVtbl->put_IsRoaming(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_get_IsRoaming(This, value) \
    ((This)->lpVtbl->get_IsRoaming(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_put_IsOverDataLimit(This, value) \
    ((This)->lpVtbl->put_IsOverDataLimit(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_get_IsOverDataLimit(This, value) \
    ((This)->lpVtbl->get_IsOverDataLimit(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_put_IsBackgroundDataUsageRestricted(This, value) \
    ((This)->lpVtbl->put_IsBackgroundDataUsageRestricted(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_get_IsBackgroundDataUsageRestricted(This, value) \
    ((This)->lpVtbl->get_IsBackgroundDataUsageRestricted(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_get_RawData(This, value) \
    ((This)->lpVtbl->get_RawData(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionProfileFilter3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionProfileFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionProfileFilter3[] = L"Windows.Networking.Connectivity.IConnectionProfileFilter3";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_PurposeGuid)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3* This,
        __FIReference_1_GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_PurposeGuid)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3* This,
        __FIReference_1_GUID** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3Vtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_put_PurposeGuid(This, value) \
    ((This)->lpVtbl->put_PurposeGuid(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_get_PurposeGuid(This, value) \
    ((This)->lpVtbl->get_PurposeGuid(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectionSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectionSession
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectionSession[] = L"Windows.Networking.Connectivity.IConnectionSession";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionProfile)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSessionVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_get_ConnectionProfile(This, value) \
    ((This)->lpVtbl->get_ConnectionProfile(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectivityInterval
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectivityInterval
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectivityInterval[] = L"Windows.Networking.Connectivity.IConnectivityInterval";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityIntervalVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_StartTime)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* startTime);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionDuration)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* duration);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityIntervalVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityIntervalVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_get_StartTime(This, startTime) \
    ((This)->lpVtbl->get_StartTime(This, startTime))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_get_ConnectionDuration(This, duration) \
    ((This)->lpVtbl->get_ConnectionDuration(This, duration))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityInterval_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IConnectivityManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ConnectivityManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IConnectivityManagerStatics[] = L"Windows.Networking.Connectivity.IConnectivityManagerStatics";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AcquireConnectionAsync)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CICellularApnContext* cellularApnContext,
        __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionSession** operation);
    HRESULT (STDMETHODCALLTYPE* AddHttpRoutePolicy)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy* routePolicy);
    HRESULT (STDMETHODCALLTYPE* RemoveHttpRoutePolicy)(__x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy* routePolicy);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStaticsVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_AcquireConnectionAsync(This, cellularApnContext, operation) \
    ((This)->lpVtbl->AcquireConnectionAsync(This, cellularApnContext, operation))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_AddHttpRoutePolicy(This, routePolicy) \
    ((This)->lpVtbl->AddHttpRoutePolicy(This, routePolicy))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_RemoveHttpRoutePolicy(This, routePolicy) \
    ((This)->lpVtbl->RemoveHttpRoutePolicy(This, routePolicy))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIConnectivityManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IDataPlanStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.DataPlanStatus
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IDataPlanStatus[] = L"Windows.Networking.Connectivity.IDataPlanStatus";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DataPlanUsage)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage** value);
    HRESULT (STDMETHODCALLTYPE* get_DataLimitInMegabytes)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus* This,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* get_InboundBitsPerSecond)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus* This,
        __FIReference_1_UINT64** value);
    HRESULT (STDMETHODCALLTYPE* get_OutboundBitsPerSecond)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus* This,
        __FIReference_1_UINT64** value);
    HRESULT (STDMETHODCALLTYPE* get_NextBillingCycle)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_MaxTransferSizeInMegabytes)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus* This,
        __FIReference_1_UINT32** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatusVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_get_DataPlanUsage(This, value) \
    ((This)->lpVtbl->get_DataPlanUsage(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_get_DataLimitInMegabytes(This, value) \
    ((This)->lpVtbl->get_DataLimitInMegabytes(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_get_InboundBitsPerSecond(This, value) \
    ((This)->lpVtbl->get_InboundBitsPerSecond(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_get_OutboundBitsPerSecond(This, value) \
    ((This)->lpVtbl->get_OutboundBitsPerSecond(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_get_NextBillingCycle(This, value) \
    ((This)->lpVtbl->get_NextBillingCycle(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_get_MaxTransferSizeInMegabytes(This, value) \
    ((This)->lpVtbl->get_MaxTransferSizeInMegabytes(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanStatus_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IDataPlanUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.DataPlanUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IDataPlanUsage[] = L"Windows.Networking.Connectivity.IDataPlanUsage";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MegabytesUsed)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_LastSyncTime)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsageVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_get_MegabytesUsed(This, value) \
    ((This)->lpVtbl->get_MegabytesUsed(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_get_LastSyncTime(This, value) \
    ((This)->lpVtbl->get_LastSyncTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIDataPlanUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IDataUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.DataUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IDataUsage[] = L"Windows.Networking.Connectivity.IDataUsage";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("IDataUsage may be altered or unavailable for releases after Windows 8.1. Instead, use INetworkUsage.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("IDataUsage may be altered or unavailable for releases after Windows 8.1. Instead, use INetworkUsage.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_BytesSent)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage* This,
        UINT64* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("IDataUsage may be altered or unavailable for releases after Windows 8.1. Instead, use INetworkUsage.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_BytesReceived)(__x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage* This,
        UINT64* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsageVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("IDataUsage may be altered or unavailable for releases after Windows 8.1. Instead, use INetworkUsage.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_get_BytesSent(This, value) \
    ((This)->lpVtbl->get_BytesSent(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("IDataUsage may be altered or unavailable for releases after Windows 8.1. Instead, use INetworkUsage.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_get_BytesReceived(This, value) \
    ((This)->lpVtbl->get_BytesReceived(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIDataUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IIPInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.IPInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IIPInformation[] = L"Windows.Networking.Connectivity.IIPInformation";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NetworkAdapter)(__x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter** value);
    HRESULT (STDMETHODCALLTYPE* get_PrefixLength)(__x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation* This,
        __FIReference_1_byte** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformationVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_get_NetworkAdapter(This, value) \
    ((This)->lpVtbl->get_NetworkAdapter(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_get_PrefixLength(This, value) \
    ((This)->lpVtbl->get_PrefixLength(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.ILanIdentifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.LanIdentifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_ILanIdentifier[] = L"Windows.Networking.Connectivity.ILanIdentifier";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InfrastructureId)(__x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData** value);
    HRESULT (STDMETHODCALLTYPE* get_PortId)(__x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData** value);
    HRESULT (STDMETHODCALLTYPE* get_NetworkAdapterId)(__x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier* This,
        GUID* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_get_InfrastructureId(This, value) \
    ((This)->lpVtbl->get_InfrastructureId(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_get_PortId(This, value) \
    ((This)->lpVtbl->get_PortId(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_get_NetworkAdapterId(This, value) \
    ((This)->lpVtbl->get_NetworkAdapterId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifier_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.ILanIdentifierData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.LanIdentifierData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_ILanIdentifierData[] = L"Windows.Networking.Connectivity.ILanIdentifierData";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData* This,
        __FIVectorView_1_byte** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierDataVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CILanIdentifierData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.INetworkAdapter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.NetworkAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_INetworkAdapter[] = L"Windows.Networking.Connectivity.INetworkAdapter";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OutboundMaxBitsPerSecond)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_InboundMaxBitsPerSecond)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_IanaInterfaceType)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_NetworkItem)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem** value);
    HRESULT (STDMETHODCALLTYPE* get_NetworkAdapterId)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* GetConnectedProfileAsync)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter* This,
        __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CConnectionProfile** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapterVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_get_OutboundMaxBitsPerSecond(This, value) \
    ((This)->lpVtbl->get_OutboundMaxBitsPerSecond(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_get_InboundMaxBitsPerSecond(This, value) \
    ((This)->lpVtbl->get_InboundMaxBitsPerSecond(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_get_IanaInterfaceType(This, value) \
    ((This)->lpVtbl->get_IanaInterfaceType(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_get_NetworkItem(This, value) \
    ((This)->lpVtbl->get_NetworkItem(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_get_NetworkAdapterId(This, value) \
    ((This)->lpVtbl->get_NetworkAdapterId(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_GetConnectedProfileAsync(This, value) \
    ((This)->lpVtbl->GetConnectedProfileAsync(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.INetworkInformationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.NetworkInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_INetworkInformationStatics[] = L"Windows.Networking.Connectivity.INetworkInformationStatics";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetConnectionProfiles)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics* This,
        __FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile** value);
    HRESULT (STDMETHODCALLTYPE* GetInternetConnectionProfile)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile** value);
    HRESULT (STDMETHODCALLTYPE* GetLanIdentifiers)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics* This,
        __FIVectorView_1_Windows__CNetworking__CConnectivity__CLanIdentifier** value);
    HRESULT (STDMETHODCALLTYPE* GetHostNames)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics* This,
        __FIVectorView_1_Windows__CNetworking__CHostName** value);
    HRESULT (STDMETHODCALLTYPE* GetProxyConfigurationAsync)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIAsyncOperation_1_Windows__CNetworking__CConnectivity__CProxyConfiguration** value);
    HRESULT (STDMETHODCALLTYPE* GetSortedEndpointPairs)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics* This,
        __FIIterable_1_Windows__CNetworking__CEndpointPair* destinationList,
        enum __x_ABI_CWindows_CNetworking_CHostNameSortOptions sortOptions,
        __FIVectorView_1_Windows__CNetworking__CEndpointPair** value);
    HRESULT (STDMETHODCALLTYPE* add_NetworkStatusChanged)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStatusChangedEventHandler* networkStatusHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_NetworkStatusChanged)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStaticsVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_GetConnectionProfiles(This, value) \
    ((This)->lpVtbl->GetConnectionProfiles(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_GetInternetConnectionProfile(This, value) \
    ((This)->lpVtbl->GetInternetConnectionProfile(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_GetLanIdentifiers(This, value) \
    ((This)->lpVtbl->GetLanIdentifiers(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_GetHostNames(This, value) \
    ((This)->lpVtbl->GetHostNames(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_GetProxyConfigurationAsync(This, uri, value) \
    ((This)->lpVtbl->GetProxyConfigurationAsync(This, uri, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_GetSortedEndpointPairs(This, destinationList, sortOptions, value) \
    ((This)->lpVtbl->GetSortedEndpointPairs(This, destinationList, sortOptions, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_add_NetworkStatusChanged(This, networkStatusHandler, eventCookie) \
    ((This)->lpVtbl->add_NetworkStatusChanged(This, networkStatusHandler, eventCookie))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_remove_NetworkStatusChanged(This, eventCookie) \
    ((This)->lpVtbl->remove_NetworkStatusChanged(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.INetworkInformationStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.NetworkInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_INetworkInformationStatics2[] = L"Windows.Networking.Connectivity.INetworkInformationStatics2";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindConnectionProfilesAsync)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfileFilter* pProfileFilter,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CConnectivity__CConnectionProfile** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2Vtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_FindConnectionProfilesAsync(This, pProfileFilter, value) \
    ((This)->lpVtbl->FindConnectionProfilesAsync(This, pProfileFilter, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkInformationStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.INetworkItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.NetworkItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_INetworkItem[] = L"Windows.Networking.Connectivity.INetworkItem";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NetworkId)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* GetNetworkTypes)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem* This,
        enum __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkTypes* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItemVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_get_NetworkId(This, value) \
    ((This)->lpVtbl->get_NetworkId(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_GetNetworkTypes(This, value) \
    ((This)->lpVtbl->GetNetworkTypes(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.INetworkSecuritySettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.NetworkSecuritySettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_INetworkSecuritySettings[] = L"Windows.Networking.Connectivity.INetworkSecuritySettings";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NetworkAuthenticationType)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings* This,
        enum __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkAuthenticationType* value);
    HRESULT (STDMETHODCALLTYPE* get_NetworkEncryptionType)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings* This,
        enum __x_ABI_CWindows_CNetworking_CConnectivity_CNetworkEncryptionType* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettingsVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_get_NetworkAuthenticationType(This, value) \
    ((This)->lpVtbl->get_NetworkAuthenticationType(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_get_NetworkEncryptionType(This, value) \
    ((This)->lpVtbl->get_NetworkEncryptionType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkSecuritySettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.INetworkStateChangeEventDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.NetworkStateChangeEventDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_INetworkStateChangeEventDetails[] = L"Windows.Networking.Connectivity.INetworkStateChangeEventDetails";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HasNewInternetConnectionProfile)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_HasNewConnectionCost)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_HasNewNetworkConnectivityLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_HasNewDomainConnectivityLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_HasNewHostNameList)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_HasNewWwanRegistrationState)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetailsVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_get_HasNewInternetConnectionProfile(This, value) \
    ((This)->lpVtbl->get_HasNewInternetConnectionProfile(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_get_HasNewConnectionCost(This, value) \
    ((This)->lpVtbl->get_HasNewConnectionCost(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_get_HasNewNetworkConnectivityLevel(This, value) \
    ((This)->lpVtbl->get_HasNewNetworkConnectivityLevel(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_get_HasNewDomainConnectivityLevel(This, value) \
    ((This)->lpVtbl->get_HasNewDomainConnectivityLevel(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_get_HasNewHostNameList(This, value) \
    ((This)->lpVtbl->get_HasNewHostNameList(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_get_HasNewWwanRegistrationState(This, value) \
    ((This)->lpVtbl->get_HasNewWwanRegistrationState(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.INetworkStateChangeEventDetails2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.NetworkStateChangeEventDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_INetworkStateChangeEventDetails2[] = L"Windows.Networking.Connectivity.INetworkStateChangeEventDetails2";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HasNewTetheringOperationalState)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_HasNewTetheringClientCount)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2Vtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_get_HasNewTetheringOperationalState(This, value) \
    ((This)->lpVtbl->get_HasNewTetheringOperationalState(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_get_HasNewTetheringClientCount(This, value) \
    ((This)->lpVtbl->get_HasNewTetheringClientCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkStateChangeEventDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.INetworkUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.NetworkUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_INetworkUsage[] = L"Windows.Networking.Connectivity.INetworkUsage";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BytesSent)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_BytesReceived)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionDuration)(__x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* duration);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsageVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_get_BytesSent(This, value) \
    ((This)->lpVtbl->get_BytesSent(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_get_BytesReceived(This, value) \
    ((This)->lpVtbl->get_BytesReceived(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_get_ConnectionDuration(This, duration) \
    ((This)->lpVtbl->get_ConnectionDuration(This, duration))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IProviderNetworkUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ProviderNetworkUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IProviderNetworkUsage[] = L"Windows.Networking.Connectivity.IProviderNetworkUsage";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BytesSent)(__x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_BytesReceived)(__x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_ProviderId)(__x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsageVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_get_BytesSent(This, value) \
    ((This)->lpVtbl->get_BytesSent(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_get_BytesReceived(This, value) \
    ((This)->lpVtbl->get_BytesReceived(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_get_ProviderId(This, value) \
    ((This)->lpVtbl->get_ProviderId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIProviderNetworkUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Networking.Connectivity.IProxyConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.ProxyConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IProxyConfiguration[] = L"Windows.Networking.Connectivity.IProxyConfiguration";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProxyUris)(__x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration* This,
        __FIVectorView_1_Windows__CFoundation__CUri** value);
    HRESULT (STDMETHODCALLTYPE* get_CanConnectDirectly)(__x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfigurationVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_get_ProxyUris(This, value) \
    ((This)->lpVtbl->get_ProxyUris(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_get_CanConnectDirectly(This, value) \
    ((This)->lpVtbl->get_CanConnectDirectly(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIProxyConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IRoutePolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.RoutePolicy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IRoutePolicy[] = L"Windows.Networking.Connectivity.IRoutePolicy";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionProfile)(__x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile** value);
    HRESULT (STDMETHODCALLTYPE* get_HostName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* get_HostNameType)(__x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy* This,
        enum __x_ABI_CWindows_CNetworking_CDomainNameType* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_get_ConnectionProfile(This, value) \
    ((This)->lpVtbl->get_ConnectionProfile(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_get_HostName(This, value) \
    ((This)->lpVtbl->get_HostName(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_get_HostNameType(This, value) \
    ((This)->lpVtbl->get_HostNameType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IRoutePolicyFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.RoutePolicy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IRoutePolicyFactory[] = L"Windows.Networking.Connectivity.IRoutePolicyFactory";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateRoutePolicy)(__x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIConnectionProfile* connectionProfile,
        __x_ABI_CWindows_CNetworking_CIHostName* hostName,
        enum __x_ABI_CWindows_CNetworking_CDomainNameType type,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicy** routePolicy);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactoryVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_CreateRoutePolicy(This, connectionProfile, hostName, type, routePolicy) \
    ((This)->lpVtbl->CreateRoutePolicy(This, connectionProfile, hostName, type, routePolicy))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIRoutePolicyFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IWlanConnectionProfileDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.WlanConnectionProfileDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IWlanConnectionProfileDetails[] = L"Windows.Networking.Connectivity.IWlanConnectionProfileDetails";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetConnectedSsid)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetailsVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_GetConnectedSsid(This, value) \
    ((This)->lpVtbl->GetConnectedSsid(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIWlanConnectionProfileDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IWwanConnectionProfileDetails
 *
 * Introduced to Windows.Networking.Connectivity.WwanContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.WwanConnectionProfileDetails
 *
 */
#if WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IWwanConnectionProfileDetails[] = L"Windows.Networking.Connectivity.IWwanConnectionProfileDetails";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HomeProviderId)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AccessPointName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetNetworkRegistrationState)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails* This,
        enum __x_ABI_CWindows_CNetworking_CConnectivity_CWwanNetworkRegistrationState* value);
    HRESULT (STDMETHODCALLTYPE* GetCurrentDataClass)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails* This,
        enum __x_ABI_CWindows_CNetworking_CConnectivity_CWwanDataClass* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetailsVtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_get_HomeProviderId(This, value) \
    ((This)->lpVtbl->get_HomeProviderId(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_get_AccessPointName(This, value) \
    ((This)->lpVtbl->get_AccessPointName(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_GetNetworkRegistrationState(This, value) \
    ((This)->lpVtbl->GetNetworkRegistrationState(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_GetCurrentDataClass(This, value) \
    ((This)->lpVtbl->GetCurrentDataClass(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Connectivity.IWwanConnectionProfileDetails2
 *
 * Introduced to Windows.Networking.Connectivity.WwanContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Connectivity.WwanConnectionProfileDetails
 *
 */
#if WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Connectivity_IWwanConnectionProfileDetails2[] = L"Windows.Networking.Connectivity.IWwanConnectionProfileDetails2";
typedef struct __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IPKind)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2* This,
        enum __x_ABI_CWindows_CNetworking_CConnectivity_CWwanNetworkIPKind* value);
    HRESULT (STDMETHODCALLTYPE* get_PurposeGuids)(__x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2* This,
        __FIVectorView_1_GUID** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2Vtbl;

interface __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_get_IPKind(This, value) \
    ((This)->lpVtbl->get_IPKind(This, value))

#define __x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_get_PurposeGuids(This, value) \
    ((This)->lpVtbl->get_PurposeGuids(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CConnectivity_CIWwanConnectionProfileDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Networking.Connectivity.AttributedNetworkUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IAttributedNetworkUsage ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_AttributedNetworkUsage_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_AttributedNetworkUsage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_AttributedNetworkUsage[] = L"Windows.Networking.Connectivity.AttributedNetworkUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.CellularApnContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.ICellularApnContext ** Default Interface **
 *    Windows.Networking.Connectivity.ICellularApnContext2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_CellularApnContext_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_CellularApnContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_CellularApnContext[] = L"Windows.Networking.Connectivity.CellularApnContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.ConnectionCost
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IConnectionCost ** Default Interface **
 *    Windows.Networking.Connectivity.IConnectionCost2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_ConnectionCost_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_ConnectionCost_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_ConnectionCost[] = L"Windows.Networking.Connectivity.ConnectionCost";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.ConnectionProfile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IConnectionProfile ** Default Interface **
 *    Windows.Networking.Connectivity.IConnectionProfile2
 *    Windows.Networking.Connectivity.IConnectionProfile3
 *    Windows.Networking.Connectivity.IConnectionProfile4
 *    Windows.Networking.Connectivity.IConnectionProfile5
 *    Windows.Networking.Connectivity.IConnectionProfile6
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_ConnectionProfile_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_ConnectionProfile_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_ConnectionProfile[] = L"Windows.Networking.Connectivity.ConnectionProfile";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.ConnectionProfileFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IConnectionProfileFilter ** Default Interface **
 *    Windows.Networking.Connectivity.IConnectionProfileFilter2
 *    Windows.Networking.Connectivity.IConnectionProfileFilter3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_ConnectionProfileFilter_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_ConnectionProfileFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_ConnectionProfileFilter[] = L"Windows.Networking.Connectivity.ConnectionProfileFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.ConnectionSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IConnectionSession ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_ConnectionSession_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_ConnectionSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_ConnectionSession[] = L"Windows.Networking.Connectivity.ConnectionSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.ConnectivityInterval
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IConnectivityInterval ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_ConnectivityInterval_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_ConnectivityInterval_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_ConnectivityInterval[] = L"Windows.Networking.Connectivity.ConnectivityInterval";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.ConnectivityManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Connectivity.IConnectivityManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_ConnectivityManager_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_ConnectivityManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_ConnectivityManager[] = L"Windows.Networking.Connectivity.ConnectivityManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.DataPlanStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IDataPlanStatus ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_DataPlanStatus_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_DataPlanStatus_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_DataPlanStatus[] = L"Windows.Networking.Connectivity.DataPlanStatus";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.DataPlanUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IDataPlanUsage ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_DataPlanUsage_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_DataPlanUsage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_DataPlanUsage[] = L"Windows.Networking.Connectivity.DataPlanUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.DataUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IDataUsage ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_DataUsage_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_DataUsage_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("DataUsage may be altered or unavailable for releases after Windows 8.1. Instead, use NetworkUsage.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_DataUsage[] = L"Windows.Networking.Connectivity.DataUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.IPInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IIPInformation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_IPInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_IPInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_IPInformation[] = L"Windows.Networking.Connectivity.IPInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.LanIdentifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.ILanIdentifier ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_LanIdentifier_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_LanIdentifier_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_LanIdentifier[] = L"Windows.Networking.Connectivity.LanIdentifier";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.LanIdentifierData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.ILanIdentifierData ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_LanIdentifierData_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_LanIdentifierData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_LanIdentifierData[] = L"Windows.Networking.Connectivity.LanIdentifierData";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.NetworkAdapter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.INetworkAdapter ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_NetworkAdapter_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_NetworkAdapter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_NetworkAdapter[] = L"Windows.Networking.Connectivity.NetworkAdapter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.NetworkInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Connectivity.INetworkInformationStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.Connectivity.INetworkInformationStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_NetworkInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_NetworkInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_NetworkInformation[] = L"Windows.Networking.Connectivity.NetworkInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.NetworkItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.INetworkItem ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_NetworkItem_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_NetworkItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_NetworkItem[] = L"Windows.Networking.Connectivity.NetworkItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.NetworkSecuritySettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.INetworkSecuritySettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_NetworkSecuritySettings_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_NetworkSecuritySettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_NetworkSecuritySettings[] = L"Windows.Networking.Connectivity.NetworkSecuritySettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.NetworkStateChangeEventDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.INetworkStateChangeEventDetails ** Default Interface **
 *    Windows.Networking.Connectivity.INetworkStateChangeEventDetails2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_NetworkStateChangeEventDetails_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_NetworkStateChangeEventDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_NetworkStateChangeEventDetails[] = L"Windows.Networking.Connectivity.NetworkStateChangeEventDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.NetworkUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.INetworkUsage ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_NetworkUsage_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_NetworkUsage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_NetworkUsage[] = L"Windows.Networking.Connectivity.NetworkUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.ProviderNetworkUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IProviderNetworkUsage ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_ProviderNetworkUsage_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_ProviderNetworkUsage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_ProviderNetworkUsage[] = L"Windows.Networking.Connectivity.ProviderNetworkUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Networking.Connectivity.ProxyConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IProxyConfiguration ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_ProxyConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_ProxyConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_ProxyConfiguration[] = L"Windows.Networking.Connectivity.ProxyConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.RoutePolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Connectivity.IRoutePolicyFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IRoutePolicy ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_RoutePolicy_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_RoutePolicy_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_RoutePolicy[] = L"Windows.Networking.Connectivity.RoutePolicy";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.WlanConnectionProfileDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IWlanConnectionProfileDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_WlanConnectionProfileDetails_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_WlanConnectionProfileDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_WlanConnectionProfileDetails[] = L"Windows.Networking.Connectivity.WlanConnectionProfileDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Connectivity.WwanConnectionProfileDetails
 *
 * Introduced to Windows.Networking.Connectivity.WwanContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Connectivity.IWwanConnectionProfileDetails ** Default Interface **
 *    Windows.Networking.Connectivity.IWwanConnectionProfileDetails2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Connectivity_WwanConnectionProfileDetails_DEFINED
#define RUNTIMECLASS_Windows_Networking_Connectivity_WwanConnectionProfileDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Connectivity_WwanConnectionProfileDetails[] = L"Windows.Networking.Connectivity.WwanConnectionProfileDetails";
#endif
#endif // WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Enetworking2Econnectivity_p_h__

#endif // __windows2Enetworking2Econnectivity_h__
