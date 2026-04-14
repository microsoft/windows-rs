
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
#ifndef __windows2Eperception2Espatial_h__
#define __windows2Eperception2Espatial_h__
#ifndef __windows2Eperception2Espatial_p_h__
#define __windows2Eperception2Espatial_p_h__


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
#include "Windows.Foundation.Numerics.h"
#include "Windows.Perception.h"
#include "Windows.Storage.Streams.h"
#include "Windows.System.RemoteSystems.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialAnchor;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor ABI::Windows::Perception::Spatial::ISpatialAnchor

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialAnchor2;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2 ABI::Windows::Perception::Spatial::ISpatialAnchor2

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialAnchorExportSufficiency;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency ABI::Windows::Perception::Spatial::ISpatialAnchorExportSufficiency

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialAnchorExporter;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter ABI::Windows::Perception::Spatial::ISpatialAnchorExporter

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialAnchorExporterStatics;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics ABI::Windows::Perception::Spatial::ISpatialAnchorExporterStatics

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialAnchorManagerStatics;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics ABI::Windows::Perception::Spatial::ISpatialAnchorManagerStatics

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialAnchorRawCoordinateSystemAdjustedEventArgs;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs ABI::Windows::Perception::Spatial::ISpatialAnchorRawCoordinateSystemAdjustedEventArgs

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialAnchorStatics;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics ABI::Windows::Perception::Spatial::ISpatialAnchorStatics

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialAnchorStore;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore ABI::Windows::Perception::Spatial::ISpatialAnchorStore

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialAnchorTransferManagerStatics;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics ABI::Windows::Perception::Spatial::ISpatialAnchorTransferManagerStatics

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialBoundingVolume;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume ABI::Windows::Perception::Spatial::ISpatialBoundingVolume

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialBoundingVolumeStatics;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics ABI::Windows::Perception::Spatial::ISpatialBoundingVolumeStatics

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialCoordinateSystem;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialEntity;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity ABI::Windows::Perception::Spatial::ISpatialEntity

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialEntityAddedEventArgs;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs ABI::Windows::Perception::Spatial::ISpatialEntityAddedEventArgs

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialEntityFactory;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory ABI::Windows::Perception::Spatial::ISpatialEntityFactory

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialEntityRemovedEventArgs;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs ABI::Windows::Perception::Spatial::ISpatialEntityRemovedEventArgs

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialEntityStore;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore ABI::Windows::Perception::Spatial::ISpatialEntityStore

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialEntityStoreStatics;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics ABI::Windows::Perception::Spatial::ISpatialEntityStoreStatics

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialEntityUpdatedEventArgs;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs ABI::Windows::Perception::Spatial::ISpatialEntityUpdatedEventArgs

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialEntityWatcher;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher ABI::Windows::Perception::Spatial::ISpatialEntityWatcher

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialLocation;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation ABI::Windows::Perception::Spatial::ISpatialLocation

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialLocation2;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2 ABI::Windows::Perception::Spatial::ISpatialLocation2

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialLocator;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator ABI::Windows::Perception::Spatial::ISpatialLocator

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialLocatorAttachedFrameOfReference;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference ABI::Windows::Perception::Spatial::ISpatialLocatorAttachedFrameOfReference

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialLocatorPositionalTrackingDeactivatingEventArgs;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs ABI::Windows::Perception::Spatial::ISpatialLocatorPositionalTrackingDeactivatingEventArgs

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialLocatorStatics;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics ABI::Windows::Perception::Spatial::ISpatialLocatorStatics

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialStageFrameOfReference;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference ABI::Windows::Perception::Spatial::ISpatialStageFrameOfReference

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialStageFrameOfReferenceStatics;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics ABI::Windows::Perception::Spatial::ISpatialStageFrameOfReferenceStatics

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialStationaryFrameOfReference;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference ABI::Windows::Perception::Spatial::ISpatialStationaryFrameOfReference

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_FWD_DEFINED__

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
        namespace Perception {
            namespace Spatial {
                class SpatialAnchor;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("627298e7-068d-53f6-9154-d7d8d8091463"))
IKeyValuePair<HSTRING, ABI::Windows::Perception::Spatial::SpatialAnchor*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialAnchor*, ABI::Windows::Perception::Spatial::ISpatialAnchor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Perception.Spatial.SpatialAnchor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::Perception::Spatial::SpatialAnchor*> __FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("67a5f318-0232-5900-ac7e-5c647d731cbc"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Perception.Spatial.SpatialAnchor>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("55f0fa8a-afd4-5541-a1c3-36f12147d606"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Perception.Spatial.SpatialAnchor>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_USE
#define DEF___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2d344564-21b1-5470-b013-488cdde45c48"))
IMapView<HSTRING, ABI::Windows::Perception::Spatial::SpatialAnchor*> : IMapView_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialAnchor*, ABI::Windows::Perception::Spatial::ISpatialAnchor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.Perception.Spatial.SpatialAnchor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::Perception::Spatial::SpatialAnchor*> __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_t;
#define __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_USE
#define DEF___FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bbe07728-da33-52c5-aae0-a5e74cdf0471"))
IAsyncOperation<__FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor*> : IAsyncOperation_impl<__FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IMapView`2<String, Windows.Perception.Spatial.SpatialAnchor>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor*> __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_t;
#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor ABI::Windows::Foundation::__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3a950aa3-9c65-586e-af75-1acf07190e90"))
IAsyncOperationCompletedHandler<__FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor*> : IAsyncOperationCompletedHandler_impl<__FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IMapView`2<String, Windows.Perception.Spatial.SpatialAnchor>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor*> __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_t;
#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialAnchorExportSufficiency;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_USE
#define DEF___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("260957b8-5b76-5159-8dc5-e03d74aa5f3d"))
IAsyncOperation<ABI::Windows::Perception::Spatial::SpatialAnchorExportSufficiency*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialAnchorExportSufficiency*, ABI::Windows::Perception::Spatial::ISpatialAnchorExportSufficiency*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Perception.Spatial.SpatialAnchorExportSufficiency>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Perception::Spatial::SpatialAnchorExportSufficiency*> __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_t;
#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4b6593d2-11a8-513e-838d-4226fb1e3c1f"))
IAsyncOperationCompletedHandler<ABI::Windows::Perception::Spatial::SpatialAnchorExportSufficiency*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialAnchorExportSufficiency*, ABI::Windows::Perception::Spatial::ISpatialAnchorExportSufficiency*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Perception.Spatial.SpatialAnchorExportSufficiency>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Perception::Spatial::SpatialAnchorExportSufficiency*> __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialAnchorStore;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_USE
#define DEF___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1cd05e51-1457-5023-8f5d-fe5e5a953423"))
IAsyncOperation<ABI::Windows::Perception::Spatial::SpatialAnchorStore*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialAnchorStore*, ABI::Windows::Perception::Spatial::ISpatialAnchorStore*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Perception.Spatial.SpatialAnchorStore>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Perception::Spatial::SpatialAnchorStore*> __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_t;
#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("84c21a3a-037a-503f-8006-ab577b7f6f66"))
IAsyncOperationCompletedHandler<ABI::Windows::Perception::Spatial::SpatialAnchorStore*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialAnchorStore*, ABI::Windows::Perception::Spatial::ISpatialAnchorStore*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Perception.Spatial.SpatialAnchorStore>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Perception::Spatial::SpatialAnchorStore*> __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                typedef enum SpatialPerceptionAccessStatus : int SpatialPerceptionAccessStatus;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b425d126-1069-563f-a863-44a30a8f071d"))
IAsyncOperation<enum ABI::Windows::Perception::Spatial::SpatialPerceptionAccessStatus> : IAsyncOperation_impl<enum ABI::Windows::Perception::Spatial::SpatialPerceptionAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Perception.Spatial.SpatialPerceptionAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Perception::Spatial::SpatialPerceptionAccessStatus> __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_t;
#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6ced54c8-7689-525a-80e1-956a9d85cd83"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Perception::Spatial::SpatialPerceptionAccessStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Perception::Spatial::SpatialPerceptionAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Perception.Spatial.SpatialPerceptionAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Perception::Spatial::SpatialPerceptionAccessStatus> __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialStageFrameOfReference;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_USE
#define DEF___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b4d8b1bf-1d66-5458-a5df-3f4f6c366c58"))
IAsyncOperation<ABI::Windows::Perception::Spatial::SpatialStageFrameOfReference*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialStageFrameOfReference*, ABI::Windows::Perception::Spatial::ISpatialStageFrameOfReference*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Perception.Spatial.SpatialStageFrameOfReference>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Perception::Spatial::SpatialStageFrameOfReference*> __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_t;
#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fbb7e9fb-e49a-54e1-8c83-d1a87e4d2304"))
IAsyncOperationCompletedHandler<ABI::Windows::Perception::Spatial::SpatialStageFrameOfReference*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialStageFrameOfReference*, ABI::Windows::Perception::Spatial::ISpatialStageFrameOfReference*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Perception.Spatial.SpatialStageFrameOfReference>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Perception::Spatial::SpatialStageFrameOfReference*> __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000


#ifndef DEF___FIEventHandler_1_IInspectable_USE
#define DEF___FIEventHandler_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c50898f6-c536-5f47-8583-8b2c2438a13b"))
IEventHandler<IInspectable*> : IEventHandler_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<IInspectable*> __FIEventHandler_1_IInspectable_t;
#define __FIEventHandler_1_IInspectable ABI::Windows::Foundation::__FIEventHandler_1_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_IInspectable_USE */



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
            namespace Numerics {
                typedef struct Matrix4x4 Matrix4x4;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4_USE
#define DEF___FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("dacbffdc-68ef-5fd0-b657-782d0ac9807e"))
IReference<struct ABI::Windows::Foundation::Numerics::Matrix4x4> : IReference_impl<struct ABI::Windows::Foundation::Numerics::Matrix4x4>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Foundation.Numerics.Matrix4x4>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Foundation::Numerics::Matrix4x4> __FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4_t;
#define __FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4 ABI::Windows::Foundation::__FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialAnchorRawCoordinateSystemAdjustedEventArgs;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fa43f9e4-3558-59c8-9a77-6e8b765adcc8"))
ITypedEventHandler<ABI::Windows::Perception::Spatial::SpatialAnchor*, ABI::Windows::Perception::Spatial::SpatialAnchorRawCoordinateSystemAdjustedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialAnchor*, ABI::Windows::Perception::Spatial::ISpatialAnchor*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialAnchorRawCoordinateSystemAdjustedEventArgs*, ABI::Windows::Perception::Spatial::ISpatialAnchorRawCoordinateSystemAdjustedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Perception.Spatial.SpatialAnchor, Windows.Perception.Spatial.SpatialAnchorRawCoordinateSystemAdjustedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Perception::Spatial::SpatialAnchor*, ABI::Windows::Perception::Spatial::SpatialAnchorRawCoordinateSystemAdjustedEventArgs*> __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialEntityWatcher;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("50171823-30a9-5938-9f3b-358d86169f2e"))
ITypedEventHandler<ABI::Windows::Perception::Spatial::SpatialEntityWatcher*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialEntityWatcher*, ABI::Windows::Perception::Spatial::ISpatialEntityWatcher*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Perception.Spatial.SpatialEntityWatcher, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Perception::Spatial::SpatialEntityWatcher*, IInspectable*> __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialEntityAddedEventArgs;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f8edae01-6a30-52cc-b543-8abdb26529b4"))
ITypedEventHandler<ABI::Windows::Perception::Spatial::SpatialEntityWatcher*, ABI::Windows::Perception::Spatial::SpatialEntityAddedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialEntityWatcher*, ABI::Windows::Perception::Spatial::ISpatialEntityWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialEntityAddedEventArgs*, ABI::Windows::Perception::Spatial::ISpatialEntityAddedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Perception.Spatial.SpatialEntityWatcher, Windows.Perception.Spatial.SpatialEntityAddedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Perception::Spatial::SpatialEntityWatcher*, ABI::Windows::Perception::Spatial::SpatialEntityAddedEventArgs*> __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialEntityRemovedEventArgs;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("36f982ad-eaa2-5263-861e-2acf030c9e17"))
ITypedEventHandler<ABI::Windows::Perception::Spatial::SpatialEntityWatcher*, ABI::Windows::Perception::Spatial::SpatialEntityRemovedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialEntityWatcher*, ABI::Windows::Perception::Spatial::ISpatialEntityWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialEntityRemovedEventArgs*, ABI::Windows::Perception::Spatial::ISpatialEntityRemovedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Perception.Spatial.SpatialEntityWatcher, Windows.Perception.Spatial.SpatialEntityRemovedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Perception::Spatial::SpatialEntityWatcher*, ABI::Windows::Perception::Spatial::SpatialEntityRemovedEventArgs*> __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialEntityUpdatedEventArgs;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a15fd0c0-8a0a-5a7d-897a-f206cc509190"))
ITypedEventHandler<ABI::Windows::Perception::Spatial::SpatialEntityWatcher*, ABI::Windows::Perception::Spatial::SpatialEntityUpdatedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialEntityWatcher*, ABI::Windows::Perception::Spatial::ISpatialEntityWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialEntityUpdatedEventArgs*, ABI::Windows::Perception::Spatial::ISpatialEntityUpdatedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Perception.Spatial.SpatialEntityWatcher, Windows.Perception.Spatial.SpatialEntityUpdatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Perception::Spatial::SpatialEntityWatcher*, ABI::Windows::Perception::Spatial::SpatialEntityUpdatedEventArgs*> __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialLocator;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("dbb08ab5-6b40-55fb-83d3-50d5373a3b20"))
ITypedEventHandler<ABI::Windows::Perception::Spatial::SpatialLocator*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialLocator*, ABI::Windows::Perception::Spatial::ISpatialLocator*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Perception.Spatial.SpatialLocator, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Perception::Spatial::SpatialLocator*, IInspectable*> __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialLocatorPositionalTrackingDeactivatingEventArgs;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("34bf236c-e5d6-501f-8693-bc1d8d431d7e"))
ITypedEventHandler<ABI::Windows::Perception::Spatial::SpatialLocator*, ABI::Windows::Perception::Spatial::SpatialLocatorPositionalTrackingDeactivatingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialLocator*, ABI::Windows::Perception::Spatial::ISpatialLocator*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialLocatorPositionalTrackingDeactivatingEventArgs*, ABI::Windows::Perception::Spatial::ISpatialLocatorPositionalTrackingDeactivatingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Perception.Spatial.SpatialLocator, Windows.Perception.Spatial.SpatialLocatorPositionalTrackingDeactivatingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Perception::Spatial::SpatialLocator*, ABI::Windows::Perception::Spatial::SpatialLocatorPositionalTrackingDeactivatingEventArgs*> __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

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
            namespace Numerics {
                typedef struct Plane Plane;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Numerics {
                typedef struct Quaternion Quaternion;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Numerics {
                typedef struct Vector3 Vector3;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            class PerceptionTimestamp;
        } /* Perception */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            interface IPerceptionTimestamp;
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CIPerceptionTimestamp ABI::Windows::Perception::IPerceptionTimestamp

#endif // ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__

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

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemSession;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                typedef enum SpatialAnchorExportPurpose : int SpatialAnchorExportPurpose;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                typedef enum SpatialEntityWatcherStatus : int SpatialEntityWatcherStatus;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                typedef enum SpatialLocatability : int SpatialLocatability;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                typedef enum SpatialLookDirectionRange : int SpatialLookDirectionRange;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                typedef enum SpatialMovementRange : int SpatialMovementRange;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                typedef struct SpatialBoundingBox SpatialBoundingBox;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                typedef struct SpatialBoundingFrustum SpatialBoundingFrustum;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                typedef struct SpatialBoundingOrientedBox SpatialBoundingOrientedBox;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                typedef struct SpatialBoundingSphere SpatialBoundingSphere;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialAnchorExporter;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialBoundingVolume;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialCoordinateSystem;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialEntity;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialEntityStore;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialLocation;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialLocatorAttachedFrameOfReference;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialStationaryFrameOfReference;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Perception.Spatial.SpatialAnchorExportPurpose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                enum SpatialAnchorExportPurpose : int
                {
                    SpatialAnchorExportPurpose_Relocalization = 0,
                    SpatialAnchorExportPurpose_Sharing = 1,
                };
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialEntityWatcherStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                enum SpatialEntityWatcherStatus : int
                {
                    SpatialEntityWatcherStatus_Created = 0,
                    SpatialEntityWatcherStatus_Started = 1,
                    SpatialEntityWatcherStatus_EnumerationCompleted = 2,
                    SpatialEntityWatcherStatus_Stopping = 3,
                    SpatialEntityWatcherStatus_Stopped = 4,
                    SpatialEntityWatcherStatus_Aborted = 5,
                };
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialLocatability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                enum SpatialLocatability : int
                {
                    SpatialLocatability_Unavailable = 0,
                    SpatialLocatability_OrientationOnly = 1,
                    SpatialLocatability_PositionalTrackingActivating = 2,
                    SpatialLocatability_PositionalTrackingActive = 3,
                    SpatialLocatability_PositionalTrackingInhibited = 4,
                };
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialLookDirectionRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                enum SpatialLookDirectionRange : int
                {
                    SpatialLookDirectionRange_ForwardOnly = 0,
                    SpatialLookDirectionRange_Omnidirectional = 1,
                };
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialMovementRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                enum SpatialMovementRange : int
                {
                    SpatialMovementRange_NoMovement = 0,
                    SpatialMovementRange_Bounded = 1,
                };
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialPerceptionAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                enum SpatialPerceptionAccessStatus : int
                {
                    SpatialPerceptionAccessStatus_Unspecified = 0,
                    SpatialPerceptionAccessStatus_Allowed = 1,
                    SpatialPerceptionAccessStatus_DeniedByUser = 2,
                    SpatialPerceptionAccessStatus_DeniedBySystem = 3,
                };
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialBoundingBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                struct SpatialBoundingBox
                {
                    ABI::Windows::Foundation::Numerics::Vector3 Center;
                    ABI::Windows::Foundation::Numerics::Vector3 Extents;
                };
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialBoundingFrustum
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                struct SpatialBoundingFrustum
                {
                    ABI::Windows::Foundation::Numerics::Plane Near;
                    ABI::Windows::Foundation::Numerics::Plane Far;
                    ABI::Windows::Foundation::Numerics::Plane Right;
                    ABI::Windows::Foundation::Numerics::Plane Left;
                    ABI::Windows::Foundation::Numerics::Plane Top;
                    ABI::Windows::Foundation::Numerics::Plane Bottom;
                };
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialBoundingOrientedBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                struct SpatialBoundingOrientedBox
                {
                    ABI::Windows::Foundation::Numerics::Vector3 Center;
                    ABI::Windows::Foundation::Numerics::Vector3 Extents;
                    ABI::Windows::Foundation::Numerics::Quaternion Orientation;
                };
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialBoundingSphere
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                struct SpatialBoundingSphere
                {
                    ABI::Windows::Foundation::Numerics::Vector3 Center;
                    FLOAT Radius;
                };
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialRay
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                struct SpatialRay
                {
                    ABI::Windows::Foundation::Numerics::Vector3 Origin;
                    ABI::Windows::Foundation::Numerics::Vector3 Direction;
                };
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchor[] = L"Windows.Perception.Spatial.ISpatialAnchor";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("0529e5ce-1d34-3702-bcec-eabff578a869")
                ISpatialAnchor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CoordinateSystem(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RawCoordinateSystem(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_RawCoordinateSystemAdjusted(
                        __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RawCoordinateSystemAdjusted(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialAnchor = __uuidof(ISpatialAnchor);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchor2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchor2[] = L"Windows.Perception.Spatial.ISpatialAnchor2";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("ed17c908-a695-4cf6-92fd-97263ba71047")
                ISpatialAnchor2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RemovedByUser(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialAnchor2 = __uuidof(ISpatialAnchor2);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchorExportSufficiency
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchorExportSufficiency
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchorExportSufficiency[] = L"Windows.Perception.Spatial.ISpatialAnchorExportSufficiency";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("77c25b2b-3409-4088-b91b-fdfd05d1648f")
                ISpatialAnchorExportSufficiency : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsMinimallySufficient(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SufficiencyLevel(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RecommendedSufficiencyLevel(
                        DOUBLE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialAnchorExportSufficiency = __uuidof(ISpatialAnchorExportSufficiency);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchorExporter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchorExporter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchorExporter[] = L"Windows.Perception.Spatial.ISpatialAnchorExporter";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("9a2a4338-24fb-4269-89c5-88304aeef20f")
                ISpatialAnchorExporter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAnchorExportSufficiencyAsync(
                        ABI::Windows::Perception::Spatial::ISpatialAnchor* anchor,
                        ABI::Windows::Perception::Spatial::SpatialAnchorExportPurpose purpose,
                        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryExportAnchorAsync(
                        ABI::Windows::Perception::Spatial::ISpatialAnchor* anchor,
                        ABI::Windows::Perception::Spatial::SpatialAnchorExportPurpose purpose,
                        ABI::Windows::Storage::Streams::IOutputStream* stream,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialAnchorExporter = __uuidof(ISpatialAnchorExporter);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchorExporterStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchorExporter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchorExporterStatics[] = L"Windows.Perception.Spatial.ISpatialAnchorExporterStatics";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("ed2507b8-2475-439c-85ff-7fed341fdc88")
                ISpatialAnchorExporterStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::Perception::Spatial::ISpatialAnchorExporter** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestAccessAsync(
                        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialAnchorExporterStatics = __uuidof(ISpatialAnchorExporterStatics);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchorManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchorManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchorManagerStatics[] = L"Windows.Perception.Spatial.ISpatialAnchorManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("88e30eab-f3b7-420b-b086-8a80c07d910d")
                ISpatialAnchorManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestStoreAsync(
                        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialAnchorManagerStatics = __uuidof(ISpatialAnchorManagerStatics);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchorRawCoordinateSystemAdjustedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchorRawCoordinateSystemAdjustedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchorRawCoordinateSystemAdjustedEventArgs[] = L"Windows.Perception.Spatial.ISpatialAnchorRawCoordinateSystemAdjustedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("a1e81eb8-56c7-3117-a2e4-81e0fcf28e00")
                ISpatialAnchorRawCoordinateSystemAdjustedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_OldRawCoordinateSystemToNewRawCoordinateSystemTransform(
                        ABI::Windows::Foundation::Numerics::Matrix4x4* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialAnchorRawCoordinateSystemAdjustedEventArgs = __uuidof(ISpatialAnchorRawCoordinateSystemAdjustedEventArgs);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchorStatics[] = L"Windows.Perception.Spatial.ISpatialAnchorStatics";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("a9928642-0174-311c-ae79-0e5107669f16")
                ISpatialAnchorStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryCreateRelativeTo(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        ABI::Windows::Perception::Spatial::ISpatialAnchor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryCreateWithPositionRelativeTo(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        ABI::Windows::Foundation::Numerics::Vector3 position,
                        ABI::Windows::Perception::Spatial::ISpatialAnchor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryCreateWithPositionAndOrientationRelativeTo(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        ABI::Windows::Foundation::Numerics::Vector3 position,
                        ABI::Windows::Foundation::Numerics::Quaternion orientation,
                        ABI::Windows::Perception::Spatial::ISpatialAnchor** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialAnchorStatics = __uuidof(ISpatialAnchorStatics);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchorStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchorStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchorStore[] = L"Windows.Perception.Spatial.ISpatialAnchorStore";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("b0bc3636-486a-3cb0-9e6f-1245165c4db6")
                ISpatialAnchorStore : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAllSavedAnchors(
                        __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySave(
                        HSTRING id,
                        ABI::Windows::Perception::Spatial::ISpatialAnchor* anchor,
                        boolean* succeeded
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Remove(
                        HSTRING id
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Clear(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialAnchorStore = __uuidof(ISpatialAnchorStore);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchorTransferManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchorTransferManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchorTransferManagerStatics[] = L"Windows.Perception.Spatial.ISpatialAnchorTransferManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("03bbf9b9-12d8-4bce-8835-c5df3ac0adab")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                DEPRECATED("Use SpatialEntityStore instead of SpatialAnchorTransferManager. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                ISpatialAnchorTransferManagerStatics : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("Use SpatialEntityStore instead of SpatialAnchorTransferManager. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    virtual HRESULT STDMETHODCALLTYPE TryImportAnchorsAsync(
                        ABI::Windows::Storage::Streams::IInputStream* stream,
                        __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("Use SpatialEntityStore instead of SpatialAnchorTransferManager. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    virtual HRESULT STDMETHODCALLTYPE TryExportAnchorsAsync(
                        __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* anchors,
                        ABI::Windows::Storage::Streams::IOutputStream* stream,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("Use SpatialEntityStore instead of SpatialAnchorTransferManager. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    virtual HRESULT STDMETHODCALLTYPE RequestAccessAsync(
                        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialAnchorTransferManagerStatics = __uuidof(ISpatialAnchorTransferManagerStatics);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialBoundingVolume
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialBoundingVolume
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialBoundingVolume[] = L"Windows.Perception.Spatial.ISpatialBoundingVolume";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("fb2065da-68c3-33df-b7af-4c787207999c")
                ISpatialBoundingVolume : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_ISpatialBoundingVolume = __uuidof(ISpatialBoundingVolume);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialBoundingVolumeStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialBoundingVolume
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialBoundingVolumeStatics[] = L"Windows.Perception.Spatial.ISpatialBoundingVolumeStatics";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("05889117-b3e1-36d8-b017-566181a5b196")
                ISpatialBoundingVolumeStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FromBox(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        ABI::Windows::Perception::Spatial::SpatialBoundingBox box,
                        ABI::Windows::Perception::Spatial::ISpatialBoundingVolume** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromOrientedBox(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        ABI::Windows::Perception::Spatial::SpatialBoundingOrientedBox box,
                        ABI::Windows::Perception::Spatial::ISpatialBoundingVolume** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromSphere(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        ABI::Windows::Perception::Spatial::SpatialBoundingSphere sphere,
                        ABI::Windows::Perception::Spatial::ISpatialBoundingVolume** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromFrustum(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        ABI::Windows::Perception::Spatial::SpatialBoundingFrustum frustum,
                        ABI::Windows::Perception::Spatial::ISpatialBoundingVolume** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialBoundingVolumeStatics = __uuidof(ISpatialBoundingVolumeStatics);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialCoordinateSystem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialCoordinateSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialCoordinateSystem[] = L"Windows.Perception.Spatial.ISpatialCoordinateSystem";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("69ebca4b-60a3-3586-a653-59a7bd676d07")
                ISpatialCoordinateSystem : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryGetTransformTo(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* target,
                        __FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialCoordinateSystem = __uuidof(ISpatialCoordinateSystem);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialEntity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialEntity
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialEntity[] = L"Windows.Perception.Spatial.ISpatialEntity";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("166de955-e1eb-454c-ba08-e6c0668ddc65")
                ISpatialEntity : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Anchor(
                        ABI::Windows::Perception::Spatial::ISpatialAnchor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialEntity = __uuidof(ISpatialEntity);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialEntityAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialEntityAddedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialEntityAddedEventArgs[] = L"Windows.Perception.Spatial.ISpatialEntityAddedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("a397f49b-156a-4707-ac2c-d31d570ed399")
                ISpatialEntityAddedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Entity(
                        ABI::Windows::Perception::Spatial::ISpatialEntity** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialEntityAddedEventArgs = __uuidof(ISpatialEntityAddedEventArgs);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialEntityFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialEntity
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialEntityFactory[] = L"Windows.Perception.Spatial.ISpatialEntityFactory";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("e1f1e325-349f-4225-a2f3-4b01c15fe056")
                ISpatialEntityFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWithSpatialAnchor(
                        ABI::Windows::Perception::Spatial::ISpatialAnchor* spatialAnchor,
                        ABI::Windows::Perception::Spatial::ISpatialEntity** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithSpatialAnchorAndProperties(
                        ABI::Windows::Perception::Spatial::ISpatialAnchor* spatialAnchor,
                        ABI::Windows::Foundation::Collections::IPropertySet* propertySet,
                        ABI::Windows::Perception::Spatial::ISpatialEntity** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialEntityFactory = __uuidof(ISpatialEntityFactory);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialEntityRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialEntityRemovedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialEntityRemovedEventArgs[] = L"Windows.Perception.Spatial.ISpatialEntityRemovedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("91741800-536d-4e9f-abf6-415b5444d651")
                ISpatialEntityRemovedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Entity(
                        ABI::Windows::Perception::Spatial::ISpatialEntity** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialEntityRemovedEventArgs = __uuidof(ISpatialEntityRemovedEventArgs);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialEntityStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialEntityStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialEntityStore[] = L"Windows.Perception.Spatial.ISpatialEntityStore";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("329788ba-e513-4f06-889d-1be30ecf43e6")
                ISpatialEntityStore : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SaveAsync(
                        ABI::Windows::Perception::Spatial::ISpatialEntity* entity,
                        ABI::Windows::Foundation::IAsyncAction** action
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveAsync(
                        ABI::Windows::Perception::Spatial::ISpatialEntity* entity,
                        ABI::Windows::Foundation::IAsyncAction** action
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateEntityWatcher(
                        ABI::Windows::Perception::Spatial::ISpatialEntityWatcher** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialEntityStore = __uuidof(ISpatialEntityStore);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialEntityStoreStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialEntityStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialEntityStoreStatics[] = L"Windows.Perception.Spatial.ISpatialEntityStoreStatics";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("6b4b389e-7c50-4e92-8a62-4d1d4b7ccd3e")
                ISpatialEntityStoreStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryGetForRemoteSystemSession(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemSession* session,
                        ABI::Windows::Perception::Spatial::ISpatialEntityStore** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialEntityStoreStatics = __uuidof(ISpatialEntityStoreStatics);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialEntityUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialEntityUpdatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialEntityUpdatedEventArgs[] = L"Windows.Perception.Spatial.ISpatialEntityUpdatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("e5671766-627b-43cb-a49f-b3be6d47deed")
                ISpatialEntityUpdatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Entity(
                        ABI::Windows::Perception::Spatial::ISpatialEntity** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialEntityUpdatedEventArgs = __uuidof(ISpatialEntityUpdatedEventArgs);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialEntityWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialEntityWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialEntityWatcher[] = L"Windows.Perception.Spatial.ISpatialEntityWatcher";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("b3b85fa0-6d5e-4bbc-805d-5fe5b9ba1959")
                ISpatialEntityWatcher : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Perception::Spatial::SpatialEntityWatcherStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Added(
                        __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Added(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Updated(
                        __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Updated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Removed(
                        __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Removed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_EnumerationCompleted(
                        __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_EnumerationCompleted(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialEntityWatcher = __uuidof(ISpatialEntityWatcher);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialLocation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialLocation[] = L"Windows.Perception.Spatial.ISpatialLocation";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("1d81d29d-24a1-37d5-8fa1-39b4f9ad67e2")
                ISpatialLocation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::Numerics::Vector3* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Orientation(
                        ABI::Windows::Foundation::Numerics::Quaternion* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AbsoluteLinearVelocity(
                        ABI::Windows::Foundation::Numerics::Vector3* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AbsoluteLinearAcceleration(
                        ABI::Windows::Foundation::Numerics::Vector3* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                    DEPRECATED("Use AbsoluteAngularVelocityAxisAngle instead of AbsoluteAngularVelocity. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                    virtual HRESULT STDMETHODCALLTYPE get_AbsoluteAngularVelocity(
                        ABI::Windows::Foundation::Numerics::Quaternion* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                    DEPRECATED("Use AbsoluteAngularAccelerationAxisAngle instead of AbsoluteAngularAcceleration. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                    virtual HRESULT STDMETHODCALLTYPE get_AbsoluteAngularAcceleration(
                        ABI::Windows::Foundation::Numerics::Quaternion* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialLocation = __uuidof(ISpatialLocation);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialLocation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialLocation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialLocation2[] = L"Windows.Perception.Spatial.ISpatialLocation2";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("117f2416-38a7-4a18-b404-ab8fabe1d78b")
                ISpatialLocation2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AbsoluteAngularVelocityAxisAngle(
                        ABI::Windows::Foundation::Numerics::Vector3* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AbsoluteAngularAccelerationAxisAngle(
                        ABI::Windows::Foundation::Numerics::Vector3* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialLocation2 = __uuidof(ISpatialLocation2);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialLocator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialLocator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialLocator[] = L"Windows.Perception.Spatial.ISpatialLocator";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("f6478925-9e0c-3bb6-997e-b64ecca24cf4")
                ISpatialLocator : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Locatability(
                        ABI::Windows::Perception::Spatial::SpatialLocatability* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_LocatabilityChanged(
                        __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_LocatabilityChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PositionalTrackingDeactivating(
                        __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PositionalTrackingDeactivating(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryLocateAtTimestamp(
                        ABI::Windows::Perception::IPerceptionTimestamp* timestamp,
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        ABI::Windows::Perception::Spatial::ISpatialLocation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAttachedFrameOfReferenceAtCurrentHeading(
                        ABI::Windows::Perception::Spatial::ISpatialLocatorAttachedFrameOfReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition(
                        ABI::Windows::Foundation::Numerics::Vector3 relativePosition,
                        ABI::Windows::Perception::Spatial::ISpatialLocatorAttachedFrameOfReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation(
                        ABI::Windows::Foundation::Numerics::Vector3 relativePosition,
                        ABI::Windows::Foundation::Numerics::Quaternion relativeOrientation,
                        ABI::Windows::Perception::Spatial::ISpatialLocatorAttachedFrameOfReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading(
                        ABI::Windows::Foundation::Numerics::Vector3 relativePosition,
                        ABI::Windows::Foundation::Numerics::Quaternion relativeOrientation,
                        DOUBLE relativeHeadingInRadians,
                        ABI::Windows::Perception::Spatial::ISpatialLocatorAttachedFrameOfReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateStationaryFrameOfReferenceAtCurrentLocation(
                        ABI::Windows::Perception::Spatial::ISpatialStationaryFrameOfReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition(
                        ABI::Windows::Foundation::Numerics::Vector3 relativePosition,
                        ABI::Windows::Perception::Spatial::ISpatialStationaryFrameOfReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation(
                        ABI::Windows::Foundation::Numerics::Vector3 relativePosition,
                        ABI::Windows::Foundation::Numerics::Quaternion relativeOrientation,
                        ABI::Windows::Perception::Spatial::ISpatialStationaryFrameOfReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading(
                        ABI::Windows::Foundation::Numerics::Vector3 relativePosition,
                        ABI::Windows::Foundation::Numerics::Quaternion relativeOrientation,
                        DOUBLE relativeHeadingInRadians,
                        ABI::Windows::Perception::Spatial::ISpatialStationaryFrameOfReference** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialLocator = __uuidof(ISpatialLocator);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialLocatorAttachedFrameOfReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialLocatorAttachedFrameOfReference
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialLocatorAttachedFrameOfReference[] = L"Windows.Perception.Spatial.ISpatialLocatorAttachedFrameOfReference";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("e1774ef6-1f4f-499c-9625-ef5e6ed7a048")
                ISpatialLocatorAttachedFrameOfReference : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RelativePosition(
                        ABI::Windows::Foundation::Numerics::Vector3* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RelativePosition(
                        ABI::Windows::Foundation::Numerics::Vector3 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RelativeOrientation(
                        ABI::Windows::Foundation::Numerics::Quaternion* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RelativeOrientation(
                        ABI::Windows::Foundation::Numerics::Quaternion value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AdjustHeading(
                        DOUBLE headingOffsetInRadians
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetStationaryCoordinateSystemAtTimestamp(
                        ABI::Windows::Perception::IPerceptionTimestamp* timestamp,
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryGetRelativeHeadingAtTimestamp(
                        ABI::Windows::Perception::IPerceptionTimestamp* timestamp,
                        __FIReference_1_double** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialLocatorAttachedFrameOfReference = __uuidof(ISpatialLocatorAttachedFrameOfReference);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialLocatorPositionalTrackingDeactivatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialLocatorPositionalTrackingDeactivatingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialLocatorPositionalTrackingDeactivatingEventArgs[] = L"Windows.Perception.Spatial.ISpatialLocatorPositionalTrackingDeactivatingEventArgs";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("b8a84063-e3f4-368b-9061-9ea9d1d6cc16")
                ISpatialLocatorPositionalTrackingDeactivatingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Canceled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Canceled(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialLocatorPositionalTrackingDeactivatingEventArgs = __uuidof(ISpatialLocatorPositionalTrackingDeactivatingEventArgs);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialLocatorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialLocator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialLocatorStatics[] = L"Windows.Perception.Spatial.ISpatialLocatorStatics";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("b76e3340-a7c2-361b-bb82-56e93b89b1bb")
                ISpatialLocatorStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::Perception::Spatial::ISpatialLocator** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialLocatorStatics = __uuidof(ISpatialLocatorStatics);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialStageFrameOfReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialStageFrameOfReference
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialStageFrameOfReference[] = L"Windows.Perception.Spatial.ISpatialStageFrameOfReference";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("7a8a3464-ad0d-4590-ab86-33062b674926")
                ISpatialStageFrameOfReference : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CoordinateSystem(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MovementRange(
                        ABI::Windows::Perception::Spatial::SpatialMovementRange* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LookDirectionRange(
                        ABI::Windows::Perception::Spatial::SpatialLookDirectionRange* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCoordinateSystemAtCurrentLocation(
                        ABI::Windows::Perception::Spatial::ISpatialLocator* locator,
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryGetMovementBounds(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        UINT32* valueLength,
                        ABI::Windows::Foundation::Numerics::Vector3** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialStageFrameOfReference = __uuidof(ISpatialStageFrameOfReference);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialStageFrameOfReferenceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialStageFrameOfReference
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialStageFrameOfReferenceStatics[] = L"Windows.Perception.Spatial.ISpatialStageFrameOfReferenceStatics";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("f78d5c4d-a0a4-499c-8d91-a8c965d40654")
                ISpatialStageFrameOfReferenceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Current(
                        ABI::Windows::Perception::Spatial::ISpatialStageFrameOfReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_CurrentChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_CurrentChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestNewStageAsync(
                        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialStageFrameOfReferenceStatics = __uuidof(ISpatialStageFrameOfReferenceStatics);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialStationaryFrameOfReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialStationaryFrameOfReference
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialStationaryFrameOfReference[] = L"Windows.Perception.Spatial.ISpatialStationaryFrameOfReference";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                MIDL_INTERFACE("09dbccb9-bcf8-3e7f-be7e-7edccbb178a8")
                ISpatialStationaryFrameOfReference : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CoordinateSystem(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialStationaryFrameOfReference = __uuidof(ISpatialStationaryFrameOfReference);
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialAnchor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.ISpatialAnchorStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialAnchor ** Default Interface **
 *    Windows.Perception.Spatial.ISpatialAnchor2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchor_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialAnchor[] = L"Windows.Perception.Spatial.SpatialAnchor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialAnchorExportSufficiency
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialAnchorExportSufficiency ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorExportSufficiency_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorExportSufficiency_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialAnchorExportSufficiency[] = L"Windows.Perception.Spatial.SpatialAnchorExportSufficiency";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Perception.Spatial.SpatialAnchorExporter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.ISpatialAnchorExporterStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialAnchorExporter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorExporter_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorExporter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialAnchorExporter[] = L"Windows.Perception.Spatial.SpatialAnchorExporter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Perception.Spatial.SpatialAnchorManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.ISpatialAnchorManagerStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorManager_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialAnchorManager[] = L"Windows.Perception.Spatial.SpatialAnchorManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialAnchorRawCoordinateSystemAdjustedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialAnchorRawCoordinateSystemAdjustedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorRawCoordinateSystemAdjustedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorRawCoordinateSystemAdjustedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialAnchorRawCoordinateSystemAdjustedEventArgs[] = L"Windows.Perception.Spatial.SpatialAnchorRawCoordinateSystemAdjustedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialAnchorStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialAnchorStore ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorStore_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialAnchorStore[] = L"Windows.Perception.Spatial.SpatialAnchorStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialAnchorTransferManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.ISpatialAnchorTransferManagerStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorTransferManager_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorTransferManager_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("Use SpatialEntityStore instead of SpatialAnchorTransferManager. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialAnchorTransferManager[] = L"Windows.Perception.Spatial.SpatialAnchorTransferManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialBoundingVolume
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.ISpatialBoundingVolumeStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialBoundingVolume ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialBoundingVolume_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialBoundingVolume_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialBoundingVolume[] = L"Windows.Perception.Spatial.SpatialBoundingVolume";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialCoordinateSystem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialCoordinateSystem ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialCoordinateSystem_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialCoordinateSystem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialCoordinateSystem[] = L"Windows.Perception.Spatial.SpatialCoordinateSystem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialEntity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Perception.Spatial.ISpatialEntityFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntity_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialEntity[] = L"Windows.Perception.Spatial.SpatialEntity";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Perception.Spatial.SpatialEntityAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialEntityAddedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityAddedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityAddedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialEntityAddedEventArgs[] = L"Windows.Perception.Spatial.SpatialEntityAddedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Perception.Spatial.SpatialEntityRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialEntityRemovedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityRemovedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityRemovedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialEntityRemovedEventArgs[] = L"Windows.Perception.Spatial.SpatialEntityRemovedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Perception.Spatial.SpatialEntityStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.ISpatialEntityStoreStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialEntityStore ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityStore_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialEntityStore[] = L"Windows.Perception.Spatial.SpatialEntityStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Perception.Spatial.SpatialEntityUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialEntityUpdatedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityUpdatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityUpdatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialEntityUpdatedEventArgs[] = L"Windows.Perception.Spatial.SpatialEntityUpdatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Perception.Spatial.SpatialEntityWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialEntityWatcher ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityWatcher_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialEntityWatcher[] = L"Windows.Perception.Spatial.SpatialEntityWatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Perception.Spatial.SpatialLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialLocation ** Default Interface **
 *    Windows.Perception.Spatial.ISpatialLocation2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialLocation_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialLocation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialLocation[] = L"Windows.Perception.Spatial.SpatialLocation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialLocator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.ISpatialLocatorStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialLocator ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialLocator_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialLocator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialLocator[] = L"Windows.Perception.Spatial.SpatialLocator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialLocatorAttachedFrameOfReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialLocatorAttachedFrameOfReference ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialLocatorAttachedFrameOfReference_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialLocatorAttachedFrameOfReference_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialLocatorAttachedFrameOfReference[] = L"Windows.Perception.Spatial.SpatialLocatorAttachedFrameOfReference";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialLocatorPositionalTrackingDeactivatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialLocatorPositionalTrackingDeactivatingEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialLocatorPositionalTrackingDeactivatingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialLocatorPositionalTrackingDeactivatingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialLocatorPositionalTrackingDeactivatingEventArgs[] = L"Windows.Perception.Spatial.SpatialLocatorPositionalTrackingDeactivatingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialStageFrameOfReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.ISpatialStageFrameOfReferenceStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialStageFrameOfReference ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialStageFrameOfReference_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialStageFrameOfReference_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialStageFrameOfReference[] = L"Windows.Perception.Spatial.SpatialStageFrameOfReference";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Perception.Spatial.SpatialStationaryFrameOfReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialStationaryFrameOfReference ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialStationaryFrameOfReference_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialStationaryFrameOfReference_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialStationaryFrameOfReference[] = L"Windows.Perception.Spatial.SpatialStationaryFrameOfReference";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2 __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2 __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference __x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_FWD_DEFINED__

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor __FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchorVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        __FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchorVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchorVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

typedef interface __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor;

typedef struct __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        HSTRING key,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor** first,
        __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchorVtbl;

interface __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

typedef interface __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor;

typedef struct __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchorVtbl;

interface __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor
{
    CONST_VTBL struct __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor;

typedef struct __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* This,
        __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchorVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency;

typedef struct __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiencyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiencyVtbl;

interface __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiencyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiencyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency* This,
        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiencyVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiencyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore;

typedef struct __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStoreVtbl;

interface __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore* This,
        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStoreVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialAnchorStore_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

typedef enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialPerceptionAccessStatus __x_ABI_CWindows_CPerception_CSpatial_CSpatialPerceptionAccessStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus;

typedef struct __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This,
        enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialPerceptionAccessStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatusVtbl;

interface __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This,
        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference;

typedef struct __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReferenceVtbl;

interface __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference* This,
        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReferenceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if !defined(____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_IInspectable __FIEventHandler_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_IInspectable;

typedef struct __FIEventHandler_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_IInspectable* This,
        IInspectable* sender,
        IInspectable* args);

    END_INTERFACE
} __FIEventHandler_1_IInspectableVtbl;

interface __FIEventHandler_1_IInspectable
{
    CONST_VTBL struct __FIEventHandler_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

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

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix4x4 __x_ABI_CWindows_CFoundation_CNumerics_CMatrix4x4;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4 __FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4;

typedef struct __FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix4x4* result);

    END_INTERFACE
} __FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4Vtbl;

interface __FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4
{
    CONST_VTBL struct __FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor* sender,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* sender,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* sender,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* sender,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* sender,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CPlane __x_ABI_CWindows_CFoundation_CNumerics_CPlane;

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion;

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 __x_ABI_CWindows_CFoundation_CNumerics_CVector3;

#ifndef ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CIPerceptionTimestamp __x_ABI_CWindows_CPerception_CIPerceptionTimestamp;

#endif // ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIOutputStream __x_ABI_CWindows_CStorage_CStreams_CIOutputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialAnchorExportPurpose __x_ABI_CWindows_CPerception_CSpatial_CSpatialAnchorExportPurpose;

typedef enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialEntityWatcherStatus __x_ABI_CWindows_CPerception_CSpatial_CSpatialEntityWatcherStatus;

typedef enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialLocatability __x_ABI_CWindows_CPerception_CSpatial_CSpatialLocatability;

typedef enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialLookDirectionRange __x_ABI_CWindows_CPerception_CSpatial_CSpatialLookDirectionRange;

typedef enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialMovementRange __x_ABI_CWindows_CPerception_CSpatial_CSpatialMovementRange;

typedef struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingBox __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingBox;

typedef struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingFrustum __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingFrustum;

typedef struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingOrientedBox __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingOrientedBox;

typedef struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingSphere __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingSphere;

/*
 *
 * Struct Windows.Perception.Spatial.SpatialAnchorExportPurpose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialAnchorExportPurpose
{
    SpatialAnchorExportPurpose_Relocalization = 0,
    SpatialAnchorExportPurpose_Sharing = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialEntityWatcherStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialEntityWatcherStatus
{
    SpatialEntityWatcherStatus_Created = 0,
    SpatialEntityWatcherStatus_Started = 1,
    SpatialEntityWatcherStatus_EnumerationCompleted = 2,
    SpatialEntityWatcherStatus_Stopping = 3,
    SpatialEntityWatcherStatus_Stopped = 4,
    SpatialEntityWatcherStatus_Aborted = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialLocatability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialLocatability
{
    SpatialLocatability_Unavailable = 0,
    SpatialLocatability_OrientationOnly = 1,
    SpatialLocatability_PositionalTrackingActivating = 2,
    SpatialLocatability_PositionalTrackingActive = 3,
    SpatialLocatability_PositionalTrackingInhibited = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialLookDirectionRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialLookDirectionRange
{
    SpatialLookDirectionRange_ForwardOnly = 0,
    SpatialLookDirectionRange_Omnidirectional = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialMovementRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialMovementRange
{
    SpatialMovementRange_NoMovement = 0,
    SpatialMovementRange_Bounded = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialPerceptionAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialPerceptionAccessStatus
{
    SpatialPerceptionAccessStatus_Unspecified = 0,
    SpatialPerceptionAccessStatus_Allowed = 1,
    SpatialPerceptionAccessStatus_DeniedByUser = 2,
    SpatialPerceptionAccessStatus_DeniedBySystem = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialBoundingBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingBox
{
    struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 Center;
    struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 Extents;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialBoundingFrustum
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingFrustum
{
    struct __x_ABI_CWindows_CFoundation_CNumerics_CPlane Near;
    struct __x_ABI_CWindows_CFoundation_CNumerics_CPlane Far;
    struct __x_ABI_CWindows_CFoundation_CNumerics_CPlane Right;
    struct __x_ABI_CWindows_CFoundation_CNumerics_CPlane Left;
    struct __x_ABI_CWindows_CFoundation_CNumerics_CPlane Top;
    struct __x_ABI_CWindows_CFoundation_CNumerics_CPlane Bottom;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialBoundingOrientedBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingOrientedBox
{
    struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 Center;
    struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 Extents;
    struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion Orientation;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialBoundingSphere
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingSphere
{
    struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 Center;
    FLOAT Radius;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Perception.Spatial.SpatialRay
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialRay
{
    struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 Origin;
    struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 Direction;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchor[] = L"Windows.Perception.Spatial.ISpatialAnchor";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CoordinateSystem)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem** value);
    HRESULT (STDMETHODCALLTYPE* get_RawCoordinateSystem)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem** value);
    HRESULT (STDMETHODCALLTYPE* add_RawCoordinateSystemAdjusted)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor* This,
        __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialAnchor_Windows__CPerception__CSpatial__CSpatialAnchorRawCoordinateSystemAdjustedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_RawCoordinateSystemAdjusted)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_get_CoordinateSystem(This, value) \
    ((This)->lpVtbl->get_CoordinateSystem(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_get_RawCoordinateSystem(This, value) \
    ((This)->lpVtbl->get_RawCoordinateSystem(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_add_RawCoordinateSystemAdjusted(This, handler, cookie) \
    ((This)->lpVtbl->add_RawCoordinateSystemAdjusted(This, handler, cookie))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_remove_RawCoordinateSystemAdjusted(This, cookie) \
    ((This)->lpVtbl->remove_RawCoordinateSystemAdjusted(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchor2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchor2[] = L"Windows.Perception.Spatial.ISpatialAnchor2";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RemovedByUser)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2Vtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_get_RemovedByUser(This, value) \
    ((This)->lpVtbl->get_RemovedByUser(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchorExportSufficiency
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchorExportSufficiency
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchorExportSufficiency[] = L"Windows.Perception.Spatial.ISpatialAnchorExportSufficiency";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiencyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsMinimallySufficient)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SufficiencyLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_RecommendedSufficiencyLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiencyVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiencyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_get_IsMinimallySufficient(This, value) \
    ((This)->lpVtbl->get_IsMinimallySufficient(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_get_SufficiencyLevel(This, value) \
    ((This)->lpVtbl->get_SufficiencyLevel(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_get_RecommendedSufficiencyLevel(This, value) \
    ((This)->lpVtbl->get_RecommendedSufficiencyLevel(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExportSufficiency_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchorExporter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchorExporter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchorExporter[] = L"Windows.Perception.Spatial.ISpatialAnchorExporter";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAnchorExportSufficiencyAsync)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor* anchor,
        enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialAnchorExportPurpose purpose,
        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorExportSufficiency** operation);
    HRESULT (STDMETHODCALLTYPE* TryExportAnchorAsync)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor* anchor,
        enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialAnchorExportPurpose purpose,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream* stream,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_GetAnchorExportSufficiencyAsync(This, anchor, purpose, operation) \
    ((This)->lpVtbl->GetAnchorExportSufficiencyAsync(This, anchor, purpose, operation))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_TryExportAnchorAsync(This, anchor, purpose, stream, operation) \
    ((This)->lpVtbl->TryExportAnchorAsync(This, anchor, purpose, stream, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchorExporterStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchorExporter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchorExporterStatics[] = L"Windows.Perception.Spatial.ISpatialAnchorExporterStatics";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporter** value);
    HRESULT (STDMETHODCALLTYPE* RequestAccessAsync)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics* This,
        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus** result);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStaticsVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_GetDefault(This, value) \
    ((This)->lpVtbl->GetDefault(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_RequestAccessAsync(This, result) \
    ((This)->lpVtbl->RequestAccessAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorExporterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchorManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchorManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchorManagerStatics[] = L"Windows.Perception.Spatial.ISpatialAnchorManagerStatics";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestStoreAsync)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics* This,
        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialAnchorStore** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStaticsVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_RequestStoreAsync(This, value) \
    ((This)->lpVtbl->RequestStoreAsync(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchorRawCoordinateSystemAdjustedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchorRawCoordinateSystemAdjustedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchorRawCoordinateSystemAdjustedEventArgs[] = L"Windows.Perception.Spatial.ISpatialAnchorRawCoordinateSystemAdjustedEventArgs";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OldRawCoordinateSystemToNewRawCoordinateSystemTransform)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix4x4* value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgsVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_get_OldRawCoordinateSystemToNewRawCoordinateSystemTransform(This, value) \
    ((This)->lpVtbl->get_OldRawCoordinateSystemToNewRawCoordinateSystemTransform(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorRawCoordinateSystemAdjustedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchorStatics[] = L"Windows.Perception.Spatial.ISpatialAnchorStatics";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryCreateRelativeTo)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor** value);
    HRESULT (STDMETHODCALLTYPE* TryCreateWithPositionRelativeTo)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 position,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor** value);
    HRESULT (STDMETHODCALLTYPE* TryCreateWithPositionAndOrientationRelativeTo)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 position,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion orientation,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStaticsVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_TryCreateRelativeTo(This, coordinateSystem, value) \
    ((This)->lpVtbl->TryCreateRelativeTo(This, coordinateSystem, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_TryCreateWithPositionRelativeTo(This, coordinateSystem, position, value) \
    ((This)->lpVtbl->TryCreateWithPositionRelativeTo(This, coordinateSystem, position, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_TryCreateWithPositionAndOrientationRelativeTo(This, coordinateSystem, position, orientation, value) \
    ((This)->lpVtbl->TryCreateWithPositionAndOrientationRelativeTo(This, coordinateSystem, position, orientation, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchorStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchorStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchorStore[] = L"Windows.Perception.Spatial.ISpatialAnchorStore";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAllSavedAnchors)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore* This,
        __FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor** value);
    HRESULT (STDMETHODCALLTYPE* TrySave)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore* This,
        HSTRING id,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor* anchor,
        boolean* succeeded);
    HRESULT (STDMETHODCALLTYPE* Remove)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore* This,
        HSTRING id);
    HRESULT (STDMETHODCALLTYPE* Clear)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore* This);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStoreVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_GetAllSavedAnchors(This, value) \
    ((This)->lpVtbl->GetAllSavedAnchors(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_TrySave(This, id, anchor, succeeded) \
    ((This)->lpVtbl->TrySave(This, id, anchor, succeeded))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_Remove(This, id) \
    ((This)->lpVtbl->Remove(This, id))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialAnchorTransferManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialAnchorTransferManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialAnchorTransferManagerStatics[] = L"Windows.Perception.Spatial.ISpatialAnchorTransferManagerStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("Use SpatialEntityStore instead of SpatialAnchorTransferManager. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("Use SpatialEntityStore instead of SpatialAnchorTransferManager. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* TryImportAnchorsAsync)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* stream,
        __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("Use SpatialEntityStore instead of SpatialAnchorTransferManager. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* TryExportAnchorsAsync)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics* This,
        __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CPerception__CSpatial__CSpatialAnchor* anchors,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream* stream,
        __FIAsyncOperation_1_boolean** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("Use SpatialEntityStore instead of SpatialAnchorTransferManager. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* RequestAccessAsync)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics* This,
        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus** result);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStaticsVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("Use SpatialEntityStore instead of SpatialAnchorTransferManager. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_TryImportAnchorsAsync(This, stream, operation) \
    ((This)->lpVtbl->TryImportAnchorsAsync(This, stream, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("Use SpatialEntityStore instead of SpatialAnchorTransferManager. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_TryExportAnchorsAsync(This, anchors, stream, operation) \
    ((This)->lpVtbl->TryExportAnchorsAsync(This, anchors, stream, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("Use SpatialEntityStore instead of SpatialAnchorTransferManager. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_RequestAccessAsync(This, result) \
    ((This)->lpVtbl->RequestAccessAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchorTransferManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialBoundingVolume
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialBoundingVolume
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialBoundingVolume[] = L"Windows.Perception.Spatial.ISpatialBoundingVolume";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialBoundingVolumeStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialBoundingVolume
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialBoundingVolumeStatics[] = L"Windows.Perception.Spatial.ISpatialBoundingVolumeStatics";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromBox)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingBox box,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume** value);
    HRESULT (STDMETHODCALLTYPE* FromOrientedBox)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingOrientedBox box,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume** value);
    HRESULT (STDMETHODCALLTYPE* FromSphere)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingSphere sphere,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume** value);
    HRESULT (STDMETHODCALLTYPE* FromFrustum)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingFrustum frustum,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStaticsVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_FromBox(This, coordinateSystem, box, value) \
    ((This)->lpVtbl->FromBox(This, coordinateSystem, box, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_FromOrientedBox(This, coordinateSystem, box, value) \
    ((This)->lpVtbl->FromOrientedBox(This, coordinateSystem, box, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_FromSphere(This, coordinateSystem, sphere, value) \
    ((This)->lpVtbl->FromSphere(This, coordinateSystem, sphere, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_FromFrustum(This, coordinateSystem, frustum, value) \
    ((This)->lpVtbl->FromFrustum(This, coordinateSystem, frustum, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolumeStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialCoordinateSystem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialCoordinateSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialCoordinateSystem[] = L"Windows.Perception.Spatial.ISpatialCoordinateSystem";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryGetTransformTo)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* target,
        __FIReference_1_Windows__CFoundation__CNumerics__CMatrix4x4** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystemVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_TryGetTransformTo(This, target, value) \
    ((This)->lpVtbl->TryGetTransformTo(This, target, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialEntity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialEntity
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialEntity[] = L"Windows.Perception.Spatial.ISpatialEntity";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Anchor)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor** value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_get_Anchor(This, value) \
    ((This)->lpVtbl->get_Anchor(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialEntityAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialEntityAddedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialEntityAddedEventArgs[] = L"Windows.Perception.Spatial.ISpatialEntityAddedEventArgs";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Entity)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgsVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_get_Entity(This, value) \
    ((This)->lpVtbl->get_Entity(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityAddedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialEntityFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialEntity
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialEntityFactory[] = L"Windows.Perception.Spatial.ISpatialEntityFactory";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithSpatialAnchor)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor* spatialAnchor,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithSpatialAnchorAndProperties)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialAnchor* spatialAnchor,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* propertySet,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactoryVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_CreateWithSpatialAnchor(This, spatialAnchor, value) \
    ((This)->lpVtbl->CreateWithSpatialAnchor(This, spatialAnchor, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_CreateWithSpatialAnchorAndProperties(This, spatialAnchor, propertySet, value) \
    ((This)->lpVtbl->CreateWithSpatialAnchorAndProperties(This, spatialAnchor, propertySet, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialEntityRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialEntityRemovedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialEntityRemovedEventArgs[] = L"Windows.Perception.Spatial.ISpatialEntityRemovedEventArgs";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Entity)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgsVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_get_Entity(This, value) \
    ((This)->lpVtbl->get_Entity(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityRemovedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialEntityStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialEntityStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialEntityStore[] = L"Windows.Perception.Spatial.ISpatialEntityStore";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SaveAsync)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity* entity,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** action);
    HRESULT (STDMETHODCALLTYPE* RemoveAsync)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity* entity,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** action);
    HRESULT (STDMETHODCALLTYPE* CreateEntityWatcher)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_SaveAsync(This, entity, action) \
    ((This)->lpVtbl->SaveAsync(This, entity, action))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_RemoveAsync(This, entity, action) \
    ((This)->lpVtbl->RemoveAsync(This, entity, action))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_CreateEntityWatcher(This, value) \
    ((This)->lpVtbl->CreateEntityWatcher(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialEntityStoreStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialEntityStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialEntityStoreStatics[] = L"Windows.Perception.Spatial.ISpatialEntityStoreStatics";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsSupported)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* TryGetForRemoteSystemSession)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemSession* session,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStore** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStaticsVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_get_IsSupported(This, value) \
    ((This)->lpVtbl->get_IsSupported(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_TryGetForRemoteSystemSession(This, session, value) \
    ((This)->lpVtbl->TryGetForRemoteSystemSession(This, session, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityStoreStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialEntityUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialEntityUpdatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialEntityUpdatedEventArgs[] = L"Windows.Perception.Spatial.ISpatialEntityUpdatedEventArgs";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Entity)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntity** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgsVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_get_Entity(This, value) \
    ((This)->lpVtbl->get_Entity(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityUpdatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialEntityWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialEntityWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialEntityWatcher[] = L"Windows.Perception.Spatial.ISpatialEntityWatcher";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcherVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* This,
        enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialEntityWatcherStatus* value);
    HRESULT (STDMETHODCALLTYPE* add_Added)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* This,
        __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityAddedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Added)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Updated)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* This,
        __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityUpdatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Updated)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Removed)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* This,
        __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_Windows__CPerception__CSpatial__CSpatialEntityRemovedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Removed)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_EnumerationCompleted)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* This,
        __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialEntityWatcher_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_EnumerationCompleted)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher* This);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcherVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcherVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_add_Added(This, handler, token) \
    ((This)->lpVtbl->add_Added(This, handler, token))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_remove_Added(This, token) \
    ((This)->lpVtbl->remove_Added(This, token))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_add_Updated(This, handler, token) \
    ((This)->lpVtbl->add_Updated(This, handler, token))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_remove_Updated(This, token) \
    ((This)->lpVtbl->remove_Updated(This, token))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_add_Removed(This, handler, token) \
    ((This)->lpVtbl->add_Removed(This, handler, token))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_remove_Removed(This, token) \
    ((This)->lpVtbl->remove_Removed(This, token))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_add_EnumerationCompleted(This, handler, token) \
    ((This)->lpVtbl->add_EnumerationCompleted(This, handler, token))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_remove_EnumerationCompleted(This, token) \
    ((This)->lpVtbl->remove_EnumerationCompleted(This, token))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialEntityWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialLocation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialLocation[] = L"Windows.Perception.Spatial.ISpatialLocation";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* get_Orientation)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion* value);
    HRESULT (STDMETHODCALLTYPE* get_AbsoluteLinearVelocity)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* get_AbsoluteLinearAcceleration)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATED("Use AbsoluteAngularVelocityAxisAngle instead of AbsoluteAngularVelocity. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* get_AbsoluteAngularVelocity)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATED("Use AbsoluteAngularAccelerationAxisAngle instead of AbsoluteAngularAcceleration. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* get_AbsoluteAngularAcceleration)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion* value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocationVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_get_Orientation(This, value) \
    ((This)->lpVtbl->get_Orientation(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_get_AbsoluteLinearVelocity(This, value) \
    ((This)->lpVtbl->get_AbsoluteLinearVelocity(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_get_AbsoluteLinearAcceleration(This, value) \
    ((This)->lpVtbl->get_AbsoluteLinearAcceleration(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATED("Use AbsoluteAngularVelocityAxisAngle instead of AbsoluteAngularVelocity. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_get_AbsoluteAngularVelocity(This, value) \
    ((This)->lpVtbl->get_AbsoluteAngularVelocity(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATED("Use AbsoluteAngularAccelerationAxisAngle instead of AbsoluteAngularAcceleration. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_get_AbsoluteAngularAcceleration(This, value) \
    ((This)->lpVtbl->get_AbsoluteAngularAcceleration(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialLocation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialLocation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialLocation2[] = L"Windows.Perception.Spatial.ISpatialLocation2";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AbsoluteAngularVelocityAxisAngle)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* get_AbsoluteAngularAccelerationAxisAngle)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2Vtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_get_AbsoluteAngularVelocityAxisAngle(This, value) \
    ((This)->lpVtbl->get_AbsoluteAngularVelocityAxisAngle(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_get_AbsoluteAngularAccelerationAxisAngle(This, value) \
    ((This)->lpVtbl->get_AbsoluteAngularAccelerationAxisAngle(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialLocator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialLocator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialLocator[] = L"Windows.Perception.Spatial.ISpatialLocator";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Locatability)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This,
        enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialLocatability* value);
    HRESULT (STDMETHODCALLTYPE* add_LocatabilityChanged)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This,
        __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_IInspectable* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_LocatabilityChanged)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PositionalTrackingDeactivating)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This,
        __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSpatialLocator_Windows__CPerception__CSpatial__CSpatialLocatorPositionalTrackingDeactivatingEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PositionalTrackingDeactivating)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* TryLocateAtTimestamp)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This,
        __x_ABI_CWindows_CPerception_CIPerceptionTimestamp* timestamp,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocation** value);
    HRESULT (STDMETHODCALLTYPE* CreateAttachedFrameOfReferenceAtCurrentHeading)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference** value);
    HRESULT (STDMETHODCALLTYPE* CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 relativePosition,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference** value);
    HRESULT (STDMETHODCALLTYPE* CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 relativePosition,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion relativeOrientation,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference** value);
    HRESULT (STDMETHODCALLTYPE* CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 relativePosition,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion relativeOrientation,
        DOUBLE relativeHeadingInRadians,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference** value);
    HRESULT (STDMETHODCALLTYPE* CreateStationaryFrameOfReferenceAtCurrentLocation)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference** value);
    HRESULT (STDMETHODCALLTYPE* CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 relativePosition,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference** value);
    HRESULT (STDMETHODCALLTYPE* CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 relativePosition,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion relativeOrientation,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference** value);
    HRESULT (STDMETHODCALLTYPE* CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 relativePosition,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion relativeOrientation,
        DOUBLE relativeHeadingInRadians,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_get_Locatability(This, value) \
    ((This)->lpVtbl->get_Locatability(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_add_LocatabilityChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_LocatabilityChanged(This, handler, cookie))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_remove_LocatabilityChanged(This, cookie) \
    ((This)->lpVtbl->remove_LocatabilityChanged(This, cookie))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_add_PositionalTrackingDeactivating(This, handler, cookie) \
    ((This)->lpVtbl->add_PositionalTrackingDeactivating(This, handler, cookie))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_remove_PositionalTrackingDeactivating(This, cookie) \
    ((This)->lpVtbl->remove_PositionalTrackingDeactivating(This, cookie))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_TryLocateAtTimestamp(This, timestamp, coordinateSystem, value) \
    ((This)->lpVtbl->TryLocateAtTimestamp(This, timestamp, coordinateSystem, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_CreateAttachedFrameOfReferenceAtCurrentHeading(This, value) \
    ((This)->lpVtbl->CreateAttachedFrameOfReferenceAtCurrentHeading(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition(This, relativePosition, value) \
    ((This)->lpVtbl->CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition(This, relativePosition, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation(This, relativePosition, relativeOrientation, value) \
    ((This)->lpVtbl->CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation(This, relativePosition, relativeOrientation, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading(This, relativePosition, relativeOrientation, relativeHeadingInRadians, value) \
    ((This)->lpVtbl->CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading(This, relativePosition, relativeOrientation, relativeHeadingInRadians, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_CreateStationaryFrameOfReferenceAtCurrentLocation(This, value) \
    ((This)->lpVtbl->CreateStationaryFrameOfReferenceAtCurrentLocation(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition(This, relativePosition, value) \
    ((This)->lpVtbl->CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition(This, relativePosition, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation(This, relativePosition, relativeOrientation, value) \
    ((This)->lpVtbl->CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation(This, relativePosition, relativeOrientation, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading(This, relativePosition, relativeOrientation, relativeHeadingInRadians, value) \
    ((This)->lpVtbl->CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading(This, relativePosition, relativeOrientation, relativeHeadingInRadians, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialLocatorAttachedFrameOfReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialLocatorAttachedFrameOfReference
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialLocatorAttachedFrameOfReference[] = L"Windows.Perception.Spatial.ISpatialLocatorAttachedFrameOfReference";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RelativePosition)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* put_RelativePosition)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 value);
    HRESULT (STDMETHODCALLTYPE* get_RelativeOrientation)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion* value);
    HRESULT (STDMETHODCALLTYPE* put_RelativeOrientation)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion value);
    HRESULT (STDMETHODCALLTYPE* AdjustHeading)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference* This,
        DOUBLE headingOffsetInRadians);
    HRESULT (STDMETHODCALLTYPE* GetStationaryCoordinateSystemAtTimestamp)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference* This,
        __x_ABI_CWindows_CPerception_CIPerceptionTimestamp* timestamp,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem** value);
    HRESULT (STDMETHODCALLTYPE* TryGetRelativeHeadingAtTimestamp)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference* This,
        __x_ABI_CWindows_CPerception_CIPerceptionTimestamp* timestamp,
        __FIReference_1_double** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReferenceVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_get_RelativePosition(This, value) \
    ((This)->lpVtbl->get_RelativePosition(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_put_RelativePosition(This, value) \
    ((This)->lpVtbl->put_RelativePosition(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_get_RelativeOrientation(This, value) \
    ((This)->lpVtbl->get_RelativeOrientation(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_put_RelativeOrientation(This, value) \
    ((This)->lpVtbl->put_RelativeOrientation(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_AdjustHeading(This, headingOffsetInRadians) \
    ((This)->lpVtbl->AdjustHeading(This, headingOffsetInRadians))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_GetStationaryCoordinateSystemAtTimestamp(This, timestamp, value) \
    ((This)->lpVtbl->GetStationaryCoordinateSystemAtTimestamp(This, timestamp, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_TryGetRelativeHeadingAtTimestamp(This, timestamp, value) \
    ((This)->lpVtbl->TryGetRelativeHeadingAtTimestamp(This, timestamp, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorAttachedFrameOfReference_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialLocatorPositionalTrackingDeactivatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialLocatorPositionalTrackingDeactivatingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialLocatorPositionalTrackingDeactivatingEventArgs[] = L"Windows.Perception.Spatial.ISpatialLocatorPositionalTrackingDeactivatingEventArgs";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Canceled)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Canceled)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgsVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_get_Canceled(This, value) \
    ((This)->lpVtbl->get_Canceled(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_put_Canceled(This, value) \
    ((This)->lpVtbl->put_Canceled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorPositionalTrackingDeactivatingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialLocatorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialLocator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialLocatorStatics[] = L"Windows.Perception.Spatial.ISpatialLocatorStatics";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStaticsVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_GetDefault(This, value) \
    ((This)->lpVtbl->GetDefault(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocatorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialStageFrameOfReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialStageFrameOfReference
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialStageFrameOfReference[] = L"Windows.Perception.Spatial.ISpatialStageFrameOfReference";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CoordinateSystem)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem** value);
    HRESULT (STDMETHODCALLTYPE* get_MovementRange)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference* This,
        enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialMovementRange* value);
    HRESULT (STDMETHODCALLTYPE* get_LookDirectionRange)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference* This,
        enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialLookDirectionRange* value);
    HRESULT (STDMETHODCALLTYPE* GetCoordinateSystemAtCurrentLocation)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator* locator,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem** result);
    HRESULT (STDMETHODCALLTYPE* TryGetMovementBounds)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        UINT32* valueLength,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_get_CoordinateSystem(This, value) \
    ((This)->lpVtbl->get_CoordinateSystem(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_get_MovementRange(This, value) \
    ((This)->lpVtbl->get_MovementRange(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_get_LookDirectionRange(This, value) \
    ((This)->lpVtbl->get_LookDirectionRange(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_GetCoordinateSystemAtCurrentLocation(This, locator, result) \
    ((This)->lpVtbl->GetCoordinateSystemAtCurrentLocation(This, locator, result))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_TryGetMovementBounds(This, coordinateSystem, valueLength, value) \
    ((This)->lpVtbl->TryGetMovementBounds(This, coordinateSystem, valueLength, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialStageFrameOfReferenceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialStageFrameOfReference
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialStageFrameOfReferenceStatics[] = L"Windows.Perception.Spatial.ISpatialStageFrameOfReferenceStatics";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReference** value);
    HRESULT (STDMETHODCALLTYPE* add_CurrentChanged)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_CurrentChanged)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* RequestNewStageAsync)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics* This,
        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialStageFrameOfReference** result);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStaticsVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_get_Current(This, value) \
    ((This)->lpVtbl->get_Current(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_add_CurrentChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_CurrentChanged(This, handler, cookie))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_remove_CurrentChanged(This, cookie) \
    ((This)->lpVtbl->remove_CurrentChanged(This, cookie))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_RequestNewStageAsync(This, result) \
    ((This)->lpVtbl->RequestNewStageAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialStageFrameOfReferenceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Perception.Spatial.ISpatialStationaryFrameOfReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.SpatialStationaryFrameOfReference
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_ISpatialStationaryFrameOfReference[] = L"Windows.Perception.Spatial.ISpatialStationaryFrameOfReference";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CoordinateSystem)(__x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReferenceVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_get_CoordinateSystem(This, value) \
    ((This)->lpVtbl->get_CoordinateSystem(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CISpatialStationaryFrameOfReference_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialAnchor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.ISpatialAnchorStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialAnchor ** Default Interface **
 *    Windows.Perception.Spatial.ISpatialAnchor2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchor_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialAnchor[] = L"Windows.Perception.Spatial.SpatialAnchor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialAnchorExportSufficiency
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialAnchorExportSufficiency ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorExportSufficiency_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorExportSufficiency_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialAnchorExportSufficiency[] = L"Windows.Perception.Spatial.SpatialAnchorExportSufficiency";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Perception.Spatial.SpatialAnchorExporter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.ISpatialAnchorExporterStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialAnchorExporter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorExporter_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorExporter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialAnchorExporter[] = L"Windows.Perception.Spatial.SpatialAnchorExporter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Perception.Spatial.SpatialAnchorManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.ISpatialAnchorManagerStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorManager_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialAnchorManager[] = L"Windows.Perception.Spatial.SpatialAnchorManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialAnchorRawCoordinateSystemAdjustedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialAnchorRawCoordinateSystemAdjustedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorRawCoordinateSystemAdjustedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorRawCoordinateSystemAdjustedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialAnchorRawCoordinateSystemAdjustedEventArgs[] = L"Windows.Perception.Spatial.SpatialAnchorRawCoordinateSystemAdjustedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialAnchorStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialAnchorStore ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorStore_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialAnchorStore[] = L"Windows.Perception.Spatial.SpatialAnchorStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialAnchorTransferManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.ISpatialAnchorTransferManagerStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorTransferManager_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialAnchorTransferManager_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("Use SpatialEntityStore instead of SpatialAnchorTransferManager. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialAnchorTransferManager[] = L"Windows.Perception.Spatial.SpatialAnchorTransferManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialBoundingVolume
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.ISpatialBoundingVolumeStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialBoundingVolume ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialBoundingVolume_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialBoundingVolume_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialBoundingVolume[] = L"Windows.Perception.Spatial.SpatialBoundingVolume";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialCoordinateSystem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialCoordinateSystem ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialCoordinateSystem_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialCoordinateSystem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialCoordinateSystem[] = L"Windows.Perception.Spatial.SpatialCoordinateSystem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialEntity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Perception.Spatial.ISpatialEntityFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntity_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialEntity[] = L"Windows.Perception.Spatial.SpatialEntity";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Perception.Spatial.SpatialEntityAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialEntityAddedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityAddedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityAddedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialEntityAddedEventArgs[] = L"Windows.Perception.Spatial.SpatialEntityAddedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Perception.Spatial.SpatialEntityRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialEntityRemovedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityRemovedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityRemovedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialEntityRemovedEventArgs[] = L"Windows.Perception.Spatial.SpatialEntityRemovedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Perception.Spatial.SpatialEntityStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.ISpatialEntityStoreStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialEntityStore ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityStore_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialEntityStore[] = L"Windows.Perception.Spatial.SpatialEntityStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Perception.Spatial.SpatialEntityUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialEntityUpdatedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityUpdatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityUpdatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialEntityUpdatedEventArgs[] = L"Windows.Perception.Spatial.SpatialEntityUpdatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Perception.Spatial.SpatialEntityWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialEntityWatcher ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityWatcher_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialEntityWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialEntityWatcher[] = L"Windows.Perception.Spatial.SpatialEntityWatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Perception.Spatial.SpatialLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialLocation ** Default Interface **
 *    Windows.Perception.Spatial.ISpatialLocation2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialLocation_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialLocation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialLocation[] = L"Windows.Perception.Spatial.SpatialLocation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialLocator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.ISpatialLocatorStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialLocator ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialLocator_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialLocator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialLocator[] = L"Windows.Perception.Spatial.SpatialLocator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialLocatorAttachedFrameOfReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialLocatorAttachedFrameOfReference ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialLocatorAttachedFrameOfReference_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialLocatorAttachedFrameOfReference_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialLocatorAttachedFrameOfReference[] = L"Windows.Perception.Spatial.SpatialLocatorAttachedFrameOfReference";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialLocatorPositionalTrackingDeactivatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialLocatorPositionalTrackingDeactivatingEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialLocatorPositionalTrackingDeactivatingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialLocatorPositionalTrackingDeactivatingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialLocatorPositionalTrackingDeactivatingEventArgs[] = L"Windows.Perception.Spatial.SpatialLocatorPositionalTrackingDeactivatingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.SpatialStageFrameOfReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.ISpatialStageFrameOfReferenceStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialStageFrameOfReference ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialStageFrameOfReference_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialStageFrameOfReference_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialStageFrameOfReference[] = L"Windows.Perception.Spatial.SpatialStageFrameOfReference";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Perception.Spatial.SpatialStationaryFrameOfReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.ISpatialStationaryFrameOfReference ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_SpatialStationaryFrameOfReference_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_SpatialStationaryFrameOfReference_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_SpatialStationaryFrameOfReference[] = L"Windows.Perception.Spatial.SpatialStationaryFrameOfReference";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eperception2Espatial_p_h__

#endif // __windows2Eperception2Espatial_h__
