
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
#ifndef __windows2Esystem2Eremotesystems_h__
#define __windows2Esystem2Eremotesystems_h__
#ifndef __windows2Esystem2Eremotesystems_p_h__
#define __windows2Esystem2Eremotesystems_p_h__


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
#include "Windows.ApplicationModel.AppService.h"
#include "Windows.Networking.h"
#include "Windows.Security.Credentials.h"
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IKnownRemoteSystemCapabilitiesStatics;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics ABI::Windows::System::RemoteSystems::IKnownRemoteSystemCapabilitiesStatics

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystem;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem ABI::Windows::System::RemoteSystems::IRemoteSystem

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystem2;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2 ABI::Windows::System::RemoteSystems::IRemoteSystem2

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystem3;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3 ABI::Windows::System::RemoteSystems::IRemoteSystem3

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystem4;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4 ABI::Windows::System::RemoteSystems::IRemoteSystem4

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystem5;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5 ABI::Windows::System::RemoteSystems::IRemoteSystem5

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystem6;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6 ABI::Windows::System::RemoteSystems::IRemoteSystem6

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemAddedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs ABI::Windows::System::RemoteSystems::IRemoteSystemAddedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemApp;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp ABI::Windows::System::RemoteSystems::IRemoteSystemApp

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemApp2;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2 ABI::Windows::System::RemoteSystems::IRemoteSystemApp2

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemAppRegistration;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration ABI::Windows::System::RemoteSystems::IRemoteSystemAppRegistration

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemAppRegistrationStatics;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics ABI::Windows::System::RemoteSystems::IRemoteSystemAppRegistrationStatics

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemAuthorizationKindFilter;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter ABI::Windows::System::RemoteSystems::IRemoteSystemAuthorizationKindFilter

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemAuthorizationKindFilterFactory;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory ABI::Windows::System::RemoteSystems::IRemoteSystemAuthorizationKindFilterFactory

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemConnectionInfo;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo ABI::Windows::System::RemoteSystems::IRemoteSystemConnectionInfo

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemConnectionInfoStatics;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics ABI::Windows::System::RemoteSystems::IRemoteSystemConnectionInfoStatics

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemConnectionRequest;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest ABI::Windows::System::RemoteSystems::IRemoteSystemConnectionRequest

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemConnectionRequest2;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2 ABI::Windows::System::RemoteSystems::IRemoteSystemConnectionRequest2

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemConnectionRequest3;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3 ABI::Windows::System::RemoteSystems::IRemoteSystemConnectionRequest3

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemConnectionRequestFactory;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory ABI::Windows::System::RemoteSystems::IRemoteSystemConnectionRequestFactory

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemConnectionRequestStatics;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics ABI::Windows::System::RemoteSystems::IRemoteSystemConnectionRequestStatics

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemConnectionRequestStatics2;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2 ABI::Windows::System::RemoteSystems::IRemoteSystemConnectionRequestStatics2

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemDiscoveryTypeFilter;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter ABI::Windows::System::RemoteSystems::IRemoteSystemDiscoveryTypeFilter

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemDiscoveryTypeFilterFactory;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory ABI::Windows::System::RemoteSystems::IRemoteSystemDiscoveryTypeFilterFactory

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemEnumerationCompletedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs ABI::Windows::System::RemoteSystems::IRemoteSystemEnumerationCompletedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemFilter;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter ABI::Windows::System::RemoteSystems::IRemoteSystemFilter

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemKindFilter;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter ABI::Windows::System::RemoteSystems::IRemoteSystemKindFilter

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemKindFilterFactory;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory ABI::Windows::System::RemoteSystems::IRemoteSystemKindFilterFactory

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemKindStatics;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics ABI::Windows::System::RemoteSystems::IRemoteSystemKindStatics

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemKindStatics2;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2 ABI::Windows::System::RemoteSystems::IRemoteSystemKindStatics2

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemRemovedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs ABI::Windows::System::RemoteSystems::IRemoteSystemRemovedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSession;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession ABI::Windows::System::RemoteSystems::IRemoteSystemSession

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionAddedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs ABI::Windows::System::RemoteSystems::IRemoteSystemSessionAddedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionController;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController ABI::Windows::System::RemoteSystems::IRemoteSystemSessionController

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionControllerFactory;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory ABI::Windows::System::RemoteSystems::IRemoteSystemSessionControllerFactory

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionCreationResult;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult ABI::Windows::System::RemoteSystems::IRemoteSystemSessionCreationResult

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionDisconnectedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs ABI::Windows::System::RemoteSystems::IRemoteSystemSessionDisconnectedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionInfo;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo ABI::Windows::System::RemoteSystems::IRemoteSystemSessionInfo

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionInvitation;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation ABI::Windows::System::RemoteSystems::IRemoteSystemSessionInvitation

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionInvitationListener;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener ABI::Windows::System::RemoteSystems::IRemoteSystemSessionInvitationListener

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionInvitationReceivedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs ABI::Windows::System::RemoteSystems::IRemoteSystemSessionInvitationReceivedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionJoinRequest;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest ABI::Windows::System::RemoteSystems::IRemoteSystemSessionJoinRequest

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionJoinRequestedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs ABI::Windows::System::RemoteSystems::IRemoteSystemSessionJoinRequestedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionJoinResult;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult ABI::Windows::System::RemoteSystems::IRemoteSystemSessionJoinResult

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionMessageChannel;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel ABI::Windows::System::RemoteSystems::IRemoteSystemSessionMessageChannel

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionMessageChannelFactory;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory ABI::Windows::System::RemoteSystems::IRemoteSystemSessionMessageChannelFactory

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionOptions;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions ABI::Windows::System::RemoteSystems::IRemoteSystemSessionOptions

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionParticipant;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant ABI::Windows::System::RemoteSystems::IRemoteSystemSessionParticipant

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionParticipantAddedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs ABI::Windows::System::RemoteSystems::IRemoteSystemSessionParticipantAddedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionParticipantRemovedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs ABI::Windows::System::RemoteSystems::IRemoteSystemSessionParticipantRemovedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionParticipantWatcher;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher ABI::Windows::System::RemoteSystems::IRemoteSystemSessionParticipantWatcher

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionRemovedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs ABI::Windows::System::RemoteSystems::IRemoteSystemSessionRemovedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionStatics;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics ABI::Windows::System::RemoteSystems::IRemoteSystemSessionStatics

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionUpdatedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs ABI::Windows::System::RemoteSystems::IRemoteSystemSessionUpdatedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionValueSetReceivedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs ABI::Windows::System::RemoteSystems::IRemoteSystemSessionValueSetReceivedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemSessionWatcher;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher ABI::Windows::System::RemoteSystems::IRemoteSystemSessionWatcher

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemStatics;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics ABI::Windows::System::RemoteSystems::IRemoteSystemStatics

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemStatics2;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2 ABI::Windows::System::RemoteSystems::IRemoteSystemStatics2

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemStatics3;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3 ABI::Windows::System::RemoteSystems::IRemoteSystemStatics3

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemStatusTypeFilter;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter ABI::Windows::System::RemoteSystems::IRemoteSystemStatusTypeFilter

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemStatusTypeFilterFactory;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory ABI::Windows::System::RemoteSystems::IRemoteSystemStatusTypeFilterFactory

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemUpdatedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs ABI::Windows::System::RemoteSystems::IRemoteSystemUpdatedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemWatcher;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher ABI::Windows::System::RemoteSystems::IRemoteSystemWatcher

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemWatcher2;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2 ABI::Windows::System::RemoteSystems::IRemoteSystemWatcher2

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemWatcher3;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3 ABI::Windows::System::RemoteSystems::IRemoteSystemWatcher3

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemWatcherErrorOccurredEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs ABI::Windows::System::RemoteSystems::IRemoteSystemWatcherErrorOccurredEventArgs

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemWebAccountFilter;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter ABI::Windows::System::RemoteSystems::IRemoteSystemWebAccountFilter

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemWebAccountFilterFactory;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory ABI::Windows::System::RemoteSystems::IRemoteSystemWebAccountFilterFactory

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIAsyncOperation_1_boolean_USE
#define DEF___FIAsyncOperation_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cdb5efb3-5788-509d-9be1-71ccb8a3362a"))
IAsyncOperation<bool> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<bool> __FIAsyncOperation_1_boolean_t;
#define __FIAsyncOperation_1_boolean ABI::Windows::Foundation::__FIAsyncOperation_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_boolean_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_boolean_USE
#define DEF___FIAsyncOperationCompletedHandler_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c1d3d1a2-ae17-5a5f-b5a2-bdcc8844889a"))
IAsyncOperationCompletedHandler<bool> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<bool> __FIAsyncOperationCompletedHandler_1_boolean_t;
#define __FIAsyncOperationCompletedHandler_1_boolean ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_boolean_USE */


namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystem;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_USE
#define DEF___FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0d39f546-0eca-5236-a5ca-7e3660658462"))
IAsyncOperation<ABI::Windows::System::RemoteSystems::RemoteSystem*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystem*, ABI::Windows::System::RemoteSystems::IRemoteSystem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.System.RemoteSystems.RemoteSystem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::System::RemoteSystems::RemoteSystem*> __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_t;
#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3a0b522d-98d0-5d34-ace6-2c7346613f1d"))
IAsyncOperationCompletedHandler<ABI::Windows::System::RemoteSystems::RemoteSystem*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystem*, ABI::Windows::System::RemoteSystems::IRemoteSystem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.System.RemoteSystems.RemoteSystem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::System::RemoteSystems::RemoteSystem*> __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                typedef enum RemoteSystemAccessStatus : int RemoteSystemAccessStatus;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d76da678-dd76-5460-8745-915b4410c905"))
IAsyncOperation<enum ABI::Windows::System::RemoteSystems::RemoteSystemAccessStatus> : IAsyncOperation_impl<enum ABI::Windows::System::RemoteSystems::RemoteSystemAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.System.RemoteSystems.RemoteSystemAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::System::RemoteSystems::RemoteSystemAccessStatus> __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_t;
#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("543a221d-ef39-57f5-9741-b052dbc29249"))
IAsyncOperationCompletedHandler<enum ABI::Windows::System::RemoteSystems::RemoteSystemAccessStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::System::RemoteSystems::RemoteSystemAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.System.RemoteSystems.RemoteSystemAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::System::RemoteSystems::RemoteSystemAccessStatus> __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionCreationResult;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("90364bf5-d084-5f50-9729-82025326abef"))
IAsyncOperation<ABI::Windows::System::RemoteSystems::RemoteSystemSessionCreationResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionCreationResult*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionCreationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.System.RemoteSystems.RemoteSystemSessionCreationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::System::RemoteSystems::RemoteSystemSessionCreationResult*> __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_t;
#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6e72c549-73aa-5168-8560-c7236493b504"))
IAsyncOperationCompletedHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionCreationResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionCreationResult*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionCreationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.System.RemoteSystems.RemoteSystemSessionCreationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionCreationResult*> __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionJoinResult;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c58dbd1e-e300-55a8-ada5-e25aaaa86667"))
IAsyncOperation<ABI::Windows::System::RemoteSystems::RemoteSystemSessionJoinResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionJoinResult*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionJoinResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.System.RemoteSystems.RemoteSystemSessionJoinResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::System::RemoteSystems::RemoteSystemSessionJoinResult*> __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_t;
#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("379adf35-4cb4-522f-91be-913b5690568f"))
IAsyncOperationCompletedHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionJoinResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionJoinResult*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionJoinResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.System.RemoteSystems.RemoteSystemSessionJoinResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionJoinResult*> __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000


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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_USE
#define DEF___FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6a2c5aef-9f30-58ae-a6cb-9ac9c8092a41"))
IIterator<ABI::Windows::System::RemoteSystems::IRemoteSystemFilter*> : IIterator_impl<ABI::Windows::System::RemoteSystems::IRemoteSystemFilter*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.System.RemoteSystems.IRemoteSystemFilter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::System::RemoteSystems::IRemoteSystemFilter*> __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_t;
#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_USE
#define DEF___FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("13966c92-a8de-50c0-b16b-00c2c48f5f37"))
IIterable<ABI::Windows::System::RemoteSystems::IRemoteSystemFilter*> : IIterable_impl<ABI::Windows::System::RemoteSystems::IRemoteSystemFilter*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.System.RemoteSystems.IRemoteSystemFilter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::System::RemoteSystems::IRemoteSystemFilter*> __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_t;
#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemApp;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_USE
#define DEF___FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8aa2049c-eaa5-534e-a144-5217499c739c"))
IIterator<ABI::Windows::System::RemoteSystems::RemoteSystemApp*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemApp*, ABI::Windows::System::RemoteSystems::IRemoteSystemApp*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.System.RemoteSystems.RemoteSystemApp>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::System::RemoteSystems::RemoteSystemApp*> __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_t;
#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_USE
#define DEF___FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4cfc1093-6de6-5d4d-8073-30e64b4dfa15"))
IIterable<ABI::Windows::System::RemoteSystems::RemoteSystemApp*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemApp*, ABI::Windows::System::RemoteSystems::IRemoteSystemApp*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.System.RemoteSystems.RemoteSystemApp>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::System::RemoteSystems::RemoteSystemApp*> __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_t;
#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionParticipant;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_USE
#define DEF___FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("05fec44b-3dd9-5cf1-a100-bedc9233292d"))
IIterator<ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipant*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipant*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionParticipant*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.System.RemoteSystems.RemoteSystemSessionParticipant>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipant*> __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_t;
#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_USE
#define DEF___FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("00189d10-16ec-5d1a-8369-4870c69e52b3"))
IIterable<ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipant*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipant*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionParticipant*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.System.RemoteSystems.RemoteSystemSessionParticipant>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipant*> __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_t;
#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000


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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_USE
#define DEF___FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("40011f82-e7e8-5a96-b767-399c6a4be101"))
IVectorView<ABI::Windows::System::RemoteSystems::RemoteSystemApp*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemApp*, ABI::Windows::System::RemoteSystems::IRemoteSystemApp*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.System.RemoteSystems.RemoteSystemApp>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::System::RemoteSystems::RemoteSystemApp*> __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_t;
#define __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSession;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionDisconnectedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fba14773-5038-511a-95a3-4ba45349100a"))
ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSession*, ABI::Windows::System::RemoteSystems::RemoteSystemSessionDisconnectedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSession*, ABI::Windows::System::RemoteSystems::IRemoteSystemSession*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionDisconnectedEventArgs*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionDisconnectedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.RemoteSystems.RemoteSystemSession, Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSession*, ABI::Windows::System::RemoteSystems::RemoteSystemSessionDisconnectedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionController;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionJoinRequestedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d8e04916-b452-5322-aec9-e3d4d581c772"))
ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionController*, ABI::Windows::System::RemoteSystems::RemoteSystemSessionJoinRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionController*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionController*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionJoinRequestedEventArgs*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionJoinRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.RemoteSystems.RemoteSystemSessionController, Windows.System.RemoteSystems.RemoteSystemSessionJoinRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionController*, ABI::Windows::System::RemoteSystems::RemoteSystemSessionJoinRequestedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionInvitationListener;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionInvitationReceivedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("18a242bb-d338-56c4-9559-568d5c2c3e93"))
ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionInvitationListener*, ABI::Windows::System::RemoteSystems::RemoteSystemSessionInvitationReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionInvitationListener*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionInvitationListener*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionInvitationReceivedEventArgs*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionInvitationReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.RemoteSystems.RemoteSystemSessionInvitationListener, Windows.System.RemoteSystems.RemoteSystemSessionInvitationReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionInvitationListener*, ABI::Windows::System::RemoteSystems::RemoteSystemSessionInvitationReceivedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionMessageChannel;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionValueSetReceivedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c476232d-8c76-5ba6-99f5-14557484c20d"))
ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionMessageChannel*, ABI::Windows::System::RemoteSystems::RemoteSystemSessionValueSetReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionMessageChannel*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionMessageChannel*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionValueSetReceivedEventArgs*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionValueSetReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.RemoteSystems.RemoteSystemSessionMessageChannel, Windows.System.RemoteSystems.RemoteSystemSessionValueSetReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionMessageChannel*, ABI::Windows::System::RemoteSystems::RemoteSystemSessionValueSetReceivedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionParticipantWatcher;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d4cf5bda-cc7a-52ef-a256-c4b36163beec"))
ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipantWatcher*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipantWatcher*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionParticipantWatcher*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcher, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipantWatcher*, IInspectable*> __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionParticipantAddedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7d42fff3-fd21-5e15-b21a-75e1bbcd13c7"))
ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipantWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipantAddedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipantWatcher*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionParticipantWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipantAddedEventArgs*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionParticipantAddedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcher, Windows.System.RemoteSystems.RemoteSystemSessionParticipantAddedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipantWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipantAddedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionParticipantRemovedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("73d7e8b3-7d44-50c8-afad-450de59fd0ae"))
ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipantWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipantRemovedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipantWatcher*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionParticipantWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipantRemovedEventArgs*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionParticipantRemovedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcher, Windows.System.RemoteSystems.RemoteSystemSessionParticipantRemovedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipantWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipantRemovedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionWatcher;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionAddedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1b036c4f-6b8f-55d6-b6df-45e46a823b1d"))
ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemSessionAddedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionWatcher*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionAddedEventArgs*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionAddedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.RemoteSystems.RemoteSystemSessionWatcher, Windows.System.RemoteSystems.RemoteSystemSessionAddedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemSessionAddedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionRemovedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1cbc54f0-0c9d-59cc-8055-5e017a317812"))
ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemSessionRemovedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionWatcher*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionRemovedEventArgs*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionRemovedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.RemoteSystems.RemoteSystemSessionWatcher, Windows.System.RemoteSystems.RemoteSystemSessionRemovedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemSessionRemovedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionUpdatedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("040f48b7-0d41-5aa2-85e8-6311666f0324"))
ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemSessionUpdatedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionWatcher*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemSessionUpdatedEventArgs*, ABI::Windows::System::RemoteSystems::IRemoteSystemSessionUpdatedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.RemoteSystems.RemoteSystemSessionWatcher, Windows.System.RemoteSystems.RemoteSystemSessionUpdatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemSessionWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemSessionUpdatedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemWatcher;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemAddedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a9b98f4a-b63d-5d07-92bb-e2acb39455d1"))
ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemAddedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemWatcher*, ABI::Windows::System::RemoteSystems::IRemoteSystemWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemAddedEventArgs*, ABI::Windows::System::RemoteSystems::IRemoteSystemAddedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.RemoteSystems.RemoteSystemWatcher, Windows.System.RemoteSystems.RemoteSystemAddedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemAddedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemEnumerationCompletedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("42ae9d52-fd0d-5411-8c8c-d676a09767e9"))
ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemEnumerationCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemWatcher*, ABI::Windows::System::RemoteSystems::IRemoteSystemWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemEnumerationCompletedEventArgs*, ABI::Windows::System::RemoteSystems::IRemoteSystemEnumerationCompletedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.RemoteSystems.RemoteSystemWatcher, Windows.System.RemoteSystems.RemoteSystemEnumerationCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemEnumerationCompletedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemRemovedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c290fb5a-3ed0-5858-af19-f85553cb96b8"))
ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemRemovedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemWatcher*, ABI::Windows::System::RemoteSystems::IRemoteSystemWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemRemovedEventArgs*, ABI::Windows::System::RemoteSystems::IRemoteSystemRemovedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.RemoteSystems.RemoteSystemWatcher, Windows.System.RemoteSystems.RemoteSystemRemovedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemRemovedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemUpdatedEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("88f9d23f-8946-5a1e-8da1-82c66982a6d2"))
ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemUpdatedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemWatcher*, ABI::Windows::System::RemoteSystems::IRemoteSystemWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemUpdatedEventArgs*, ABI::Windows::System::RemoteSystems::IRemoteSystemUpdatedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.RemoteSystems.RemoteSystemWatcher, Windows.System.RemoteSystems.RemoteSystemUpdatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemUpdatedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemWatcherErrorOccurredEventArgs;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a4a023b4-567b-5d4b-a4c8-5ca4f886d3a3"))
ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemWatcherErrorOccurredEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemWatcher*, ABI::Windows::System::RemoteSystems::IRemoteSystemWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteSystems::RemoteSystemWatcherErrorOccurredEventArgs*, ABI::Windows::System::RemoteSystems::IRemoteSystemWatcherErrorOccurredEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.RemoteSystems.RemoteSystemWatcher, Windows.System.RemoteSystems.RemoteSystemWatcherErrorOccurredEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::RemoteSystems::RemoteSystemWatcher*, ABI::Windows::System::RemoteSystems::RemoteSystemWatcherErrorOccurredEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

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
            namespace Collections {
                class ValueSet;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                interface IPropertySet;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet ABI::Windows::Foundation::Collections::IPropertySet

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

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
        namespace Security {
            namespace Credentials {
                class WebAccount;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IWebAccount;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount ABI::Windows::Security::Credentials::IWebAccount

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__

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
        namespace System {
            namespace RemoteSystems {
                typedef enum RemoteSystemAuthorizationKind : int RemoteSystemAuthorizationKind;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                typedef enum RemoteSystemDiscoveryType : int RemoteSystemDiscoveryType;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                typedef enum RemoteSystemPlatform : int RemoteSystemPlatform;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                typedef enum RemoteSystemSessionCreationStatus : int RemoteSystemSessionCreationStatus;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                typedef enum RemoteSystemSessionDisconnectedReason : int RemoteSystemSessionDisconnectedReason;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                typedef enum RemoteSystemSessionJoinStatus : int RemoteSystemSessionJoinStatus;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                typedef enum RemoteSystemSessionMessageChannelReliability : int RemoteSystemSessionMessageChannelReliability;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                typedef enum RemoteSystemSessionParticipantWatcherStatus : int RemoteSystemSessionParticipantWatcherStatus;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                typedef enum RemoteSystemSessionWatcherStatus : int RemoteSystemSessionWatcherStatus;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                typedef enum RemoteSystemStatus : int RemoteSystemStatus;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                typedef enum RemoteSystemStatusType : int RemoteSystemStatusType;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                typedef enum RemoteSystemWatcherError : int RemoteSystemWatcherError;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemAppRegistration;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemAuthorizationKindFilter;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemConnectionInfo;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemConnectionRequest;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemDiscoveryTypeFilter;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemKindFilter;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionInfo;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionInvitation;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionJoinRequest;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSessionOptions;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemStatusTypeFilter;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemWebAccountFilter;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                enum RemoteSystemAccessStatus : int
                {
                    RemoteSystemAccessStatus_Unspecified = 0,
                    RemoteSystemAccessStatus_Allowed = 1,
                    RemoteSystemAccessStatus_DeniedByUser = 2,
                    RemoteSystemAccessStatus_DeniedBySystem = 3,
                };
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemAuthorizationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                enum RemoteSystemAuthorizationKind : int
                {
                    RemoteSystemAuthorizationKind_SameUser = 0,
                    RemoteSystemAuthorizationKind_Anonymous = 1,
                    RemoteSystemAuthorizationKind_SameFamily = 2,
                };
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemDiscoveryType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                enum RemoteSystemDiscoveryType : int
                {
                    RemoteSystemDiscoveryType_Any = 0,
                    RemoteSystemDiscoveryType_Proximal = 1,
                    RemoteSystemDiscoveryType_Cloud = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    RemoteSystemDiscoveryType_SpatiallyProximal = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                };
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemPlatform
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                enum RemoteSystemPlatform : int
                {
                    RemoteSystemPlatform_Unknown = 0,
                    RemoteSystemPlatform_Windows = 1,
                    RemoteSystemPlatform_Android = 2,
                    RemoteSystemPlatform_Ios = 3,
                    RemoteSystemPlatform_Linux = 4,
                };
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemSessionCreationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                enum RemoteSystemSessionCreationStatus : int
                {
                    RemoteSystemSessionCreationStatus_Success = 0,
                    RemoteSystemSessionCreationStatus_SessionLimitsExceeded = 1,
                    RemoteSystemSessionCreationStatus_OperationAborted = 2,
                };
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                enum RemoteSystemSessionDisconnectedReason : int
                {
                    RemoteSystemSessionDisconnectedReason_SessionUnavailable = 0,
                    RemoteSystemSessionDisconnectedReason_RemovedByController = 1,
                    RemoteSystemSessionDisconnectedReason_SessionClosed = 2,
                };
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemSessionJoinStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                enum RemoteSystemSessionJoinStatus : int
                {
                    RemoteSystemSessionJoinStatus_Success = 0,
                    RemoteSystemSessionJoinStatus_SessionLimitsExceeded = 1,
                    RemoteSystemSessionJoinStatus_OperationAborted = 2,
                    RemoteSystemSessionJoinStatus_SessionUnavailable = 3,
                    RemoteSystemSessionJoinStatus_RejectedByController = 4,
                };
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemSessionMessageChannelReliability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                enum RemoteSystemSessionMessageChannelReliability : int
                {
                    RemoteSystemSessionMessageChannelReliability_Reliable = 0,
                    RemoteSystemSessionMessageChannelReliability_Unreliable = 1,
                };
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcherStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                enum RemoteSystemSessionParticipantWatcherStatus : int
                {
                    RemoteSystemSessionParticipantWatcherStatus_Created = 0,
                    RemoteSystemSessionParticipantWatcherStatus_Started = 1,
                    RemoteSystemSessionParticipantWatcherStatus_EnumerationCompleted = 2,
                    RemoteSystemSessionParticipantWatcherStatus_Stopping = 3,
                    RemoteSystemSessionParticipantWatcherStatus_Stopped = 4,
                    RemoteSystemSessionParticipantWatcherStatus_Aborted = 5,
                };
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemSessionWatcherStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                enum RemoteSystemSessionWatcherStatus : int
                {
                    RemoteSystemSessionWatcherStatus_Created = 0,
                    RemoteSystemSessionWatcherStatus_Started = 1,
                    RemoteSystemSessionWatcherStatus_EnumerationCompleted = 2,
                    RemoteSystemSessionWatcherStatus_Stopping = 3,
                    RemoteSystemSessionWatcherStatus_Stopped = 4,
                    RemoteSystemSessionWatcherStatus_Aborted = 5,
                };
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                enum RemoteSystemStatus : int
                {
                    RemoteSystemStatus_Unavailable = 0,
                    RemoteSystemStatus_DiscoveringAvailability = 1,
                    RemoteSystemStatus_Available = 2,
                    RemoteSystemStatus_Unknown = 3,
                };
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemStatusType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                enum RemoteSystemStatusType : int
                {
                    RemoteSystemStatusType_Any = 0,
                    RemoteSystemStatusType_Available = 1,
                };
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemWatcherError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                enum RemoteSystemWatcherError : int
                {
                    RemoteSystemWatcherError_Unknown = 0,
                    RemoteSystemWatcherError_InternetNotAvailable = 1,
                    RemoteSystemWatcherError_AuthenticationError = 2,
                };
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.RemoteSystems.IKnownRemoteSystemCapabilitiesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.KnownRemoteSystemCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IKnownRemoteSystemCapabilitiesStatics[] = L"Windows.System.RemoteSystems.IKnownRemoteSystemCapabilitiesStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("8108e380-7f8a-44e4-92cd-03b6469b94a3")
                IKnownRemoteSystemCapabilitiesStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppService(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LaunchUri(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteSession(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SpatialEntity(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKnownRemoteSystemCapabilitiesStatics = __uuidof(IKnownRemoteSystemCapabilitiesStatics);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystem[] = L"Windows.System.RemoteSystems.IRemoteSystem";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("ed5838cd-1e10-4a8c-b4a6-4e5fd6f97721")
                IRemoteSystem : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::System::RemoteSystems::RemoteSystemStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsAvailableByProximity(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystem = __uuidof(IRemoteSystem);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystem2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystem2[] = L"Windows.System.RemoteSystems.IRemoteSystem2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("09dfe4ec-fb8b-4a08-a758-6876435d769e")
                IRemoteSystem2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsAvailableBySpatialProximity(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCapabilitySupportedAsync(
                        HSTRING capabilityName,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystem2 = __uuidof(IRemoteSystem2);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystem3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystem3[] = L"Windows.System.RemoteSystems.IRemoteSystem3";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("72b4b495-b7c6-40be-831b-73562f12ffa8")
                IRemoteSystem3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ManufacturerDisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ModelDisplayName(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystem3 = __uuidof(IRemoteSystem3);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystem4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystem4[] = L"Windows.System.RemoteSystems.IRemoteSystem4";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("f164ffe5-b987-4ca5-9926-fa0438be6273")
                IRemoteSystem4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Platform(
                        ABI::Windows::System::RemoteSystems::RemoteSystemPlatform* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystem4 = __uuidof(IRemoteSystem4);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystem5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystem5[] = L"Windows.System.RemoteSystems.IRemoteSystem5";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("eb2ad723-e5e2-4ae2-a7a7-a1097a098e90")
                IRemoteSystem5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Apps(
                        __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystem5 = __uuidof(IRemoteSystem5);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystem6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystem6[] = L"Windows.System.RemoteSystems.IRemoteSystem6";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("d4cda942-c027-533e-9384-3a19b4f7eef3")
                IRemoteSystem6 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystem6 = __uuidof(IRemoteSystem6);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemAddedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemAddedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemAddedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("8f39560f-e534-4697-8836-7abea151516e")
                IRemoteSystemAddedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteSystem(
                        ABI::Windows::System::RemoteSystems::IRemoteSystem** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemAddedEventArgs = __uuidof(IRemoteSystemAddedEventArgs);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemApp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemApp[] = L"Windows.System.RemoteSystems.IRemoteSystemApp";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("80e5bcbd-d54d-41b1-9b16-6810a871ed4f")
                IRemoteSystemApp : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsAvailableByProximity(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsAvailableBySpatialProximity(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Attributes(
                        __FIMapView_2_HSTRING_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemApp = __uuidof(IRemoteSystemApp);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemApp2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemApp2[] = L"Windows.System.RemoteSystems.IRemoteSystemApp2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("6369bf15-0a96-577a-8ff6-c35904dfa8f3")
                IRemoteSystemApp2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ConnectionToken(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemApp2 = __uuidof(IRemoteSystemApp2);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemAppRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemAppRegistration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemAppRegistration[] = L"Windows.System.RemoteSystems.IRemoteSystemAppRegistration";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("b47947b5-7035-4a5a-b8df-962d8f8431f4")
                IRemoteSystemAppRegistration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Attributes(
                        __FIMap_2_HSTRING_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SaveAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemAppRegistration = __uuidof(IRemoteSystemAppRegistration);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemAppRegistrationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemAppRegistration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemAppRegistrationStatics[] = L"Windows.System.RemoteSystems.IRemoteSystemAppRegistrationStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("01b99840-cfd2-453f-ae25-c2539f086afd")
                IRemoteSystemAppRegistrationStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemAppRegistration** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemAppRegistration** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemAppRegistrationStatics = __uuidof(IRemoteSystemAppRegistrationStatics);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemAuthorizationKindFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemAuthorizationKindFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemAuthorizationKindFilter[] = L"Windows.System.RemoteSystems.IRemoteSystemAuthorizationKindFilter";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("6b0dde8e-04d0-40f4-a27f-c2acbbd6b734")
                IRemoteSystemAuthorizationKindFilter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteSystemAuthorizationKind(
                        ABI::Windows::System::RemoteSystems::RemoteSystemAuthorizationKind* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemAuthorizationKindFilter = __uuidof(IRemoteSystemAuthorizationKindFilter);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemAuthorizationKindFilterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemAuthorizationKindFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemAuthorizationKindFilterFactory[] = L"Windows.System.RemoteSystems.IRemoteSystemAuthorizationKindFilterFactory";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("ad65df4d-b66a-45a4-8177-8caed75d9e5a")
                IRemoteSystemAuthorizationKindFilterFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::System::RemoteSystems::RemoteSystemAuthorizationKind remoteSystemAuthorizationKind,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemAuthorizationKindFilter** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemAuthorizationKindFilterFactory = __uuidof(IRemoteSystemAuthorizationKindFilterFactory);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemConnectionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemConnectionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemConnectionInfo[] = L"Windows.System.RemoteSystems.IRemoteSystemConnectionInfo";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("23278bc3-0d09-52cb-9c6a-eed2940bee43")
                IRemoteSystemConnectionInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsProximal(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemConnectionInfo = __uuidof(IRemoteSystemConnectionInfo);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemConnectionInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemConnectionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemConnectionInfoStatics[] = L"Windows.System.RemoteSystems.IRemoteSystemConnectionInfoStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("ac831e2d-66c5-56d7-a4ce-705d94925ad6")
                IRemoteSystemConnectionInfoStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryCreateFromAppServiceConnection(
                        ABI::Windows::ApplicationModel::AppService::IAppServiceConnection* connection,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemConnectionInfo** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemConnectionInfoStatics = __uuidof(IRemoteSystemConnectionInfoStatics);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemConnectionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemConnectionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemConnectionRequest[] = L"Windows.System.RemoteSystems.IRemoteSystemConnectionRequest";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("84ed4104-8d5e-4d72-8238-7621576c7a67")
                IRemoteSystemConnectionRequest : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteSystem(
                        ABI::Windows::System::RemoteSystems::IRemoteSystem** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemConnectionRequest = __uuidof(IRemoteSystemConnectionRequest);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemConnectionRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemConnectionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemConnectionRequest2[] = L"Windows.System.RemoteSystems.IRemoteSystemConnectionRequest2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("12df6d6f-bffc-483a-8abe-d34a6c19f92b")
                IRemoteSystemConnectionRequest2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteSystemApp(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemApp** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemConnectionRequest2 = __uuidof(IRemoteSystemConnectionRequest2);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemConnectionRequest3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemConnectionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemConnectionRequest3[] = L"Windows.System.RemoteSystems.IRemoteSystemConnectionRequest3";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("de86c3e7-c9cc-5a50-b8d9-ba7b34bb8d0e")
                IRemoteSystemConnectionRequest3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ConnectionToken(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemConnectionRequest3 = __uuidof(IRemoteSystemConnectionRequest3);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemConnectionRequestFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemConnectionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemConnectionRequestFactory[] = L"Windows.System.RemoteSystems.IRemoteSystemConnectionRequestFactory";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("aa0a0a20-baeb-4575-b530-810bb9786334")
                IRemoteSystemConnectionRequestFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::System::RemoteSystems::IRemoteSystem* remoteSystem,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemConnectionRequest** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemConnectionRequestFactory = __uuidof(IRemoteSystemConnectionRequestFactory);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemConnectionRequestStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemConnectionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemConnectionRequestStatics[] = L"Windows.System.RemoteSystems.IRemoteSystemConnectionRequestStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("86ca143d-8214-425c-8932-db49032d1306")
                IRemoteSystemConnectionRequestStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateForApp(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemApp* remoteSystemApp,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemConnectionRequest** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemConnectionRequestStatics = __uuidof(IRemoteSystemConnectionRequestStatics);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemConnectionRequestStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemConnectionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemConnectionRequestStatics2[] = L"Windows.System.RemoteSystems.IRemoteSystemConnectionRequestStatics2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("460f1027-64ec-598e-a800-4f2ee58def19")
                IRemoteSystemConnectionRequestStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromConnectionToken(
                        HSTRING connectionToken,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemConnectionRequest** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromConnectionTokenForUser(
                        ABI::Windows::System::IUser* user,
                        HSTRING connectionToken,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemConnectionRequest** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemConnectionRequestStatics2 = __uuidof(IRemoteSystemConnectionRequestStatics2);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemDiscoveryTypeFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemDiscoveryTypeFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemDiscoveryTypeFilter[] = L"Windows.System.RemoteSystems.IRemoteSystemDiscoveryTypeFilter";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("42d9041f-ee5a-43da-ac6a-6fee25460741")
                IRemoteSystemDiscoveryTypeFilter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteSystemDiscoveryType(
                        ABI::Windows::System::RemoteSystems::RemoteSystemDiscoveryType* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemDiscoveryTypeFilter = __uuidof(IRemoteSystemDiscoveryTypeFilter);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemDiscoveryTypeFilterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemDiscoveryTypeFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemDiscoveryTypeFilterFactory[] = L"Windows.System.RemoteSystems.IRemoteSystemDiscoveryTypeFilterFactory";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("9f9eb993-c260-4161-92f2-9c021f23fe5d")
                IRemoteSystemDiscoveryTypeFilterFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::System::RemoteSystems::RemoteSystemDiscoveryType discoveryType,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemDiscoveryTypeFilter** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemDiscoveryTypeFilterFactory = __uuidof(IRemoteSystemDiscoveryTypeFilterFactory);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemEnumerationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemEnumerationCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemEnumerationCompletedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemEnumerationCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("c6e83d5f-4030-4354-a060-14f1b22c545d")
                IRemoteSystemEnumerationCompletedEventArgs : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemEnumerationCompletedEventArgs = __uuidof(IRemoteSystemEnumerationCompletedEventArgs);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemFilter[] = L"Windows.System.RemoteSystems.IRemoteSystemFilter";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("4a3ba9e4-99eb-45eb-ba16-0367728ff374")
                IRemoteSystemFilter : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemFilter = __uuidof(IRemoteSystemFilter);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemKindFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemKindFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemKindFilter[] = L"Windows.System.RemoteSystems.IRemoteSystemKindFilter";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("38e1c9ec-22c3-4ef6-901a-bbb1c7aad4ed")
                IRemoteSystemKindFilter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteSystemKinds(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemKindFilter = __uuidof(IRemoteSystemKindFilter);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemKindFilterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemKindFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemKindFilterFactory[] = L"Windows.System.RemoteSystems.IRemoteSystemKindFilterFactory";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("a1fb18ee-99ea-40bc-9a39-c670aa804a28")
                IRemoteSystemKindFilterFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        __FIIterable_1_HSTRING* remoteSystemKinds,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemKindFilter** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemKindFilterFactory = __uuidof(IRemoteSystemKindFilterFactory);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemKindStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemKinds
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemKindStatics[] = L"Windows.System.RemoteSystems.IRemoteSystemKindStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("f6317633-ab14-41d0-9553-796aadb882db")
                IRemoteSystemKindStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Phone(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Hub(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Holographic(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Desktop(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Xbox(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemKindStatics = __uuidof(IRemoteSystemKindStatics);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemKindStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemKinds
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemKindStatics2[] = L"Windows.System.RemoteSystems.IRemoteSystemKindStatics2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("b9e3a3d0-0466-4749-91e8-65f9d19a96a5")
                IRemoteSystemKindStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Iot(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Tablet(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Laptop(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemKindStatics2 = __uuidof(IRemoteSystemKindStatics2);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemRemovedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemRemovedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemRemovedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("8b3d16bb-7306-49ea-b7df-67d5714cb013")
                IRemoteSystemRemovedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteSystemId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemRemovedEventArgs = __uuidof(IRemoteSystemRemovedEventArgs);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSession[] = L"Windows.System.RemoteSystems.IRemoteSystemSession";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("69476a01-9ada-490f-9549-d31cb14c9e95")
                IRemoteSystemSession : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ControllerDisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Disconnected(
                        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Disconnected(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateParticipantWatcher(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionParticipantWatcher** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendInvitationAsync(
                        ABI::Windows::System::RemoteSystems::IRemoteSystem* invitee,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSession = __uuidof(IRemoteSystemSession);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionAddedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionAddedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionAddedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("d585d754-bc97-4c39-99b4-beca76e04c3f")
                IRemoteSystemSessionAddedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SessionInfo(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionAddedEventArgs = __uuidof(IRemoteSystemSessionAddedEventArgs);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionController[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionController";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("e48b2dd2-6820-4867-b425-d89c0a3ef7ba")
                IRemoteSystemSessionController : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_JoinRequested(
                        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_JoinRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveParticipantAsync(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionParticipant* pParticipant,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateSessionAsync(
                        __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionController = __uuidof(IRemoteSystemSessionController);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionControllerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionControllerFactory[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionControllerFactory";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("bfcc2f6b-ac3d-4199-82cd-6670a773ef2e")
                IRemoteSystemSessionControllerFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateController(
                        HSTRING displayName,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionController** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateControllerWithSessionOptions(
                        HSTRING displayName,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionOptions* options,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionController** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionControllerFactory = __uuidof(IRemoteSystemSessionControllerFactory);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionCreationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionCreationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionCreationResult[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionCreationResult";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("a79812c2-37de-448c-8b83-a30aa3c4ead6")
                IRemoteSystemSessionCreationResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::System::RemoteSystems::RemoteSystemSessionCreationStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Session(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSession** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionCreationResult = __uuidof(IRemoteSystemSessionCreationResult);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionDisconnectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionDisconnectedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionDisconnectedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("de0bc69b-77c5-461c-8209-7c6c5d3111ab")
                IRemoteSystemSessionDisconnectedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Reason(
                        ABI::Windows::System::RemoteSystems::RemoteSystemSessionDisconnectedReason* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionDisconnectedEventArgs = __uuidof(IRemoteSystemSessionDisconnectedEventArgs);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionInfo[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionInfo";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("ff4df648-8b0a-4e9a-9905-69e4b841c588")
                IRemoteSystemSessionInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ControllerDisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE JoinAsync(
                        __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionInfo = __uuidof(IRemoteSystemSessionInfo);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionInvitation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionInvitation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionInvitation[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionInvitation";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("3e32cc91-51d7-4766-a121-25516c3b8294")
                IRemoteSystemSessionInvitation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Sender(
                        ABI::Windows::System::RemoteSystems::IRemoteSystem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SessionInfo(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionInvitation = __uuidof(IRemoteSystemSessionInvitation);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionInvitationListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionInvitationListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionInvitationListener[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionInvitationListener";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("08f4003f-bc71-49e1-874a-31ddff9a27b9")
                IRemoteSystemSessionInvitationListener : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_InvitationReceived(
                        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_InvitationReceived(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionInvitationListener = __uuidof(IRemoteSystemSessionInvitationListener);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionInvitationReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionInvitationReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionInvitationReceivedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionInvitationReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("5e964a2d-a10d-4edb-8dea-54d20ac19543")
                IRemoteSystemSessionInvitationReceivedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Invitation(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionInvitation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionInvitationReceivedEventArgs = __uuidof(IRemoteSystemSessionInvitationReceivedEventArgs);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionJoinRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionJoinRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionJoinRequest[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionJoinRequest";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("20600068-7994-4331-86d1-d89d882585ee")
                IRemoteSystemSessionJoinRequest : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Participant(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionParticipant** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Accept(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionJoinRequest = __uuidof(IRemoteSystemSessionJoinRequest);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionJoinRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionJoinRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionJoinRequestedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionJoinRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("dbca4fc3-82b9-4816-9c24-e40e61774bd8")
                IRemoteSystemSessionJoinRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_JoinRequest(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionJoinRequest** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionJoinRequestedEventArgs = __uuidof(IRemoteSystemSessionJoinRequestedEventArgs);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionJoinResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionJoinResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionJoinResult[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionJoinResult";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("ce7b1f04-a03e-41a4-900b-1e79328c1267")
                IRemoteSystemSessionJoinResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::System::RemoteSystems::RemoteSystemSessionJoinStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Session(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSession** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionJoinResult = __uuidof(IRemoteSystemSessionJoinResult);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionMessageChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionMessageChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionMessageChannel[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionMessageChannel";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("9524d12a-73d9-4c10-b751-c26784437127")
                IRemoteSystemSessionMessageChannel : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Session(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSession** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE BroadcastValueSetAsync(
                        ABI::Windows::Foundation::Collections::IPropertySet* messageData,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendValueSetAsync(
                        ABI::Windows::Foundation::Collections::IPropertySet* messageData,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionParticipant* participant,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendValueSetToParticipantsAsync(
                        ABI::Windows::Foundation::Collections::IPropertySet* messageData,
                        __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* participants,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ValueSetReceived(
                        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ValueSetReceived(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionMessageChannel = __uuidof(IRemoteSystemSessionMessageChannel);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionMessageChannelFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionMessageChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionMessageChannelFactory[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionMessageChannelFactory";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("295e1c4a-bd16-4298-b7ce-415482b0e11d")
                IRemoteSystemSessionMessageChannelFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSession* session,
                        HSTRING channelName,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionMessageChannel** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithReliability(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSession* session,
                        HSTRING channelName,
                        ABI::Windows::System::RemoteSystems::RemoteSystemSessionMessageChannelReliability reliability,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionMessageChannel** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionMessageChannelFactory = __uuidof(IRemoteSystemSessionMessageChannelFactory);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionOptions[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionOptions";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("740ed755-8418-4f01-9353-e21c9ecc6cfc")
                IRemoteSystemSessionOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsInviteOnly(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsInviteOnly(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionOptions = __uuidof(IRemoteSystemSessionOptions);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionParticipant
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionParticipant
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionParticipant[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionParticipant";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("7e90058c-acf9-4729-8a17-44e7baed5dcc")
                IRemoteSystemSessionParticipant : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteSystem(
                        ABI::Windows::System::RemoteSystems::IRemoteSystem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetHostNames(
                        __FIVectorView_1_Windows__CNetworking__CHostName** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionParticipant = __uuidof(IRemoteSystemSessionParticipant);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionParticipantAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionParticipantAddedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionParticipantAddedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionParticipantAddedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("d35a57d8-c9a1-4bb7-b6b0-79bb91adf93d")
                IRemoteSystemSessionParticipantAddedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Participant(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionParticipant** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionParticipantAddedEventArgs = __uuidof(IRemoteSystemSessionParticipantAddedEventArgs);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionParticipantRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionParticipantRemovedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionParticipantRemovedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionParticipantRemovedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("866ef088-de68-4abf-88a1-f90d16274192")
                IRemoteSystemSessionParticipantRemovedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Participant(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionParticipant** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionParticipantRemovedEventArgs = __uuidof(IRemoteSystemSessionParticipantRemovedEventArgs);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionParticipantWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionParticipantWatcher[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionParticipantWatcher";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("dcdd02cc-aa87-4d79-b6cc-4459b3e92075")
                IRemoteSystemSessionParticipantWatcher : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::System::RemoteSystems::RemoteSystemSessionParticipantWatcherStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Added(
                        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Added(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Removed(
                        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Removed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_EnumerationCompleted(
                        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_EnumerationCompleted(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionParticipantWatcher = __uuidof(IRemoteSystemSessionParticipantWatcher);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionRemovedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionRemovedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionRemovedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("af82914e-39a1-4dea-9d63-43798d5bbbd0")
                IRemoteSystemSessionRemovedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SessionInfo(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionRemovedEventArgs = __uuidof(IRemoteSystemSessionRemovedEventArgs);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionStatics[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("8524899f-fd20-44e3-9565-e75a3b14c66e")
                IRemoteSystemSessionStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWatcher(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionWatcher** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionStatics = __uuidof(IRemoteSystemSessionStatics);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionUpdatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionUpdatedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionUpdatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("16875069-231e-4c91-8ec8-b3a39d9e55a3")
                IRemoteSystemSessionUpdatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SessionInfo(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionUpdatedEventArgs = __uuidof(IRemoteSystemSessionUpdatedEventArgs);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionValueSetReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionValueSetReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionValueSetReceivedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionValueSetReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("06f31785-2da5-4e58-a78f-9e8d0784ee25")
                IRemoteSystemSessionValueSetReceivedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Sender(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSessionParticipant** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Message(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionValueSetReceivedEventArgs = __uuidof(IRemoteSystemSessionValueSetReceivedEventArgs);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionWatcher[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionWatcher";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("8003e340-0c41-4a62-b6d7-bdbe2b19be2d")
                IRemoteSystemSessionWatcher : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::System::RemoteSystems::RemoteSystemSessionWatcherStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Added(
                        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Added(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Updated(
                        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Updated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Removed(
                        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Removed(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemSessionWatcher = __uuidof(IRemoteSystemSessionWatcher);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemStatics[] = L"Windows.System.RemoteSystems.IRemoteSystemStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("a485b392-ff2b-4b47-be62-743f2f140f30")
                IRemoteSystemStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FindByHostNameAsync(
                        ABI::Windows::Networking::IHostName* hostName,
                        __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWatcher(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemWatcher** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWatcherWithFilters(
                        __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* filters,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemWatcher** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestAccessAsync(
                        __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemStatics = __uuidof(IRemoteSystemStatics);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemStatics2[] = L"Windows.System.RemoteSystems.IRemoteSystemStatics2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("0c98edca-6f99-4c52-a272-ea4f36471744")
                IRemoteSystemStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsAuthorizationKindEnabled(
                        ABI::Windows::System::RemoteSystems::RemoteSystemAuthorizationKind kind,
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemStatics2 = __uuidof(IRemoteSystemStatics2);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemStatics3[] = L"Windows.System.RemoteSystems.IRemoteSystemStatics3";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("9995f16f-0b3c-5ac5-b325-cc73f437dfcd")
                IRemoteSystemStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWatcherForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemWatcher** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWatcherWithFiltersForUser(
                        ABI::Windows::System::IUser* user,
                        __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* filters,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemWatcher** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemStatics3 = __uuidof(IRemoteSystemStatics3);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemStatusTypeFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemStatusTypeFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemStatusTypeFilter[] = L"Windows.System.RemoteSystems.IRemoteSystemStatusTypeFilter";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("0c39514e-cbb6-4777-8534-2e0c521affa2")
                IRemoteSystemStatusTypeFilter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteSystemStatusType(
                        ABI::Windows::System::RemoteSystems::RemoteSystemStatusType* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemStatusTypeFilter = __uuidof(IRemoteSystemStatusTypeFilter);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemStatusTypeFilterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemStatusTypeFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemStatusTypeFilterFactory[] = L"Windows.System.RemoteSystems.IRemoteSystemStatusTypeFilterFactory";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("33cf78fa-d724-4125-ac7a-8d281e44c949")
                IRemoteSystemStatusTypeFilterFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::System::RemoteSystems::RemoteSystemStatusType remoteSystemStatusType,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemStatusTypeFilter** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemStatusTypeFilterFactory = __uuidof(IRemoteSystemStatusTypeFilterFactory);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemUpdatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemUpdatedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemUpdatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("7502ff0e-dbcb-4155-b4ca-b30a04f27627")
                IRemoteSystemUpdatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteSystem(
                        ABI::Windows::System::RemoteSystems::IRemoteSystem** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemUpdatedEventArgs = __uuidof(IRemoteSystemUpdatedEventArgs);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemWatcher[] = L"Windows.System.RemoteSystems.IRemoteSystemWatcher";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("5d600c7e-2c07-48c5-889c-455d2b099771")
                IRemoteSystemWatcher : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_RemoteSystemAdded(
                        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RemoteSystemAdded(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_RemoteSystemUpdated(
                        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RemoteSystemUpdated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_RemoteSystemRemoved(
                        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RemoteSystemRemoved(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemWatcher = __uuidof(IRemoteSystemWatcher);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemWatcher2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemWatcher2[] = L"Windows.System.RemoteSystems.IRemoteSystemWatcher2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("73436700-19ca-48f9-a4cd-780f7ad58c71")
                IRemoteSystemWatcher2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_EnumerationCompleted(
                        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_EnumerationCompleted(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ErrorOccurred(
                        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ErrorOccurred(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemWatcher2 = __uuidof(IRemoteSystemWatcher2);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemWatcher3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemWatcher3[] = L"Windows.System.RemoteSystems.IRemoteSystemWatcher3";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("f79c0fcf-a913-55d3-8413-418fcf15ba54")
                IRemoteSystemWatcher3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemWatcher3 = __uuidof(IRemoteSystemWatcher3);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemWatcherErrorOccurredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemWatcherErrorOccurredEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemWatcherErrorOccurredEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemWatcherErrorOccurredEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("74c5c6af-5114-4426-9216-20d81f8519ae")
                IRemoteSystemWatcherErrorOccurredEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Error(
                        ABI::Windows::System::RemoteSystems::RemoteSystemWatcherError* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemWatcherErrorOccurredEventArgs = __uuidof(IRemoteSystemWatcherErrorOccurredEventArgs);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemWebAccountFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemWebAccountFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemWebAccountFilter[] = L"Windows.System.RemoteSystems.IRemoteSystemWebAccountFilter";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("3fb75873-87c8-5d8f-977e-f69f96d67238")
                IRemoteSystemWebAccountFilter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Account(
                        ABI::Windows::Security::Credentials::IWebAccount** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemWebAccountFilter = __uuidof(IRemoteSystemWebAccountFilter);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemWebAccountFilterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemWebAccountFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemWebAccountFilterFactory[] = L"Windows.System.RemoteSystems.IRemoteSystemWebAccountFilterFactory";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                MIDL_INTERFACE("348a2709-5f4d-5127-b4a7-bf99d5252b1b")
                IRemoteSystemWebAccountFilterFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Security::Credentials::IWebAccount* account,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemWebAccountFilter** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteSystemWebAccountFilterFactory = __uuidof(IRemoteSystemWebAccountFilterFactory);
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.RemoteSystems.KnownRemoteSystemCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteSystems.IKnownRemoteSystemCapabilitiesStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_KnownRemoteSystemCapabilities_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_KnownRemoteSystemCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_KnownRemoteSystemCapabilities[] = L"Windows.System.RemoteSystems.KnownRemoteSystemCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemStatics3 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystem ** Default Interface **
 *    Windows.System.RemoteSystems.IRemoteSystem2
 *    Windows.System.RemoteSystems.IRemoteSystem3
 *    Windows.System.RemoteSystems.IRemoteSystem4
 *    Windows.System.RemoteSystems.IRemoteSystem5
 *    Windows.System.RemoteSystems.IRemoteSystem6
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystem_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystem[] = L"Windows.System.RemoteSystems.RemoteSystem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemAddedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemAddedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemAddedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemAddedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemAddedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemApp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemApp ** Default Interface **
 *    Windows.System.RemoteSystems.IRemoteSystemApp2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemApp_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemApp_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemApp[] = L"Windows.System.RemoteSystems.RemoteSystemApp";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemAppRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemAppRegistrationStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemAppRegistration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemAppRegistration_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemAppRegistration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemAppRegistration[] = L"Windows.System.RemoteSystems.RemoteSystemAppRegistration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemAuthorizationKindFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteSystems.IRemoteSystemAuthorizationKindFilterFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemAuthorizationKindFilter ** Default Interface **
 *    Windows.System.RemoteSystems.IRemoteSystemFilter
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemAuthorizationKindFilter_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemAuthorizationKindFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemAuthorizationKindFilter[] = L"Windows.System.RemoteSystems.RemoteSystemAuthorizationKindFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemConnectionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemConnectionInfoStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemConnectionInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemConnectionInfo_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemConnectionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemConnectionInfo[] = L"Windows.System.RemoteSystems.RemoteSystemConnectionInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemConnectionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteSystems.IRemoteSystemConnectionRequestFactory interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemConnectionRequestStatics2 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemConnectionRequestStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemConnectionRequest ** Default Interface **
 *    Windows.System.RemoteSystems.IRemoteSystemConnectionRequest2
 *    Windows.System.RemoteSystems.IRemoteSystemConnectionRequest3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemConnectionRequest_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemConnectionRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemConnectionRequest[] = L"Windows.System.RemoteSystems.RemoteSystemConnectionRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemDiscoveryTypeFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteSystems.IRemoteSystemDiscoveryTypeFilterFactory interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemDiscoveryTypeFilter ** Default Interface **
 *    Windows.System.RemoteSystems.IRemoteSystemFilter
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemDiscoveryTypeFilter_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemDiscoveryTypeFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemDiscoveryTypeFilter[] = L"Windows.System.RemoteSystems.RemoteSystemDiscoveryTypeFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemEnumerationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemEnumerationCompletedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemEnumerationCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemEnumerationCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemEnumerationCompletedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemEnumerationCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemKindFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteSystems.IRemoteSystemKindFilterFactory interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemKindFilter ** Default Interface **
 *    Windows.System.RemoteSystems.IRemoteSystemFilter
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemKindFilter_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemKindFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemKindFilter[] = L"Windows.System.RemoteSystems.RemoteSystemKindFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemKinds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemKindStatics2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemKindStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemKinds_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemKinds_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemKinds[] = L"Windows.System.RemoteSystems.RemoteSystemKinds";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemRemovedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemRemovedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemRemovedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemRemovedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemRemovedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemSessionStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSession ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSession_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSession[] = L"Windows.System.RemoteSystems.RemoteSystemSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionAddedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionAddedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionAddedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionAddedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemSessionAddedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteSystems.IRemoteSystemSessionControllerFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionController ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionController_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionController[] = L"Windows.System.RemoteSystems.RemoteSystemSessionController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionCreationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionCreationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionCreationResult_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionCreationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionCreationResult[] = L"Windows.System.RemoteSystems.RemoteSystemSessionCreationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionDisconnectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionDisconnectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionDisconnectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionDisconnectedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionInfo_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionInfo[] = L"Windows.System.RemoteSystems.RemoteSystemSessionInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionInvitation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionInvitation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionInvitation_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionInvitation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionInvitation[] = L"Windows.System.RemoteSystems.RemoteSystemSessionInvitation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionInvitationListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionInvitationListener ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionInvitationListener_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionInvitationListener_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionInvitationListener[] = L"Windows.System.RemoteSystems.RemoteSystemSessionInvitationListener";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionInvitationReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionInvitationReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionInvitationReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionInvitationReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionInvitationReceivedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemSessionInvitationReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionJoinRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionJoinRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionJoinRequest_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionJoinRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionJoinRequest[] = L"Windows.System.RemoteSystems.RemoteSystemSessionJoinRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionJoinRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionJoinRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionJoinRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionJoinRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionJoinRequestedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemSessionJoinRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionJoinResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionJoinResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionJoinResult_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionJoinResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionJoinResult[] = L"Windows.System.RemoteSystems.RemoteSystemSessionJoinResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionMessageChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteSystems.IRemoteSystemSessionMessageChannelFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionMessageChannel ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionMessageChannel_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionMessageChannel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionMessageChannel[] = L"Windows.System.RemoteSystems.RemoteSystemSessionMessageChannel";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionOptions_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionOptions[] = L"Windows.System.RemoteSystems.RemoteSystemSessionOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionParticipant
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionParticipant ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionParticipant_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionParticipant_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionParticipant[] = L"Windows.System.RemoteSystems.RemoteSystemSessionParticipant";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionParticipantAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionParticipantAddedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionParticipantAddedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionParticipantAddedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionParticipantAddedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemSessionParticipantAddedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionParticipantRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionParticipantRemovedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionParticipantRemovedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionParticipantRemovedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionParticipantRemovedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemSessionParticipantRemovedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionParticipantWatcher ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionParticipantWatcher_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionParticipantWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionParticipantWatcher[] = L"Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionRemovedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionRemovedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionRemovedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionRemovedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemSessionRemovedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionUpdatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionUpdatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionUpdatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionUpdatedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemSessionUpdatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionValueSetReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionValueSetReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionValueSetReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionValueSetReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionValueSetReceivedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemSessionValueSetReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionWatcher ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionWatcher_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionWatcher[] = L"Windows.System.RemoteSystems.RemoteSystemSessionWatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemStatusTypeFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteSystems.IRemoteSystemStatusTypeFilterFactory interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemStatusTypeFilter ** Default Interface **
 *    Windows.System.RemoteSystems.IRemoteSystemFilter
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemStatusTypeFilter_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemStatusTypeFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemStatusTypeFilter[] = L"Windows.System.RemoteSystems.RemoteSystemStatusTypeFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemUpdatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemUpdatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemUpdatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemUpdatedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemUpdatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemWatcher ** Default Interface **
 *    Windows.System.RemoteSystems.IRemoteSystemWatcher2
 *    Windows.System.RemoteSystems.IRemoteSystemWatcher3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemWatcher_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemWatcher[] = L"Windows.System.RemoteSystems.RemoteSystemWatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemWatcherErrorOccurredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemWatcherErrorOccurredEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemWatcherErrorOccurredEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemWatcherErrorOccurredEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemWatcherErrorOccurredEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemWatcherErrorOccurredEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemWebAccountFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteSystems.IRemoteSystemWebAccountFilterFactory interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemWebAccountFilter ** Default Interface **
 *    Windows.System.RemoteSystems.IRemoteSystemFilter
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemWebAccountFilter_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemWebAccountFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemWebAccountFilter[] = L"Windows.System.RemoteSystems.RemoteSystemWebAccountFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics __x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2 __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3 __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4 __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5 __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6 __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2 __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2 __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3 __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2 __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2 __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2 __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3 __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2 __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3 __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_boolean __FIAsyncOperationCompletedHandler_1_boolean;

#if !defined(____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_boolean __FIAsyncOperation_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_boolean;

typedef struct __FIAsyncOperation_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_boolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_boolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_boolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_boolean* This,
        __FIAsyncOperationCompletedHandler_1_boolean* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_boolean* This,
        __FIAsyncOperationCompletedHandler_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_boolean* This,
        boolean* result);

    END_INTERFACE
} __FIAsyncOperation_1_booleanVtbl;

interface __FIAsyncOperation_1_boolean
{
    CONST_VTBL struct __FIAsyncOperation_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_boolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_boolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_boolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_boolean_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_boolean_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_boolean_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_boolean __FIAsyncOperationCompletedHandler_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_boolean;

typedef struct __FIAsyncOperationCompletedHandler_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_boolean* This,
        __FIAsyncOperation_1_boolean* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_booleanVtbl;

interface __FIAsyncOperationCompletedHandler_1_boolean
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_boolean_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem;

typedef struct __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemVtbl;

interface __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem* This,
        __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemAccessStatus __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemAccessStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus;

typedef struct __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus* This,
        enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemAccessStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatusVtbl;

interface __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus* This,
        __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult;

typedef struct __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResultVtbl;

interface __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult* This,
        __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult;

typedef struct __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResultVtbl;

interface __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult* This,
        __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter;

typedef struct __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilterVtbl;

interface __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter
{
    CONST_VTBL struct __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter;

typedef struct __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* This,
        __FIIterator_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilterVtbl;

interface __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter
{
    CONST_VTBL struct __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp;

typedef struct __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAppVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAppVtbl;

interface __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp
{
    CONST_VTBL struct __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAppVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp;

typedef struct __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAppVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAppVtbl;

interface __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp
{
    CONST_VTBL struct __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAppVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant;

typedef struct __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantVtbl;

interface __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant
{
    CONST_VTBL struct __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant;

typedef struct __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* This,
        __FIIterator_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantVtbl;

interface __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant
{
    CONST_VTBL struct __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp;

typedef struct __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAppVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        UINT32 index,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAppVtbl;

interface __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAppVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession* sender,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController* sender,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener* sender,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel* sender,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher* sender,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher* sender,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher* sender,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher* sender,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher* sender,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* sender,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* sender,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* sender,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* sender,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* sender,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemAuthorizationKind __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemAuthorizationKind;

typedef enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemDiscoveryType __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemDiscoveryType;

typedef enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemPlatform __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemPlatform;

typedef enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionCreationStatus __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionCreationStatus;

typedef enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionDisconnectedReason __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionDisconnectedReason;

typedef enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionJoinStatus __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionJoinStatus;

typedef enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionMessageChannelReliability __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionMessageChannelReliability;

typedef enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionParticipantWatcherStatus __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionParticipantWatcherStatus;

typedef enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionWatcherStatus __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionWatcherStatus;

typedef enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemStatus __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemStatus;

typedef enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemStatusType __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemStatusType;

typedef enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemWatcherError __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemWatcherError;

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemAccessStatus
{
    RemoteSystemAccessStatus_Unspecified = 0,
    RemoteSystemAccessStatus_Allowed = 1,
    RemoteSystemAccessStatus_DeniedByUser = 2,
    RemoteSystemAccessStatus_DeniedBySystem = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemAuthorizationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemAuthorizationKind
{
    RemoteSystemAuthorizationKind_SameUser = 0,
    RemoteSystemAuthorizationKind_Anonymous = 1,
    RemoteSystemAuthorizationKind_SameFamily = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemDiscoveryType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemDiscoveryType
{
    RemoteSystemDiscoveryType_Any = 0,
    RemoteSystemDiscoveryType_Proximal = 1,
    RemoteSystemDiscoveryType_Cloud = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    RemoteSystemDiscoveryType_SpatiallyProximal = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemPlatform
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemPlatform
{
    RemoteSystemPlatform_Unknown = 0,
    RemoteSystemPlatform_Windows = 1,
    RemoteSystemPlatform_Android = 2,
    RemoteSystemPlatform_Ios = 3,
    RemoteSystemPlatform_Linux = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemSessionCreationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionCreationStatus
{
    RemoteSystemSessionCreationStatus_Success = 0,
    RemoteSystemSessionCreationStatus_SessionLimitsExceeded = 1,
    RemoteSystemSessionCreationStatus_OperationAborted = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionDisconnectedReason
{
    RemoteSystemSessionDisconnectedReason_SessionUnavailable = 0,
    RemoteSystemSessionDisconnectedReason_RemovedByController = 1,
    RemoteSystemSessionDisconnectedReason_SessionClosed = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemSessionJoinStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionJoinStatus
{
    RemoteSystemSessionJoinStatus_Success = 0,
    RemoteSystemSessionJoinStatus_SessionLimitsExceeded = 1,
    RemoteSystemSessionJoinStatus_OperationAborted = 2,
    RemoteSystemSessionJoinStatus_SessionUnavailable = 3,
    RemoteSystemSessionJoinStatus_RejectedByController = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemSessionMessageChannelReliability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionMessageChannelReliability
{
    RemoteSystemSessionMessageChannelReliability_Reliable = 0,
    RemoteSystemSessionMessageChannelReliability_Unreliable = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcherStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionParticipantWatcherStatus
{
    RemoteSystemSessionParticipantWatcherStatus_Created = 0,
    RemoteSystemSessionParticipantWatcherStatus_Started = 1,
    RemoteSystemSessionParticipantWatcherStatus_EnumerationCompleted = 2,
    RemoteSystemSessionParticipantWatcherStatus_Stopping = 3,
    RemoteSystemSessionParticipantWatcherStatus_Stopped = 4,
    RemoteSystemSessionParticipantWatcherStatus_Aborted = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemSessionWatcherStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionWatcherStatus
{
    RemoteSystemSessionWatcherStatus_Created = 0,
    RemoteSystemSessionWatcherStatus_Started = 1,
    RemoteSystemSessionWatcherStatus_EnumerationCompleted = 2,
    RemoteSystemSessionWatcherStatus_Stopping = 3,
    RemoteSystemSessionWatcherStatus_Stopped = 4,
    RemoteSystemSessionWatcherStatus_Aborted = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemStatus
{
    RemoteSystemStatus_Unavailable = 0,
    RemoteSystemStatus_DiscoveringAvailability = 1,
    RemoteSystemStatus_Available = 2,
    RemoteSystemStatus_Unknown = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemStatusType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemStatusType
{
    RemoteSystemStatusType_Any = 0,
    RemoteSystemStatusType_Available = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.System.RemoteSystems.RemoteSystemWatcherError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemWatcherError
{
    RemoteSystemWatcherError_Unknown = 0,
    RemoteSystemWatcherError_InternetNotAvailable = 1,
    RemoteSystemWatcherError_AuthenticationError = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.RemoteSystems.IKnownRemoteSystemCapabilitiesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.KnownRemoteSystemCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IKnownRemoteSystemCapabilitiesStatics[] = L"Windows.System.RemoteSystems.IKnownRemoteSystemCapabilitiesStatics";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppService)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LaunchUri)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteSession)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SpatialEntity)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_get_AppService(This, value) \
    ((This)->lpVtbl->get_AppService(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_get_LaunchUri(This, value) \
    ((This)->lpVtbl->get_LaunchUri(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_get_RemoteSession(This, value) \
    ((This)->lpVtbl->get_RemoteSession(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_get_SpatialEntity(This, value) \
    ((This)->lpVtbl->get_SpatialEntity(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIKnownRemoteSystemCapabilitiesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystem[] = L"Windows.System.RemoteSystems.IRemoteSystem";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem* This,
        enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_IsAvailableByProximity)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_get_IsAvailableByProximity(This, value) \
    ((This)->lpVtbl->get_IsAvailableByProximity(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystem2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystem2[] = L"Windows.System.RemoteSystems.IRemoteSystem2";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsAvailableBySpatialProximity)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetCapabilitySupportedAsync)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2* This,
        HSTRING capabilityName,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2Vtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_get_IsAvailableBySpatialProximity(This, value) \
    ((This)->lpVtbl->get_IsAvailableBySpatialProximity(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_GetCapabilitySupportedAsync(This, capabilityName, operation) \
    ((This)->lpVtbl->GetCapabilitySupportedAsync(This, capabilityName, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystem3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystem3[] = L"Windows.System.RemoteSystems.IRemoteSystem3";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ManufacturerDisplayName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ModelDisplayName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3Vtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_get_ManufacturerDisplayName(This, value) \
    ((This)->lpVtbl->get_ManufacturerDisplayName(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_get_ModelDisplayName(This, value) \
    ((This)->lpVtbl->get_ModelDisplayName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystem4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystem4[] = L"Windows.System.RemoteSystems.IRemoteSystem4";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Platform)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4* This,
        enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemPlatform* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4Vtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_get_Platform(This, value) \
    ((This)->lpVtbl->get_Platform(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystem5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystem5[] = L"Windows.System.RemoteSystems.IRemoteSystem5";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Apps)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5* This,
        __FIVectorView_1_Windows__CSystem__CRemoteSystems__CRemoteSystemApp** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5Vtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_get_Apps(This, value) \
    ((This)->lpVtbl->get_Apps(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystem6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystem6[] = L"Windows.System.RemoteSystems.IRemoteSystem6";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6Vtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemAddedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemAddedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemAddedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RemoteSystem)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_get_RemoteSystem(This, value) \
    ((This)->lpVtbl->get_RemoteSystem(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAddedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemApp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemApp[] = L"Windows.System.RemoteSystems.IRemoteSystemApp";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsAvailableByProximity)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsAvailableBySpatialProximity)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Attributes)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp* This,
        __FIMapView_2_HSTRING_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_get_IsAvailableByProximity(This, value) \
    ((This)->lpVtbl->get_IsAvailableByProximity(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_get_IsAvailableBySpatialProximity(This, value) \
    ((This)->lpVtbl->get_IsAvailableBySpatialProximity(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_get_Attributes(This, value) \
    ((This)->lpVtbl->get_Attributes(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemApp2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemApp2[] = L"Windows.System.RemoteSystems.IRemoteSystemApp2";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2* This,
        __x_ABI_CWindows_CSystem_CIUser** value);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionToken)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2Vtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_get_ConnectionToken(This, value) \
    ((This)->lpVtbl->get_ConnectionToken(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemAppRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemAppRegistration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemAppRegistration[] = L"Windows.System.RemoteSystems.IRemoteSystemAppRegistration";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration* This,
        __x_ABI_CWindows_CSystem_CIUser** value);
    HRESULT (STDMETHODCALLTYPE* get_Attributes)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration* This,
        __FIMap_2_HSTRING_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* SaveAsync)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration* This,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_get_Attributes(This, value) \
    ((This)->lpVtbl->get_Attributes(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_SaveAsync(This, operation) \
    ((This)->lpVtbl->SaveAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemAppRegistrationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemAppRegistration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemAppRegistrationStatics[] = L"Windows.System.RemoteSystems.IRemoteSystemAppRegistrationStatics";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration** result);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistration** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_GetForUser(This, user, result) \
    ((This)->lpVtbl->GetForUser(This, user, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAppRegistrationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemAuthorizationKindFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemAuthorizationKindFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemAuthorizationKindFilter[] = L"Windows.System.RemoteSystems.IRemoteSystemAuthorizationKindFilter";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RemoteSystemAuthorizationKind)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter* This,
        enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemAuthorizationKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_get_RemoteSystemAuthorizationKind(This, value) \
    ((This)->lpVtbl->get_RemoteSystemAuthorizationKind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemAuthorizationKindFilterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemAuthorizationKindFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemAuthorizationKindFilterFactory[] = L"Windows.System.RemoteSystems.IRemoteSystemAuthorizationKindFilterFactory";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory* This,
        enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemAuthorizationKind remoteSystemAuthorizationKind,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilter** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactoryVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_Create(This, remoteSystemAuthorizationKind, value) \
    ((This)->lpVtbl->Create(This, remoteSystemAuthorizationKind, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemAuthorizationKindFilterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemConnectionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemConnectionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemConnectionInfo[] = L"Windows.System.RemoteSystems.IRemoteSystemConnectionInfo";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsProximal)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_get_IsProximal(This, value) \
    ((This)->lpVtbl->get_IsProximal(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemConnectionInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemConnectionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemConnectionInfoStatics[] = L"Windows.System.RemoteSystems.IRemoteSystemConnectionInfoStatics";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryCreateFromAppServiceConnection)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics* This,
        __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* connection,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfo** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_TryCreateFromAppServiceConnection(This, connection, result) \
    ((This)->lpVtbl->TryCreateFromAppServiceConnection(This, connection, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemConnectionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemConnectionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemConnectionRequest[] = L"Windows.System.RemoteSystems.IRemoteSystemConnectionRequest";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RemoteSystem)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_get_RemoteSystem(This, value) \
    ((This)->lpVtbl->get_RemoteSystem(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemConnectionRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemConnectionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemConnectionRequest2[] = L"Windows.System.RemoteSystems.IRemoteSystemConnectionRequest2";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RemoteSystemApp)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2Vtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_get_RemoteSystemApp(This, value) \
    ((This)->lpVtbl->get_RemoteSystemApp(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemConnectionRequest3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemConnectionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemConnectionRequest3[] = L"Windows.System.RemoteSystems.IRemoteSystemConnectionRequest3";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionToken)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3Vtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_get_ConnectionToken(This, value) \
    ((This)->lpVtbl->get_ConnectionToken(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemConnectionRequestFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemConnectionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemConnectionRequestFactory[] = L"Windows.System.RemoteSystems.IRemoteSystemConnectionRequestFactory";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem* remoteSystem,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactoryVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_Create(This, remoteSystem, value) \
    ((This)->lpVtbl->Create(This, remoteSystem, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemConnectionRequestStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemConnectionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemConnectionRequestStatics[] = L"Windows.System.RemoteSystems.IRemoteSystemConnectionRequestStatics";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateForApp)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemApp* remoteSystemApp,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_CreateForApp(This, remoteSystemApp, result) \
    ((This)->lpVtbl->CreateForApp(This, remoteSystemApp, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemConnectionRequestStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemConnectionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemConnectionRequestStatics2[] = L"Windows.System.RemoteSystems.IRemoteSystemConnectionRequestStatics2";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromConnectionToken)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2* This,
        HSTRING connectionToken,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromConnectionTokenForUser)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING connectionToken,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2Vtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_CreateFromConnectionToken(This, connectionToken, result) \
    ((This)->lpVtbl->CreateFromConnectionToken(This, connectionToken, result))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_CreateFromConnectionTokenForUser(This, user, connectionToken, result) \
    ((This)->lpVtbl->CreateFromConnectionTokenForUser(This, user, connectionToken, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequestStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemDiscoveryTypeFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemDiscoveryTypeFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemDiscoveryTypeFilter[] = L"Windows.System.RemoteSystems.IRemoteSystemDiscoveryTypeFilter";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RemoteSystemDiscoveryType)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter* This,
        enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemDiscoveryType* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_get_RemoteSystemDiscoveryType(This, value) \
    ((This)->lpVtbl->get_RemoteSystemDiscoveryType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemDiscoveryTypeFilterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemDiscoveryTypeFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemDiscoveryTypeFilterFactory[] = L"Windows.System.RemoteSystems.IRemoteSystemDiscoveryTypeFilterFactory";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory* This,
        enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemDiscoveryType discoveryType,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilter** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactoryVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_Create(This, discoveryType, value) \
    ((This)->lpVtbl->Create(This, discoveryType, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemDiscoveryTypeFilterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemEnumerationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemEnumerationCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemEnumerationCompletedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemEnumerationCompletedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemEnumerationCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemFilter[] = L"Windows.System.RemoteSystems.IRemoteSystemFilter";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilterVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemKindFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemKindFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemKindFilter[] = L"Windows.System.RemoteSystems.IRemoteSystemKindFilter";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RemoteSystemKinds)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter* This,
        __FIVectorView_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_get_RemoteSystemKinds(This, value) \
    ((This)->lpVtbl->get_RemoteSystemKinds(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemKindFilterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemKindFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemKindFilterFactory[] = L"Windows.System.RemoteSystems.IRemoteSystemKindFilterFactory";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory* This,
        __FIIterable_1_HSTRING* remoteSystemKinds,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilter** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactoryVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_Create(This, remoteSystemKinds, value) \
    ((This)->lpVtbl->Create(This, remoteSystemKinds, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindFilterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemKindStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemKinds
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemKindStatics[] = L"Windows.System.RemoteSystems.IRemoteSystemKindStatics";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Phone)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Hub)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Holographic)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Desktop)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Xbox)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_get_Phone(This, value) \
    ((This)->lpVtbl->get_Phone(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_get_Hub(This, value) \
    ((This)->lpVtbl->get_Hub(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_get_Holographic(This, value) \
    ((This)->lpVtbl->get_Holographic(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_get_Desktop(This, value) \
    ((This)->lpVtbl->get_Desktop(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_get_Xbox(This, value) \
    ((This)->lpVtbl->get_Xbox(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemKindStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemKinds
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemKindStatics2[] = L"Windows.System.RemoteSystems.IRemoteSystemKindStatics2";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Iot)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Tablet)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Laptop)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2Vtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_get_Iot(This, value) \
    ((This)->lpVtbl->get_Iot(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_get_Tablet(This, value) \
    ((This)->lpVtbl->get_Tablet(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_get_Laptop(This, value) \
    ((This)->lpVtbl->get_Laptop(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemKindStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemRemovedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemRemovedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemRemovedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RemoteSystemId)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_get_RemoteSystemId(This, value) \
    ((This)->lpVtbl->get_RemoteSystemId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemRemovedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSession[] = L"Windows.System.RemoteSystems.IRemoteSystemSession";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ControllerDisplayName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* add_Disconnected)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession* This,
        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSession_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionDisconnectedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Disconnected)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* CreateParticipantWatcher)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher** result);
    HRESULT (STDMETHODCALLTYPE* SendInvitationAsync)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem* invitee,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_get_ControllerDisplayName(This, value) \
    ((This)->lpVtbl->get_ControllerDisplayName(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_add_Disconnected(This, handler, token) \
    ((This)->lpVtbl->add_Disconnected(This, handler, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_remove_Disconnected(This, token) \
    ((This)->lpVtbl->remove_Disconnected(This, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_CreateParticipantWatcher(This, result) \
    ((This)->lpVtbl->CreateParticipantWatcher(This, result))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_SendInvitationAsync(This, invitee, operation) \
    ((This)->lpVtbl->SendInvitationAsync(This, invitee, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionAddedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionAddedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionAddedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SessionInfo)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_get_SessionInfo(This, value) \
    ((This)->lpVtbl->get_SessionInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionAddedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionController[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionController";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_JoinRequested)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController* This,
        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionController_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_JoinRequested)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* RemoveParticipantAsync)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant* pParticipant,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* CreateSessionAsync)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController* This,
        __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionCreationResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_add_JoinRequested(This, handler, token) \
    ((This)->lpVtbl->add_JoinRequested(This, handler, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_remove_JoinRequested(This, token) \
    ((This)->lpVtbl->remove_JoinRequested(This, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_RemoveParticipantAsync(This, pParticipant, operation) \
    ((This)->lpVtbl->RemoveParticipantAsync(This, pParticipant, operation))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_CreateSessionAsync(This, operation) \
    ((This)->lpVtbl->CreateSessionAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionControllerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionControllerFactory[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionControllerFactory";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateController)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory* This,
        HSTRING displayName,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController** value);
    HRESULT (STDMETHODCALLTYPE* CreateControllerWithSessionOptions)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory* This,
        HSTRING displayName,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions* options,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionController** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactoryVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_CreateController(This, displayName, value) \
    ((This)->lpVtbl->CreateController(This, displayName, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_CreateControllerWithSessionOptions(This, displayName, options, value) \
    ((This)->lpVtbl->CreateControllerWithSessionOptions(This, displayName, options, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionControllerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionCreationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionCreationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionCreationResult[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionCreationResult";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult* This,
        enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionCreationStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Session)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResultVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_get_Session(This, value) \
    ((This)->lpVtbl->get_Session(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionCreationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionDisconnectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionDisconnectedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionDisconnectedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Reason)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs* This,
        enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionDisconnectedReason* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_get_Reason(This, value) \
    ((This)->lpVtbl->get_Reason(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionDisconnectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionInfo[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionInfo";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ControllerDisplayName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* JoinAsync)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo* This,
        __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionJoinResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfoVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_get_ControllerDisplayName(This, value) \
    ((This)->lpVtbl->get_ControllerDisplayName(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_JoinAsync(This, operation) \
    ((This)->lpVtbl->JoinAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionInvitation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionInvitation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionInvitation[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionInvitation";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Sender)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem** value);
    HRESULT (STDMETHODCALLTYPE* get_SessionInfo)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_get_Sender(This, value) \
    ((This)->lpVtbl->get_Sender(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_get_SessionInfo(This, value) \
    ((This)->lpVtbl->get_SessionInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionInvitationListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionInvitationListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionInvitationListener[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionInvitationListener";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListenerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_InvitationReceived)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener* This,
        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationListener_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionInvitationReceivedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_InvitationReceived)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListenerVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListenerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_add_InvitationReceived(This, handler, token) \
    ((This)->lpVtbl->add_InvitationReceived(This, handler, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_remove_InvitationReceived(This, token) \
    ((This)->lpVtbl->remove_InvitationReceived(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationListener_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionInvitationReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionInvitationReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionInvitationReceivedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionInvitationReceivedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Invitation)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitation** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_get_Invitation(This, value) \
    ((This)->lpVtbl->get_Invitation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInvitationReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionJoinRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionJoinRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionJoinRequest[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionJoinRequest";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Participant)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant** value);
    HRESULT (STDMETHODCALLTYPE* Accept)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest* This);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_get_Participant(This, value) \
    ((This)->lpVtbl->get_Participant(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_Accept(This) \
    ((This)->lpVtbl->Accept(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionJoinRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionJoinRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionJoinRequestedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionJoinRequestedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_JoinRequest)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_get_JoinRequest(This, value) \
    ((This)->lpVtbl->get_JoinRequest(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionJoinResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionJoinResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionJoinResult[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionJoinResult";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult* This,
        enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionJoinStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Session)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResultVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_get_Session(This, value) \
    ((This)->lpVtbl->get_Session(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionJoinResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionMessageChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionMessageChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionMessageChannel[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionMessageChannel";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Session)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession** value);
    HRESULT (STDMETHODCALLTYPE* BroadcastValueSetAsync)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* messageData,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* SendValueSetAsync)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* messageData,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant* participant,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* SendValueSetToParticipantsAsync)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* messageData,
        __FIIterable_1_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipant* participants,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* add_ValueSetReceived)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel* This,
        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionMessageChannel_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionValueSetReceivedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ValueSetReceived)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_get_Session(This, value) \
    ((This)->lpVtbl->get_Session(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_BroadcastValueSetAsync(This, messageData, operation) \
    ((This)->lpVtbl->BroadcastValueSetAsync(This, messageData, operation))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_SendValueSetAsync(This, messageData, participant, operation) \
    ((This)->lpVtbl->SendValueSetAsync(This, messageData, participant, operation))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_SendValueSetToParticipantsAsync(This, messageData, participants, operation) \
    ((This)->lpVtbl->SendValueSetToParticipantsAsync(This, messageData, participants, operation))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_add_ValueSetReceived(This, handler, token) \
    ((This)->lpVtbl->add_ValueSetReceived(This, handler, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_remove_ValueSetReceived(This, token) \
    ((This)->lpVtbl->remove_ValueSetReceived(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionMessageChannelFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionMessageChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionMessageChannelFactory[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionMessageChannelFactory";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession* session,
        HSTRING channelName,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithReliability)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession* session,
        HSTRING channelName,
        enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionMessageChannelReliability reliability,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannel** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactoryVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_Create(This, session, channelName, value) \
    ((This)->lpVtbl->Create(This, session, channelName, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_CreateWithReliability(This, session, channelName, reliability, value) \
    ((This)->lpVtbl->CreateWithReliability(This, session, channelName, reliability, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionMessageChannelFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionOptions[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionOptions";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsInviteOnly)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsInviteOnly)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptionsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_get_IsInviteOnly(This, value) \
    ((This)->lpVtbl->get_IsInviteOnly(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_put_IsInviteOnly(This, value) \
    ((This)->lpVtbl->put_IsInviteOnly(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionParticipant
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionParticipant
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionParticipant[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionParticipant";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RemoteSystem)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem** value);
    HRESULT (STDMETHODCALLTYPE* GetHostNames)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant* This,
        __FIVectorView_1_Windows__CNetworking__CHostName** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_get_RemoteSystem(This, value) \
    ((This)->lpVtbl->get_RemoteSystem(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_GetHostNames(This, result) \
    ((This)->lpVtbl->GetHostNames(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionParticipantAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionParticipantAddedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionParticipantAddedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionParticipantAddedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Participant)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_get_Participant(This, value) \
    ((This)->lpVtbl->get_Participant(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantAddedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionParticipantRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionParticipantRemovedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionParticipantRemovedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionParticipantRemovedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Participant)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_get_Participant(This, value) \
    ((This)->lpVtbl->get_Participant(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantRemovedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionParticipantWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionParticipantWatcher[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionParticipantWatcher";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcherVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher* This);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher* This,
        enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionParticipantWatcherStatus* value);
    HRESULT (STDMETHODCALLTYPE* add_Added)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher* This,
        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantAddedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Added)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Removed)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher* This,
        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantRemovedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Removed)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_EnumerationCompleted)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher* This,
        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionParticipantWatcher_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_EnumerationCompleted)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcherVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcherVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_add_Added(This, handler, token) \
    ((This)->lpVtbl->add_Added(This, handler, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_remove_Added(This, token) \
    ((This)->lpVtbl->remove_Added(This, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_add_Removed(This, handler, token) \
    ((This)->lpVtbl->add_Removed(This, handler, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_remove_Removed(This, token) \
    ((This)->lpVtbl->remove_Removed(This, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_add_EnumerationCompleted(This, handler, token) \
    ((This)->lpVtbl->add_EnumerationCompleted(This, handler, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_remove_EnumerationCompleted(This, token) \
    ((This)->lpVtbl->remove_EnumerationCompleted(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipantWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionRemovedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionRemovedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionRemovedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SessionInfo)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_get_SessionInfo(This, value) \
    ((This)->lpVtbl->get_SessionInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionRemovedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionStatics[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionStatics";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWatcher)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_CreateWatcher(This, result) \
    ((This)->lpVtbl->CreateWatcher(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionUpdatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionUpdatedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionUpdatedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SessionInfo)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_get_SessionInfo(This, value) \
    ((This)->lpVtbl->get_SessionInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionUpdatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionValueSetReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionValueSetReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionValueSetReceivedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionValueSetReceivedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Sender)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionParticipant** value);
    HRESULT (STDMETHODCALLTYPE* get_Message)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_get_Sender(This, value) \
    ((This)->lpVtbl->get_Sender(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_get_Message(This, value) \
    ((This)->lpVtbl->get_Message(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionValueSetReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemSessionWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemSessionWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemSessionWatcher[] = L"Windows.System.RemoteSystems.IRemoteSystemSessionWatcher";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcherVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher* This);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher* This,
        enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemSessionWatcherStatus* value);
    HRESULT (STDMETHODCALLTYPE* add_Added)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher* This,
        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionAddedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Added)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Updated)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher* This,
        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionUpdatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Updated)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Removed)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher* This,
        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemSessionRemovedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Removed)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcherVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcherVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_add_Added(This, handler, token) \
    ((This)->lpVtbl->add_Added(This, handler, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_remove_Added(This, token) \
    ((This)->lpVtbl->remove_Added(This, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_add_Updated(This, handler, token) \
    ((This)->lpVtbl->add_Updated(This, handler, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_remove_Updated(This, token) \
    ((This)->lpVtbl->remove_Updated(This, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_add_Removed(This, handler, token) \
    ((This)->lpVtbl->add_Removed(This, handler, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_remove_Removed(This, token) \
    ((This)->lpVtbl->remove_Removed(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSessionWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemStatics[] = L"Windows.System.RemoteSystems.IRemoteSystemStatics";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindByHostNameAsync)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics* This,
        __x_ABI_CWindows_CNetworking_CIHostName* hostName,
        __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystem** operation);
    HRESULT (STDMETHODCALLTYPE* CreateWatcher)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher** result);
    HRESULT (STDMETHODCALLTYPE* CreateWatcherWithFilters)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics* This,
        __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* filters,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher** result);
    HRESULT (STDMETHODCALLTYPE* RequestAccessAsync)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics* This,
        __FIAsyncOperation_1_Windows__CSystem__CRemoteSystems__CRemoteSystemAccessStatus** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_FindByHostNameAsync(This, hostName, operation) \
    ((This)->lpVtbl->FindByHostNameAsync(This, hostName, operation))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_CreateWatcher(This, result) \
    ((This)->lpVtbl->CreateWatcher(This, result))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_CreateWatcherWithFilters(This, filters, result) \
    ((This)->lpVtbl->CreateWatcherWithFilters(This, filters, result))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_RequestAccessAsync(This, operation) \
    ((This)->lpVtbl->RequestAccessAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemStatics2[] = L"Windows.System.RemoteSystems.IRemoteSystemStatics2";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsAuthorizationKindEnabled)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2* This,
        enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemAuthorizationKind kind,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2Vtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_IsAuthorizationKindEnabled(This, kind, result) \
    ((This)->lpVtbl->IsAuthorizationKindEnabled(This, kind, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemStatics3[] = L"Windows.System.RemoteSystems.IRemoteSystemStatics3";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWatcherForUser)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher** result);
    HRESULT (STDMETHODCALLTYPE* CreateWatcherWithFiltersForUser)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __FIIterable_1_Windows__CSystem__CRemoteSystems__CIRemoteSystemFilter* filters,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3Vtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_CreateWatcherForUser(This, user, result) \
    ((This)->lpVtbl->CreateWatcherForUser(This, user, result))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_CreateWatcherWithFiltersForUser(This, user, filters, result) \
    ((This)->lpVtbl->CreateWatcherWithFiltersForUser(This, user, filters, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemStatusTypeFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemStatusTypeFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemStatusTypeFilter[] = L"Windows.System.RemoteSystems.IRemoteSystemStatusTypeFilter";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RemoteSystemStatusType)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter* This,
        enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemStatusType* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_get_RemoteSystemStatusType(This, value) \
    ((This)->lpVtbl->get_RemoteSystemStatusType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemStatusTypeFilterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemStatusTypeFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemStatusTypeFilterFactory[] = L"Windows.System.RemoteSystems.IRemoteSystemStatusTypeFilterFactory";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory* This,
        enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemStatusType remoteSystemStatusType,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilter** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactoryVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_Create(This, remoteSystemStatusType, value) \
    ((This)->lpVtbl->Create(This, remoteSystemStatusType, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemStatusTypeFilterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemUpdatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemUpdatedEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemUpdatedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RemoteSystem)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystem** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_get_RemoteSystem(This, value) \
    ((This)->lpVtbl->get_RemoteSystem(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemUpdatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemWatcher[] = L"Windows.System.RemoteSystems.IRemoteSystemWatcher";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* This);
    HRESULT (STDMETHODCALLTYPE* add_RemoteSystemAdded)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* This,
        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemAddedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RemoteSystemAdded)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_RemoteSystemUpdated)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* This,
        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemUpdatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RemoteSystemUpdated)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_RemoteSystemRemoved)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* This,
        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemRemovedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RemoteSystemRemoved)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_add_RemoteSystemAdded(This, handler, token) \
    ((This)->lpVtbl->add_RemoteSystemAdded(This, handler, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_remove_RemoteSystemAdded(This, token) \
    ((This)->lpVtbl->remove_RemoteSystemAdded(This, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_add_RemoteSystemUpdated(This, handler, token) \
    ((This)->lpVtbl->add_RemoteSystemUpdated(This, handler, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_remove_RemoteSystemUpdated(This, token) \
    ((This)->lpVtbl->remove_RemoteSystemUpdated(This, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_add_RemoteSystemRemoved(This, handler, token) \
    ((This)->lpVtbl->add_RemoteSystemRemoved(This, handler, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_remove_RemoteSystemRemoved(This, token) \
    ((This)->lpVtbl->remove_RemoteSystemRemoved(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemWatcher2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemWatcher2[] = L"Windows.System.RemoteSystems.IRemoteSystemWatcher2";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_EnumerationCompleted)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2* This,
        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemEnumerationCompletedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_EnumerationCompleted)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ErrorOccurred)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2* This,
        __FITypedEventHandler_2_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcher_Windows__CSystem__CRemoteSystems__CRemoteSystemWatcherErrorOccurredEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ErrorOccurred)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2Vtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_add_EnumerationCompleted(This, handler, token) \
    ((This)->lpVtbl->add_EnumerationCompleted(This, handler, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_remove_EnumerationCompleted(This, token) \
    ((This)->lpVtbl->remove_EnumerationCompleted(This, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_add_ErrorOccurred(This, handler, token) \
    ((This)->lpVtbl->add_ErrorOccurred(This, handler, token))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_remove_ErrorOccurred(This, token) \
    ((This)->lpVtbl->remove_ErrorOccurred(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemWatcher3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemWatcher3[] = L"Windows.System.RemoteSystems.IRemoteSystemWatcher3";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3Vtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcher3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemWatcherErrorOccurredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemWatcherErrorOccurredEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemWatcherErrorOccurredEventArgs[] = L"Windows.System.RemoteSystems.IRemoteSystemWatcherErrorOccurredEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Error)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs* This,
        enum __x_ABI_CWindows_CSystem_CRemoteSystems_CRemoteSystemWatcherError* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_get_Error(This, value) \
    ((This)->lpVtbl->get_Error(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWatcherErrorOccurredEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemWebAccountFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemWebAccountFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemWebAccountFilter[] = L"Windows.System.RemoteSystems.IRemoteSystemWebAccountFilter";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Account)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_get_Account(This, value) \
    ((This)->lpVtbl->get_Account(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.RemoteSystems.IRemoteSystemWebAccountFilterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteSystems.RemoteSystemWebAccountFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteSystems_IRemoteSystemWebAccountFilterFactory[] = L"Windows.System.RemoteSystems.IRemoteSystemWebAccountFilterFactory";
typedef struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* account,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilter** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactoryVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_Create(This, account, value) \
    ((This)->lpVtbl->Create(This, account, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemWebAccountFilterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.RemoteSystems.KnownRemoteSystemCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteSystems.IKnownRemoteSystemCapabilitiesStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_KnownRemoteSystemCapabilities_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_KnownRemoteSystemCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_KnownRemoteSystemCapabilities[] = L"Windows.System.RemoteSystems.KnownRemoteSystemCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemStatics3 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystem ** Default Interface **
 *    Windows.System.RemoteSystems.IRemoteSystem2
 *    Windows.System.RemoteSystems.IRemoteSystem3
 *    Windows.System.RemoteSystems.IRemoteSystem4
 *    Windows.System.RemoteSystems.IRemoteSystem5
 *    Windows.System.RemoteSystems.IRemoteSystem6
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystem_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystem[] = L"Windows.System.RemoteSystems.RemoteSystem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemAddedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemAddedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemAddedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemAddedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemAddedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemApp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemApp ** Default Interface **
 *    Windows.System.RemoteSystems.IRemoteSystemApp2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemApp_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemApp_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemApp[] = L"Windows.System.RemoteSystems.RemoteSystemApp";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemAppRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemAppRegistrationStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemAppRegistration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemAppRegistration_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemAppRegistration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemAppRegistration[] = L"Windows.System.RemoteSystems.RemoteSystemAppRegistration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemAuthorizationKindFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteSystems.IRemoteSystemAuthorizationKindFilterFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemAuthorizationKindFilter ** Default Interface **
 *    Windows.System.RemoteSystems.IRemoteSystemFilter
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemAuthorizationKindFilter_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemAuthorizationKindFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemAuthorizationKindFilter[] = L"Windows.System.RemoteSystems.RemoteSystemAuthorizationKindFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemConnectionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemConnectionInfoStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemConnectionInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemConnectionInfo_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemConnectionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemConnectionInfo[] = L"Windows.System.RemoteSystems.RemoteSystemConnectionInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemConnectionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteSystems.IRemoteSystemConnectionRequestFactory interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemConnectionRequestStatics2 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemConnectionRequestStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemConnectionRequest ** Default Interface **
 *    Windows.System.RemoteSystems.IRemoteSystemConnectionRequest2
 *    Windows.System.RemoteSystems.IRemoteSystemConnectionRequest3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemConnectionRequest_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemConnectionRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemConnectionRequest[] = L"Windows.System.RemoteSystems.RemoteSystemConnectionRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemDiscoveryTypeFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteSystems.IRemoteSystemDiscoveryTypeFilterFactory interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemDiscoveryTypeFilter ** Default Interface **
 *    Windows.System.RemoteSystems.IRemoteSystemFilter
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemDiscoveryTypeFilter_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemDiscoveryTypeFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemDiscoveryTypeFilter[] = L"Windows.System.RemoteSystems.RemoteSystemDiscoveryTypeFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemEnumerationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemEnumerationCompletedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemEnumerationCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemEnumerationCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemEnumerationCompletedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemEnumerationCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemKindFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteSystems.IRemoteSystemKindFilterFactory interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemKindFilter ** Default Interface **
 *    Windows.System.RemoteSystems.IRemoteSystemFilter
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemKindFilter_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemKindFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemKindFilter[] = L"Windows.System.RemoteSystems.RemoteSystemKindFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemKinds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemKindStatics2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemKindStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemKinds_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemKinds_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemKinds[] = L"Windows.System.RemoteSystems.RemoteSystemKinds";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemRemovedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemRemovedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemRemovedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemRemovedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemRemovedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteSystems.IRemoteSystemSessionStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSession ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSession_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSession[] = L"Windows.System.RemoteSystems.RemoteSystemSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionAddedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionAddedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionAddedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionAddedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemSessionAddedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteSystems.IRemoteSystemSessionControllerFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionController ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionController_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionController[] = L"Windows.System.RemoteSystems.RemoteSystemSessionController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionCreationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionCreationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionCreationResult_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionCreationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionCreationResult[] = L"Windows.System.RemoteSystems.RemoteSystemSessionCreationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionDisconnectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionDisconnectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionDisconnectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionDisconnectedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionInfo_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionInfo[] = L"Windows.System.RemoteSystems.RemoteSystemSessionInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionInvitation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionInvitation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionInvitation_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionInvitation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionInvitation[] = L"Windows.System.RemoteSystems.RemoteSystemSessionInvitation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionInvitationListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionInvitationListener ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionInvitationListener_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionInvitationListener_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionInvitationListener[] = L"Windows.System.RemoteSystems.RemoteSystemSessionInvitationListener";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionInvitationReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionInvitationReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionInvitationReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionInvitationReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionInvitationReceivedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemSessionInvitationReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionJoinRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionJoinRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionJoinRequest_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionJoinRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionJoinRequest[] = L"Windows.System.RemoteSystems.RemoteSystemSessionJoinRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionJoinRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionJoinRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionJoinRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionJoinRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionJoinRequestedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemSessionJoinRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionJoinResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionJoinResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionJoinResult_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionJoinResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionJoinResult[] = L"Windows.System.RemoteSystems.RemoteSystemSessionJoinResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionMessageChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteSystems.IRemoteSystemSessionMessageChannelFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionMessageChannel ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionMessageChannel_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionMessageChannel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionMessageChannel[] = L"Windows.System.RemoteSystems.RemoteSystemSessionMessageChannel";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionOptions_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionOptions[] = L"Windows.System.RemoteSystems.RemoteSystemSessionOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionParticipant
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionParticipant ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionParticipant_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionParticipant_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionParticipant[] = L"Windows.System.RemoteSystems.RemoteSystemSessionParticipant";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionParticipantAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionParticipantAddedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionParticipantAddedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionParticipantAddedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionParticipantAddedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemSessionParticipantAddedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionParticipantRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionParticipantRemovedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionParticipantRemovedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionParticipantRemovedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionParticipantRemovedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemSessionParticipantRemovedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionParticipantWatcher ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionParticipantWatcher_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionParticipantWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionParticipantWatcher[] = L"Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionRemovedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionRemovedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionRemovedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionRemovedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemSessionRemovedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionUpdatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionUpdatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionUpdatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionUpdatedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemSessionUpdatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionValueSetReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionValueSetReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionValueSetReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionValueSetReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionValueSetReceivedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemSessionValueSetReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemSessionWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemSessionWatcher ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionWatcher_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemSessionWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemSessionWatcher[] = L"Windows.System.RemoteSystems.RemoteSystemSessionWatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemStatusTypeFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteSystems.IRemoteSystemStatusTypeFilterFactory interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemStatusTypeFilter ** Default Interface **
 *    Windows.System.RemoteSystems.IRemoteSystemFilter
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemStatusTypeFilter_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemStatusTypeFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemStatusTypeFilter[] = L"Windows.System.RemoteSystems.RemoteSystemStatusTypeFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemUpdatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemUpdatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemUpdatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemUpdatedEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemUpdatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemWatcher ** Default Interface **
 *    Windows.System.RemoteSystems.IRemoteSystemWatcher2
 *    Windows.System.RemoteSystems.IRemoteSystemWatcher3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemWatcher_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemWatcher[] = L"Windows.System.RemoteSystems.RemoteSystemWatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemWatcherErrorOccurredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemWatcherErrorOccurredEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemWatcherErrorOccurredEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemWatcherErrorOccurredEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemWatcherErrorOccurredEventArgs[] = L"Windows.System.RemoteSystems.RemoteSystemWatcherErrorOccurredEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.System.RemoteSystems.RemoteSystemWebAccountFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteSystems.IRemoteSystemWebAccountFilterFactory interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteSystems.IRemoteSystemWebAccountFilter ** Default Interface **
 *    Windows.System.RemoteSystems.IRemoteSystemFilter
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemWebAccountFilter_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteSystems_RemoteSystemWebAccountFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteSystems_RemoteSystemWebAccountFilter[] = L"Windows.System.RemoteSystems.RemoteSystemWebAccountFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Esystem2Eremotesystems_p_h__

#endif // __windows2Esystem2Eremotesystems_h__
