
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
#ifndef __windows2Edevices2Egeolocation_h__
#define __windows2Edevices2Egeolocation_h__
#ifndef __windows2Edevices2Egeolocation_p_h__
#define __windows2Edevices2Egeolocation_p_h__


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
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface ICivicAddress;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress ABI::Windows::Devices::Geolocation::ICivicAddress

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeoboundingBoxFactory;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory ABI::Windows::Devices::Geolocation::IGeoboundingBoxFactory

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeoboundingBoxStatics;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics ABI::Windows::Devices::Geolocation::IGeoboundingBoxStatics

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeocircle;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle ABI::Windows::Devices::Geolocation::IGeocircle

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeocircleFactory;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory ABI::Windows::Devices::Geolocation::IGeocircleFactory

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeocoordinate;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate ABI::Windows::Devices::Geolocation::IGeocoordinate

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeocoordinateSatelliteData;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData ABI::Windows::Devices::Geolocation::IGeocoordinateSatelliteData

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeocoordinateSatelliteData2;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2 ABI::Windows::Devices::Geolocation::IGeocoordinateSatelliteData2

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeocoordinateWithPoint;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint ABI::Windows::Devices::Geolocation::IGeocoordinateWithPoint

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeocoordinateWithPositionData;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData ABI::Windows::Devices::Geolocation::IGeocoordinateWithPositionData

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeocoordinateWithPositionSourceTimestamp;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp ABI::Windows::Devices::Geolocation::IGeocoordinateWithPositionSourceTimestamp

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeocoordinateWithRemoteSource;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource ABI::Windows::Devices::Geolocation::IGeocoordinateWithRemoteSource

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeolocator;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator ABI::Windows::Devices::Geolocation::IGeolocator

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeolocator2;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2 ABI::Windows::Devices::Geolocation::IGeolocator2

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeolocatorStatics;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics ABI::Windows::Devices::Geolocation::IGeolocatorStatics

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeolocatorStatics2;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2 ABI::Windows::Devices::Geolocation::IGeolocatorStatics2

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeolocatorWithScalarAccuracy;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy ABI::Windows::Devices::Geolocation::IGeolocatorWithScalarAccuracy

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeopathFactory;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory ABI::Windows::Devices::Geolocation::IGeopathFactory

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeopointFactory;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory ABI::Windows::Devices::Geolocation::IGeopointFactory

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeoposition;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition ABI::Windows::Devices::Geolocation::IGeoposition

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeoposition2;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2 ABI::Windows::Devices::Geolocation::IGeoposition2

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeovisit;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit ABI::Windows::Devices::Geolocation::IGeovisit

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeovisitMonitor;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor ABI::Windows::Devices::Geolocation::IGeovisitMonitor

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeovisitMonitorStatics;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics ABI::Windows::Devices::Geolocation::IGeovisitMonitorStatics

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeovisitStateChangedEventArgs;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs ABI::Windows::Devices::Geolocation::IGeovisitStateChangedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeovisitTriggerDetails;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails ABI::Windows::Devices::Geolocation::IGeovisitTriggerDetails

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IPositionChangedEventArgs;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs ABI::Windows::Devices::Geolocation::IPositionChangedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IStatusChangedEventArgs;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs ABI::Windows::Devices::Geolocation::IStatusChangedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IVenueData;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIVenueData ABI::Windows::Devices::Geolocation::IVenueData

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                typedef enum GeolocationAccessStatus : int GeolocationAccessStatus;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("de2b24d0-b726-57b1-a7c5-e5a13932b7de"))
IAsyncOperation<enum ABI::Windows::Devices::Geolocation::GeolocationAccessStatus> : IAsyncOperation_impl<enum ABI::Windows::Devices::Geolocation::GeolocationAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Geolocation.GeolocationAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Devices::Geolocation::GeolocationAccessStatus> __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_t;
#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f3524c93-e5c7-5b88-bedb-d3e637cff271"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Devices::Geolocation::GeolocationAccessStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Devices::Geolocation::GeolocationAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Geolocation.GeolocationAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Devices::Geolocation::GeolocationAccessStatus> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class Geoposition;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ee73ecf0-099d-57e5-8407-5b32e5af1cc4"))
IAsyncOperation<ABI::Windows::Devices::Geolocation::Geoposition*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geoposition*, ABI::Windows::Devices::Geolocation::IGeoposition*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Geolocation.Geoposition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Geolocation::Geoposition*> __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition_t;
#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7668a704-244e-5e12-8dcb-92a3299eba26"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Geolocation::Geoposition*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geoposition*, ABI::Windows::Devices::Geolocation::IGeoposition*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Geolocation.Geoposition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Geolocation::Geoposition*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class Geovisit;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8d1c950a-efb9-5440-a6a7-820a839be07b"))
IAsyncOperation<ABI::Windows::Devices::Geolocation::Geovisit*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geovisit*, ABI::Windows::Devices::Geolocation::IGeovisit*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Geolocation.Geovisit>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Geolocation::Geovisit*> __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit_t;
#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b9bce8cb-2e04-5269-9b03-1614d0c00b01"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Geolocation::Geovisit*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geovisit*, ABI::Windows::Devices::Geolocation::IGeovisit*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Geolocation.Geovisit>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Geolocation::Geovisit*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition_USE
#define DEF___FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a99b4206-263e-5308-82f2-31315c65135c"))
IIterator<ABI::Windows::Devices::Geolocation::Geoposition*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geoposition*, ABI::Windows::Devices::Geolocation::IGeoposition*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Geolocation.Geoposition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Geolocation::Geoposition*> __FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition_t;
#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition_USE
#define DEF___FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("135ed72d-75b1-5881-be41-6ffeaa202044"))
IIterable<ABI::Windows::Devices::Geolocation::Geoposition*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geoposition*, ABI::Windows::Devices::Geolocation::IGeoposition*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Geolocation.Geoposition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Geolocation::Geoposition*> __FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition_t;
#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_USE
#define DEF___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d572ccf3-0c60-553f-a624-c71648af8e7a"))
IVectorView<ABI::Windows::Devices::Geolocation::Geoposition*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geoposition*, ABI::Windows::Devices::Geolocation::IGeoposition*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Geolocation.Geoposition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Geolocation::Geoposition*> __FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_t;
#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9454b533-efb4-5385-8d3a-437fabc32d91"))
IAsyncOperation<__FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Geolocation.Geoposition>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition*> __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6c67a1d1-9441-5aee-b625-e3c1b5676a6d"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Geolocation.Geoposition>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                typedef struct BasicGeoposition BasicGeoposition;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition_USE
#define DEF___FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1b4e26a1-88e4-5872-bb2d-4f31700828b2"))
IIterator<struct ABI::Windows::Devices::Geolocation::BasicGeoposition> : IIterator_impl<struct ABI::Windows::Devices::Geolocation::BasicGeoposition>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Geolocation.BasicGeoposition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<struct ABI::Windows::Devices::Geolocation::BasicGeoposition> __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition_t;
#define __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition_USE
#define DEF___FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("922399a8-0093-5009-a8d2-f87b0eae75f5"))
IIterable<struct ABI::Windows::Devices::Geolocation::BasicGeoposition> : IIterable_impl<struct ABI::Windows::Devices::Geolocation::BasicGeoposition>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Geolocation.BasicGeoposition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<struct ABI::Windows::Devices::Geolocation::BasicGeoposition> __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition_t;
#define __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit_USE
#define DEF___FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f525fc34-b058-5345-8e28-3e69e5f59070"))
IIterator<ABI::Windows::Devices::Geolocation::Geovisit*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geovisit*, ABI::Windows::Devices::Geolocation::IGeovisit*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Geolocation.Geovisit>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Geolocation::Geovisit*> __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit_t;
#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit_USE
#define DEF___FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d5800189-0f3f-54a0-a749-6000c1e12e58"))
IIterable<ABI::Windows::Devices::Geolocation::Geovisit*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geovisit*, ABI::Windows::Devices::Geolocation::IGeovisit*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Geolocation.Geovisit>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Geolocation::Geovisit*> __FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit_t;
#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition_USE
#define DEF___FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("177f5719-e234-59db-99ba-f7fdddf31430"))
IVectorView<struct ABI::Windows::Devices::Geolocation::BasicGeoposition> : IVectorView_impl<struct ABI::Windows::Devices::Geolocation::BasicGeoposition>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Geolocation.BasicGeoposition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<struct ABI::Windows::Devices::Geolocation::BasicGeoposition> __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition_t;
#define __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit_USE
#define DEF___FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("90ad35e9-f1de-5ba7-abbf-04a21976d362"))
IVectorView<ABI::Windows::Devices::Geolocation::Geovisit*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geovisit*, ABI::Windows::Devices::Geolocation::IGeovisit*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Geolocation.Geovisit>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Geolocation::Geovisit*> __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit_t;
#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000


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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition_USE
#define DEF___FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e4d5dda6-f57c-57cc-b67f-2939a901dabe"))
IReference<struct ABI::Windows::Devices::Geolocation::BasicGeoposition> : IReference_impl<struct ABI::Windows::Devices::Geolocation::BasicGeoposition>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Devices.Geolocation.BasicGeoposition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Devices::Geolocation::BasicGeoposition> __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition_t;
#define __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition ABI::Windows::Foundation::__FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
                class Geolocator;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class PositionChangedEventArgs;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("df3c6164-4e7b-5e8e-9a7e-13da059dec1e"))
ITypedEventHandler<ABI::Windows::Devices::Geolocation::Geolocator*, ABI::Windows::Devices::Geolocation::PositionChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geolocator*, ABI::Windows::Devices::Geolocation::IGeolocator*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::PositionChangedEventArgs*, ABI::Windows::Devices::Geolocation::IPositionChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Geolocation.Geolocator, Windows.Devices.Geolocation.PositionChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Geolocation::Geolocator*, ABI::Windows::Devices::Geolocation::PositionChangedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class StatusChangedEventArgs;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("97fcf582-de6b-5cd3-9690-e2ecbb66da4d"))
ITypedEventHandler<ABI::Windows::Devices::Geolocation::Geolocator*, ABI::Windows::Devices::Geolocation::StatusChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geolocator*, ABI::Windows::Devices::Geolocation::IGeolocator*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::StatusChangedEventArgs*, ABI::Windows::Devices::Geolocation::IStatusChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Geolocation.Geolocator, Windows.Devices.Geolocation.StatusChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Geolocation::Geolocator*, ABI::Windows::Devices::Geolocation::StatusChangedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class GeovisitMonitor;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class GeovisitStateChangedEventArgs;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("76abc5ea-ee4f-5391-9b50-deca5d4311c9"))
ITypedEventHandler<ABI::Windows::Devices::Geolocation::GeovisitMonitor*, ABI::Windows::Devices::Geolocation::GeovisitStateChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::GeovisitMonitor*, ABI::Windows::Devices::Geolocation::IGeovisitMonitor*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::GeovisitStateChangedEventArgs*, ABI::Windows::Devices::Geolocation::IGeovisitStateChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Geolocation.GeovisitMonitor, Windows.Devices.Geolocation.GeovisitStateChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Geolocation::GeovisitMonitor*, ABI::Windows::Devices::Geolocation::GeovisitStateChangedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

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
        namespace Devices {
            namespace Geolocation {
                typedef enum AltitudeReferenceSystem : int AltitudeReferenceSystem;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                typedef enum GeoshapeType : int GeoshapeType;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                typedef enum PositionAccuracy : int PositionAccuracy;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                typedef enum PositionSource : int PositionSource;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                typedef enum PositionStatus : int PositionStatus;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                typedef enum VisitMonitoringScope : int VisitMonitoringScope;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                typedef enum VisitStateChange : int VisitStateChange;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class CivicAddress;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class GeoboundingBox;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class Geocircle;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class Geocoordinate;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class GeocoordinateSatelliteData;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class Geopath;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class Geopoint;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class VenueData;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Geolocation.AltitudeReferenceSystem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                enum AltitudeReferenceSystem : int
                {
                    AltitudeReferenceSystem_Unspecified = 0,
                    AltitudeReferenceSystem_Terrain = 1,
                    AltitudeReferenceSystem_Ellipsoid = 2,
                    AltitudeReferenceSystem_Geoid = 3,
                    AltitudeReferenceSystem_Surface = 4,
                };
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Geolocation.GeolocationAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                enum GeolocationAccessStatus : int
                {
                    GeolocationAccessStatus_Unspecified = 0,
                    GeolocationAccessStatus_Allowed = 1,
                    GeolocationAccessStatus_Denied = 2,
                };
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Geolocation.GeoshapeType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                enum GeoshapeType : int
                {
                    GeoshapeType_Geopoint = 0,
                    GeoshapeType_Geocircle = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    GeoshapeType_Geopath = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    GeoshapeType_GeoboundingBox = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Geolocation.PositionAccuracy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                enum PositionAccuracy : int
                {
                    PositionAccuracy_Default = 0,
                    PositionAccuracy_High = 1,
                };
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Geolocation.PositionSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                enum PositionSource : int
                {
                    PositionSource_Cellular = 0,
                    PositionSource_Satellite = 1,
                    PositionSource_WiFi = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    PositionSource_IPAddress = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    PositionSource_Unknown = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    PositionSource_Default = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    PositionSource_Obfuscated = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                };
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Geolocation.PositionStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                enum PositionStatus : int
                {
                    PositionStatus_Ready = 0,
                    PositionStatus_Initializing = 1,
                    PositionStatus_NoData = 2,
                    PositionStatus_Disabled = 3,
                    PositionStatus_NotInitialized = 4,
                    PositionStatus_NotAvailable = 5,
                };
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Geolocation.VisitMonitoringScope
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                enum VisitMonitoringScope : int
                {
                    VisitMonitoringScope_Venue = 0,
                    VisitMonitoringScope_City = 1,
                };
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Devices.Geolocation.VisitStateChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                enum VisitStateChange : int
                {
                    VisitStateChange_TrackingLost = 0,
                    VisitStateChange_Arrived = 1,
                    VisitStateChange_Departed = 2,
                    VisitStateChange_OtherMovement = 3,
                };
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Devices.Geolocation.BasicGeoposition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                struct BasicGeoposition
                {
                    DOUBLE Latitude;
                    DOUBLE Longitude;
                    DOUBLE Altitude;
                };
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.ICivicAddress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.CivicAddress
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_ICivicAddress[] = L"Windows.Devices.Geolocation.ICivicAddress";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("a8567a1a-64f4-4d48-bcea-f6b008eca34c")
                ICivicAddress : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Country(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_City(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PostalCode(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICivicAddress = __uuidof(ICivicAddress);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeoboundingBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.GeoboundingBox
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Geolocation.IGeoshape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeoboundingBox[] = L"Windows.Devices.Geolocation.IGeoboundingBox";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("0896c80b-274f-43da-9a06-cbfcdaeb4ec2")
                IGeoboundingBox : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NorthwestCorner(
                        ABI::Windows::Devices::Geolocation::BasicGeoposition* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SoutheastCorner(
                        ABI::Windows::Devices::Geolocation::BasicGeoposition* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Center(
                        ABI::Windows::Devices::Geolocation::BasicGeoposition* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinAltitude(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxAltitude(
                        DOUBLE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeoboundingBox = __uuidof(IGeoboundingBox);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeoboundingBoxFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.GeoboundingBox
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeoboundingBoxFactory[] = L"Windows.Devices.Geolocation.IGeoboundingBoxFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("4dfba589-0411-4abc-b3b5-5bbccb57d98c")
                IGeoboundingBoxFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Devices::Geolocation::BasicGeoposition northwestCorner,
                        ABI::Windows::Devices::Geolocation::BasicGeoposition southeastCorner,
                        ABI::Windows::Devices::Geolocation::IGeoboundingBox** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithAltitudeReference(
                        ABI::Windows::Devices::Geolocation::BasicGeoposition northwestCorner,
                        ABI::Windows::Devices::Geolocation::BasicGeoposition southeastCorner,
                        ABI::Windows::Devices::Geolocation::AltitudeReferenceSystem altitudeReferenceSystem,
                        ABI::Windows::Devices::Geolocation::IGeoboundingBox** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithAltitudeReferenceAndSpatialReference(
                        ABI::Windows::Devices::Geolocation::BasicGeoposition northwestCorner,
                        ABI::Windows::Devices::Geolocation::BasicGeoposition southeastCorner,
                        ABI::Windows::Devices::Geolocation::AltitudeReferenceSystem altitudeReferenceSystem,
                        UINT32 spatialReferenceId,
                        ABI::Windows::Devices::Geolocation::IGeoboundingBox** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeoboundingBoxFactory = __uuidof(IGeoboundingBoxFactory);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeoboundingBoxStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.GeoboundingBox
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeoboundingBoxStatics[] = L"Windows.Devices.Geolocation.IGeoboundingBoxStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("67b80708-e61a-4cd0-841b-93233792b5ca")
                IGeoboundingBoxStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryCompute(
                        __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* positions,
                        ABI::Windows::Devices::Geolocation::IGeoboundingBox** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryComputeWithAltitudeReference(
                        __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* positions,
                        ABI::Windows::Devices::Geolocation::AltitudeReferenceSystem altitudeRefSystem,
                        ABI::Windows::Devices::Geolocation::IGeoboundingBox** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryComputeWithAltitudeReferenceAndSpatialReference(
                        __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* positions,
                        ABI::Windows::Devices::Geolocation::AltitudeReferenceSystem altitudeRefSystem,
                        UINT32 spatialReferenceId,
                        ABI::Windows::Devices::Geolocation::IGeoboundingBox** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeoboundingBoxStatics = __uuidof(IGeoboundingBoxStatics);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeocircle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geocircle
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Geolocation.IGeoshape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeocircle[] = L"Windows.Devices.Geolocation.IGeocircle";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("39e45843-a7f9-4e63-92a7-ba0c28d124b1")
                IGeocircle : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Center(
                        ABI::Windows::Devices::Geolocation::BasicGeoposition* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Radius(
                        DOUBLE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeocircle = __uuidof(IGeocircle);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeocircleFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geocircle
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeocircleFactory[] = L"Windows.Devices.Geolocation.IGeocircleFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("afd6531f-72b1-4f7d-87cc-4ed4c9849c05")
                IGeocircleFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Devices::Geolocation::BasicGeoposition position,
                        DOUBLE radius,
                        ABI::Windows::Devices::Geolocation::IGeocircle** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithAltitudeReferenceSystem(
                        ABI::Windows::Devices::Geolocation::BasicGeoposition position,
                        DOUBLE radius,
                        ABI::Windows::Devices::Geolocation::AltitudeReferenceSystem altitudeReferenceSystem,
                        ABI::Windows::Devices::Geolocation::IGeocircle** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithAltitudeReferenceSystemAndSpatialReferenceId(
                        ABI::Windows::Devices::Geolocation::BasicGeoposition position,
                        DOUBLE radius,
                        ABI::Windows::Devices::Geolocation::AltitudeReferenceSystem altitudeReferenceSystem,
                        UINT32 spatialReferenceId,
                        ABI::Windows::Devices::Geolocation::IGeocircle** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeocircleFactory = __uuidof(IGeocircleFactory);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeocoordinate
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geocoordinate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeocoordinate[] = L"Windows.Devices.Geolocation.IGeocoordinate";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("ee21a3aa-976a-4c70-803d-083ea55bcbc4")
                IGeocoordinate : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Latitude may be altered or unavailable after Windows 8.1. Instead, use Point.Position.Latitude")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Latitude(
                        DOUBLE* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Longitude may be altered or unavailable after Windows 8.1. Instead, use Point.Position.Longitude")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Longitude(
                        DOUBLE* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Altitude may be altered or unavailable after Windows 8.1. Instead, use Point.Position.Altitude")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Altitude(
                        __FIReference_1_double** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Accuracy(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AltitudeAccuracy(
                        __FIReference_1_double** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Heading(
                        __FIReference_1_double** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Speed(
                        __FIReference_1_double** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeocoordinate = __uuidof(IGeocoordinate);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeocoordinateSatelliteData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.GeocoordinateSatelliteData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeocoordinateSatelliteData[] = L"Windows.Devices.Geolocation.IGeocoordinateSatelliteData";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("c32a74d9-2608-474c-912c-06dd490f4af7")
                IGeocoordinateSatelliteData : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PositionDilutionOfPrecision(
                        __FIReference_1_double** ppValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HorizontalDilutionOfPrecision(
                        __FIReference_1_double** ppValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VerticalDilutionOfPrecision(
                        __FIReference_1_double** ppValue
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeocoordinateSatelliteData = __uuidof(IGeocoordinateSatelliteData);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeocoordinateSatelliteData2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.GeocoordinateSatelliteData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeocoordinateSatelliteData2[] = L"Windows.Devices.Geolocation.IGeocoordinateSatelliteData2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("761c8cfd-a19d-5a51-80f5-71676115483e")
                IGeocoordinateSatelliteData2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_GeometricDilutionOfPrecision(
                        __FIReference_1_double** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TimeDilutionOfPrecision(
                        __FIReference_1_double** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeocoordinateSatelliteData2 = __uuidof(IGeocoordinateSatelliteData2);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeocoordinateWithPoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geocoordinate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeocoordinateWithPoint[] = L"Windows.Devices.Geolocation.IGeocoordinateWithPoint";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("feea0525-d22c-4d46-b527-0b96066fc7db")
                IGeocoordinateWithPoint : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Point(
                        ABI::Windows::Devices::Geolocation::IGeopoint** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeocoordinateWithPoint = __uuidof(IGeocoordinateWithPoint);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeocoordinateWithPositionData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geocoordinate
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Geolocation.IGeocoordinate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeocoordinateWithPositionData[] = L"Windows.Devices.Geolocation.IGeocoordinateWithPositionData";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("95e634be-dbd6-40ac-b8f2-a65c0340d9a6")
                IGeocoordinateWithPositionData : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PositionSource(
                        ABI::Windows::Devices::Geolocation::PositionSource* pValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SatelliteData(
                        ABI::Windows::Devices::Geolocation::IGeocoordinateSatelliteData** ppValue
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeocoordinateWithPositionData = __uuidof(IGeocoordinateWithPositionData);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeocoordinateWithPositionSourceTimestamp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geocoordinate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeocoordinateWithPositionSourceTimestamp[] = L"Windows.Devices.Geolocation.IGeocoordinateWithPositionSourceTimestamp";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("8543fc02-c9f1-4610-afe0-8bc3a6a87036")
                IGeocoordinateWithPositionSourceTimestamp : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PositionSourceTimestamp(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeocoordinateWithPositionSourceTimestamp = __uuidof(IGeocoordinateWithPositionSourceTimestamp);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeocoordinateWithRemoteSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 14.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geocoordinate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeocoordinateWithRemoteSource[] = L"Windows.Devices.Geolocation.IGeocoordinateWithRemoteSource";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("397cebd7-ee38-5f3b-8900-c4a7bc9cf953")
                IGeocoordinateWithRemoteSource : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsRemoteSource(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeocoordinateWithRemoteSource = __uuidof(IGeocoordinateWithRemoteSource);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeolocator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geolocator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeolocator[] = L"Windows.Devices.Geolocation.IGeolocator";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("a9c3bf62-4524-4989-8aa9-de019d2e551f")
                IGeolocator : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DesiredAccuracy(
                        ABI::Windows::Devices::Geolocation::PositionAccuracy* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DesiredAccuracy(
                        ABI::Windows::Devices::Geolocation::PositionAccuracy value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MovementThreshold(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MovementThreshold(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReportInterval(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ReportInterval(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LocationStatus(
                        ABI::Windows::Devices::Geolocation::PositionStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetGeopositionAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetGeopositionAsyncWithAgeAndTimeout(
                        ABI::Windows::Foundation::TimeSpan maximumAge,
                        ABI::Windows::Foundation::TimeSpan timeout,
                        __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PositionChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PositionChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_StatusChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_StatusChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeolocator = __uuidof(IGeolocator);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeolocator2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geolocator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeolocator2[] = L"Windows.Devices.Geolocation.IGeolocator2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("d1b42e6d-8891-43b4-ad36-27c6fe9a97b1")
                IGeolocator2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE AllowFallbackToConsentlessPositions(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeolocator2 = __uuidof(IGeolocator2);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeolocatorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geolocator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeolocatorStatics[] = L"Windows.Devices.Geolocation.IGeolocatorStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("9a8e7571-2df5-4591-9f87-eb5fd894e9b7")
                IGeolocatorStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestAccessAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus** result
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
                    DEPRECATED("GetGeopositionHistoryAsync is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE GetGeopositionHistoryAsync(
                        ABI::Windows::Foundation::DateTime startTime,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition** result
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
                    DEPRECATED("GetGeopositionHistoryWithDurationAsync is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE GetGeopositionHistoryWithDurationAsync(
                        ABI::Windows::Foundation::DateTime startTime,
                        ABI::Windows::Foundation::TimeSpan duration,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeolocatorStatics = __uuidof(IGeolocatorStatics);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeolocatorStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geolocator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeolocatorStatics2[] = L"Windows.Devices.Geolocation.IGeolocatorStatics2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("993011a2-fa1c-4631-a71d-0dbeb1250d9c")
                IGeolocatorStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsDefaultGeopositionRecommended(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DefaultGeoposition(
                        __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DefaultGeoposition(
                        __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeolocatorStatics2 = __uuidof(IGeolocatorStatics2);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeolocatorWithScalarAccuracy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geolocator
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Geolocation.IGeolocator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeolocatorWithScalarAccuracy[] = L"Windows.Devices.Geolocation.IGeolocatorWithScalarAccuracy";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("96f5d3c1-b80f-460a-994d-a96c47a51aa4")
                IGeolocatorWithScalarAccuracy : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DesiredAccuracyInMeters(
                        __FIReference_1_UINT32** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DesiredAccuracyInMeters(
                        __FIReference_1_UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeolocatorWithScalarAccuracy = __uuidof(IGeolocatorWithScalarAccuracy);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeopath
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geopath
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Geolocation.IGeoshape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeopath[] = L"Windows.Devices.Geolocation.IGeopath";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("e53fd7b9-2da4-4714-a652-de8593289898")
                IGeopath : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Positions(
                        __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeopath = __uuidof(IGeopath);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeopath;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeopathFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geopath
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeopathFactory[] = L"Windows.Devices.Geolocation.IGeopathFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("27bea9c8-c7e7-4359-9b9b-fca3e05ef593")
                IGeopathFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* positions,
                        ABI::Windows::Devices::Geolocation::IGeopath** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithAltitudeReference(
                        __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* positions,
                        ABI::Windows::Devices::Geolocation::AltitudeReferenceSystem altitudeReferenceSystem,
                        ABI::Windows::Devices::Geolocation::IGeopath** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithAltitudeReferenceAndSpatialReference(
                        __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* positions,
                        ABI::Windows::Devices::Geolocation::AltitudeReferenceSystem altitudeReferenceSystem,
                        UINT32 spatialReferenceId,
                        ABI::Windows::Devices::Geolocation::IGeopath** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeopathFactory = __uuidof(IGeopathFactory);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeopoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geopoint
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Geolocation.IGeoshape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeopoint[] = L"Windows.Devices.Geolocation.IGeopoint";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("6bfa00eb-e56e-49bb-9caf-cbaa78a8bcef")
                IGeopoint : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Devices::Geolocation::BasicGeoposition* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeopoint = __uuidof(IGeopoint);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeopointFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geopoint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeopointFactory[] = L"Windows.Devices.Geolocation.IGeopointFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("db6b8d33-76bd-4e30-8af7-a844dc37b7a0")
                IGeopointFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Devices::Geolocation::BasicGeoposition position,
                        ABI::Windows::Devices::Geolocation::IGeopoint** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithAltitudeReferenceSystem(
                        ABI::Windows::Devices::Geolocation::BasicGeoposition position,
                        ABI::Windows::Devices::Geolocation::AltitudeReferenceSystem altitudeReferenceSystem,
                        ABI::Windows::Devices::Geolocation::IGeopoint** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithAltitudeReferenceSystemAndSpatialReferenceId(
                        ABI::Windows::Devices::Geolocation::BasicGeoposition position,
                        ABI::Windows::Devices::Geolocation::AltitudeReferenceSystem altitudeReferenceSystem,
                        UINT32 spatialReferenceId,
                        ABI::Windows::Devices::Geolocation::IGeopoint** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeopointFactory = __uuidof(IGeopointFactory);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeoposition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geoposition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeoposition[] = L"Windows.Devices.Geolocation.IGeoposition";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("c18d0454-7d41-4ff7-a957-9dffb4ef7f5b")
                IGeoposition : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Coordinate(
                        ABI::Windows::Devices::Geolocation::IGeocoordinate** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CivicAddress(
                        ABI::Windows::Devices::Geolocation::ICivicAddress** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeoposition = __uuidof(IGeoposition);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeoposition2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geoposition
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Geolocation.IGeoposition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeoposition2[] = L"Windows.Devices.Geolocation.IGeoposition2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("7f62f697-8671-4b0d-86f8-474a8496187c")
                IGeoposition2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_VenueData(
                        ABI::Windows::Devices::Geolocation::IVenueData** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeoposition2 = __uuidof(IGeoposition2);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeoshape
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeoshape[] = L"Windows.Devices.Geolocation.IGeoshape";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("c99ca2af-c729-43c1-8fab-d6dec914df7e")
                IGeoshape : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_GeoshapeType(
                        ABI::Windows::Devices::Geolocation::GeoshapeType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SpatialReferenceId(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AltitudeReferenceSystem(
                        ABI::Windows::Devices::Geolocation::AltitudeReferenceSystem* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeoshape = __uuidof(IGeoshape);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeovisit
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geovisit
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeovisit[] = L"Windows.Devices.Geolocation.IGeovisit";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("b1877a76-9ef6-41ab-a0dd-793ece76e2de")
                IGeovisit : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Devices::Geolocation::IGeoposition** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StateChange(
                        ABI::Windows::Devices::Geolocation::VisitStateChange* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeovisit = __uuidof(IGeovisit);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeovisitMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.GeovisitMonitor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeovisitMonitor[] = L"Windows.Devices.Geolocation.IGeovisitMonitor";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("80118aaf-5944-4591-83c1-396647f54f2c")
                IGeovisitMonitor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MonitoringScope(
                        ABI::Windows::Devices::Geolocation::VisitMonitoringScope* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Start(
                        ABI::Windows::Devices::Geolocation::VisitMonitoringScope value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_VisitStateChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_VisitStateChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeovisitMonitor = __uuidof(IGeovisitMonitor);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeovisitMonitorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.GeovisitMonitor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeovisitMonitorStatics[] = L"Windows.Devices.Geolocation.IGeovisitMonitorStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("bcf976a7-bbf2-4cdd-95cf-554c82edfb87")
                IGeovisitMonitorStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetLastReportAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeovisitMonitorStatics = __uuidof(IGeovisitMonitorStatics);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeovisitStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.GeovisitStateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeovisitStateChangedEventArgs[] = L"Windows.Devices.Geolocation.IGeovisitStateChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("ceb4d1ff-8b53-4968-beed-4cecd029ce15")
                IGeovisitStateChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Visit(
                        ABI::Windows::Devices::Geolocation::IGeovisit** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeovisitStateChangedEventArgs = __uuidof(IGeovisitStateChangedEventArgs);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeovisitTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.GeovisitTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeovisitTriggerDetails[] = L"Windows.Devices.Geolocation.IGeovisitTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("ea770d9e-d1c9-454b-99b7-b2f8cdd2482f")
                IGeovisitTriggerDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ReadReports(
                        __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit** values
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeovisitTriggerDetails = __uuidof(IGeovisitTriggerDetails);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Geolocation.IPositionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.PositionChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IPositionChangedEventArgs[] = L"Windows.Devices.Geolocation.IPositionChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("37859ce5-9d1e-46c5-bf3b-6ad8cac1a093")
                IPositionChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Devices::Geolocation::IGeoposition** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPositionChangedEventArgs = __uuidof(IPositionChangedEventArgs);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.StatusChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IStatusChangedEventArgs[] = L"Windows.Devices.Geolocation.IStatusChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("3453d2da-8c93-4111-a205-9aecfc9be5c0")
                IStatusChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Devices::Geolocation::PositionStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStatusChangedEventArgs = __uuidof(IStatusChangedEventArgs);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IVenueData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.VenueData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IVenueData[] = L"Windows.Devices.Geolocation.IVenueData";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                MIDL_INTERFACE("66f39187-60e3-4b2f-b527-4f53f1c3c677")
                IVenueData : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Level(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVenueData = __uuidof(IVenueData);
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIVenueData;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.CivicAddress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.ICivicAddress ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_CivicAddress_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_CivicAddress_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_CivicAddress[] = L"Windows.Devices.Geolocation.CivicAddress";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.GeoboundingBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Geolocation.IGeoboundingBoxFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Geolocation.IGeoboundingBoxStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeoboundingBox ** Default Interface **
 *    Windows.Devices.Geolocation.IGeoshape
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_GeoboundingBox_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_GeoboundingBox_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_GeoboundingBox[] = L"Windows.Devices.Geolocation.GeoboundingBox";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geocircle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Geolocation.IGeocircleFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeocircle ** Default Interface **
 *    Windows.Devices.Geolocation.IGeoshape
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geocircle_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geocircle_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geocircle[] = L"Windows.Devices.Geolocation.Geocircle";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geocoordinate
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeocoordinate ** Default Interface **
 *    Windows.Devices.Geolocation.IGeocoordinateWithPositionData
 *    Windows.Devices.Geolocation.IGeocoordinateWithPoint
 *    Windows.Devices.Geolocation.IGeocoordinateWithPositionSourceTimestamp
 *    Windows.Devices.Geolocation.IGeocoordinateWithRemoteSource
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geocoordinate_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geocoordinate_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geocoordinate[] = L"Windows.Devices.Geolocation.Geocoordinate";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.GeocoordinateSatelliteData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeocoordinateSatelliteData ** Default Interface **
 *    Windows.Devices.Geolocation.IGeocoordinateSatelliteData2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_GeocoordinateSatelliteData_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_GeocoordinateSatelliteData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_GeocoordinateSatelliteData[] = L"Windows.Devices.Geolocation.GeocoordinateSatelliteData";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geolocator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Geolocation.IGeolocatorStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Devices.Geolocation.IGeolocatorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeolocator ** Default Interface **
 *    Windows.Devices.Geolocation.IGeolocatorWithScalarAccuracy
 *    Windows.Devices.Geolocation.IGeolocator2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geolocator_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geolocator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geolocator[] = L"Windows.Devices.Geolocation.Geolocator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geopath
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Geolocation.IGeopathFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeopath ** Default Interface **
 *    Windows.Devices.Geolocation.IGeoshape
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geopath_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geopath_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geopath[] = L"Windows.Devices.Geolocation.Geopath";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geopoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Geolocation.IGeopointFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeopoint ** Default Interface **
 *    Windows.Devices.Geolocation.IGeoshape
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geopoint_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geopoint_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geopoint[] = L"Windows.Devices.Geolocation.Geopoint";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geoposition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeoposition ** Default Interface **
 *    Windows.Devices.Geolocation.IGeoposition2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geoposition_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geoposition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geoposition[] = L"Windows.Devices.Geolocation.Geoposition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geovisit
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeovisit ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geovisit_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geovisit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geovisit[] = L"Windows.Devices.Geolocation.Geovisit";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Devices.Geolocation.GeovisitMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Geolocation.IGeovisitMonitorStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeovisitMonitor ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_GeovisitMonitor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_GeovisitMonitor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_GeovisitMonitor[] = L"Windows.Devices.Geolocation.GeovisitMonitor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Devices.Geolocation.GeovisitStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeovisitStateChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_GeovisitStateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_GeovisitStateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_GeovisitStateChangedEventArgs[] = L"Windows.Devices.Geolocation.GeovisitStateChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Devices.Geolocation.GeovisitTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeovisitTriggerDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_GeovisitTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_GeovisitTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_GeovisitTriggerDetails[] = L"Windows.Devices.Geolocation.GeovisitTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Devices.Geolocation.PositionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IPositionChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_PositionChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_PositionChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_PositionChangedEventArgs[] = L"Windows.Devices.Geolocation.PositionChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.StatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IStatusChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_StatusChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_StatusChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_StatusChangedEventArgs[] = L"Windows.Devices.Geolocation.StatusChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.VenueData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IVenueData ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_VenueData_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_VenueData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_VenueData[] = L"Windows.Devices.Geolocation.VenueData";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2 __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2 __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2 __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2 __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs __x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs __x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIVenueData __x_ABI_CWindows_CDevices_CGeolocation_CIVenueData;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef enum __x_ABI_CWindows_CDevices_CGeolocation_CGeolocationAccessStatus __x_ABI_CWindows_CDevices_CGeolocation_CGeolocationAccessStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus* This,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CGeolocationAccessStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatusVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus* This,
        __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopositionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopositionVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopositionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopositionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopositionVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopositionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisitVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisitVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisitVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisitVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisitVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisitVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeovisit_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition __FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition;

typedef struct __FIIterator_1_Windows__CDevices__CGeolocation__CGeopositionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CGeolocation__CGeopositionVtbl;

interface __FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CGeolocation__CGeopositionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition __FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition;

typedef struct __FIIterable_1_Windows__CDevices__CGeolocation__CGeopositionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        __FIIterator_1_Windows__CDevices__CGeolocation__CGeoposition** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CGeolocation__CGeopositionVtbl;

interface __FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CGeolocation__CGeopositionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition __FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition;

typedef struct __FIVectorView_1_Windows__CDevices__CGeolocation__CGeopositionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CGeolocation__CGeopositionVtbl;

interface __FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CGeolocation__CGeopositionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeopositionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        __FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeopositionVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeopositionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeopositionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeopositionVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeopositionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition;

typedef struct __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeopositionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeopositionVtbl;

interface __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeopositionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition;

typedef struct __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeopositionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        __FIIterator_1_Windows__CDevices__CGeolocation__CBasicGeoposition** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeopositionVtbl;

interface __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeopositionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit;

typedef struct __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisitVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisitVtbl;

interface __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisitVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit __FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit;

typedef struct __FIIterable_1_Windows__CDevices__CGeolocation__CGeovisitVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        __FIIterator_1_Windows__CDevices__CGeolocation__CGeovisit** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CGeolocation__CGeovisitVtbl;

interface __FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CGeolocation__CGeovisitVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CGeolocation__CGeovisit_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition;

typedef struct __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeopositionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        UINT32 index,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeopositionVtbl;

interface __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeopositionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit;

typedef struct __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisitVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisitVtbl;

interface __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisitVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition;

typedef struct __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeopositionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition* result);

    END_INTERFACE
} __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeopositionVtbl;

interface __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition
{
    CONST_VTBL struct __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeopositionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* sender,
        __x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* sender,
        __x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor* sender,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef enum __x_ABI_CWindows_CDevices_CGeolocation_CAltitudeReferenceSystem __x_ABI_CWindows_CDevices_CGeolocation_CAltitudeReferenceSystem;

typedef enum __x_ABI_CWindows_CDevices_CGeolocation_CGeoshapeType __x_ABI_CWindows_CDevices_CGeolocation_CGeoshapeType;

typedef enum __x_ABI_CWindows_CDevices_CGeolocation_CPositionAccuracy __x_ABI_CWindows_CDevices_CGeolocation_CPositionAccuracy;

typedef enum __x_ABI_CWindows_CDevices_CGeolocation_CPositionSource __x_ABI_CWindows_CDevices_CGeolocation_CPositionSource;

typedef enum __x_ABI_CWindows_CDevices_CGeolocation_CPositionStatus __x_ABI_CWindows_CDevices_CGeolocation_CPositionStatus;

typedef enum __x_ABI_CWindows_CDevices_CGeolocation_CVisitMonitoringScope __x_ABI_CWindows_CDevices_CGeolocation_CVisitMonitoringScope;

typedef enum __x_ABI_CWindows_CDevices_CGeolocation_CVisitStateChange __x_ABI_CWindows_CDevices_CGeolocation_CVisitStateChange;

/*
 *
 * Struct Windows.Devices.Geolocation.AltitudeReferenceSystem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CGeolocation_CAltitudeReferenceSystem
{
    AltitudeReferenceSystem_Unspecified = 0,
    AltitudeReferenceSystem_Terrain = 1,
    AltitudeReferenceSystem_Ellipsoid = 2,
    AltitudeReferenceSystem_Geoid = 3,
    AltitudeReferenceSystem_Surface = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Geolocation.GeolocationAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CGeolocation_CGeolocationAccessStatus
{
    GeolocationAccessStatus_Unspecified = 0,
    GeolocationAccessStatus_Allowed = 1,
    GeolocationAccessStatus_Denied = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Geolocation.GeoshapeType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CGeolocation_CGeoshapeType
{
    GeoshapeType_Geopoint = 0,
    GeoshapeType_Geocircle = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    GeoshapeType_Geopath = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    GeoshapeType_GeoboundingBox = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Geolocation.PositionAccuracy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CGeolocation_CPositionAccuracy
{
    PositionAccuracy_Default = 0,
    PositionAccuracy_High = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Geolocation.PositionSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CGeolocation_CPositionSource
{
    PositionSource_Cellular = 0,
    PositionSource_Satellite = 1,
    PositionSource_WiFi = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    PositionSource_IPAddress = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    PositionSource_Unknown = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    PositionSource_Default = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    PositionSource_Obfuscated = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Geolocation.PositionStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CGeolocation_CPositionStatus
{
    PositionStatus_Ready = 0,
    PositionStatus_Initializing = 1,
    PositionStatus_NoData = 2,
    PositionStatus_Disabled = 3,
    PositionStatus_NotInitialized = 4,
    PositionStatus_NotAvailable = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Geolocation.VisitMonitoringScope
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CDevices_CGeolocation_CVisitMonitoringScope
{
    VisitMonitoringScope_Venue = 0,
    VisitMonitoringScope_City = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Devices.Geolocation.VisitStateChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CDevices_CGeolocation_CVisitStateChange
{
    VisitStateChange_TrackingLost = 0,
    VisitStateChange_Arrived = 1,
    VisitStateChange_Departed = 2,
    VisitStateChange_OtherMovement = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Devices.Geolocation.BasicGeoposition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition
{
    DOUBLE Latitude;
    DOUBLE Longitude;
    DOUBLE Altitude;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.ICivicAddress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.CivicAddress
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_ICivicAddress[] = L"Windows.Devices.Geolocation.ICivicAddress";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Country)(__x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_City)(__x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PostalCode)(__x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddressVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_get_Country(This, value) \
    ((This)->lpVtbl->get_Country(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_get_City(This, value) \
    ((This)->lpVtbl->get_City(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_get_PostalCode(This, value) \
    ((This)->lpVtbl->get_PostalCode(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeoboundingBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.GeoboundingBox
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Geolocation.IGeoshape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeoboundingBox[] = L"Windows.Devices.Geolocation.IGeoboundingBox";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NorthwestCorner)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition* value);
    HRESULT (STDMETHODCALLTYPE* get_SoutheastCorner)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition* value);
    HRESULT (STDMETHODCALLTYPE* get_Center)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition* value);
    HRESULT (STDMETHODCALLTYPE* get_MinAltitude)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxAltitude)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_get_NorthwestCorner(This, value) \
    ((This)->lpVtbl->get_NorthwestCorner(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_get_SoutheastCorner(This, value) \
    ((This)->lpVtbl->get_SoutheastCorner(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_get_Center(This, value) \
    ((This)->lpVtbl->get_Center(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_get_MinAltitude(This, value) \
    ((This)->lpVtbl->get_MinAltitude(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_get_MaxAltitude(This, value) \
    ((This)->lpVtbl->get_MaxAltitude(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeoboundingBoxFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.GeoboundingBox
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeoboundingBoxFactory[] = L"Windows.Devices.Geolocation.IGeoboundingBoxFactory";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition northwestCorner,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition southeastCorner,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithAltitudeReference)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition northwestCorner,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition southeastCorner,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CAltitudeReferenceSystem altitudeReferenceSystem,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithAltitudeReferenceAndSpatialReference)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition northwestCorner,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition southeastCorner,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CAltitudeReferenceSystem altitudeReferenceSystem,
        UINT32 spatialReferenceId,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_Create(This, northwestCorner, southeastCorner, value) \
    ((This)->lpVtbl->Create(This, northwestCorner, southeastCorner, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_CreateWithAltitudeReference(This, northwestCorner, southeastCorner, altitudeReferenceSystem, value) \
    ((This)->lpVtbl->CreateWithAltitudeReference(This, northwestCorner, southeastCorner, altitudeReferenceSystem, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_CreateWithAltitudeReferenceAndSpatialReference(This, northwestCorner, southeastCorner, altitudeReferenceSystem, spatialReferenceId, value) \
    ((This)->lpVtbl->CreateWithAltitudeReferenceAndSpatialReference(This, northwestCorner, southeastCorner, altitudeReferenceSystem, spatialReferenceId, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeoboundingBoxStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.GeoboundingBox
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeoboundingBoxStatics[] = L"Windows.Devices.Geolocation.IGeoboundingBoxStatics";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryCompute)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics* This,
        __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* positions,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox** value);
    HRESULT (STDMETHODCALLTYPE* TryComputeWithAltitudeReference)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics* This,
        __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* positions,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CAltitudeReferenceSystem altitudeRefSystem,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox** value);
    HRESULT (STDMETHODCALLTYPE* TryComputeWithAltitudeReferenceAndSpatialReference)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics* This,
        __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* positions,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CAltitudeReferenceSystem altitudeRefSystem,
        UINT32 spatialReferenceId,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_TryCompute(This, positions, value) \
    ((This)->lpVtbl->TryCompute(This, positions, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_TryComputeWithAltitudeReference(This, positions, altitudeRefSystem, value) \
    ((This)->lpVtbl->TryComputeWithAltitudeReference(This, positions, altitudeRefSystem, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_TryComputeWithAltitudeReferenceAndSpatialReference(This, positions, altitudeRefSystem, spatialReferenceId, value) \
    ((This)->lpVtbl->TryComputeWithAltitudeReferenceAndSpatialReference(This, positions, altitudeRefSystem, spatialReferenceId, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBoxStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeocircle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geocircle
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Geolocation.IGeoshape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeocircle[] = L"Windows.Devices.Geolocation.IGeocircle";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Center)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition* value);
    HRESULT (STDMETHODCALLTYPE* get_Radius)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_get_Center(This, value) \
    ((This)->lpVtbl->get_Center(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_get_Radius(This, value) \
    ((This)->lpVtbl->get_Radius(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeocircleFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geocircle
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeocircleFactory[] = L"Windows.Devices.Geolocation.IGeocircleFactory";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition position,
        DOUBLE radius,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithAltitudeReferenceSystem)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition position,
        DOUBLE radius,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CAltitudeReferenceSystem altitudeReferenceSystem,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithAltitudeReferenceSystemAndSpatialReferenceId)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition position,
        DOUBLE radius,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CAltitudeReferenceSystem altitudeReferenceSystem,
        UINT32 spatialReferenceId,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_Create(This, position, radius, value) \
    ((This)->lpVtbl->Create(This, position, radius, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_CreateWithAltitudeReferenceSystem(This, position, radius, altitudeReferenceSystem, value) \
    ((This)->lpVtbl->CreateWithAltitudeReferenceSystem(This, position, radius, altitudeReferenceSystem, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_CreateWithAltitudeReferenceSystemAndSpatialReferenceId(This, position, radius, altitudeReferenceSystem, spatialReferenceId, value) \
    ((This)->lpVtbl->CreateWithAltitudeReferenceSystemAndSpatialReferenceId(This, position, radius, altitudeReferenceSystem, spatialReferenceId, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircleFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeocoordinate
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geocoordinate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeocoordinate[] = L"Windows.Devices.Geolocation.IGeocoordinate";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Latitude may be altered or unavailable after Windows 8.1. Instead, use Point.Position.Latitude")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Latitude)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate* This,
        DOUBLE* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Longitude may be altered or unavailable after Windows 8.1. Instead, use Point.Position.Longitude")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Longitude)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate* This,
        DOUBLE* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Altitude may be altered or unavailable after Windows 8.1. Instead, use Point.Position.Altitude")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Altitude)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate* This,
        __FIReference_1_double** value);
    HRESULT (STDMETHODCALLTYPE* get_Accuracy)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_AltitudeAccuracy)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate* This,
        __FIReference_1_double** value);
    HRESULT (STDMETHODCALLTYPE* get_Heading)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate* This,
        __FIReference_1_double** value);
    HRESULT (STDMETHODCALLTYPE* get_Speed)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate* This,
        __FIReference_1_double** value);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Latitude may be altered or unavailable after Windows 8.1. Instead, use Point.Position.Latitude")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_get_Latitude(This, value) \
    ((This)->lpVtbl->get_Latitude(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Longitude may be altered or unavailable after Windows 8.1. Instead, use Point.Position.Longitude")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_get_Longitude(This, value) \
    ((This)->lpVtbl->get_Longitude(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Altitude may be altered or unavailable after Windows 8.1. Instead, use Point.Position.Altitude")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_get_Altitude(This, value) \
    ((This)->lpVtbl->get_Altitude(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_get_Accuracy(This, value) \
    ((This)->lpVtbl->get_Accuracy(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_get_AltitudeAccuracy(This, value) \
    ((This)->lpVtbl->get_AltitudeAccuracy(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_get_Heading(This, value) \
    ((This)->lpVtbl->get_Heading(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_get_Speed(This, value) \
    ((This)->lpVtbl->get_Speed(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeocoordinateSatelliteData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.GeocoordinateSatelliteData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeocoordinateSatelliteData[] = L"Windows.Devices.Geolocation.IGeocoordinateSatelliteData";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PositionDilutionOfPrecision)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData* This,
        __FIReference_1_double** ppValue);
    HRESULT (STDMETHODCALLTYPE* get_HorizontalDilutionOfPrecision)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData* This,
        __FIReference_1_double** ppValue);
    HRESULT (STDMETHODCALLTYPE* get_VerticalDilutionOfPrecision)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData* This,
        __FIReference_1_double** ppValue);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteDataVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_get_PositionDilutionOfPrecision(This, ppValue) \
    ((This)->lpVtbl->get_PositionDilutionOfPrecision(This, ppValue))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_get_HorizontalDilutionOfPrecision(This, ppValue) \
    ((This)->lpVtbl->get_HorizontalDilutionOfPrecision(This, ppValue))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_get_VerticalDilutionOfPrecision(This, ppValue) \
    ((This)->lpVtbl->get_VerticalDilutionOfPrecision(This, ppValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeocoordinateSatelliteData2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.GeocoordinateSatelliteData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeocoordinateSatelliteData2[] = L"Windows.Devices.Geolocation.IGeocoordinateSatelliteData2";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_GeometricDilutionOfPrecision)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2* This,
        __FIReference_1_double** value);
    HRESULT (STDMETHODCALLTYPE* get_TimeDilutionOfPrecision)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2* This,
        __FIReference_1_double** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2Vtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_get_GeometricDilutionOfPrecision(This, value) \
    ((This)->lpVtbl->get_GeometricDilutionOfPrecision(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_get_TimeDilutionOfPrecision(This, value) \
    ((This)->lpVtbl->get_TimeDilutionOfPrecision(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeocoordinateWithPoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geocoordinate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeocoordinateWithPoint[] = L"Windows.Devices.Geolocation.IGeocoordinateWithPoint";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Point)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPointVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_get_Point(This, value) \
    ((This)->lpVtbl->get_Point(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPoint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeocoordinateWithPositionData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geocoordinate
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Geolocation.IGeocoordinate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeocoordinateWithPositionData[] = L"Windows.Devices.Geolocation.IGeocoordinateWithPositionData";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PositionSource)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData* This,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CPositionSource* pValue);
    HRESULT (STDMETHODCALLTYPE* get_SatelliteData)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateSatelliteData** ppValue);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionDataVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_get_PositionSource(This, pValue) \
    ((This)->lpVtbl->get_PositionSource(This, pValue))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_get_SatelliteData(This, ppValue) \
    ((This)->lpVtbl->get_SatelliteData(This, ppValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeocoordinateWithPositionSourceTimestamp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geocoordinate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeocoordinateWithPositionSourceTimestamp[] = L"Windows.Devices.Geolocation.IGeocoordinateWithPositionSourceTimestamp";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestampVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PositionSourceTimestamp)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestampVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestampVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_get_PositionSourceTimestamp(This, value) \
    ((This)->lpVtbl->get_PositionSourceTimestamp(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithPositionSourceTimestamp_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeocoordinateWithRemoteSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 14.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geocoordinate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeocoordinateWithRemoteSource[] = L"Windows.Devices.Geolocation.IGeocoordinateWithRemoteSource";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsRemoteSource)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSourceVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_get_IsRemoteSource(This, value) \
    ((This)->lpVtbl->get_IsRemoteSource(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinateWithRemoteSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeolocator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geolocator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeolocator[] = L"Windows.Devices.Geolocation.IGeolocator";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DesiredAccuracy)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CPositionAccuracy* value);
    HRESULT (STDMETHODCALLTYPE* put_DesiredAccuracy)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CPositionAccuracy value);
    HRESULT (STDMETHODCALLTYPE* get_MovementThreshold)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_MovementThreshold)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_ReportInterval)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ReportInterval)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_LocationStatus)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CPositionStatus* value);
    HRESULT (STDMETHODCALLTYPE* GetGeopositionAsync)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This,
        __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition** value);
    HRESULT (STDMETHODCALLTYPE* GetGeopositionAsyncWithAgeAndTimeout)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan maximumAge,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan timeout,
        __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeoposition** value);
    HRESULT (STDMETHODCALLTYPE* add_PositionChanged)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This,
        __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CPositionChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PositionChanged)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_StatusChanged)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This,
        __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeolocator_Windows__CDevices__CGeolocation__CStatusChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StatusChanged)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_get_DesiredAccuracy(This, value) \
    ((This)->lpVtbl->get_DesiredAccuracy(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_put_DesiredAccuracy(This, value) \
    ((This)->lpVtbl->put_DesiredAccuracy(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_get_MovementThreshold(This, value) \
    ((This)->lpVtbl->get_MovementThreshold(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_put_MovementThreshold(This, value) \
    ((This)->lpVtbl->put_MovementThreshold(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_get_ReportInterval(This, value) \
    ((This)->lpVtbl->get_ReportInterval(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_put_ReportInterval(This, value) \
    ((This)->lpVtbl->put_ReportInterval(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_get_LocationStatus(This, value) \
    ((This)->lpVtbl->get_LocationStatus(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_GetGeopositionAsync(This, value) \
    ((This)->lpVtbl->GetGeopositionAsync(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_GetGeopositionAsyncWithAgeAndTimeout(This, maximumAge, timeout, value) \
    ((This)->lpVtbl->GetGeopositionAsyncWithAgeAndTimeout(This, maximumAge, timeout, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_add_PositionChanged(This, handler, token) \
    ((This)->lpVtbl->add_PositionChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_remove_PositionChanged(This, token) \
    ((This)->lpVtbl->remove_PositionChanged(This, token))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_add_StatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_StatusChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_remove_StatusChanged(This, token) \
    ((This)->lpVtbl->remove_StatusChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeolocator2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geolocator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeolocator2[] = L"Windows.Devices.Geolocation.IGeolocator2";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AllowFallbackToConsentlessPositions)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2* This);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2Vtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_AllowFallbackToConsentlessPositions(This) \
    ((This)->lpVtbl->AllowFallbackToConsentlessPositions(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeolocatorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geolocator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeolocatorStatics[] = L"Windows.Devices.Geolocation.IGeolocatorStatics";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestAccessAsync)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics* This,
        __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeolocationAccessStatus** result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
    DEPRECATED("GetGeopositionHistoryAsync is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* GetGeopositionHistoryAsync)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime startTime,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition** result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
    DEPRECATED("GetGeopositionHistoryWithDurationAsync is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* GetGeopositionHistoryWithDurationAsync)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime startTime,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan duration,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGeolocation__CGeoposition** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_RequestAccessAsync(This, result) \
    ((This)->lpVtbl->RequestAccessAsync(This, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
    DEPRECATED("GetGeopositionHistoryAsync is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_GetGeopositionHistoryAsync(This, startTime, result) \
    ((This)->lpVtbl->GetGeopositionHistoryAsync(This, startTime, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
    DEPRECATED("GetGeopositionHistoryWithDurationAsync is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_GetGeopositionHistoryWithDurationAsync(This, startTime, duration, result) \
    ((This)->lpVtbl->GetGeopositionHistoryWithDurationAsync(This, startTime, duration, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeolocatorStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geolocator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeolocatorStatics2[] = L"Windows.Devices.Geolocation.IGeolocatorStatics2";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsDefaultGeopositionRecommended)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_DefaultGeoposition)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2* This,
        __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition* value);
    HRESULT (STDMETHODCALLTYPE* get_DefaultGeoposition)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2* This,
        __FIReference_1_Windows__CDevices__CGeolocation__CBasicGeoposition** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2Vtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_get_IsDefaultGeopositionRecommended(This, value) \
    ((This)->lpVtbl->get_IsDefaultGeopositionRecommended(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_put_DefaultGeoposition(This, value) \
    ((This)->lpVtbl->put_DefaultGeoposition(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_get_DefaultGeoposition(This, value) \
    ((This)->lpVtbl->get_DefaultGeoposition(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeolocatorWithScalarAccuracy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geolocator
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Geolocation.IGeolocator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeolocatorWithScalarAccuracy[] = L"Windows.Devices.Geolocation.IGeolocatorWithScalarAccuracy";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DesiredAccuracyInMeters)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy* This,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* put_DesiredAccuracyInMeters)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy* This,
        __FIReference_1_UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracyVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_get_DesiredAccuracyInMeters(This, value) \
    ((This)->lpVtbl->get_DesiredAccuracyInMeters(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_put_DesiredAccuracyInMeters(This, value) \
    ((This)->lpVtbl->put_DesiredAccuracyInMeters(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocatorWithScalarAccuracy_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeopath
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geopath
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Geolocation.IGeoshape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeopath[] = L"Windows.Devices.Geolocation.IGeopath";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopath* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopath* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopath* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopath* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopath* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopath* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Positions)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopath* This,
        __FIVectorView_1_Windows__CDevices__CGeolocation__CBasicGeoposition** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_get_Positions(This, value) \
    ((This)->lpVtbl->get_Positions(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeopath;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeopathFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geopath
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeopathFactory[] = L"Windows.Devices.Geolocation.IGeopathFactory";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory* This,
        __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* positions,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithAltitudeReference)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory* This,
        __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* positions,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CAltitudeReferenceSystem altitudeReferenceSystem,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithAltitudeReferenceAndSpatialReference)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory* This,
        __FIIterable_1_Windows__CDevices__CGeolocation__CBasicGeoposition* positions,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CAltitudeReferenceSystem altitudeReferenceSystem,
        UINT32 spatialReferenceId,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_Create(This, positions, value) \
    ((This)->lpVtbl->Create(This, positions, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_CreateWithAltitudeReference(This, positions, altitudeReferenceSystem, value) \
    ((This)->lpVtbl->CreateWithAltitudeReference(This, positions, altitudeReferenceSystem, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_CreateWithAltitudeReferenceAndSpatialReference(This, positions, altitudeReferenceSystem, spatialReferenceId, value) \
    ((This)->lpVtbl->CreateWithAltitudeReferenceAndSpatialReference(This, positions, altitudeReferenceSystem, spatialReferenceId, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeopathFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeopoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geopoint
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Geolocation.IGeoshape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeopoint[] = L"Windows.Devices.Geolocation.IGeopoint";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeopointFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geopoint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeopointFactory[] = L"Windows.Devices.Geolocation.IGeopointFactory";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition position,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithAltitudeReferenceSystem)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition position,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CAltitudeReferenceSystem altitudeReferenceSystem,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithAltitudeReferenceSystemAndSpatialReferenceId)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition position,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CAltitudeReferenceSystem altitudeReferenceSystem,
        UINT32 spatialReferenceId,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_Create(This, position, value) \
    ((This)->lpVtbl->Create(This, position, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_CreateWithAltitudeReferenceSystem(This, position, altitudeReferenceSystem, value) \
    ((This)->lpVtbl->CreateWithAltitudeReferenceSystem(This, position, altitudeReferenceSystem, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_CreateWithAltitudeReferenceSystemAndSpatialReferenceId(This, position, altitudeReferenceSystem, spatialReferenceId, value) \
    ((This)->lpVtbl->CreateWithAltitudeReferenceSystemAndSpatialReferenceId(This, position, altitudeReferenceSystem, spatialReferenceId, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeopointFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeoposition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geoposition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeoposition[] = L"Windows.Devices.Geolocation.IGeoposition";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeopositionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Coordinate)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate** value);
    HRESULT (STDMETHODCALLTYPE* get_CivicAddress)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CICivicAddress** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeopositionVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeopositionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_get_Coordinate(This, value) \
    ((This)->lpVtbl->get_Coordinate(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_get_CivicAddress(This, value) \
    ((This)->lpVtbl->get_CivicAddress(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeoposition2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geoposition
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Geolocation.IGeoposition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeoposition2[] = L"Windows.Devices.Geolocation.IGeoposition2";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_VenueData)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIVenueData** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2Vtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_get_VenueData(This, value) \
    ((This)->lpVtbl->get_VenueData(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeoshape
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeoshape[] = L"Windows.Devices.Geolocation.IGeoshape";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshapeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_GeoshapeType)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape* This,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CGeoshapeType* value);
    HRESULT (STDMETHODCALLTYPE* get_SpatialReferenceId)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_AltitudeReferenceSystem)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape* This,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CAltitudeReferenceSystem* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshapeVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshapeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_get_GeoshapeType(This, value) \
    ((This)->lpVtbl->get_GeoshapeType(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_get_SpatialReferenceId(This, value) \
    ((This)->lpVtbl->get_SpatialReferenceId(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_get_AltitudeReferenceSystem(This, value) \
    ((This)->lpVtbl->get_AltitudeReferenceSystem(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeovisit
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geovisit
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeovisit[] = L"Windows.Devices.Geolocation.IGeovisit";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition** value);
    HRESULT (STDMETHODCALLTYPE* get_StateChange)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit* This,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CVisitStateChange* value);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_get_StateChange(This, value) \
    ((This)->lpVtbl->get_StateChange(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeovisitMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.GeovisitMonitor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeovisitMonitor[] = L"Windows.Devices.Geolocation.IGeovisitMonitor";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MonitoringScope)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor* This,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CVisitMonitoringScope* value);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor* This,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CVisitMonitoringScope value);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor* This);
    HRESULT (STDMETHODCALLTYPE* add_VisitStateChanged)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor* This,
        __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeovisitMonitor_Windows__CDevices__CGeolocation__CGeovisitStateChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_VisitStateChanged)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_get_MonitoringScope(This, value) \
    ((This)->lpVtbl->get_MonitoringScope(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_Start(This, value) \
    ((This)->lpVtbl->Start(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_add_VisitStateChanged(This, handler, token) \
    ((This)->lpVtbl->add_VisitStateChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_remove_VisitStateChanged(This, token) \
    ((This)->lpVtbl->remove_VisitStateChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeovisitMonitorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.GeovisitMonitor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeovisitMonitorStatics[] = L"Windows.Devices.Geolocation.IGeovisitMonitorStatics";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetLastReportAsync)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics* This,
        __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeovisit** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_GetLastReportAsync(This, value) \
    ((This)->lpVtbl->GetLastReportAsync(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitMonitorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeovisitStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.GeovisitStateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeovisitStateChangedEventArgs[] = L"Windows.Devices.Geolocation.IGeovisitStateChangedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Visit)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisit** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_get_Visit(This, value) \
    ((This)->lpVtbl->get_Visit(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitStateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Geolocation.IGeovisitTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.GeovisitTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IGeovisitTriggerDetails[] = L"Windows.Devices.Geolocation.IGeovisitTriggerDetails";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReadReports)(__x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails* This,
        __FIVectorView_1_Windows__CDevices__CGeolocation__CGeovisit** values);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetailsVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_ReadReports(This, values) \
    ((This)->lpVtbl->ReadReports(This, values))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIGeovisitTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Geolocation.IPositionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.PositionChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IPositionChangedEventArgs[] = L"Windows.Devices.Geolocation.IPositionChangedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIPositionChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.StatusChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IStatusChangedEventArgs[] = L"Windows.Devices.Geolocation.IStatusChangedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CPositionStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIStatusChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.IVenueData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.VenueData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_IVenueData[] = L"Windows.Devices.Geolocation.IVenueData";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CIVenueDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CIVenueData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CIVenueData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CIVenueData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CIVenueData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CIVenueData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CIVenueData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CGeolocation_CIVenueData* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Level)(__x_ABI_CWindows_CDevices_CGeolocation_CIVenueData* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CIVenueDataVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CIVenueData
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CIVenueDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_get_Level(This, value) \
    ((This)->lpVtbl->get_Level(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CIVenueData;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CIVenueData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.CivicAddress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.ICivicAddress ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_CivicAddress_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_CivicAddress_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_CivicAddress[] = L"Windows.Devices.Geolocation.CivicAddress";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.GeoboundingBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Geolocation.IGeoboundingBoxFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Geolocation.IGeoboundingBoxStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeoboundingBox ** Default Interface **
 *    Windows.Devices.Geolocation.IGeoshape
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_GeoboundingBox_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_GeoboundingBox_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_GeoboundingBox[] = L"Windows.Devices.Geolocation.GeoboundingBox";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geocircle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Geolocation.IGeocircleFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeocircle ** Default Interface **
 *    Windows.Devices.Geolocation.IGeoshape
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geocircle_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geocircle_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geocircle[] = L"Windows.Devices.Geolocation.Geocircle";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geocoordinate
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeocoordinate ** Default Interface **
 *    Windows.Devices.Geolocation.IGeocoordinateWithPositionData
 *    Windows.Devices.Geolocation.IGeocoordinateWithPoint
 *    Windows.Devices.Geolocation.IGeocoordinateWithPositionSourceTimestamp
 *    Windows.Devices.Geolocation.IGeocoordinateWithRemoteSource
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geocoordinate_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geocoordinate_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geocoordinate[] = L"Windows.Devices.Geolocation.Geocoordinate";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.GeocoordinateSatelliteData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeocoordinateSatelliteData ** Default Interface **
 *    Windows.Devices.Geolocation.IGeocoordinateSatelliteData2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_GeocoordinateSatelliteData_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_GeocoordinateSatelliteData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_GeocoordinateSatelliteData[] = L"Windows.Devices.Geolocation.GeocoordinateSatelliteData";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geolocator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Geolocation.IGeolocatorStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Devices.Geolocation.IGeolocatorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeolocator ** Default Interface **
 *    Windows.Devices.Geolocation.IGeolocatorWithScalarAccuracy
 *    Windows.Devices.Geolocation.IGeolocator2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geolocator_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geolocator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geolocator[] = L"Windows.Devices.Geolocation.Geolocator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geopath
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Geolocation.IGeopathFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeopath ** Default Interface **
 *    Windows.Devices.Geolocation.IGeoshape
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geopath_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geopath_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geopath[] = L"Windows.Devices.Geolocation.Geopath";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geopoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Geolocation.IGeopointFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeopoint ** Default Interface **
 *    Windows.Devices.Geolocation.IGeoshape
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geopoint_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geopoint_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geopoint[] = L"Windows.Devices.Geolocation.Geopoint";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geoposition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeoposition ** Default Interface **
 *    Windows.Devices.Geolocation.IGeoposition2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geoposition_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geoposition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geoposition[] = L"Windows.Devices.Geolocation.Geoposition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geovisit
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeovisit ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geovisit_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geovisit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geovisit[] = L"Windows.Devices.Geolocation.Geovisit";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Devices.Geolocation.GeovisitMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Geolocation.IGeovisitMonitorStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeovisitMonitor ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_GeovisitMonitor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_GeovisitMonitor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_GeovisitMonitor[] = L"Windows.Devices.Geolocation.GeovisitMonitor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Devices.Geolocation.GeovisitStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeovisitStateChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_GeovisitStateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_GeovisitStateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_GeovisitStateChangedEventArgs[] = L"Windows.Devices.Geolocation.GeovisitStateChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Devices.Geolocation.GeovisitTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IGeovisitTriggerDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_GeovisitTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_GeovisitTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_GeovisitTriggerDetails[] = L"Windows.Devices.Geolocation.GeovisitTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Devices.Geolocation.PositionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IPositionChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_PositionChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_PositionChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_PositionChangedEventArgs[] = L"Windows.Devices.Geolocation.PositionChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.StatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IStatusChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_StatusChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_StatusChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_StatusChangedEventArgs[] = L"Windows.Devices.Geolocation.StatusChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.VenueData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.IVenueData ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_VenueData_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_VenueData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_VenueData[] = L"Windows.Devices.Geolocation.VenueData";
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
#endif // __windows2Edevices2Egeolocation_p_h__

#endif // __windows2Edevices2Egeolocation_h__
