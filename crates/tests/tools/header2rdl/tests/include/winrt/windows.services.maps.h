
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
#ifndef __windows2Eservices2Emaps_h__
#define __windows2Eservices2Emaps_h__
#ifndef __windows2Eservices2Emaps_p_h__
#define __windows2Eservices2Emaps_p_h__


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

#if !defined(WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION)
#define WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION)

#if !defined(WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION)
#define WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Devices.Geolocation.h"
#include "Windows.UI.Popups.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IEnhancedWaypoint;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint ABI::Windows::Services::Maps::IEnhancedWaypoint

#endif // ____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IEnhancedWaypointFactory;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory ABI::Windows::Services::Maps::IEnhancedWaypointFactory

#endif // ____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IManeuverWarning;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIManeuverWarning ABI::Windows::Services::Maps::IManeuverWarning

#endif // ____x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapAddress_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapAddress_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapAddress;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress ABI::Windows::Services::Maps::IMapAddress

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapAddress_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapAddress2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapAddress2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapAddress2;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress2 ABI::Windows::Services::Maps::IMapAddress2

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapAddress2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapLocation_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapLocation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapLocation;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapLocation ABI::Windows::Services::Maps::IMapLocation

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapLocation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapLocationFinderResult;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult ABI::Windows::Services::Maps::IMapLocationFinderResult

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapLocationFinderStatics;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics ABI::Windows::Services::Maps::IMapLocationFinderStatics

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapLocationFinderStatics2;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2 ABI::Windows::Services::Maps::IMapLocationFinderStatics2

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapManagerStatics;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics ABI::Windows::Services::Maps::IMapManagerStatics

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRoute_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRoute_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapRoute;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute ABI::Windows::Services::Maps::IMapRoute

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRoute_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRoute2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRoute2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapRoute2;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute2 ABI::Windows::Services::Maps::IMapRoute2

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRoute2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRoute3_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRoute3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapRoute3;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute3 ABI::Windows::Services::Maps::IMapRoute3

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRoute3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRoute4_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRoute4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapRoute4;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute4 ABI::Windows::Services::Maps::IMapRoute4

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRoute4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapRouteDrivingOptions;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions ABI::Windows::Services::Maps::IMapRouteDrivingOptions

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapRouteDrivingOptions2;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2 ABI::Windows::Services::Maps::IMapRouteDrivingOptions2

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapRouteFinderResult;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult ABI::Windows::Services::Maps::IMapRouteFinderResult

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapRouteFinderResult2;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2 ABI::Windows::Services::Maps::IMapRouteFinderResult2

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapRouteFinderStatics;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics ABI::Windows::Services::Maps::IMapRouteFinderStatics

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapRouteFinderStatics2;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2 ABI::Windows::Services::Maps::IMapRouteFinderStatics2

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapRouteFinderStatics3;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3 ABI::Windows::Services::Maps::IMapRouteFinderStatics3

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapRouteLeg;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg ABI::Windows::Services::Maps::IMapRouteLeg

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapRouteLeg2;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2 ABI::Windows::Services::Maps::IMapRouteLeg2

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapRouteManeuver;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver ABI::Windows::Services::Maps::IMapRouteManeuver

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapRouteManeuver2;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2 ABI::Windows::Services::Maps::IMapRouteManeuver2

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapRouteManeuver3;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3 ABI::Windows::Services::Maps::IMapRouteManeuver3

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapServiceStatics;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics ABI::Windows::Services::Maps::IMapServiceStatics

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapServiceStatics2;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2 ABI::Windows::Services::Maps::IMapServiceStatics2

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapServiceStatics3;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3 ABI::Windows::Services::Maps::IMapServiceStatics3

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IMapServiceStatics4;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4 ABI::Windows::Services::Maps::IMapServiceStatics4

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IPlaceInfo;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo ABI::Windows::Services::Maps::IPlaceInfo

#endif // ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IPlaceInfoCreateOptions;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions ABI::Windows::Services::Maps::IPlaceInfoCreateOptions

#endif // ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IPlaceInfoStatics;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics ABI::Windows::Services::Maps::IPlaceInfoStatics

#endif // ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                interface IPlaceInfoStatics2;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2 ABI::Windows::Services::Maps::IPlaceInfoStatics2

#endif // ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                class MapLocationFinderResult;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult_USE
#define DEF___FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e5e5ee33-abd8-5695-9fe5-ac95850d7198"))
IAsyncOperation<ABI::Windows::Services::Maps::MapLocationFinderResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::MapLocationFinderResult*, ABI::Windows::Services::Maps::IMapLocationFinderResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Services.Maps.MapLocationFinderResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Services::Maps::MapLocationFinderResult*> __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult_t;
#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("26ceeb11-1221-5c2b-bbf9-cfea3663c2ed"))
IAsyncOperationCompletedHandler<ABI::Windows::Services::Maps::MapLocationFinderResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::MapLocationFinderResult*, ABI::Windows::Services::Maps::IMapLocationFinderResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Services.Maps.MapLocationFinderResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Services::Maps::MapLocationFinderResult*> __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                class MapRouteFinderResult;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult_USE
#define DEF___FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ecaa3e7f-c526-5097-b624-cf743d78a9ba"))
IAsyncOperation<ABI::Windows::Services::Maps::MapRouteFinderResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::MapRouteFinderResult*, ABI::Windows::Services::Maps::IMapRouteFinderResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Services.Maps.MapRouteFinderResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Services::Maps::MapRouteFinderResult*> __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult_t;
#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6e7a2b4f-811c-54c3-8938-6795f4e67009"))
IAsyncOperationCompletedHandler<ABI::Windows::Services::Maps::MapRouteFinderResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::MapRouteFinderResult*, ABI::Windows::Services::Maps::IMapRouteFinderResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Services.Maps.MapRouteFinderResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Services::Maps::MapRouteFinderResult*> __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class Geopoint;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeopoint;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint ABI::Windows::Devices::Geolocation::IGeopoint

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint_USE
#define DEF___FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("88225b39-8be9-5c03-9714-8f1642d8a43f"))
IIterator<ABI::Windows::Devices::Geolocation::Geopoint*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geopoint*, ABI::Windows::Devices::Geolocation::IGeopoint*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Geolocation.Geopoint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Geolocation::Geopoint*> __FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint_t;
#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint_USE
#define DEF___FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e7617fc9-2cc7-5bd1-bc5a-f47260834ed8"))
IIterable<ABI::Windows::Devices::Geolocation::Geopoint*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geopoint*, ABI::Windows::Devices::Geolocation::IGeopoint*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Geolocation.Geopoint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Geolocation::Geopoint*> __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint_t;
#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                class EnhancedWaypoint;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint_USE
#define DEF___FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("164a4c21-d0a0-5d68-80e2-44889dcea6d5"))
IIterator<ABI::Windows::Services::Maps::EnhancedWaypoint*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::EnhancedWaypoint*, ABI::Windows::Services::Maps::IEnhancedWaypoint*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Services.Maps.EnhancedWaypoint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Services::Maps::EnhancedWaypoint*> __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint_t;
#define __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint_USE
#define DEF___FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d0545dba-9b05-5e37-bfc0-3da2b51d135b"))
IIterable<ABI::Windows::Services::Maps::EnhancedWaypoint*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::EnhancedWaypoint*, ABI::Windows::Services::Maps::IEnhancedWaypoint*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Services.Maps.EnhancedWaypoint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Services::Maps::EnhancedWaypoint*> __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint_t;
#define __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                class ManeuverWarning;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CServices__CMaps__CManeuverWarning_USE
#define DEF___FIIterator_1_Windows__CServices__CMaps__CManeuverWarning_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("df74a2a3-1eeb-5ac2-bc5f-9f2daffce017"))
IIterator<ABI::Windows::Services::Maps::ManeuverWarning*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::ManeuverWarning*, ABI::Windows::Services::Maps::IManeuverWarning*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Services.Maps.ManeuverWarning>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Services::Maps::ManeuverWarning*> __FIIterator_1_Windows__CServices__CMaps__CManeuverWarning_t;
#define __FIIterator_1_Windows__CServices__CMaps__CManeuverWarning ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CServices__CMaps__CManeuverWarning_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CServices__CMaps__CManeuverWarning_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CServices__CMaps__CManeuverWarning_USE
#define DEF___FIIterable_1_Windows__CServices__CMaps__CManeuverWarning_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ce0a7c13-d3c4-55af-a90f-c53f7bd93373"))
IIterable<ABI::Windows::Services::Maps::ManeuverWarning*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::ManeuverWarning*, ABI::Windows::Services::Maps::IManeuverWarning*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Services.Maps.ManeuverWarning>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Services::Maps::ManeuverWarning*> __FIIterable_1_Windows__CServices__CMaps__CManeuverWarning_t;
#define __FIIterable_1_Windows__CServices__CMaps__CManeuverWarning ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CServices__CMaps__CManeuverWarning_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CServices__CMaps__CManeuverWarning_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                class MapLocation;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CServices__CMaps__CMapLocation_USE
#define DEF___FIIterator_1_Windows__CServices__CMaps__CMapLocation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2a704d9a-3997-5f1e-8641-883eba408726"))
IIterator<ABI::Windows::Services::Maps::MapLocation*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::MapLocation*, ABI::Windows::Services::Maps::IMapLocation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Services.Maps.MapLocation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Services::Maps::MapLocation*> __FIIterator_1_Windows__CServices__CMaps__CMapLocation_t;
#define __FIIterator_1_Windows__CServices__CMaps__CMapLocation ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CServices__CMaps__CMapLocation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CServices__CMaps__CMapLocation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CServices__CMaps__CMapLocation_USE
#define DEF___FIIterable_1_Windows__CServices__CMaps__CMapLocation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("77da6151-0763-508a-9041-3310baace575"))
IIterable<ABI::Windows::Services::Maps::MapLocation*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::MapLocation*, ABI::Windows::Services::Maps::IMapLocation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Services.Maps.MapLocation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Services::Maps::MapLocation*> __FIIterable_1_Windows__CServices__CMaps__CMapLocation_t;
#define __FIIterable_1_Windows__CServices__CMaps__CMapLocation ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CServices__CMaps__CMapLocation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CServices__CMaps__CMapLocation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                class MapRoute;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CServices__CMaps__CMapRoute_USE
#define DEF___FIIterator_1_Windows__CServices__CMaps__CMapRoute_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("97e8485a-79c0-5343-93d1-47cdfb55246b"))
IIterator<ABI::Windows::Services::Maps::MapRoute*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::MapRoute*, ABI::Windows::Services::Maps::IMapRoute*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Services.Maps.MapRoute>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Services::Maps::MapRoute*> __FIIterator_1_Windows__CServices__CMaps__CMapRoute_t;
#define __FIIterator_1_Windows__CServices__CMaps__CMapRoute ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CServices__CMaps__CMapRoute_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CServices__CMaps__CMapRoute_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CServices__CMaps__CMapRoute_USE
#define DEF___FIIterable_1_Windows__CServices__CMaps__CMapRoute_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d88a62a2-0edf-5312-97a8-10aeaea80b99"))
IIterable<ABI::Windows::Services::Maps::MapRoute*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::MapRoute*, ABI::Windows::Services::Maps::IMapRoute*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Services.Maps.MapRoute>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Services::Maps::MapRoute*> __FIIterable_1_Windows__CServices__CMaps__CMapRoute_t;
#define __FIIterable_1_Windows__CServices__CMaps__CMapRoute ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CServices__CMaps__CMapRoute_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CServices__CMaps__CMapRoute_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                class MapRouteLeg;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg_USE
#define DEF___FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("dd1be7d2-de62-5752-b2e0-a2b08723b787"))
IIterator<ABI::Windows::Services::Maps::MapRouteLeg*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::MapRouteLeg*, ABI::Windows::Services::Maps::IMapRouteLeg*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Services.Maps.MapRouteLeg>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Services::Maps::MapRouteLeg*> __FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg_t;
#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg_USE
#define DEF___FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8ff98759-78cd-56e8-877b-83ce846d6f8b"))
IIterable<ABI::Windows::Services::Maps::MapRouteLeg*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::MapRouteLeg*, ABI::Windows::Services::Maps::IMapRouteLeg*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Services.Maps.MapRouteLeg>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Services::Maps::MapRouteLeg*> __FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg_t;
#define __FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                class MapRouteManeuver;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver_USE
#define DEF___FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a7ab048b-a6dc-5e4c-9321-71b0e465dfe8"))
IIterator<ABI::Windows::Services::Maps::MapRouteManeuver*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::MapRouteManeuver*, ABI::Windows::Services::Maps::IMapRouteManeuver*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Services.Maps.MapRouteManeuver>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Services::Maps::MapRouteManeuver*> __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver_t;
#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver_USE
#define DEF___FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("de9015fb-91d7-556e-bb4d-200b6f58fad4"))
IIterable<ABI::Windows::Services::Maps::MapRouteManeuver*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::MapRouteManeuver*, ABI::Windows::Services::Maps::IMapRouteManeuver*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Services.Maps.MapRouteManeuver>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Services::Maps::MapRouteManeuver*> __FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver_t;
#define __FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning_USE
#define DEF___FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("44c11b20-c16d-56e1-a0a3-6eb44f2492ea"))
IVectorView<ABI::Windows::Services::Maps::ManeuverWarning*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::ManeuverWarning*, ABI::Windows::Services::Maps::IManeuverWarning*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Services.Maps.ManeuverWarning>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Services::Maps::ManeuverWarning*> __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning_t;
#define __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CServices__CMaps__CMapLocation_USE
#define DEF___FIVectorView_1_Windows__CServices__CMaps__CMapLocation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("58d33d10-e2ef-59f1-b85e-a8819ff0d926"))
IVectorView<ABI::Windows::Services::Maps::MapLocation*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::MapLocation*, ABI::Windows::Services::Maps::IMapLocation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Services.Maps.MapLocation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Services::Maps::MapLocation*> __FIVectorView_1_Windows__CServices__CMaps__CMapLocation_t;
#define __FIVectorView_1_Windows__CServices__CMaps__CMapLocation ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CServices__CMaps__CMapLocation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CServices__CMaps__CMapLocation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CServices__CMaps__CMapRoute_USE
#define DEF___FIVectorView_1_Windows__CServices__CMaps__CMapRoute_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("265676a9-4a33-5d29-971e-8244a021b84e"))
IVectorView<ABI::Windows::Services::Maps::MapRoute*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::MapRoute*, ABI::Windows::Services::Maps::IMapRoute*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Services.Maps.MapRoute>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Services::Maps::MapRoute*> __FIVectorView_1_Windows__CServices__CMaps__CMapRoute_t;
#define __FIVectorView_1_Windows__CServices__CMaps__CMapRoute ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CServices__CMaps__CMapRoute_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CServices__CMaps__CMapRoute_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg_USE
#define DEF___FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f9976360-b3b0-5a88-b1b6-f4339bb85bf0"))
IVectorView<ABI::Windows::Services::Maps::MapRouteLeg*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::MapRouteLeg*, ABI::Windows::Services::Maps::IMapRouteLeg*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Services.Maps.MapRouteLeg>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Services::Maps::MapRouteLeg*> __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg_t;
#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver_USE
#define DEF___FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a3f56695-468f-55ef-b184-c98b4cc7e484"))
IVectorView<ABI::Windows::Services::Maps::MapRouteManeuver*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::MapRouteManeuver*, ABI::Windows::Services::Maps::IMapRouteManeuver*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Services.Maps.MapRouteManeuver>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Services::Maps::MapRouteManeuver*> __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver_t;
#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIReference_1_double_USE
#define DEF___FIReference_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2f2d6c29-5473-5f3e-92e7-96572bb990e2"))
IReference<double> : IReference_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<double> __FIReference_1_double_t;
#define __FIReference_1_double ABI::Windows::Foundation::__FIReference_1_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_double_USE */


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

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class GeoboundingBox;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeoboundingBox;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox ABI::Windows::Devices::Geolocation::IGeoboundingBox

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class Geopath;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeopath;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath ABI::Windows::Devices::Geolocation::IGeopath

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeoshape;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape ABI::Windows::Devices::Geolocation::IGeoshape

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_FWD_DEFINED__

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
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                typedef enum Placement : int Placement;
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                typedef enum ManeuverWarningKind : int ManeuverWarningKind;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                typedef enum ManeuverWarningSeverity : int ManeuverWarningSeverity;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                typedef enum MapLocationDesiredAccuracy : int MapLocationDesiredAccuracy;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                typedef enum MapLocationFinderStatus : int MapLocationFinderStatus;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                typedef enum MapManeuverNotices : unsigned int MapManeuverNotices;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                typedef enum MapRouteFinderStatus : int MapRouteFinderStatus;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                typedef enum MapRouteManeuverKind : int MapRouteManeuverKind;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                typedef enum MapRouteOptimization : int MapRouteOptimization;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                typedef enum MapRouteRestrictions : unsigned int MapRouteRestrictions;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                typedef enum MapServiceDataUsagePreference : int MapServiceDataUsagePreference;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                typedef enum TrafficCongestion : int TrafficCongestion;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                typedef enum WaypointKind : int WaypointKind;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                class MapAddress;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                class MapRouteDrivingOptions;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                class PlaceInfo;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                class PlaceInfoCreateOptions;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Services.Maps.ManeuverWarningKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                enum ManeuverWarningKind : int
                {
                    ManeuverWarningKind_None = 0,
                    ManeuverWarningKind_Accident = 1,
                    ManeuverWarningKind_AdministrativeDivisionChange = 2,
                    ManeuverWarningKind_Alert = 3,
                    ManeuverWarningKind_BlockedRoad = 4,
                    ManeuverWarningKind_CheckTimetable = 5,
                    ManeuverWarningKind_Congestion = 6,
                    ManeuverWarningKind_Construction = 7,
                    ManeuverWarningKind_CountryChange = 8,
                    ManeuverWarningKind_DisabledVehicle = 9,
                    ManeuverWarningKind_GateAccess = 10,
                    ManeuverWarningKind_GetOffTransit = 11,
                    ManeuverWarningKind_GetOnTransit = 12,
                    ManeuverWarningKind_IllegalUTurn = 13,
                    ManeuverWarningKind_MassTransit = 14,
                    ManeuverWarningKind_Miscellaneous = 15,
                    ManeuverWarningKind_NoIncident = 16,
                    ManeuverWarningKind_Other = 17,
                    ManeuverWarningKind_OtherNews = 18,
                    ManeuverWarningKind_OtherTrafficIncidents = 19,
                    ManeuverWarningKind_PlannedEvent = 20,
                    ManeuverWarningKind_PrivateRoad = 21,
                    ManeuverWarningKind_RestrictedTurn = 22,
                    ManeuverWarningKind_RoadClosures = 23,
                    ManeuverWarningKind_RoadHazard = 24,
                    ManeuverWarningKind_ScheduledConstruction = 25,
                    ManeuverWarningKind_SeasonalClosures = 26,
                    ManeuverWarningKind_Tollbooth = 27,
                    ManeuverWarningKind_TollRoad = 28,
                    ManeuverWarningKind_TollZoneEnter = 29,
                    ManeuverWarningKind_TollZoneExit = 30,
                    ManeuverWarningKind_TrafficFlow = 31,
                    ManeuverWarningKind_TransitLineChange = 32,
                    ManeuverWarningKind_UnpavedRoad = 33,
                    ManeuverWarningKind_UnscheduledConstruction = 34,
                    ManeuverWarningKind_Weather = 35,
                };
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Services.Maps.ManeuverWarningSeverity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                enum ManeuverWarningSeverity : int
                {
                    ManeuverWarningSeverity_None = 0,
                    ManeuverWarningSeverity_LowImpact = 1,
                    ManeuverWarningSeverity_Minor = 2,
                    ManeuverWarningSeverity_Moderate = 3,
                    ManeuverWarningSeverity_Serious = 4,
                };
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Services.Maps.MapLocationDesiredAccuracy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                enum MapLocationDesiredAccuracy : int
                {
                    MapLocationDesiredAccuracy_High = 0,
                    MapLocationDesiredAccuracy_Low = 1,
                };
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Services.Maps.MapLocationFinderStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                enum MapLocationFinderStatus : int
                {
                    MapLocationFinderStatus_Success = 0,
                    MapLocationFinderStatus_UnknownError = 1,
                    MapLocationFinderStatus_InvalidCredentials = 2,
                    MapLocationFinderStatus_BadLocation = 3,
                    MapLocationFinderStatus_IndexFailure = 4,
                    MapLocationFinderStatus_NetworkFailure = 5,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    MapLocationFinderStatus_NotSupported = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.MapManeuverNotices
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                enum MapManeuverNotices : unsigned int
                {
                    MapManeuverNotices_None = 0,
                    MapManeuverNotices_Toll = 0x1,
                    MapManeuverNotices_Unpaved = 0x2,
                };

                DEFINE_ENUM_FLAG_OPERATORS(MapManeuverNotices)
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.MapRouteFinderStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                enum MapRouteFinderStatus : int
                {
                    MapRouteFinderStatus_Success = 0,
                    MapRouteFinderStatus_UnknownError = 1,
                    MapRouteFinderStatus_InvalidCredentials = 2,
                    MapRouteFinderStatus_NoRouteFound = 3,
                    MapRouteFinderStatus_NoRouteFoundWithGivenOptions = 4,
                    MapRouteFinderStatus_StartPointNotFound = 5,
                    MapRouteFinderStatus_EndPointNotFound = 6,
                    MapRouteFinderStatus_NoPedestrianRouteFound = 7,
                    MapRouteFinderStatus_NetworkFailure = 8,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    MapRouteFinderStatus_NotSupported = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.MapRouteManeuverKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                enum MapRouteManeuverKind : int
                {
                    MapRouteManeuverKind_None = 0,
                    MapRouteManeuverKind_Start = 1,
                    MapRouteManeuverKind_Stopover = 2,
                    MapRouteManeuverKind_StopoverResume = 3,
                    MapRouteManeuverKind_End = 4,
                    MapRouteManeuverKind_GoStraight = 5,
                    MapRouteManeuverKind_UTurnLeft = 6,
                    MapRouteManeuverKind_UTurnRight = 7,
                    MapRouteManeuverKind_TurnKeepLeft = 8,
                    MapRouteManeuverKind_TurnKeepRight = 9,
                    MapRouteManeuverKind_TurnLightLeft = 10,
                    MapRouteManeuverKind_TurnLightRight = 11,
                    MapRouteManeuverKind_TurnLeft = 12,
                    MapRouteManeuverKind_TurnRight = 13,
                    MapRouteManeuverKind_TurnHardLeft = 14,
                    MapRouteManeuverKind_TurnHardRight = 15,
                    MapRouteManeuverKind_FreewayEnterLeft = 16,
                    MapRouteManeuverKind_FreewayEnterRight = 17,
                    MapRouteManeuverKind_FreewayLeaveLeft = 18,
                    MapRouteManeuverKind_FreewayLeaveRight = 19,
                    MapRouteManeuverKind_FreewayContinueLeft = 20,
                    MapRouteManeuverKind_FreewayContinueRight = 21,
                    MapRouteManeuverKind_TrafficCircleLeft = 22,
                    MapRouteManeuverKind_TrafficCircleRight = 23,
                    MapRouteManeuverKind_TakeFerry = 24,
                };
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.MapRouteOptimization
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                enum MapRouteOptimization : int
                {
                    MapRouteOptimization_Time = 0,
                    MapRouteOptimization_Distance = 1,
                    MapRouteOptimization_TimeWithTraffic = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                    MapRouteOptimization_Scenic = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                };
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.MapRouteRestrictions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                enum MapRouteRestrictions : unsigned int
                {
                    MapRouteRestrictions_None = 0,
                    MapRouteRestrictions_Highways = 0x1,
                    MapRouteRestrictions_TollRoads = 0x2,
                    MapRouteRestrictions_Ferries = 0x4,
                    MapRouteRestrictions_Tunnels = 0x8,
                    MapRouteRestrictions_DirtRoads = 0x10,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    MapRouteRestrictions_Motorail = 0x20,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };

                DEFINE_ENUM_FLAG_OPERATORS(MapRouteRestrictions)
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.MapServiceDataUsagePreference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                enum MapServiceDataUsagePreference : int
                {
                    MapServiceDataUsagePreference_Default = 0,
                    MapServiceDataUsagePreference_OfflineMapDataOnly = 1,
                };
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Services.Maps.TrafficCongestion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                enum TrafficCongestion : int
                {
                    TrafficCongestion_Unknown = 0,
                    TrafficCongestion_Light = 1,
                    TrafficCongestion_Mild = 2,
                    TrafficCongestion_Medium = 3,
                    TrafficCongestion_Heavy = 4,
                };
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Services.Maps.WaypointKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                enum WaypointKind : int
                {
                    WaypointKind_Stop = 0,
                    WaypointKind_Via = 1,
                };
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.IEnhancedWaypoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.EnhancedWaypoint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IEnhancedWaypoint[] = L"Windows.Services.Maps.IEnhancedWaypoint";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("ed268c74-5913-11e6-8b77-86f30ca893d3")
                IEnhancedWaypoint : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Point(
                        ABI::Windows::Devices::Geolocation::IGeopoint** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::Services::Maps::WaypointKind* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IEnhancedWaypoint = __uuidof(IEnhancedWaypoint);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.IEnhancedWaypointFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.EnhancedWaypoint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IEnhancedWaypointFactory[] = L"Windows.Services.Maps.IEnhancedWaypointFactory";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("af868477-a2aa-46dd-b645-23b31b8aa6c7")
                IEnhancedWaypointFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Devices::Geolocation::IGeopoint* point,
                        ABI::Windows::Services::Maps::WaypointKind kind,
                        ABI::Windows::Services::Maps::IEnhancedWaypoint** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IEnhancedWaypointFactory = __uuidof(IEnhancedWaypointFactory);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.IManeuverWarning
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.ManeuverWarning
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IManeuverWarning[] = L"Windows.Services.Maps.IManeuverWarning";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("c1a36d8a-2630-4378-9e4a-6e44253dceba")
                IManeuverWarning : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::Services::Maps::ManeuverWarningKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Severity(
                        ABI::Windows::Services::Maps::ManeuverWarningSeverity* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IManeuverWarning = __uuidof(IManeuverWarning);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIManeuverWarning;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.IMapAddress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapAddress
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapAddress_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapAddress_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapAddress[] = L"Windows.Services.Maps.IMapAddress";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("cfa7a973-a3b4-4494-b3ff-cba94db69699")
                IMapAddress : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BuildingName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BuildingFloor(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BuildingRoom(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BuildingWing(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StreetNumber(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Street(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Neighborhood(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_District(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Town(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Region(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RegionCode(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Country(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CountryCode(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PostCode(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Continent(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapAddress = __uuidof(IMapAddress);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapAddress;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapAddress_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapAddress2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapAddress
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapAddress2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapAddress2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapAddress2[] = L"Windows.Services.Maps.IMapAddress2";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("75cd6df1-e5ad-45a9-bf40-6cf256c1dd13")
                IMapAddress2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_FormattedAddress(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapAddress2 = __uuidof(IMapAddress2);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapAddress2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapAddress2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapLocation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapLocation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapLocation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapLocation[] = L"Windows.Services.Maps.IMapLocation";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("3c073f57-0da4-42e8-9ee2-a96fcf2371dc")
                IMapLocation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Point(
                        ABI::Windows::Devices::Geolocation::IGeopoint** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Address(
                        ABI::Windows::Services::Maps::IMapAddress** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapLocation = __uuidof(IMapLocation);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapLocation;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapLocation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapLocationFinderResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapLocationFinderResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapLocationFinderResult[] = L"Windows.Services.Maps.IMapLocationFinderResult";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("43f1f179-e8cc-45f6-bed2-54ccbf965d9a")
                IMapLocationFinderResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Locations(
                        __FIVectorView_1_Windows__CServices__CMaps__CMapLocation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Services::Maps::MapLocationFinderStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapLocationFinderResult = __uuidof(IMapLocationFinderResult);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapLocationFinderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapLocationFinder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapLocationFinderStatics[] = L"Windows.Services.Maps.IMapLocationFinderStatics";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("318adb5d-1c5d-4f35-a2df-aaca94959517")
                IMapLocationFinderStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FindLocationsAtAsync(
                        ABI::Windows::Devices::Geolocation::IGeopoint* queryPoint,
                        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindLocationsAsync(
                        HSTRING searchText,
                        ABI::Windows::Devices::Geolocation::IGeopoint* referencePoint,
                        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindLocationsWithMaxCountAsync(
                        HSTRING searchText,
                        ABI::Windows::Devices::Geolocation::IGeopoint* referencePoint,
                        UINT32 maxCount,
                        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapLocationFinderStatics = __uuidof(IMapLocationFinderStatics);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapLocationFinderStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapLocationFinder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapLocationFinderStatics2[] = L"Windows.Services.Maps.IMapLocationFinderStatics2";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("959a8b96-6485-4dfd-851a-33ac317e3af6")
                IMapLocationFinderStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FindLocationsAtWithAccuracyAsync(
                        ABI::Windows::Devices::Geolocation::IGeopoint* queryPoint,
                        ABI::Windows::Services::Maps::MapLocationDesiredAccuracy accuracy,
                        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapLocationFinderStatics2 = __uuidof(IMapLocationFinderStatics2);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Services.Maps.IMapManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapManagerStatics[] = L"Windows.Services.Maps.IMapManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("37e3e515-82b4-4d54-8fd9-af2624b3011c")
                IMapManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ShowDownloadedMapsUI(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowMapsUpdateUI(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapManagerStatics = __uuidof(IMapManagerStatics);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRoute
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRoute
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRoute_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRoute_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRoute[] = L"Windows.Services.Maps.IMapRoute";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("fb07b732-584d-4583-9c60-641fea274349")
                IMapRoute : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BoundingBox(
                        ABI::Windows::Devices::Geolocation::IGeoboundingBox** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LengthInMeters(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EstimatedDuration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Path(
                        ABI::Windows::Devices::Geolocation::IGeopath** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Legs(
                        __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsTrafficBased(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapRoute = __uuidof(IMapRoute);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRoute;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRoute_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRoute2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRoute
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRoute2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRoute2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRoute2[] = L"Windows.Services.Maps.IMapRoute2";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("d1c5d40c-2213-4ab0-a260-46b38169beac")
                IMapRoute2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ViolatedRestrictions(
                        ABI::Windows::Services::Maps::MapRouteRestrictions* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasBlockedRoads(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapRoute2 = __uuidof(IMapRoute2);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRoute2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRoute2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRoute3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRoute
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRoute3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRoute3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRoute3[] = L"Windows.Services.Maps.IMapRoute3";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("858d1eae-f2ad-429f-bb37-cd21094ffc92")
                IMapRoute3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DurationWithoutTraffic(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TrafficCongestion(
                        ABI::Windows::Services::Maps::TrafficCongestion* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapRoute3 = __uuidof(IMapRoute3);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRoute3;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRoute3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.IMapRoute4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRoute
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRoute4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRoute4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRoute4[] = L"Windows.Services.Maps.IMapRoute4";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("366c8ca5-3053-4fa1-80ff-d475f3ed1e6e")
                IMapRoute4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsScenic(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapRoute4 = __uuidof(IMapRoute4);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRoute4;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRoute4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteDrivingOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteDrivingOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteDrivingOptions[] = L"Windows.Services.Maps.IMapRouteDrivingOptions";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("6815364d-c6dc-4697-a452-b18f8f0b67a1")
                IMapRouteDrivingOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MaxAlternateRouteCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MaxAlternateRouteCount(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InitialHeading(
                        __FIReference_1_double** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_InitialHeading(
                        __FIReference_1_double* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RouteOptimization(
                        ABI::Windows::Services::Maps::MapRouteOptimization* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RouteOptimization(
                        ABI::Windows::Services::Maps::MapRouteOptimization value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RouteRestrictions(
                        ABI::Windows::Services::Maps::MapRouteRestrictions* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RouteRestrictions(
                        ABI::Windows::Services::Maps::MapRouteRestrictions value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapRouteDrivingOptions = __uuidof(IMapRouteDrivingOptions);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteDrivingOptions2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteDrivingOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteDrivingOptions2[] = L"Windows.Services.Maps.IMapRouteDrivingOptions2";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("35dc8670-c298-48d0-b5ad-825460645603")
                IMapRouteDrivingOptions2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DepartureTime(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DepartureTime(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapRouteDrivingOptions2 = __uuidof(IMapRouteDrivingOptions2);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteFinderResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteFinderResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteFinderResult[] = L"Windows.Services.Maps.IMapRouteFinderResult";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("a868a31a-9422-46ac-8ca1-b1614d4bfbe2")
                IMapRouteFinderResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Route(
                        ABI::Windows::Services::Maps::IMapRoute** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Services::Maps::MapRouteFinderStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapRouteFinderResult = __uuidof(IMapRouteFinderResult);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteFinderResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteFinderResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteFinderResult2[] = L"Windows.Services.Maps.IMapRouteFinderResult2";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("20709c6d-d90c-46c8-91c6-7d4be4efb215")
                IMapRouteFinderResult2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AlternateRoutes(
                        __FIVectorView_1_Windows__CServices__CMaps__CMapRoute** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapRouteFinderResult2 = __uuidof(IMapRouteFinderResult2);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteFinderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteFinder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteFinderStatics[] = L"Windows.Services.Maps.IMapRouteFinderStatics";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("b8a5c50f-1c64-4c3a-81eb-1f7c152afbbb")
                IMapRouteFinderStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDrivingRouteAsync(
                        ABI::Windows::Devices::Geolocation::IGeopoint* startPoint,
                        ABI::Windows::Devices::Geolocation::IGeopoint* endPoint,
                        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDrivingRouteWithOptimizationAsync(
                        ABI::Windows::Devices::Geolocation::IGeopoint* startPoint,
                        ABI::Windows::Devices::Geolocation::IGeopoint* endPoint,
                        ABI::Windows::Services::Maps::MapRouteOptimization optimization,
                        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDrivingRouteWithOptimizationAndRestrictionsAsync(
                        ABI::Windows::Devices::Geolocation::IGeopoint* startPoint,
                        ABI::Windows::Devices::Geolocation::IGeopoint* endPoint,
                        ABI::Windows::Services::Maps::MapRouteOptimization optimization,
                        ABI::Windows::Services::Maps::MapRouteRestrictions restrictions,
                        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync(
                        ABI::Windows::Devices::Geolocation::IGeopoint* startPoint,
                        ABI::Windows::Devices::Geolocation::IGeopoint* endPoint,
                        ABI::Windows::Services::Maps::MapRouteOptimization optimization,
                        ABI::Windows::Services::Maps::MapRouteRestrictions restrictions,
                        DOUBLE headingInDegrees,
                        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDrivingRouteFromWaypointsAsync(
                        __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint* wayPoints,
                        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDrivingRouteFromWaypointsAndOptimizationAsync(
                        __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint* wayPoints,
                        ABI::Windows::Services::Maps::MapRouteOptimization optimization,
                        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync(
                        __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint* wayPoints,
                        ABI::Windows::Services::Maps::MapRouteOptimization optimization,
                        ABI::Windows::Services::Maps::MapRouteRestrictions restrictions,
                        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync(
                        __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint* wayPoints,
                        ABI::Windows::Services::Maps::MapRouteOptimization optimization,
                        ABI::Windows::Services::Maps::MapRouteRestrictions restrictions,
                        DOUBLE headingInDegrees,
                        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetWalkingRouteAsync(
                        ABI::Windows::Devices::Geolocation::IGeopoint* startPoint,
                        ABI::Windows::Devices::Geolocation::IGeopoint* endPoint,
                        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetWalkingRouteFromWaypointsAsync(
                        __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint* wayPoints,
                        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapRouteFinderStatics = __uuidof(IMapRouteFinderStatics);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteFinderStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteFinder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteFinderStatics2[] = L"Windows.Services.Maps.IMapRouteFinderStatics2";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("afcc2c73-7760-49af-b3bd-baf135b703e1")
                IMapRouteFinderStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDrivingRouteWithOptionsAsync(
                        ABI::Windows::Devices::Geolocation::IGeopoint* startPoint,
                        ABI::Windows::Devices::Geolocation::IGeopoint* endPoint,
                        ABI::Windows::Services::Maps::IMapRouteDrivingOptions* options,
                        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapRouteFinderStatics2 = __uuidof(IMapRouteFinderStatics2);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteFinderStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteFinder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteFinderStatics3[] = L"Windows.Services.Maps.IMapRouteFinderStatics3";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("f6098134-5913-11e6-8b77-86f30ca893d3")
                IMapRouteFinderStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDrivingRouteFromEnhancedWaypointsAsync(
                        __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint* waypoints,
                        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync(
                        __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint* waypoints,
                        ABI::Windows::Services::Maps::IMapRouteDrivingOptions* options,
                        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapRouteFinderStatics3 = __uuidof(IMapRouteFinderStatics3);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteLeg
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteLeg
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteLeg[] = L"Windows.Services.Maps.IMapRouteLeg";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("96f8b2f6-5bba-4d17-9db6-1a263fec7471")
                IMapRouteLeg : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BoundingBox(
                        ABI::Windows::Devices::Geolocation::IGeoboundingBox** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Path(
                        ABI::Windows::Devices::Geolocation::IGeopath** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LengthInMeters(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EstimatedDuration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Maneuvers(
                        __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapRouteLeg = __uuidof(IMapRouteLeg);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteLeg2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteLeg
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteLeg2[] = L"Windows.Services.Maps.IMapRouteLeg2";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("02e2062d-c9c6-45b8-8e54-1a10b57a17e8")
                IMapRouteLeg2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DurationWithoutTraffic(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TrafficCongestion(
                        ABI::Windows::Services::Maps::TrafficCongestion* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapRouteLeg2 = __uuidof(IMapRouteLeg2);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteManeuver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteManeuver
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteManeuver[] = L"Windows.Services.Maps.IMapRouteManeuver";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("ed5c17f0-a6ab-4d65-a086-fa8a7e340df2")
                IMapRouteManeuver : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_StartingPoint(
                        ABI::Windows::Devices::Geolocation::IGeopoint** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LengthInMeters(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InstructionText(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::Services::Maps::MapRouteManeuverKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExitNumber(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ManeuverNotices(
                        ABI::Windows::Services::Maps::MapManeuverNotices* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapRouteManeuver = __uuidof(IMapRouteManeuver);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteManeuver2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteManeuver
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteManeuver2[] = L"Windows.Services.Maps.IMapRouteManeuver2";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("5d7bcd9c-7c9b-41df-838b-eae21e4b05a9")
                IMapRouteManeuver2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_StartHeading(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EndHeading(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StreetName(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapRouteManeuver2 = __uuidof(IMapRouteManeuver2);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteManeuver3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteManeuver
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteManeuver3[] = L"Windows.Services.Maps.IMapRouteManeuver3";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("a6a138df-0483-4166-85be-b99336c11875")
                IMapRouteManeuver3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Warnings(
                        __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapRouteManeuver3 = __uuidof(IMapRouteManeuver3);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.IMapServiceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapServiceStatics[] = L"Windows.Services.Maps.IMapServiceStatics";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("0144ad85-c04c-4cdd-871a-a0726d097cd4")
                IMapServiceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_ServiceToken(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceToken(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapServiceStatics = __uuidof(IMapServiceStatics);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapServiceStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapServiceStatics2[] = L"Windows.Services.Maps.IMapServiceStatics2";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("f8193eed-9c85-40a9-8896-0fc3fd2b7c2a")
                IMapServiceStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_WorldViewRegionCode(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapServiceStatics2 = __uuidof(IMapServiceStatics2);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapServiceStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapServiceStatics3[] = L"Windows.Services.Maps.IMapServiceStatics3";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("0a11ce20-63a7-4854-b355-d6dcda223d1b")
                IMapServiceStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DataAttributions(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapServiceStatics3 = __uuidof(IMapServiceStatics3);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Services.Maps.IMapServiceStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapServiceStatics4[] = L"Windows.Services.Maps.IMapServiceStatics4";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("088a2862-6abc-420e-945f-4cfd89c67356")
                IMapServiceStatics4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_DataUsagePreference(
                        ABI::Windows::Services::Maps::MapServiceDataUsagePreference value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DataUsagePreference(
                        ABI::Windows::Services::Maps::MapServiceDataUsagePreference* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapServiceStatics4 = __uuidof(IMapServiceStatics4);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.IPlaceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.PlaceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IPlaceInfo[] = L"Windows.Services.Maps.IPlaceInfo";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("9a0810b6-31c8-4f6a-9f18-950b4c38951a")
                IPlaceInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Show(
                        ABI::Windows::Foundation::Rect selection
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowWithPreferredPlacement(
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Identifier(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayAddress(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Geoshape(
                        ABI::Windows::Devices::Geolocation::IGeoshape** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlaceInfo = __uuidof(IPlaceInfo);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIPlaceInfo;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Services.Maps.IPlaceInfoCreateOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.PlaceInfoCreateOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IPlaceInfoCreateOptions[] = L"Windows.Services.Maps.IPlaceInfoCreateOptions";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("cd33c125-67f1-4bb3-9907-ecce939b0399")
                IPlaceInfoCreateOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayAddress(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayAddress(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlaceInfoCreateOptions = __uuidof(IPlaceInfoCreateOptions);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Services.Maps.IPlaceInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.PlaceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IPlaceInfoStatics[] = L"Windows.Services.Maps.IPlaceInfoStatics";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("82b9ff71-6cd0-48a4-afd9-5ed82097936b")
                IPlaceInfoStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Devices::Geolocation::IGeopoint* referencePoint,
                        ABI::Windows::Services::Maps::IPlaceInfo** resultValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithGeopointAndOptions(
                        ABI::Windows::Devices::Geolocation::IGeopoint* referencePoint,
                        ABI::Windows::Services::Maps::IPlaceInfoCreateOptions* options,
                        ABI::Windows::Services::Maps::IPlaceInfo** resultValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIdentifier(
                        HSTRING identifier,
                        ABI::Windows::Services::Maps::IPlaceInfo** resultValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIdentifierWithOptions(
                        HSTRING identifier,
                        ABI::Windows::Devices::Geolocation::IGeopoint* defaultPoint,
                        ABI::Windows::Services::Maps::IPlaceInfoCreateOptions* options,
                        ABI::Windows::Services::Maps::IPlaceInfo** resultValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromMapLocation(
                        ABI::Windows::Services::Maps::IMapLocation* location,
                        ABI::Windows::Services::Maps::IPlaceInfo** resultValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsShowSupported(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlaceInfoStatics = __uuidof(IPlaceInfoStatics);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Services.Maps.IPlaceInfoStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.PlaceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IPlaceInfoStatics2[] = L"Windows.Services.Maps.IPlaceInfoStatics2";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                MIDL_INTERFACE("730f0249-4047-44a3-8f81-2550a5216370")
                IPlaceInfoStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromAddress(
                        HSTRING displayAddress,
                        ABI::Windows::Services::Maps::IPlaceInfo** resultValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromAddressWithName(
                        HSTRING displayAddress,
                        HSTRING displayName,
                        ABI::Windows::Services::Maps::IPlaceInfo** resultValue
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlaceInfoStatics2 = __uuidof(IPlaceInfoStatics2);
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Services.Maps.EnhancedWaypoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Services.Maps.IEnhancedWaypointFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IEnhancedWaypoint ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Services_Maps_EnhancedWaypoint_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_EnhancedWaypoint_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_EnhancedWaypoint[] = L"Windows.Services.Maps.EnhancedWaypoint";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Services.Maps.ManeuverWarning
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IManeuverWarning ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Services_Maps_ManeuverWarning_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_ManeuverWarning_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_ManeuverWarning[] = L"Windows.Services.Maps.ManeuverWarning";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Services.Maps.MapAddress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IMapAddress ** Default Interface **
 *    Windows.Services.Maps.IMapAddress2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapAddress_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapAddress_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapAddress[] = L"Windows.Services.Maps.MapAddress";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IMapLocation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapLocation_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapLocation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapLocation[] = L"Windows.Services.Maps.MapLocation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapLocationFinder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.IMapLocationFinderStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Services.Maps.IMapLocationFinderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapLocationFinder_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapLocationFinder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapLocationFinder[] = L"Windows.Services.Maps.MapLocationFinder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapLocationFinderResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IMapLocationFinderResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapLocationFinderResult_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapLocationFinderResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapLocationFinderResult[] = L"Windows.Services.Maps.MapLocationFinderResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.IMapManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapManager_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapManager[] = L"Windows.Services.Maps.MapManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapRoute
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IMapRoute ** Default Interface **
 *    Windows.Services.Maps.IMapRoute2
 *    Windows.Services.Maps.IMapRoute3
 *    Windows.Services.Maps.IMapRoute4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapRoute_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapRoute_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapRoute[] = L"Windows.Services.Maps.MapRoute";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapRouteDrivingOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IMapRouteDrivingOptions ** Default Interface **
 *    Windows.Services.Maps.IMapRouteDrivingOptions2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapRouteDrivingOptions_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapRouteDrivingOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapRouteDrivingOptions[] = L"Windows.Services.Maps.MapRouteDrivingOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapRouteFinder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.IMapRouteFinderStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Services.Maps.IMapRouteFinderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Services.Maps.IMapRouteFinderStatics3 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapRouteFinder_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapRouteFinder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapRouteFinder[] = L"Windows.Services.Maps.MapRouteFinder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapRouteFinderResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IMapRouteFinderResult ** Default Interface **
 *    Windows.Services.Maps.IMapRouteFinderResult2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapRouteFinderResult_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapRouteFinderResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapRouteFinderResult[] = L"Windows.Services.Maps.MapRouteFinderResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapRouteLeg
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IMapRouteLeg ** Default Interface **
 *    Windows.Services.Maps.IMapRouteLeg2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapRouteLeg_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapRouteLeg_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapRouteLeg[] = L"Windows.Services.Maps.MapRouteLeg";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapRouteManeuver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IMapRouteManeuver ** Default Interface **
 *    Windows.Services.Maps.IMapRouteManeuver2
 *    Windows.Services.Maps.IMapRouteManeuver3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapRouteManeuver_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapRouteManeuver_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapRouteManeuver[] = L"Windows.Services.Maps.MapRouteManeuver";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.IMapServiceStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Services.Maps.IMapServiceStatics3 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Services.Maps.IMapServiceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Services.Maps.IMapServiceStatics4 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapService_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapService_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapService[] = L"Windows.Services.Maps.MapService";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.PlaceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.IPlaceInfoStatics2 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Services.Maps.IPlaceInfoStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IPlaceInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Services_Maps_PlaceInfo_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_PlaceInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_PlaceInfo[] = L"Windows.Services.Maps.PlaceInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Services.Maps.PlaceInfoCreateOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IPlaceInfoCreateOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Services_Maps_PlaceInfoCreateOptions_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_PlaceInfoCreateOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_PlaceInfoCreateOptions[] = L"Windows.Services.Maps.PlaceInfoCreateOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIManeuverWarning __x_ABI_CWindows_CServices_CMaps_CIManeuverWarning;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapAddress_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapAddress_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapAddress __x_ABI_CWindows_CServices_CMaps_CIMapAddress;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapAddress_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapAddress2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapAddress2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapAddress2 __x_ABI_CWindows_CServices_CMaps_CIMapAddress2;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapAddress2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapLocation_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapLocation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapLocation __x_ABI_CWindows_CServices_CMaps_CIMapLocation;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapLocation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2 __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics __x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRoute_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRoute_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapRoute __x_ABI_CWindows_CServices_CMaps_CIMapRoute;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRoute_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRoute2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRoute2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapRoute2 __x_ABI_CWindows_CServices_CMaps_CIMapRoute2;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRoute2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRoute3_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRoute3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapRoute3 __x_ABI_CWindows_CServices_CMaps_CIMapRoute3;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRoute3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRoute4_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRoute4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapRoute4 __x_ABI_CWindows_CServices_CMaps_CIMapRoute4;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRoute4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2 __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2 __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2 __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3 __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2 __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2 __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3 __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2 __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3 __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4 __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2 __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult;

typedef struct __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult* This,
        __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResultVtbl;

interface __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult* This,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapLocationFinderResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult;

typedef struct __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult* This,
        __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResultVtbl;

interface __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult* This,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CMapRouteFinderResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint __FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint;

typedef struct __FIIterator_1_Windows__CDevices__CGeolocation__CGeopointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CGeolocation__CGeopointVtbl;

interface __FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CGeolocation__CGeopointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint;

typedef struct __FIIterable_1_Windows__CDevices__CGeolocation__CGeopointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        __FIIterator_1_Windows__CDevices__CGeolocation__CGeopoint** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CGeolocation__CGeopointVtbl;

interface __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CGeolocation__CGeopointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint;

typedef struct __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint* This,
        __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypointVtbl;

interface __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint
{
    CONST_VTBL struct __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint;

typedef struct __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint* This,
        __FIIterator_1_Windows__CServices__CMaps__CEnhancedWaypoint** result);

    END_INTERFACE
} __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypointVtbl;

interface __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint
{
    CONST_VTBL struct __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CServices__CMaps__CManeuverWarning_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CServices__CMaps__CManeuverWarning_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CServices__CMaps__CManeuverWarning __FIIterator_1_Windows__CServices__CMaps__CManeuverWarning;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CServices__CMaps__CManeuverWarning;

typedef struct __FIIterator_1_Windows__CServices__CMaps__CManeuverWarningVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CServices__CMaps__CManeuverWarning* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CServices__CMaps__CManeuverWarning* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CServices__CMaps__CManeuverWarning* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CServices__CMaps__CManeuverWarning* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CServices__CMaps__CManeuverWarning* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CServices__CMaps__CManeuverWarning* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CServices__CMaps__CManeuverWarning* This,
        __x_ABI_CWindows_CServices_CMaps_CIManeuverWarning** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CServices__CMaps__CManeuverWarning* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CServices__CMaps__CManeuverWarning* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CServices__CMaps__CManeuverWarning* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CIManeuverWarning** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CServices__CMaps__CManeuverWarningVtbl;

interface __FIIterator_1_Windows__CServices__CMaps__CManeuverWarning
{
    CONST_VTBL struct __FIIterator_1_Windows__CServices__CMaps__CManeuverWarningVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CServices__CMaps__CManeuverWarning_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CServices__CMaps__CManeuverWarning_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CServices__CMaps__CManeuverWarning_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CServices__CMaps__CManeuverWarning_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CServices__CMaps__CManeuverWarning_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CServices__CMaps__CManeuverWarning_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CServices__CMaps__CManeuverWarning_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CManeuverWarning_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CManeuverWarning_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CManeuverWarning_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CServices__CMaps__CManeuverWarning_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CServices__CMaps__CManeuverWarning_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CServices__CMaps__CManeuverWarning_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CServices__CMaps__CManeuverWarning __FIIterable_1_Windows__CServices__CMaps__CManeuverWarning;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CServices__CMaps__CManeuverWarning;

typedef struct __FIIterable_1_Windows__CServices__CMaps__CManeuverWarningVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CServices__CMaps__CManeuverWarning* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CServices__CMaps__CManeuverWarning* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CServices__CMaps__CManeuverWarning* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CServices__CMaps__CManeuverWarning* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CServices__CMaps__CManeuverWarning* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CServices__CMaps__CManeuverWarning* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CServices__CMaps__CManeuverWarning* This,
        __FIIterator_1_Windows__CServices__CMaps__CManeuverWarning** result);

    END_INTERFACE
} __FIIterable_1_Windows__CServices__CMaps__CManeuverWarningVtbl;

interface __FIIterable_1_Windows__CServices__CMaps__CManeuverWarning
{
    CONST_VTBL struct __FIIterable_1_Windows__CServices__CMaps__CManeuverWarningVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CServices__CMaps__CManeuverWarning_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CServices__CMaps__CManeuverWarning_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CServices__CMaps__CManeuverWarning_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CServices__CMaps__CManeuverWarning_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CServices__CMaps__CManeuverWarning_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CServices__CMaps__CManeuverWarning_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CServices__CMaps__CManeuverWarning_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CServices__CMaps__CManeuverWarning_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CServices__CMaps__CMapLocation_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CServices__CMaps__CMapLocation_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CServices__CMaps__CMapLocation __FIIterator_1_Windows__CServices__CMaps__CMapLocation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CServices__CMaps__CMapLocation;

typedef struct __FIIterator_1_Windows__CServices__CMaps__CMapLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CServices__CMaps__CMapLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CServices__CMaps__CMapLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CServices__CMaps__CMapLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CServices__CMaps__CMapLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CServices__CMaps__CMapLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CServices__CMaps__CMapLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CServices__CMaps__CMapLocation* This,
        __x_ABI_CWindows_CServices_CMaps_CIMapLocation** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CServices__CMaps__CMapLocation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CServices__CMaps__CMapLocation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CServices__CMaps__CMapLocation* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CIMapLocation** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CServices__CMaps__CMapLocationVtbl;

interface __FIIterator_1_Windows__CServices__CMaps__CMapLocation
{
    CONST_VTBL struct __FIIterator_1_Windows__CServices__CMaps__CMapLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CServices__CMaps__CMapLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CServices__CMaps__CMapLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CServices__CMaps__CMapLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CServices__CMaps__CMapLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CServices__CMaps__CMapLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CServices__CMaps__CMapLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CServices__CMaps__CMapLocation_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CMapLocation_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CMapLocation_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CMapLocation_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CServices__CMaps__CMapLocation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CServices__CMaps__CMapLocation_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CServices__CMaps__CMapLocation_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CServices__CMaps__CMapLocation __FIIterable_1_Windows__CServices__CMaps__CMapLocation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CServices__CMaps__CMapLocation;

typedef struct __FIIterable_1_Windows__CServices__CMaps__CMapLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CServices__CMaps__CMapLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CServices__CMaps__CMapLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CServices__CMaps__CMapLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CServices__CMaps__CMapLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CServices__CMaps__CMapLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CServices__CMaps__CMapLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CServices__CMaps__CMapLocation* This,
        __FIIterator_1_Windows__CServices__CMaps__CMapLocation** result);

    END_INTERFACE
} __FIIterable_1_Windows__CServices__CMaps__CMapLocationVtbl;

interface __FIIterable_1_Windows__CServices__CMaps__CMapLocation
{
    CONST_VTBL struct __FIIterable_1_Windows__CServices__CMaps__CMapLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CServices__CMaps__CMapLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CServices__CMaps__CMapLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CServices__CMaps__CMapLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CServices__CMaps__CMapLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CServices__CMaps__CMapLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CServices__CMaps__CMapLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CServices__CMaps__CMapLocation_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CServices__CMaps__CMapLocation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CServices__CMaps__CMapRoute_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CServices__CMaps__CMapRoute_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CServices__CMaps__CMapRoute __FIIterator_1_Windows__CServices__CMaps__CMapRoute;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CServices__CMaps__CMapRoute;

typedef struct __FIIterator_1_Windows__CServices__CMaps__CMapRouteVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CServices__CMaps__CMapRoute* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CServices__CMaps__CMapRoute* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CServices__CMaps__CMapRoute* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CServices__CMaps__CMapRoute* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CServices__CMaps__CMapRoute* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CServices__CMaps__CMapRoute* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CServices__CMaps__CMapRoute* This,
        __x_ABI_CWindows_CServices_CMaps_CIMapRoute** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CServices__CMaps__CMapRoute* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CServices__CMaps__CMapRoute* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CServices__CMaps__CMapRoute* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CIMapRoute** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CServices__CMaps__CMapRouteVtbl;

interface __FIIterator_1_Windows__CServices__CMaps__CMapRoute
{
    CONST_VTBL struct __FIIterator_1_Windows__CServices__CMaps__CMapRouteVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CServices__CMaps__CMapRoute_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRoute_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRoute_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRoute_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRoute_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRoute_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRoute_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRoute_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRoute_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRoute_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CServices__CMaps__CMapRoute_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CServices__CMaps__CMapRoute_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CServices__CMaps__CMapRoute_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CServices__CMaps__CMapRoute __FIIterable_1_Windows__CServices__CMaps__CMapRoute;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CServices__CMaps__CMapRoute;

typedef struct __FIIterable_1_Windows__CServices__CMaps__CMapRouteVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CServices__CMaps__CMapRoute* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CServices__CMaps__CMapRoute* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CServices__CMaps__CMapRoute* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CServices__CMaps__CMapRoute* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CServices__CMaps__CMapRoute* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CServices__CMaps__CMapRoute* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CServices__CMaps__CMapRoute* This,
        __FIIterator_1_Windows__CServices__CMaps__CMapRoute** result);

    END_INTERFACE
} __FIIterable_1_Windows__CServices__CMaps__CMapRouteVtbl;

interface __FIIterable_1_Windows__CServices__CMaps__CMapRoute
{
    CONST_VTBL struct __FIIterable_1_Windows__CServices__CMaps__CMapRouteVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CServices__CMaps__CMapRoute_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CServices__CMaps__CMapRoute_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CServices__CMaps__CMapRoute_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CServices__CMaps__CMapRoute_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CServices__CMaps__CMapRoute_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CServices__CMaps__CMapRoute_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CServices__CMaps__CMapRoute_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CServices__CMaps__CMapRoute_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg __FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg;

typedef struct __FIIterator_1_Windows__CServices__CMaps__CMapRouteLegVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CServices__CMaps__CMapRouteLegVtbl;

interface __FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg
{
    CONST_VTBL struct __FIIterator_1_Windows__CServices__CMaps__CMapRouteLegVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg __FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg;

typedef struct __FIIterable_1_Windows__CServices__CMaps__CMapRouteLegVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        __FIIterator_1_Windows__CServices__CMaps__CMapRouteLeg** result);

    END_INTERFACE
} __FIIterable_1_Windows__CServices__CMaps__CMapRouteLegVtbl;

interface __FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg
{
    CONST_VTBL struct __FIIterable_1_Windows__CServices__CMaps__CMapRouteLegVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CServices__CMaps__CMapRouteLeg_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver;

typedef struct __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuverVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuverVtbl;

interface __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver
{
    CONST_VTBL struct __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuverVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver __FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver;

typedef struct __FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuverVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        __FIIterator_1_Windows__CServices__CMaps__CMapRouteManeuver** result);

    END_INTERFACE
} __FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuverVtbl;

interface __FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver
{
    CONST_VTBL struct __FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuverVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CServices__CMaps__CMapRouteManeuver_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning;

typedef struct __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarningVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning* This,
        UINT32 index,
        __x_ABI_CWindows_CServices_CMaps_CIManeuverWarning** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning* This,
        __x_ABI_CWindows_CServices_CMaps_CIManeuverWarning* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CIManeuverWarning** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarningVtbl;

interface __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning
{
    CONST_VTBL struct __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarningVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CServices__CMaps__CMapLocation_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CServices__CMaps__CMapLocation_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CServices__CMaps__CMapLocation __FIVectorView_1_Windows__CServices__CMaps__CMapLocation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CServices__CMaps__CMapLocation;

typedef struct __FIVectorView_1_Windows__CServices__CMaps__CMapLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CServices__CMaps__CMapLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CServices__CMaps__CMapLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CServices__CMaps__CMapLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CServices__CMaps__CMapLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CServices__CMaps__CMapLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CServices__CMaps__CMapLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CServices__CMaps__CMapLocation* This,
        UINT32 index,
        __x_ABI_CWindows_CServices_CMaps_CIMapLocation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CServices__CMaps__CMapLocation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CServices__CMaps__CMapLocation* This,
        __x_ABI_CWindows_CServices_CMaps_CIMapLocation* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CServices__CMaps__CMapLocation* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CIMapLocation** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CServices__CMaps__CMapLocationVtbl;

interface __FIVectorView_1_Windows__CServices__CMaps__CMapLocation
{
    CONST_VTBL struct __FIVectorView_1_Windows__CServices__CMaps__CMapLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CServices__CMaps__CMapLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapLocation_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapLocation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapLocation_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapLocation_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CServices__CMaps__CMapLocation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CServices__CMaps__CMapRoute_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CServices__CMaps__CMapRoute_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CServices__CMaps__CMapRoute __FIVectorView_1_Windows__CServices__CMaps__CMapRoute;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CServices__CMaps__CMapRoute;

typedef struct __FIVectorView_1_Windows__CServices__CMaps__CMapRouteVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CServices__CMaps__CMapRoute* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CServices__CMaps__CMapRoute* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CServices__CMaps__CMapRoute* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CServices__CMaps__CMapRoute* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CServices__CMaps__CMapRoute* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CServices__CMaps__CMapRoute* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CServices__CMaps__CMapRoute* This,
        UINT32 index,
        __x_ABI_CWindows_CServices_CMaps_CIMapRoute** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CServices__CMaps__CMapRoute* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CServices__CMaps__CMapRoute* This,
        __x_ABI_CWindows_CServices_CMaps_CIMapRoute* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CServices__CMaps__CMapRoute* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CIMapRoute** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CServices__CMaps__CMapRouteVtbl;

interface __FIVectorView_1_Windows__CServices__CMaps__CMapRoute
{
    CONST_VTBL struct __FIVectorView_1_Windows__CServices__CMaps__CMapRouteVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRoute_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRoute_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRoute_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRoute_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRoute_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRoute_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRoute_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRoute_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRoute_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRoute_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CServices__CMaps__CMapRoute_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg;

typedef struct __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLegVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        UINT32 index,
        __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLegVtbl;

interface __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg
{
    CONST_VTBL struct __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLegVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver;

typedef struct __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuverVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        UINT32 index,
        __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuverVtbl;

interface __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver
{
    CONST_VTBL struct __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuverVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIReference_1_double_INTERFACE_DEFINED__)
#define ____FIReference_1_double_INTERFACE_DEFINED__

typedef interface __FIReference_1_double __FIReference_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_double;

typedef struct __FIReference_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_double* This,
        DOUBLE* result);

    END_INTERFACE
} __FIReference_1_doubleVtbl;

interface __FIReference_1_double
{
    CONST_VTBL struct __FIReference_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_double_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_double_INTERFACE_DEFINED__

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

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef enum __x_ABI_CWindows_CUI_CPopups_CPlacement __x_ABI_CWindows_CUI_CPopups_CPlacement;

typedef enum __x_ABI_CWindows_CServices_CMaps_CManeuverWarningKind __x_ABI_CWindows_CServices_CMaps_CManeuverWarningKind;

typedef enum __x_ABI_CWindows_CServices_CMaps_CManeuverWarningSeverity __x_ABI_CWindows_CServices_CMaps_CManeuverWarningSeverity;

typedef enum __x_ABI_CWindows_CServices_CMaps_CMapLocationDesiredAccuracy __x_ABI_CWindows_CServices_CMaps_CMapLocationDesiredAccuracy;

typedef enum __x_ABI_CWindows_CServices_CMaps_CMapLocationFinderStatus __x_ABI_CWindows_CServices_CMaps_CMapLocationFinderStatus;

typedef enum __x_ABI_CWindows_CServices_CMaps_CMapManeuverNotices __x_ABI_CWindows_CServices_CMaps_CMapManeuverNotices;

typedef enum __x_ABI_CWindows_CServices_CMaps_CMapRouteFinderStatus __x_ABI_CWindows_CServices_CMaps_CMapRouteFinderStatus;

typedef enum __x_ABI_CWindows_CServices_CMaps_CMapRouteManeuverKind __x_ABI_CWindows_CServices_CMaps_CMapRouteManeuverKind;

typedef enum __x_ABI_CWindows_CServices_CMaps_CMapRouteOptimization __x_ABI_CWindows_CServices_CMaps_CMapRouteOptimization;

typedef enum __x_ABI_CWindows_CServices_CMaps_CMapRouteRestrictions __x_ABI_CWindows_CServices_CMaps_CMapRouteRestrictions;

typedef enum __x_ABI_CWindows_CServices_CMaps_CMapServiceDataUsagePreference __x_ABI_CWindows_CServices_CMaps_CMapServiceDataUsagePreference;

typedef enum __x_ABI_CWindows_CServices_CMaps_CTrafficCongestion __x_ABI_CWindows_CServices_CMaps_CTrafficCongestion;

typedef enum __x_ABI_CWindows_CServices_CMaps_CWaypointKind __x_ABI_CWindows_CServices_CMaps_CWaypointKind;

/*
 *
 * Struct Windows.Services.Maps.ManeuverWarningKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CServices_CMaps_CManeuverWarningKind
{
    ManeuverWarningKind_None = 0,
    ManeuverWarningKind_Accident = 1,
    ManeuverWarningKind_AdministrativeDivisionChange = 2,
    ManeuverWarningKind_Alert = 3,
    ManeuverWarningKind_BlockedRoad = 4,
    ManeuverWarningKind_CheckTimetable = 5,
    ManeuverWarningKind_Congestion = 6,
    ManeuverWarningKind_Construction = 7,
    ManeuverWarningKind_CountryChange = 8,
    ManeuverWarningKind_DisabledVehicle = 9,
    ManeuverWarningKind_GateAccess = 10,
    ManeuverWarningKind_GetOffTransit = 11,
    ManeuverWarningKind_GetOnTransit = 12,
    ManeuverWarningKind_IllegalUTurn = 13,
    ManeuverWarningKind_MassTransit = 14,
    ManeuverWarningKind_Miscellaneous = 15,
    ManeuverWarningKind_NoIncident = 16,
    ManeuverWarningKind_Other = 17,
    ManeuverWarningKind_OtherNews = 18,
    ManeuverWarningKind_OtherTrafficIncidents = 19,
    ManeuverWarningKind_PlannedEvent = 20,
    ManeuverWarningKind_PrivateRoad = 21,
    ManeuverWarningKind_RestrictedTurn = 22,
    ManeuverWarningKind_RoadClosures = 23,
    ManeuverWarningKind_RoadHazard = 24,
    ManeuverWarningKind_ScheduledConstruction = 25,
    ManeuverWarningKind_SeasonalClosures = 26,
    ManeuverWarningKind_Tollbooth = 27,
    ManeuverWarningKind_TollRoad = 28,
    ManeuverWarningKind_TollZoneEnter = 29,
    ManeuverWarningKind_TollZoneExit = 30,
    ManeuverWarningKind_TrafficFlow = 31,
    ManeuverWarningKind_TransitLineChange = 32,
    ManeuverWarningKind_UnpavedRoad = 33,
    ManeuverWarningKind_UnscheduledConstruction = 34,
    ManeuverWarningKind_Weather = 35,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Services.Maps.ManeuverWarningSeverity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CServices_CMaps_CManeuverWarningSeverity
{
    ManeuverWarningSeverity_None = 0,
    ManeuverWarningSeverity_LowImpact = 1,
    ManeuverWarningSeverity_Minor = 2,
    ManeuverWarningSeverity_Moderate = 3,
    ManeuverWarningSeverity_Serious = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Services.Maps.MapLocationDesiredAccuracy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CServices_CMaps_CMapLocationDesiredAccuracy
{
    MapLocationDesiredAccuracy_High = 0,
    MapLocationDesiredAccuracy_Low = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Services.Maps.MapLocationFinderStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CServices_CMaps_CMapLocationFinderStatus
{
    MapLocationFinderStatus_Success = 0,
    MapLocationFinderStatus_UnknownError = 1,
    MapLocationFinderStatus_InvalidCredentials = 2,
    MapLocationFinderStatus_BadLocation = 3,
    MapLocationFinderStatus_IndexFailure = 4,
    MapLocationFinderStatus_NetworkFailure = 5,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    MapLocationFinderStatus_NotSupported = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.MapManeuverNotices
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CServices_CMaps_CMapManeuverNotices
{
    MapManeuverNotices_None = 0,
    MapManeuverNotices_Toll = 0x1,
    MapManeuverNotices_Unpaved = 0x2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.MapRouteFinderStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CServices_CMaps_CMapRouteFinderStatus
{
    MapRouteFinderStatus_Success = 0,
    MapRouteFinderStatus_UnknownError = 1,
    MapRouteFinderStatus_InvalidCredentials = 2,
    MapRouteFinderStatus_NoRouteFound = 3,
    MapRouteFinderStatus_NoRouteFoundWithGivenOptions = 4,
    MapRouteFinderStatus_StartPointNotFound = 5,
    MapRouteFinderStatus_EndPointNotFound = 6,
    MapRouteFinderStatus_NoPedestrianRouteFound = 7,
    MapRouteFinderStatus_NetworkFailure = 8,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    MapRouteFinderStatus_NotSupported = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.MapRouteManeuverKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CServices_CMaps_CMapRouteManeuverKind
{
    MapRouteManeuverKind_None = 0,
    MapRouteManeuverKind_Start = 1,
    MapRouteManeuverKind_Stopover = 2,
    MapRouteManeuverKind_StopoverResume = 3,
    MapRouteManeuverKind_End = 4,
    MapRouteManeuverKind_GoStraight = 5,
    MapRouteManeuverKind_UTurnLeft = 6,
    MapRouteManeuverKind_UTurnRight = 7,
    MapRouteManeuverKind_TurnKeepLeft = 8,
    MapRouteManeuverKind_TurnKeepRight = 9,
    MapRouteManeuverKind_TurnLightLeft = 10,
    MapRouteManeuverKind_TurnLightRight = 11,
    MapRouteManeuverKind_TurnLeft = 12,
    MapRouteManeuverKind_TurnRight = 13,
    MapRouteManeuverKind_TurnHardLeft = 14,
    MapRouteManeuverKind_TurnHardRight = 15,
    MapRouteManeuverKind_FreewayEnterLeft = 16,
    MapRouteManeuverKind_FreewayEnterRight = 17,
    MapRouteManeuverKind_FreewayLeaveLeft = 18,
    MapRouteManeuverKind_FreewayLeaveRight = 19,
    MapRouteManeuverKind_FreewayContinueLeft = 20,
    MapRouteManeuverKind_FreewayContinueRight = 21,
    MapRouteManeuverKind_TrafficCircleLeft = 22,
    MapRouteManeuverKind_TrafficCircleRight = 23,
    MapRouteManeuverKind_TakeFerry = 24,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.MapRouteOptimization
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CServices_CMaps_CMapRouteOptimization
{
    MapRouteOptimization_Time = 0,
    MapRouteOptimization_Distance = 1,
    MapRouteOptimization_TimeWithTraffic = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    MapRouteOptimization_Scenic = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.MapRouteRestrictions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CServices_CMaps_CMapRouteRestrictions
{
    MapRouteRestrictions_None = 0,
    MapRouteRestrictions_Highways = 0x1,
    MapRouteRestrictions_TollRoads = 0x2,
    MapRouteRestrictions_Ferries = 0x4,
    MapRouteRestrictions_Tunnels = 0x8,
    MapRouteRestrictions_DirtRoads = 0x10,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    MapRouteRestrictions_Motorail = 0x20,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.MapServiceDataUsagePreference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CServices_CMaps_CMapServiceDataUsagePreference
{
    MapServiceDataUsagePreference_Default = 0,
    MapServiceDataUsagePreference_OfflineMapDataOnly = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Services.Maps.TrafficCongestion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CServices_CMaps_CTrafficCongestion
{
    TrafficCongestion_Unknown = 0,
    TrafficCongestion_Light = 1,
    TrafficCongestion_Mild = 2,
    TrafficCongestion_Medium = 3,
    TrafficCongestion_Heavy = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Services.Maps.WaypointKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CServices_CMaps_CWaypointKind
{
    WaypointKind_Stop = 0,
    WaypointKind_Via = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.IEnhancedWaypoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.EnhancedWaypoint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IEnhancedWaypoint[] = L"Windows.Services.Maps.IEnhancedWaypoint";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Point)(__x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint** value);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint* This,
        enum __x_ABI_CWindows_CServices_CMaps_CWaypointKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_get_Point(This, value) \
    ((This)->lpVtbl->get_Point(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.IEnhancedWaypointFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.EnhancedWaypoint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IEnhancedWaypointFactory[] = L"Windows.Services.Maps.IEnhancedWaypointFactory";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* point,
        enum __x_ABI_CWindows_CServices_CMaps_CWaypointKind kind,
        __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypoint** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactoryVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_Create(This, point, kind, value) \
    ((This)->lpVtbl->Create(This, point, kind, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIEnhancedWaypointFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.IManeuverWarning
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.ManeuverWarning
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IManeuverWarning[] = L"Windows.Services.Maps.IManeuverWarning";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIManeuverWarningVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIManeuverWarning* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIManeuverWarning* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIManeuverWarning* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIManeuverWarning* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIManeuverWarning* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIManeuverWarning* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CServices_CMaps_CIManeuverWarning* This,
        enum __x_ABI_CWindows_CServices_CMaps_CManeuverWarningKind* value);
    HRESULT (STDMETHODCALLTYPE* get_Severity)(__x_ABI_CWindows_CServices_CMaps_CIManeuverWarning* This,
        enum __x_ABI_CWindows_CServices_CMaps_CManeuverWarningSeverity* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIManeuverWarningVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIManeuverWarning
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIManeuverWarningVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_get_Severity(This, value) \
    ((This)->lpVtbl->get_Severity(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIManeuverWarning;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIManeuverWarning_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.IMapAddress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapAddress
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapAddress_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapAddress_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapAddress[] = L"Windows.Services.Maps.IMapAddress";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapAddressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BuildingName)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BuildingFloor)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BuildingRoom)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BuildingWing)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_StreetNumber)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Street)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Neighborhood)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_District)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Town)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Region)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RegionCode)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Country)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CountryCode)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PostCode)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Continent)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapAddressVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapAddress
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapAddressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_get_BuildingName(This, value) \
    ((This)->lpVtbl->get_BuildingName(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_get_BuildingFloor(This, value) \
    ((This)->lpVtbl->get_BuildingFloor(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_get_BuildingRoom(This, value) \
    ((This)->lpVtbl->get_BuildingRoom(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_get_BuildingWing(This, value) \
    ((This)->lpVtbl->get_BuildingWing(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_get_StreetNumber(This, value) \
    ((This)->lpVtbl->get_StreetNumber(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_get_Street(This, value) \
    ((This)->lpVtbl->get_Street(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_get_Neighborhood(This, value) \
    ((This)->lpVtbl->get_Neighborhood(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_get_District(This, value) \
    ((This)->lpVtbl->get_District(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_get_Town(This, value) \
    ((This)->lpVtbl->get_Town(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_get_Region(This, value) \
    ((This)->lpVtbl->get_Region(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_get_RegionCode(This, value) \
    ((This)->lpVtbl->get_RegionCode(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_get_Country(This, value) \
    ((This)->lpVtbl->get_Country(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_get_CountryCode(This, value) \
    ((This)->lpVtbl->get_CountryCode(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_get_PostCode(This, value) \
    ((This)->lpVtbl->get_PostCode(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress_get_Continent(This, value) \
    ((This)->lpVtbl->get_Continent(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapAddress;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapAddress_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapAddress2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapAddress
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapAddress2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapAddress2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapAddress2[] = L"Windows.Services.Maps.IMapAddress2";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapAddress2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FormattedAddress)(__x_ABI_CWindows_CServices_CMaps_CIMapAddress2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapAddress2Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapAddress2
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapAddress2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapAddress2_get_FormattedAddress(This, value) \
    ((This)->lpVtbl->get_FormattedAddress(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapAddress2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapAddress2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapLocation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapLocation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapLocation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapLocation[] = L"Windows.Services.Maps.IMapLocation";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Point)(__x_ABI_CWindows_CServices_CMaps_CIMapLocation* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint** value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CServices_CMaps_CIMapLocation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CServices_CMaps_CIMapLocation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Address)(__x_ABI_CWindows_CServices_CMaps_CIMapLocation* This,
        __x_ABI_CWindows_CServices_CMaps_CIMapAddress** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapLocationVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapLocation
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocation_get_Point(This, value) \
    ((This)->lpVtbl->get_Point(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocation_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocation_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocation_get_Address(This, value) \
    ((This)->lpVtbl->get_Address(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapLocation;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapLocation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapLocationFinderResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapLocationFinderResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapLocationFinderResult[] = L"Windows.Services.Maps.IMapLocationFinderResult";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Locations)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult* This,
        __FIVectorView_1_Windows__CServices__CMaps__CMapLocation** value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult* This,
        enum __x_ABI_CWindows_CServices_CMaps_CMapLocationFinderStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResultVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_get_Locations(This, value) \
    ((This)->lpVtbl->get_Locations(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapLocationFinderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapLocationFinder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapLocationFinderStatics[] = L"Windows.Services.Maps.IMapLocationFinderStatics";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindLocationsAtAsync)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* queryPoint,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult** result);
    HRESULT (STDMETHODCALLTYPE* FindLocationsAsync)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics* This,
        HSTRING searchText,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* referencePoint,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult** result);
    HRESULT (STDMETHODCALLTYPE* FindLocationsWithMaxCountAsync)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics* This,
        HSTRING searchText,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* referencePoint,
        UINT32 maxCount,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStaticsVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_FindLocationsAtAsync(This, queryPoint, result) \
    ((This)->lpVtbl->FindLocationsAtAsync(This, queryPoint, result))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_FindLocationsAsync(This, searchText, referencePoint, result) \
    ((This)->lpVtbl->FindLocationsAsync(This, searchText, referencePoint, result))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_FindLocationsWithMaxCountAsync(This, searchText, referencePoint, maxCount, result) \
    ((This)->lpVtbl->FindLocationsWithMaxCountAsync(This, searchText, referencePoint, maxCount, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapLocationFinderStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapLocationFinder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapLocationFinderStatics2[] = L"Windows.Services.Maps.IMapLocationFinderStatics2";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindLocationsAtWithAccuracyAsync)(__x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* queryPoint,
        enum __x_ABI_CWindows_CServices_CMaps_CMapLocationDesiredAccuracy accuracy,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapLocationFinderResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_FindLocationsAtWithAccuracyAsync(This, queryPoint, accuracy, result) \
    ((This)->lpVtbl->FindLocationsAtWithAccuracyAsync(This, queryPoint, accuracy, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapLocationFinderStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Services.Maps.IMapManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapManagerStatics[] = L"Windows.Services.Maps.IMapManagerStatics";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ShowDownloadedMapsUI)(__x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* ShowMapsUpdateUI)(__x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics* This);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapManagerStaticsVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_ShowDownloadedMapsUI(This) \
    ((This)->lpVtbl->ShowDownloadedMapsUI(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_ShowMapsUpdateUI(This) \
    ((This)->lpVtbl->ShowMapsUpdateUI(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRoute
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRoute
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRoute_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRoute_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRoute[] = L"Windows.Services.Maps.IMapRoute";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BoundingBox)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox** value);
    HRESULT (STDMETHODCALLTYPE* get_LengthInMeters)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_EstimatedDuration)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Path)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath** value);
    HRESULT (STDMETHODCALLTYPE* get_Legs)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute* This,
        __FIVectorView_1_Windows__CServices__CMaps__CMapRouteLeg** value);
    HRESULT (STDMETHODCALLTYPE* get_IsTrafficBased)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapRouteVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapRoute
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute_get_BoundingBox(This, value) \
    ((This)->lpVtbl->get_BoundingBox(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute_get_LengthInMeters(This, value) \
    ((This)->lpVtbl->get_LengthInMeters(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute_get_EstimatedDuration(This, value) \
    ((This)->lpVtbl->get_EstimatedDuration(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute_get_Path(This, value) \
    ((This)->lpVtbl->get_Path(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute_get_Legs(This, value) \
    ((This)->lpVtbl->get_Legs(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute_get_IsTrafficBased(This, value) \
    ((This)->lpVtbl->get_IsTrafficBased(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRoute;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRoute_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRoute2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRoute
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRoute2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRoute2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRoute2[] = L"Windows.Services.Maps.IMapRoute2";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapRoute2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ViolatedRestrictions)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute2* This,
        enum __x_ABI_CWindows_CServices_CMaps_CMapRouteRestrictions* value);
    HRESULT (STDMETHODCALLTYPE* get_HasBlockedRoads)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapRoute2Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapRoute2
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapRoute2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute2_get_ViolatedRestrictions(This, value) \
    ((This)->lpVtbl->get_ViolatedRestrictions(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute2_get_HasBlockedRoads(This, value) \
    ((This)->lpVtbl->get_HasBlockedRoads(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRoute2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRoute2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRoute3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRoute
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRoute3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRoute3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRoute3[] = L"Windows.Services.Maps.IMapRoute3";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapRoute3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DurationWithoutTraffic)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute3* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_TrafficCongestion)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute3* This,
        enum __x_ABI_CWindows_CServices_CMaps_CTrafficCongestion* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapRoute3Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapRoute3
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapRoute3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute3_get_DurationWithoutTraffic(This, value) \
    ((This)->lpVtbl->get_DurationWithoutTraffic(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute3_get_TrafficCongestion(This, value) \
    ((This)->lpVtbl->get_TrafficCongestion(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRoute3;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRoute3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.IMapRoute4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRoute
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRoute4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRoute4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRoute4[] = L"Windows.Services.Maps.IMapRoute4";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapRoute4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsScenic)(__x_ABI_CWindows_CServices_CMaps_CIMapRoute4* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapRoute4Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapRoute4
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapRoute4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRoute4_get_IsScenic(This, value) \
    ((This)->lpVtbl->get_IsScenic(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRoute4;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRoute4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteDrivingOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteDrivingOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteDrivingOptions[] = L"Windows.Services.Maps.IMapRouteDrivingOptions";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaxAlternateRouteCount)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_MaxAlternateRouteCount)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_InitialHeading)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions* This,
        __FIReference_1_double** value);
    HRESULT (STDMETHODCALLTYPE* put_InitialHeading)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions* This,
        __FIReference_1_double* value);
    HRESULT (STDMETHODCALLTYPE* get_RouteOptimization)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions* This,
        enum __x_ABI_CWindows_CServices_CMaps_CMapRouteOptimization* value);
    HRESULT (STDMETHODCALLTYPE* put_RouteOptimization)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions* This,
        enum __x_ABI_CWindows_CServices_CMaps_CMapRouteOptimization value);
    HRESULT (STDMETHODCALLTYPE* get_RouteRestrictions)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions* This,
        enum __x_ABI_CWindows_CServices_CMaps_CMapRouteRestrictions* value);
    HRESULT (STDMETHODCALLTYPE* put_RouteRestrictions)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions* This,
        enum __x_ABI_CWindows_CServices_CMaps_CMapRouteRestrictions value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptionsVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_get_MaxAlternateRouteCount(This, value) \
    ((This)->lpVtbl->get_MaxAlternateRouteCount(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_put_MaxAlternateRouteCount(This, value) \
    ((This)->lpVtbl->put_MaxAlternateRouteCount(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_get_InitialHeading(This, value) \
    ((This)->lpVtbl->get_InitialHeading(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_put_InitialHeading(This, value) \
    ((This)->lpVtbl->put_InitialHeading(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_get_RouteOptimization(This, value) \
    ((This)->lpVtbl->get_RouteOptimization(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_put_RouteOptimization(This, value) \
    ((This)->lpVtbl->put_RouteOptimization(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_get_RouteRestrictions(This, value) \
    ((This)->lpVtbl->get_RouteRestrictions(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_put_RouteRestrictions(This, value) \
    ((This)->lpVtbl->put_RouteRestrictions(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteDrivingOptions2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteDrivingOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteDrivingOptions2[] = L"Windows.Services.Maps.IMapRouteDrivingOptions2";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DepartureTime)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* put_DepartureTime)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_get_DepartureTime(This, value) \
    ((This)->lpVtbl->get_DepartureTime(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_put_DepartureTime(This, value) \
    ((This)->lpVtbl->put_DepartureTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteFinderResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteFinderResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteFinderResult[] = L"Windows.Services.Maps.IMapRouteFinderResult";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Route)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult* This,
        __x_ABI_CWindows_CServices_CMaps_CIMapRoute** value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult* This,
        enum __x_ABI_CWindows_CServices_CMaps_CMapRouteFinderStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResultVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_get_Route(This, value) \
    ((This)->lpVtbl->get_Route(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteFinderResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteFinderResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteFinderResult2[] = L"Windows.Services.Maps.IMapRouteFinderResult2";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AlternateRoutes)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2* This,
        __FIVectorView_1_Windows__CServices__CMaps__CMapRoute** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_get_AlternateRoutes(This, value) \
    ((This)->lpVtbl->get_AlternateRoutes(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteFinderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteFinder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteFinderStatics[] = L"Windows.Services.Maps.IMapRouteFinderStatics";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDrivingRouteAsync)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* startPoint,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* endPoint,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result);
    HRESULT (STDMETHODCALLTYPE* GetDrivingRouteWithOptimizationAsync)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* startPoint,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* endPoint,
        enum __x_ABI_CWindows_CServices_CMaps_CMapRouteOptimization optimization,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result);
    HRESULT (STDMETHODCALLTYPE* GetDrivingRouteWithOptimizationAndRestrictionsAsync)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* startPoint,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* endPoint,
        enum __x_ABI_CWindows_CServices_CMaps_CMapRouteOptimization optimization,
        enum __x_ABI_CWindows_CServices_CMaps_CMapRouteRestrictions restrictions,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result);
    HRESULT (STDMETHODCALLTYPE* GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* startPoint,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* endPoint,
        enum __x_ABI_CWindows_CServices_CMaps_CMapRouteOptimization optimization,
        enum __x_ABI_CWindows_CServices_CMaps_CMapRouteRestrictions restrictions,
        DOUBLE headingInDegrees,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result);
    HRESULT (STDMETHODCALLTYPE* GetDrivingRouteFromWaypointsAsync)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics* This,
        __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint* wayPoints,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result);
    HRESULT (STDMETHODCALLTYPE* GetDrivingRouteFromWaypointsAndOptimizationAsync)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics* This,
        __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint* wayPoints,
        enum __x_ABI_CWindows_CServices_CMaps_CMapRouteOptimization optimization,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result);
    HRESULT (STDMETHODCALLTYPE* GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics* This,
        __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint* wayPoints,
        enum __x_ABI_CWindows_CServices_CMaps_CMapRouteOptimization optimization,
        enum __x_ABI_CWindows_CServices_CMaps_CMapRouteRestrictions restrictions,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result);
    HRESULT (STDMETHODCALLTYPE* GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics* This,
        __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint* wayPoints,
        enum __x_ABI_CWindows_CServices_CMaps_CMapRouteOptimization optimization,
        enum __x_ABI_CWindows_CServices_CMaps_CMapRouteRestrictions restrictions,
        DOUBLE headingInDegrees,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result);
    HRESULT (STDMETHODCALLTYPE* GetWalkingRouteAsync)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* startPoint,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* endPoint,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result);
    HRESULT (STDMETHODCALLTYPE* GetWalkingRouteFromWaypointsAsync)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics* This,
        __FIIterable_1_Windows__CDevices__CGeolocation__CGeopoint* wayPoints,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStaticsVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_GetDrivingRouteAsync(This, startPoint, endPoint, result) \
    ((This)->lpVtbl->GetDrivingRouteAsync(This, startPoint, endPoint, result))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_GetDrivingRouteWithOptimizationAsync(This, startPoint, endPoint, optimization, result) \
    ((This)->lpVtbl->GetDrivingRouteWithOptimizationAsync(This, startPoint, endPoint, optimization, result))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_GetDrivingRouteWithOptimizationAndRestrictionsAsync(This, startPoint, endPoint, optimization, restrictions, result) \
    ((This)->lpVtbl->GetDrivingRouteWithOptimizationAndRestrictionsAsync(This, startPoint, endPoint, optimization, restrictions, result))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync(This, startPoint, endPoint, optimization, restrictions, headingInDegrees, result) \
    ((This)->lpVtbl->GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync(This, startPoint, endPoint, optimization, restrictions, headingInDegrees, result))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_GetDrivingRouteFromWaypointsAsync(This, wayPoints, result) \
    ((This)->lpVtbl->GetDrivingRouteFromWaypointsAsync(This, wayPoints, result))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_GetDrivingRouteFromWaypointsAndOptimizationAsync(This, wayPoints, optimization, result) \
    ((This)->lpVtbl->GetDrivingRouteFromWaypointsAndOptimizationAsync(This, wayPoints, optimization, result))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync(This, wayPoints, optimization, restrictions, result) \
    ((This)->lpVtbl->GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync(This, wayPoints, optimization, restrictions, result))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync(This, wayPoints, optimization, restrictions, headingInDegrees, result) \
    ((This)->lpVtbl->GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync(This, wayPoints, optimization, restrictions, headingInDegrees, result))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_GetWalkingRouteAsync(This, startPoint, endPoint, result) \
    ((This)->lpVtbl->GetWalkingRouteAsync(This, startPoint, endPoint, result))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_GetWalkingRouteFromWaypointsAsync(This, wayPoints, result) \
    ((This)->lpVtbl->GetWalkingRouteFromWaypointsAsync(This, wayPoints, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteFinderStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteFinder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteFinderStatics2[] = L"Windows.Services.Maps.IMapRouteFinderStatics2";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDrivingRouteWithOptionsAsync)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* startPoint,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* endPoint,
        __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions* options,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_GetDrivingRouteWithOptionsAsync(This, startPoint, endPoint, options, result) \
    ((This)->lpVtbl->GetDrivingRouteWithOptionsAsync(This, startPoint, endPoint, options, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteFinderStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteFinder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteFinderStatics3[] = L"Windows.Services.Maps.IMapRouteFinderStatics3";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDrivingRouteFromEnhancedWaypointsAsync)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3* This,
        __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint* waypoints,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result);
    HRESULT (STDMETHODCALLTYPE* GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3* This,
        __FIIterable_1_Windows__CServices__CMaps__CEnhancedWaypoint* waypoints,
        __x_ABI_CWindows_CServices_CMaps_CIMapRouteDrivingOptions* options,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CMapRouteFinderResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_GetDrivingRouteFromEnhancedWaypointsAsync(This, waypoints, result) \
    ((This)->lpVtbl->GetDrivingRouteFromEnhancedWaypointsAsync(This, waypoints, result))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync(This, waypoints, options, result) \
    ((This)->lpVtbl->GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync(This, waypoints, options, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteFinderStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteLeg
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteLeg
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteLeg[] = L"Windows.Services.Maps.IMapRouteLeg";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteLegVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BoundingBox)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox** value);
    HRESULT (STDMETHODCALLTYPE* get_Path)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath** value);
    HRESULT (STDMETHODCALLTYPE* get_LengthInMeters)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_EstimatedDuration)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Maneuvers)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg* This,
        __FIVectorView_1_Windows__CServices__CMaps__CMapRouteManeuver** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapRouteLegVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteLegVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_get_BoundingBox(This, value) \
    ((This)->lpVtbl->get_BoundingBox(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_get_Path(This, value) \
    ((This)->lpVtbl->get_Path(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_get_LengthInMeters(This, value) \
    ((This)->lpVtbl->get_LengthInMeters(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_get_EstimatedDuration(This, value) \
    ((This)->lpVtbl->get_EstimatedDuration(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_get_Maneuvers(This, value) \
    ((This)->lpVtbl->get_Maneuvers(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteLeg2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteLeg
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteLeg2[] = L"Windows.Services.Maps.IMapRouteLeg2";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DurationWithoutTraffic)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_TrafficCongestion)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2* This,
        enum __x_ABI_CWindows_CServices_CMaps_CTrafficCongestion* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_get_DurationWithoutTraffic(This, value) \
    ((This)->lpVtbl->get_DurationWithoutTraffic(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_get_TrafficCongestion(This, value) \
    ((This)->lpVtbl->get_TrafficCongestion(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteLeg2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteManeuver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteManeuver
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteManeuver[] = L"Windows.Services.Maps.IMapRouteManeuver";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuverVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_StartingPoint)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint** value);
    HRESULT (STDMETHODCALLTYPE* get_LengthInMeters)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_InstructionText)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver* This,
        enum __x_ABI_CWindows_CServices_CMaps_CMapRouteManeuverKind* value);
    HRESULT (STDMETHODCALLTYPE* get_ExitNumber)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ManeuverNotices)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver* This,
        enum __x_ABI_CWindows_CServices_CMaps_CMapManeuverNotices* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuverVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuverVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_get_StartingPoint(This, value) \
    ((This)->lpVtbl->get_StartingPoint(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_get_LengthInMeters(This, value) \
    ((This)->lpVtbl->get_LengthInMeters(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_get_InstructionText(This, value) \
    ((This)->lpVtbl->get_InstructionText(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_get_ExitNumber(This, value) \
    ((This)->lpVtbl->get_ExitNumber(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_get_ManeuverNotices(This, value) \
    ((This)->lpVtbl->get_ManeuverNotices(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteManeuver2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteManeuver
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteManeuver2[] = L"Windows.Services.Maps.IMapRouteManeuver2";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_StartHeading)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_EndHeading)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_StreetName)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_get_StartHeading(This, value) \
    ((This)->lpVtbl->get_StartHeading(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_get_EndHeading(This, value) \
    ((This)->lpVtbl->get_EndHeading(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_get_StreetName(This, value) \
    ((This)->lpVtbl->get_StreetName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapRouteManeuver3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapRouteManeuver
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapRouteManeuver3[] = L"Windows.Services.Maps.IMapRouteManeuver3";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Warnings)(__x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3* This,
        __FIVectorView_1_Windows__CServices__CMaps__CManeuverWarning** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_get_Warnings(This, value) \
    ((This)->lpVtbl->get_Warnings(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapRouteManeuver3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.IMapServiceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapServiceStatics[] = L"Windows.Services.Maps.IMapServiceStatics";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapServiceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_ServiceToken)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceToken)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapServiceStaticsVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapServiceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_put_ServiceToken(This, value) \
    ((This)->lpVtbl->put_ServiceToken(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_get_ServiceToken(This, value) \
    ((This)->lpVtbl->get_ServiceToken(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapServiceStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapServiceStatics2[] = L"Windows.Services.Maps.IMapServiceStatics2";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WorldViewRegionCode)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_get_WorldViewRegionCode(This, value) \
    ((This)->lpVtbl->get_WorldViewRegionCode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.IMapServiceStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapServiceStatics3[] = L"Windows.Services.Maps.IMapServiceStatics3";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DataAttributions)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_get_DataAttributions(This, value) \
    ((This)->lpVtbl->get_DataAttributions(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Services.Maps.IMapServiceStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.MapService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IMapServiceStatics4[] = L"Windows.Services.Maps.IMapServiceStatics4";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_DataUsagePreference)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4* This,
        enum __x_ABI_CWindows_CServices_CMaps_CMapServiceDataUsagePreference value);
    HRESULT (STDMETHODCALLTYPE* get_DataUsagePreference)(__x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4* This,
        enum __x_ABI_CWindows_CServices_CMaps_CMapServiceDataUsagePreference* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_put_DataUsagePreference(This, value) \
    ((This)->lpVtbl->put_DataUsagePreference(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_get_DataUsagePreference(This, value) \
    ((This)->lpVtbl->get_DataUsagePreference(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIMapServiceStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.IPlaceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.PlaceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IPlaceInfo[] = L"Windows.Services.Maps.IPlaceInfo";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Show)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfo* This,
        struct __x_ABI_CWindows_CFoundation_CRect selection);
    HRESULT (STDMETHODCALLTYPE* ShowWithPreferredPlacement)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfo* This,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement);
    HRESULT (STDMETHODCALLTYPE* get_Identifier)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayAddress)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Geoshape)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfo* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_Show(This, selection) \
    ((This)->lpVtbl->Show(This, selection))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_ShowWithPreferredPlacement(This, selection, preferredPlacement) \
    ((This)->lpVtbl->ShowWithPreferredPlacement(This, selection, preferredPlacement))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_get_Identifier(This, value) \
    ((This)->lpVtbl->get_Identifier(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_get_DisplayAddress(This, value) \
    ((This)->lpVtbl->get_DisplayAddress(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_get_Geoshape(This, value) \
    ((This)->lpVtbl->get_Geoshape(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIPlaceInfo;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Services.Maps.IPlaceInfoCreateOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.PlaceInfoCreateOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IPlaceInfoCreateOptions[] = L"Windows.Services.Maps.IPlaceInfoCreateOptions";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayAddress)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayAddress)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptionsVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_put_DisplayAddress(This, value) \
    ((This)->lpVtbl->put_DisplayAddress(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_get_DisplayAddress(This, value) \
    ((This)->lpVtbl->get_DisplayAddress(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Services.Maps.IPlaceInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.PlaceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IPlaceInfoStatics[] = L"Windows.Services.Maps.IPlaceInfoStatics";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* referencePoint,
        __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo** resultValue);
    HRESULT (STDMETHODCALLTYPE* CreateWithGeopointAndOptions)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* referencePoint,
        __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions* options,
        __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo** resultValue);
    HRESULT (STDMETHODCALLTYPE* CreateFromIdentifier)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics* This,
        HSTRING identifier,
        __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo** resultValue);
    HRESULT (STDMETHODCALLTYPE* CreateFromIdentifierWithOptions)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics* This,
        HSTRING identifier,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* defaultPoint,
        __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoCreateOptions* options,
        __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo** resultValue);
    HRESULT (STDMETHODCALLTYPE* CreateFromMapLocation)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics* This,
        __x_ABI_CWindows_CServices_CMaps_CIMapLocation* location,
        __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo** resultValue);
    HRESULT (STDMETHODCALLTYPE* get_IsShowSupported)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStaticsVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_Create(This, referencePoint, resultValue) \
    ((This)->lpVtbl->Create(This, referencePoint, resultValue))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_CreateWithGeopointAndOptions(This, referencePoint, options, resultValue) \
    ((This)->lpVtbl->CreateWithGeopointAndOptions(This, referencePoint, options, resultValue))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_CreateFromIdentifier(This, identifier, resultValue) \
    ((This)->lpVtbl->CreateFromIdentifier(This, identifier, resultValue))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_CreateFromIdentifierWithOptions(This, identifier, defaultPoint, options, resultValue) \
    ((This)->lpVtbl->CreateFromIdentifierWithOptions(This, identifier, defaultPoint, options, resultValue))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_CreateFromMapLocation(This, location, resultValue) \
    ((This)->lpVtbl->CreateFromMapLocation(This, location, resultValue))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_get_IsShowSupported(This, value) \
    ((This)->lpVtbl->get_IsShowSupported(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Services.Maps.IPlaceInfoStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.PlaceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_IPlaceInfoStatics2[] = L"Windows.Services.Maps.IPlaceInfoStatics2";
typedef struct __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromAddress)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2* This,
        HSTRING displayAddress,
        __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo** resultValue);
    HRESULT (STDMETHODCALLTYPE* CreateFromAddressWithName)(__x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2* This,
        HSTRING displayAddress,
        HSTRING displayName,
        __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo** resultValue);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_CreateFromAddress(This, displayAddress, resultValue) \
    ((This)->lpVtbl->CreateFromAddress(This, displayAddress, resultValue))

#define __x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_CreateFromAddressWithName(This, displayAddress, displayName, resultValue) \
    ((This)->lpVtbl->CreateFromAddressWithName(This, displayAddress, displayName, resultValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CIPlaceInfoStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Services.Maps.EnhancedWaypoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Services.Maps.IEnhancedWaypointFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IEnhancedWaypoint ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Services_Maps_EnhancedWaypoint_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_EnhancedWaypoint_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_EnhancedWaypoint[] = L"Windows.Services.Maps.EnhancedWaypoint";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Services.Maps.ManeuverWarning
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IManeuverWarning ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Services_Maps_ManeuverWarning_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_ManeuverWarning_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_ManeuverWarning[] = L"Windows.Services.Maps.ManeuverWarning";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Services.Maps.MapAddress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IMapAddress ** Default Interface **
 *    Windows.Services.Maps.IMapAddress2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapAddress_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapAddress_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapAddress[] = L"Windows.Services.Maps.MapAddress";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IMapLocation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapLocation_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapLocation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapLocation[] = L"Windows.Services.Maps.MapLocation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapLocationFinder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.IMapLocationFinderStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Services.Maps.IMapLocationFinderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapLocationFinder_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapLocationFinder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapLocationFinder[] = L"Windows.Services.Maps.MapLocationFinder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapLocationFinderResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IMapLocationFinderResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapLocationFinderResult_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapLocationFinderResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapLocationFinderResult[] = L"Windows.Services.Maps.MapLocationFinderResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.IMapManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapManager_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapManager[] = L"Windows.Services.Maps.MapManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapRoute
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IMapRoute ** Default Interface **
 *    Windows.Services.Maps.IMapRoute2
 *    Windows.Services.Maps.IMapRoute3
 *    Windows.Services.Maps.IMapRoute4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapRoute_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapRoute_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapRoute[] = L"Windows.Services.Maps.MapRoute";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapRouteDrivingOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IMapRouteDrivingOptions ** Default Interface **
 *    Windows.Services.Maps.IMapRouteDrivingOptions2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapRouteDrivingOptions_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapRouteDrivingOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapRouteDrivingOptions[] = L"Windows.Services.Maps.MapRouteDrivingOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapRouteFinder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.IMapRouteFinderStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Services.Maps.IMapRouteFinderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Services.Maps.IMapRouteFinderStatics3 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapRouteFinder_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapRouteFinder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapRouteFinder[] = L"Windows.Services.Maps.MapRouteFinder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapRouteFinderResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IMapRouteFinderResult ** Default Interface **
 *    Windows.Services.Maps.IMapRouteFinderResult2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapRouteFinderResult_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapRouteFinderResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapRouteFinderResult[] = L"Windows.Services.Maps.MapRouteFinderResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapRouteLeg
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IMapRouteLeg ** Default Interface **
 *    Windows.Services.Maps.IMapRouteLeg2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapRouteLeg_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapRouteLeg_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapRouteLeg[] = L"Windows.Services.Maps.MapRouteLeg";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapRouteManeuver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IMapRouteManeuver ** Default Interface **
 *    Windows.Services.Maps.IMapRouteManeuver2
 *    Windows.Services.Maps.IMapRouteManeuver3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapRouteManeuver_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapRouteManeuver_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapRouteManeuver[] = L"Windows.Services.Maps.MapRouteManeuver";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.MapService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.IMapServiceStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Services.Maps.IMapServiceStatics3 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Services.Maps.IMapServiceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Services.Maps.IMapServiceStatics4 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_MapService_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_MapService_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_MapService[] = L"Windows.Services.Maps.MapService";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.PlaceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.IPlaceInfoStatics2 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Services.Maps.IPlaceInfoStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IPlaceInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Services_Maps_PlaceInfo_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_PlaceInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_PlaceInfo[] = L"Windows.Services.Maps.PlaceInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Services.Maps.PlaceInfoCreateOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.IPlaceInfoCreateOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Services_Maps_PlaceInfoCreateOptions_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_PlaceInfoCreateOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_PlaceInfoCreateOptions[] = L"Windows.Services.Maps.PlaceInfoCreateOptions";
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
#endif // __windows2Eservices2Emaps_p_h__

#endif // __windows2Eservices2Emaps_h__
