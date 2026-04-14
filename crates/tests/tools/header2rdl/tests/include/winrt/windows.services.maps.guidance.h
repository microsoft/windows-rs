
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
#ifndef __windows2Eservices2Emaps2Eguidance_h__
#define __windows2Eservices2Emaps2Eguidance_h__
#ifndef __windows2Eservices2Emaps2Eguidance_p_h__
#define __windows2Eservices2Emaps2Eguidance_p_h__


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
#include "Windows.Services.Maps.h"
#include "Windows.UI.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    interface IGuidanceAudioNotificationRequestedEventArgs;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs ABI::Windows::Services::Maps::Guidance::IGuidanceAudioNotificationRequestedEventArgs

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    interface IGuidanceLaneInfo;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo ABI::Windows::Services::Maps::Guidance::IGuidanceLaneInfo

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    interface IGuidanceManeuver;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver ABI::Windows::Services::Maps::Guidance::IGuidanceManeuver

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    interface IGuidanceMapMatchedCoordinate;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate ABI::Windows::Services::Maps::Guidance::IGuidanceMapMatchedCoordinate

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    interface IGuidanceNavigator;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator ABI::Windows::Services::Maps::Guidance::IGuidanceNavigator

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    interface IGuidanceNavigator2;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2 ABI::Windows::Services::Maps::Guidance::IGuidanceNavigator2

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    interface IGuidanceNavigatorStatics;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics ABI::Windows::Services::Maps::Guidance::IGuidanceNavigatorStatics

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    interface IGuidanceNavigatorStatics2;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2 ABI::Windows::Services::Maps::Guidance::IGuidanceNavigatorStatics2

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    interface IGuidanceReroutedEventArgs;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs ABI::Windows::Services::Maps::Guidance::IGuidanceReroutedEventArgs

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    interface IGuidanceRoadSegment;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment ABI::Windows::Services::Maps::Guidance::IGuidanceRoadSegment

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    interface IGuidanceRoadSegment2;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2 ABI::Windows::Services::Maps::Guidance::IGuidanceRoadSegment2

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    interface IGuidanceRoadSignpost;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost ABI::Windows::Services::Maps::Guidance::IGuidanceRoadSignpost

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    interface IGuidanceRoute;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute ABI::Windows::Services::Maps::Guidance::IGuidanceRoute

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    interface IGuidanceRouteStatics;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics ABI::Windows::Services::Maps::Guidance::IGuidanceRouteStatics

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    interface IGuidanceTelemetryCollector;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector ABI::Windows::Services::Maps::Guidance::IGuidanceTelemetryCollector

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    interface IGuidanceTelemetryCollectorStatics;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics ABI::Windows::Services::Maps::Guidance::IGuidanceTelemetryCollectorStatics

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    interface IGuidanceUpdatedEventArgs;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs ABI::Windows::Services::Maps::Guidance::IGuidanceUpdatedEventArgs

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_FWD_DEFINED__

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


namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    class GuidanceLaneInfo;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_USE
#define DEF___FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4d4ce4d8-7ce0-5168-ab29-6bcf7f198a58"))
IIterator<ABI::Windows::Services::Maps::Guidance::GuidanceLaneInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::Guidance::GuidanceLaneInfo*, ABI::Windows::Services::Maps::Guidance::IGuidanceLaneInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Services.Maps.Guidance.GuidanceLaneInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Services::Maps::Guidance::GuidanceLaneInfo*> __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_t;
#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_USE */

#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_USE
#define DEF___FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("45960d72-1bf6-5a1d-a17f-e83f56f1ab57"))
IIterable<ABI::Windows::Services::Maps::Guidance::GuidanceLaneInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::Guidance::GuidanceLaneInfo*, ABI::Windows::Services::Maps::Guidance::IGuidanceLaneInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Services.Maps.Guidance.GuidanceLaneInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Services::Maps::Guidance::GuidanceLaneInfo*> __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_t;
#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_USE */

#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    class GuidanceManeuver;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_USE
#define DEF___FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("74a37092-2641-5c3d-b3cb-689dc8aba22e"))
IIterator<ABI::Windows::Services::Maps::Guidance::GuidanceManeuver*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::Guidance::GuidanceManeuver*, ABI::Windows::Services::Maps::Guidance::IGuidanceManeuver*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Services.Maps.Guidance.GuidanceManeuver>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Services::Maps::Guidance::GuidanceManeuver*> __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_t;
#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_USE */

#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_USE
#define DEF___FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b5780d67-8a8b-558f-a4b6-c4531ef32ec8"))
IIterable<ABI::Windows::Services::Maps::Guidance::GuidanceManeuver*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::Guidance::GuidanceManeuver*, ABI::Windows::Services::Maps::Guidance::IGuidanceManeuver*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Services.Maps.Guidance.GuidanceManeuver>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Services::Maps::Guidance::GuidanceManeuver*> __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_t;
#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_USE */

#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    class GuidanceRoadSegment;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_USE
#define DEF___FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("148cb8ff-3ab9-53ab-8824-256a62047b43"))
IIterator<ABI::Windows::Services::Maps::Guidance::GuidanceRoadSegment*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::Guidance::GuidanceRoadSegment*, ABI::Windows::Services::Maps::Guidance::IGuidanceRoadSegment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Services.Maps.Guidance.GuidanceRoadSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Services::Maps::Guidance::GuidanceRoadSegment*> __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_t;
#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_USE */

#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_USE
#define DEF___FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f7c614c4-0fca-5eda-804c-85c829956334"))
IIterable<ABI::Windows::Services::Maps::Guidance::GuidanceRoadSegment*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::Guidance::GuidanceRoadSegment*, ABI::Windows::Services::Maps::Guidance::IGuidanceRoadSegment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Services.Maps.Guidance.GuidanceRoadSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Services::Maps::Guidance::GuidanceRoadSegment*> __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_t;
#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_USE */

#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000


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


#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_USE
#define DEF___FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("81493670-e515-5c62-b34c-6e3d996cad31"))
IVectorView<ABI::Windows::Services::Maps::Guidance::GuidanceLaneInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::Guidance::GuidanceLaneInfo*, ABI::Windows::Services::Maps::Guidance::IGuidanceLaneInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Services.Maps.Guidance.GuidanceLaneInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Services::Maps::Guidance::GuidanceLaneInfo*> __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_t;
#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_USE */

#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_USE
#define DEF___FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("875644d8-57a4-59d6-9d2c-5d450d39d2f6"))
IVectorView<ABI::Windows::Services::Maps::Guidance::GuidanceManeuver*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::Guidance::GuidanceManeuver*, ABI::Windows::Services::Maps::Guidance::IGuidanceManeuver*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Services.Maps.Guidance.GuidanceManeuver>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Services::Maps::Guidance::GuidanceManeuver*> __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_t;
#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_USE */

#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_USE
#define DEF___FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f04c7cc2-4d54-5244-beb2-8f4f05c184e6"))
IVectorView<ABI::Windows::Services::Maps::Guidance::GuidanceRoadSegment*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::Guidance::GuidanceRoadSegment*, ABI::Windows::Services::Maps::Guidance::IGuidanceRoadSegment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Services.Maps.Guidance.GuidanceRoadSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Services::Maps::Guidance::GuidanceRoadSegment*> __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_t;
#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_USE */

#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    class GuidanceNavigator;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3f496c35-3dca-5e91-8730-6ef77e9d70bd"))
ITypedEventHandler<ABI::Windows::Services::Maps::Guidance::GuidanceNavigator*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::Guidance::GuidanceNavigator*, ABI::Windows::Services::Maps::Guidance::IGuidanceNavigator*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Services.Maps.Guidance.GuidanceNavigator, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Services::Maps::Guidance::GuidanceNavigator*, IInspectable*> __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable_USE */

#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    class GuidanceAudioNotificationRequestedEventArgs;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("743db36f-e9aa-557a-9fd7-304c9b0499df"))
ITypedEventHandler<ABI::Windows::Services::Maps::Guidance::GuidanceNavigator*, ABI::Windows::Services::Maps::Guidance::GuidanceAudioNotificationRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::Guidance::GuidanceNavigator*, ABI::Windows::Services::Maps::Guidance::IGuidanceNavigator*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::Guidance::GuidanceAudioNotificationRequestedEventArgs*, ABI::Windows::Services::Maps::Guidance::IGuidanceAudioNotificationRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Services.Maps.Guidance.GuidanceNavigator, Windows.Services.Maps.Guidance.GuidanceAudioNotificationRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Services::Maps::Guidance::GuidanceNavigator*, ABI::Windows::Services::Maps::Guidance::GuidanceAudioNotificationRequestedEventArgs*> __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs_USE */

#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    class GuidanceReroutedEventArgs;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("61b92b1b-f22f-581b-bfa0-4868c314c63a"))
ITypedEventHandler<ABI::Windows::Services::Maps::Guidance::GuidanceNavigator*, ABI::Windows::Services::Maps::Guidance::GuidanceReroutedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::Guidance::GuidanceNavigator*, ABI::Windows::Services::Maps::Guidance::IGuidanceNavigator*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::Guidance::GuidanceReroutedEventArgs*, ABI::Windows::Services::Maps::Guidance::IGuidanceReroutedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Services.Maps.Guidance.GuidanceNavigator, Windows.Services.Maps.Guidance.GuidanceReroutedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Services::Maps::Guidance::GuidanceNavigator*, ABI::Windows::Services::Maps::Guidance::GuidanceReroutedEventArgs*> __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs_USE */

#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    class GuidanceUpdatedEventArgs;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("82b3f7df-bf13-5445-aadc-ec61b50fbb46"))
ITypedEventHandler<ABI::Windows::Services::Maps::Guidance::GuidanceNavigator*, ABI::Windows::Services::Maps::Guidance::GuidanceUpdatedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::Guidance::GuidanceNavigator*, ABI::Windows::Services::Maps::Guidance::IGuidanceNavigator*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::Guidance::GuidanceUpdatedEventArgs*, ABI::Windows::Services::Maps::Guidance::IGuidanceUpdatedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Services.Maps.Guidance.GuidanceNavigator, Windows.Services.Maps.Guidance.GuidanceUpdatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Services::Maps::Guidance::GuidanceNavigator*, ABI::Windows::Services::Maps::Guidance::GuidanceUpdatedEventArgs*> __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs_USE */

#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                typedef struct BasicGeoposition BasicGeoposition;
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
                class Geocoordinate;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                class MapRoute;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct Color Color;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    typedef enum GuidanceAudioMeasurementSystem : int GuidanceAudioMeasurementSystem;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    typedef enum GuidanceAudioNotificationKind : int GuidanceAudioNotificationKind;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    typedef enum GuidanceAudioNotifications : unsigned int GuidanceAudioNotifications;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    typedef enum GuidanceLaneMarkers : unsigned int GuidanceLaneMarkers;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    typedef enum GuidanceManeuverKind : int GuidanceManeuverKind;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    typedef enum GuidanceMode : int GuidanceMode;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    class GuidanceMapMatchedCoordinate;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    class GuidanceRoadSignpost;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    class GuidanceRoute;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    class GuidanceTelemetryCollector;
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Services.Maps.Guidance.GuidanceAudioMeasurementSystem
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    enum GuidanceAudioMeasurementSystem : int
                    {
                        GuidanceAudioMeasurementSystem_Meters = 0,
                        GuidanceAudioMeasurementSystem_MilesAndYards = 1,
                        GuidanceAudioMeasurementSystem_MilesAndFeet = 2,
                    };
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.Guidance.GuidanceAudioNotificationKind
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 2.0
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    enum GuidanceAudioNotificationKind : int
                    {
                        GuidanceAudioNotificationKind_Maneuver = 0,
                        GuidanceAudioNotificationKind_Route = 1,
                        GuidanceAudioNotificationKind_Gps = 2,
                        GuidanceAudioNotificationKind_SpeedLimit = 3,
                        GuidanceAudioNotificationKind_Traffic = 4,
                        GuidanceAudioNotificationKind_TrafficCamera = 5,
                    };
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Services.Maps.Guidance.GuidanceAudioNotifications
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    enum GuidanceAudioNotifications : unsigned int
                    {
                        GuidanceAudioNotifications_None = 0,
                        GuidanceAudioNotifications_Maneuver = 0x1,
                        GuidanceAudioNotifications_Route = 0x2,
                        GuidanceAudioNotifications_Gps = 0x4,
                        GuidanceAudioNotifications_SpeedLimit = 0x8,
                        GuidanceAudioNotifications_Traffic = 0x10,
                        GuidanceAudioNotifications_TrafficCamera = 0x20,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(GuidanceAudioNotifications)
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.Guidance.GuidanceLaneMarkers
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    enum GuidanceLaneMarkers : unsigned int
                    {
                        GuidanceLaneMarkers_None = 0,
                        GuidanceLaneMarkers_LightRight = 0x1,
                        GuidanceLaneMarkers_Right = 0x2,
                        GuidanceLaneMarkers_HardRight = 0x4,
                        GuidanceLaneMarkers_Straight = 0x8,
                        GuidanceLaneMarkers_UTurnLeft = 0x10,
                        GuidanceLaneMarkers_HardLeft = 0x20,
                        GuidanceLaneMarkers_Left = 0x40,
                        GuidanceLaneMarkers_LightLeft = 0x80,
                        GuidanceLaneMarkers_UTurnRight = 0x100,
                        GuidanceLaneMarkers_Unknown = 0xffffffff,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(GuidanceLaneMarkers)
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.Guidance.GuidanceManeuverKind
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    enum GuidanceManeuverKind : int
                    {
                        GuidanceManeuverKind_None = 0,
                        GuidanceManeuverKind_GoStraight = 1,
                        GuidanceManeuverKind_UTurnRight = 2,
                        GuidanceManeuverKind_UTurnLeft = 3,
                        GuidanceManeuverKind_TurnKeepRight = 4,
                        GuidanceManeuverKind_TurnLightRight = 5,
                        GuidanceManeuverKind_TurnRight = 6,
                        GuidanceManeuverKind_TurnHardRight = 7,
                        GuidanceManeuverKind_KeepMiddle = 8,
                        GuidanceManeuverKind_TurnKeepLeft = 9,
                        GuidanceManeuverKind_TurnLightLeft = 10,
                        GuidanceManeuverKind_TurnLeft = 11,
                        GuidanceManeuverKind_TurnHardLeft = 12,
                        GuidanceManeuverKind_FreewayEnterRight = 13,
                        GuidanceManeuverKind_FreewayEnterLeft = 14,
                        GuidanceManeuverKind_FreewayLeaveRight = 15,
                        GuidanceManeuverKind_FreewayLeaveLeft = 16,
                        GuidanceManeuverKind_FreewayKeepRight = 17,
                        GuidanceManeuverKind_FreewayKeepLeft = 18,
                        GuidanceManeuverKind_TrafficCircleRight1 = 19,
                        GuidanceManeuverKind_TrafficCircleRight2 = 20,
                        GuidanceManeuverKind_TrafficCircleRight3 = 21,
                        GuidanceManeuverKind_TrafficCircleRight4 = 22,
                        GuidanceManeuverKind_TrafficCircleRight5 = 23,
                        GuidanceManeuverKind_TrafficCircleRight6 = 24,
                        GuidanceManeuverKind_TrafficCircleRight7 = 25,
                        GuidanceManeuverKind_TrafficCircleRight8 = 26,
                        GuidanceManeuverKind_TrafficCircleRight9 = 27,
                        GuidanceManeuverKind_TrafficCircleRight10 = 28,
                        GuidanceManeuverKind_TrafficCircleRight11 = 29,
                        GuidanceManeuverKind_TrafficCircleRight12 = 30,
                        GuidanceManeuverKind_TrafficCircleLeft1 = 31,
                        GuidanceManeuverKind_TrafficCircleLeft2 = 32,
                        GuidanceManeuverKind_TrafficCircleLeft3 = 33,
                        GuidanceManeuverKind_TrafficCircleLeft4 = 34,
                        GuidanceManeuverKind_TrafficCircleLeft5 = 35,
                        GuidanceManeuverKind_TrafficCircleLeft6 = 36,
                        GuidanceManeuverKind_TrafficCircleLeft7 = 37,
                        GuidanceManeuverKind_TrafficCircleLeft8 = 38,
                        GuidanceManeuverKind_TrafficCircleLeft9 = 39,
                        GuidanceManeuverKind_TrafficCircleLeft10 = 40,
                        GuidanceManeuverKind_TrafficCircleLeft11 = 41,
                        GuidanceManeuverKind_TrafficCircleLeft12 = 42,
                        GuidanceManeuverKind_Start = 43,
                        GuidanceManeuverKind_End = 44,
                        GuidanceManeuverKind_TakeFerry = 45,
                        GuidanceManeuverKind_PassTransitStation = 46,
                        GuidanceManeuverKind_LeaveTransitStation = 47,
                    };
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.Guidance.GuidanceMode
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    enum GuidanceMode : int
                    {
                        GuidanceMode_None = 0,
                        GuidanceMode_Simulation = 1,
                        GuidanceMode_Navigation = 2,
                        GuidanceMode_Tracking = 3,
                    };
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceAudioNotificationRequestedEventArgs
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceAudioNotificationRequestedEventArgs
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceAudioNotificationRequestedEventArgs[] = L"Windows.Services.Maps.Guidance.IGuidanceAudioNotificationRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    MIDL_INTERFACE("ca2aa24a-c7c2-4d4c-9d7c-499576bceddb")
                    IGuidanceAudioNotificationRequestedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AudioNotification(
                            ABI::Windows::Services::Maps::Guidance::GuidanceAudioNotificationKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AudioFilePaths(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AudioText(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGuidanceAudioNotificationRequestedEventArgs = __uuidof(IGuidanceAudioNotificationRequestedEventArgs);
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceLaneInfo
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceLaneInfo
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceLaneInfo[] = L"Windows.Services.Maps.Guidance.IGuidanceLaneInfo";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    MIDL_INTERFACE("8404d114-6581-43b7-ac15-c9079bf90df1")
                    IGuidanceLaneInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_LaneMarkers(
                            ABI::Windows::Services::Maps::Guidance::GuidanceLaneMarkers* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsOnRoute(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGuidanceLaneInfo = __uuidof(IGuidanceLaneInfo);
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceManeuver
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceManeuver
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceManeuver[] = L"Windows.Services.Maps.Guidance.IGuidanceManeuver";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    MIDL_INTERFACE("fc09326c-ecc9-4928-a2a1-7232b99b94a1")
                    IGuidanceManeuver : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_StartLocation(
                            ABI::Windows::Devices::Geolocation::IGeopoint** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DistanceFromRouteStart(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DistanceFromPreviousManeuver(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DepartureRoadName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NextRoadName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DepartureShortRoadName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NextShortRoadName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Kind(
                            ABI::Windows::Services::Maps::Guidance::GuidanceManeuverKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StartAngle(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EndAngle(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RoadSignpost(
                            ABI::Windows::Services::Maps::Guidance::IGuidanceRoadSignpost** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_InstructionText(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGuidanceManeuver = __uuidof(IGuidanceManeuver);
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceMapMatchedCoordinate
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceMapMatchedCoordinate
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceMapMatchedCoordinate[] = L"Windows.Services.Maps.Guidance.IGuidanceMapMatchedCoordinate";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    MIDL_INTERFACE("b7acb168-2912-4a99-aff1-798609b981fe")
                    IGuidanceMapMatchedCoordinate : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Location(
                            ABI::Windows::Devices::Geolocation::IGeopoint** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CurrentHeading(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CurrentSpeed(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsOnStreet(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Road(
                            ABI::Windows::Services::Maps::Guidance::IGuidanceRoadSegment** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGuidanceMapMatchedCoordinate = __uuidof(IGuidanceMapMatchedCoordinate);
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceNavigator
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceNavigator
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceNavigator[] = L"Windows.Services.Maps.Guidance.IGuidanceNavigator";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    MIDL_INTERFACE("08f17ef7-8e3f-4d9a-be8a-108f9a012c67")
                    IGuidanceNavigator : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE StartNavigating(
                            ABI::Windows::Services::Maps::Guidance::IGuidanceRoute* route
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE StartSimulating(
                            ABI::Windows::Services::Maps::Guidance::IGuidanceRoute* route,
                            INT32 speedInMetersPerSecond
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE StartTracking(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Pause(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Resume(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RepeatLastAudioNotification(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AudioMeasurementSystem(
                            ABI::Windows::Services::Maps::Guidance::GuidanceAudioMeasurementSystem* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AudioMeasurementSystem(
                            ABI::Windows::Services::Maps::Guidance::GuidanceAudioMeasurementSystem value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AudioNotifications(
                            ABI::Windows::Services::Maps::Guidance::GuidanceAudioNotifications* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AudioNotifications(
                            ABI::Windows::Services::Maps::Guidance::GuidanceAudioNotifications value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_GuidanceUpdated(
                            __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_GuidanceUpdated(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_DestinationReached(
                            __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_DestinationReached(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Rerouting(
                            __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Rerouting(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Rerouted(
                            __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Rerouted(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_RerouteFailed(
                            __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_RerouteFailed(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_UserLocationLost(
                            __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_UserLocationLost(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_UserLocationRestored(
                            __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_UserLocationRestored(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetGuidanceVoice(
                            INT32 voiceId,
                            HSTRING voiceFolder
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UpdateUserLocation(
                            ABI::Windows::Devices::Geolocation::IGeocoordinate* userLocation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UpdateUserLocationWithPositionOverride(
                            ABI::Windows::Devices::Geolocation::IGeocoordinate* userLocation,
                            ABI::Windows::Devices::Geolocation::BasicGeoposition positionOverride
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGuidanceNavigator = __uuidof(IGuidanceNavigator);
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceNavigator2
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceNavigator
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceNavigator2[] = L"Windows.Services.Maps.Guidance.IGuidanceNavigator2";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    MIDL_INTERFACE("6cdc50d1-041c-4bf3-b633-a101fc2f6b57")
                    IGuidanceNavigator2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_AudioNotificationRequested(
                            __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs* value,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_AudioNotificationRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsGuidanceAudioMuted(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsGuidanceAudioMuted(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGuidanceNavigator2 = __uuidof(IGuidanceNavigator2);
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceNavigatorStatics
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceNavigator
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceNavigatorStatics[] = L"Windows.Services.Maps.Guidance.IGuidanceNavigatorStatics";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    MIDL_INTERFACE("00fd9513-4456-4e66-a143-3add6be08426")
                    IGuidanceNavigatorStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetCurrent(
                            ABI::Windows::Services::Maps::Guidance::IGuidanceNavigator** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGuidanceNavigatorStatics = __uuidof(IGuidanceNavigatorStatics);
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceNavigatorStatics2
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceNavigator
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceNavigatorStatics2[] = L"Windows.Services.Maps.Guidance.IGuidanceNavigatorStatics2";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    MIDL_INTERFACE("54c5c3e2-7784-4c85-8c95-d0c6efb43965")
                    IGuidanceNavigatorStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_UseAppProvidedVoice(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGuidanceNavigatorStatics2 = __uuidof(IGuidanceNavigatorStatics2);
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceReroutedEventArgs
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceReroutedEventArgs
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceReroutedEventArgs[] = L"Windows.Services.Maps.Guidance.IGuidanceReroutedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    MIDL_INTERFACE("115d4008-d528-454e-bb94-a50341d2c9f1")
                    IGuidanceReroutedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Route(
                            ABI::Windows::Services::Maps::Guidance::IGuidanceRoute** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGuidanceReroutedEventArgs = __uuidof(IGuidanceReroutedEventArgs);
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceRoadSegment
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceRoadSegment
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceRoadSegment[] = L"Windows.Services.Maps.Guidance.IGuidanceRoadSegment";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    MIDL_INTERFACE("b32758a6-be78-4c63-afe7-6c2957479b3e")
                    IGuidanceRoadSegment : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_RoadName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ShortRoadName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SpeedLimit(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TravelTime(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Path(
                            ABI::Windows::Devices::Geolocation::IGeopath** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsHighway(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsTunnel(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsTollRoad(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGuidanceRoadSegment = __uuidof(IGuidanceRoadSegment);
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceRoadSegment2
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceRoadSegment
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceRoadSegment2[] = L"Windows.Services.Maps.Guidance.IGuidanceRoadSegment2";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    MIDL_INTERFACE("2474a61d-1723-49f1-895b-47a2c4aa9c55")
                    IGuidanceRoadSegment2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsScenic(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGuidanceRoadSegment2 = __uuidof(IGuidanceRoadSegment2);
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceRoadSignpost
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceRoadSignpost
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceRoadSignpost[] = L"Windows.Services.Maps.Guidance.IGuidanceRoadSignpost";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    MIDL_INTERFACE("f1a728b6-f77a-4742-8312-53300f9845f0")
                    IGuidanceRoadSignpost : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ExitNumber(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Exit(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BackgroundColor(
                            ABI::Windows::UI::Color* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ForegroundColor(
                            ABI::Windows::UI::Color* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExitDirections(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGuidanceRoadSignpost = __uuidof(IGuidanceRoadSignpost);
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceRoute
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceRoute
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceRoute[] = L"Windows.Services.Maps.Guidance.IGuidanceRoute";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    MIDL_INTERFACE("3a14545d-801a-40bd-a286-afb2010cce6c")
                    IGuidanceRoute : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Duration(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Distance(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Maneuvers(
                            __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BoundingBox(
                            ABI::Windows::Devices::Geolocation::IGeoboundingBox** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Path(
                            ABI::Windows::Devices::Geolocation::IGeopath** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RoadSegments(
                            __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ConvertToMapRoute(
                            ABI::Windows::Services::Maps::IMapRoute** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGuidanceRoute = __uuidof(IGuidanceRoute);
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceRouteStatics
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceRoute
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceRouteStatics[] = L"Windows.Services.Maps.Guidance.IGuidanceRouteStatics";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    MIDL_INTERFACE("f56d926a-55ed-49c1-b09c-4b8223b50db3")
                    IGuidanceRouteStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CanCreateFromMapRoute(
                            ABI::Windows::Services::Maps::IMapRoute* mapRoute,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryCreateFromMapRoute(
                            ABI::Windows::Services::Maps::IMapRoute* mapRoute,
                            ABI::Windows::Services::Maps::Guidance::IGuidanceRoute** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGuidanceRouteStatics = __uuidof(IGuidanceRouteStatics);
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceTelemetryCollector
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceTelemetryCollector
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceTelemetryCollector[] = L"Windows.Services.Maps.Guidance.IGuidanceTelemetryCollector";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    MIDL_INTERFACE("db1f8da5-b878-4d92-98dd-347d23d38262")
                    IGuidanceTelemetryCollector : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Enabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Enabled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ClearLocalData(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SpeedTrigger(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SpeedTrigger(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UploadFrequency(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_UploadFrequency(
                            INT32 value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGuidanceTelemetryCollector = __uuidof(IGuidanceTelemetryCollector);
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceTelemetryCollectorStatics
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceTelemetryCollector
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceTelemetryCollectorStatics[] = L"Windows.Services.Maps.Guidance.IGuidanceTelemetryCollectorStatics";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    MIDL_INTERFACE("36532047-f160-44fb-b578-94577ca05990")
                    IGuidanceTelemetryCollectorStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetCurrent(
                            ABI::Windows::Services::Maps::Guidance::IGuidanceTelemetryCollector** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGuidanceTelemetryCollectorStatics = __uuidof(IGuidanceTelemetryCollectorStatics);
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceUpdatedEventArgs
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceUpdatedEventArgs
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceUpdatedEventArgs[] = L"Windows.Services.Maps.Guidance.IGuidanceUpdatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace Guidance {
                    MIDL_INTERFACE("fdac160b-9e8d-4de3-a9fa-b06321d18db9")
                    IGuidanceUpdatedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Mode(
                            ABI::Windows::Services::Maps::Guidance::GuidanceMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NextManeuver(
                            ABI::Windows::Services::Maps::Guidance::IGuidanceManeuver** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NextManeuverDistance(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AfterNextManeuver(
                            ABI::Windows::Services::Maps::Guidance::IGuidanceManeuver** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AfterNextManeuverDistance(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DistanceToDestination(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ElapsedDistance(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ElapsedTime(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TimeToDestination(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RoadName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Route(
                            ABI::Windows::Services::Maps::Guidance::IGuidanceRoute** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CurrentLocation(
                            ABI::Windows::Services::Maps::Guidance::IGuidanceMapMatchedCoordinate** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsNewManeuver(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LaneInfo(
                            __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGuidanceUpdatedEventArgs = __uuidof(IGuidanceUpdatedEventArgs);
                } /* Guidance */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceAudioNotificationRequestedEventArgs
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceAudioNotificationRequestedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceAudioNotificationRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceAudioNotificationRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceAudioNotificationRequestedEventArgs[] = L"Windows.Services.Maps.Guidance.GuidanceAudioNotificationRequestedEventArgs";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceLaneInfo
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceLaneInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceLaneInfo_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceLaneInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceLaneInfo[] = L"Windows.Services.Maps.Guidance.GuidanceLaneInfo";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceManeuver
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceManeuver ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceManeuver_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceManeuver_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceManeuver[] = L"Windows.Services.Maps.Guidance.GuidanceManeuver";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceMapMatchedCoordinate
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceMapMatchedCoordinate ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceMapMatchedCoordinate_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceMapMatchedCoordinate_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceMapMatchedCoordinate[] = L"Windows.Services.Maps.Guidance.GuidanceMapMatchedCoordinate";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceNavigator
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.Guidance.IGuidanceNavigatorStatics interface starting with version 1.0 of the Windows.Services.Maps.GuidanceContract API contract
 *   Static Methods exist on the Windows.Services.Maps.Guidance.IGuidanceNavigatorStatics2 interface starting with version 2.0 of the Windows.Services.Maps.GuidanceContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceNavigator ** Default Interface **
 *    Windows.Services.Maps.Guidance.IGuidanceNavigator2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceNavigator_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceNavigator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceNavigator[] = L"Windows.Services.Maps.Guidance.GuidanceNavigator";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceReroutedEventArgs
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceReroutedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceReroutedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceReroutedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceReroutedEventArgs[] = L"Windows.Services.Maps.Guidance.GuidanceReroutedEventArgs";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceRoadSegment
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceRoadSegment ** Default Interface **
 *    Windows.Services.Maps.Guidance.IGuidanceRoadSegment2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceRoadSegment_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceRoadSegment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceRoadSegment[] = L"Windows.Services.Maps.Guidance.GuidanceRoadSegment";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceRoadSignpost
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceRoadSignpost ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceRoadSignpost_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceRoadSignpost_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceRoadSignpost[] = L"Windows.Services.Maps.Guidance.GuidanceRoadSignpost";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceRoute
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.Guidance.IGuidanceRouteStatics interface starting with version 1.0 of the Windows.Services.Maps.GuidanceContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceRoute ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceRoute_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceRoute_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceRoute[] = L"Windows.Services.Maps.Guidance.GuidanceRoute";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceTelemetryCollector
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.Guidance.IGuidanceTelemetryCollectorStatics interface starting with version 1.0 of the Windows.Services.Maps.GuidanceContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceTelemetryCollector ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceTelemetryCollector_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceTelemetryCollector_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceTelemetryCollector[] = L"Windows.Services.Maps.Guidance.GuidanceTelemetryCollector";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceUpdatedEventArgs
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceUpdatedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceUpdatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceUpdatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceUpdatedEventArgs[] = L"Windows.Services.Maps.Guidance.GuidanceUpdatedEventArgs";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs;

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo;

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver;

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate;

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator;

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2 __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2;

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics;

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2 __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2;

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs;

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment;

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2 __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2;

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost;

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute;

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics;

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector;

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics;

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs;

#endif // ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_FWD_DEFINED__

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

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo;

typedef struct __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfoVtbl;

interface __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo;

typedef struct __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfoVtbl;

interface __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver;

typedef struct __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuverVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuverVtbl;

interface __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver
{
    CONST_VTBL struct __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuverVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver;

typedef struct __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuverVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver** result);

    END_INTERFACE
} __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuverVtbl;

interface __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver
{
    CONST_VTBL struct __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuverVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment;

typedef struct __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegmentVtbl;

interface __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment
{
    CONST_VTBL struct __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment;

typedef struct __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        __FIIterator_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment** result);

    END_INTERFACE
} __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegmentVtbl;

interface __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment
{
    CONST_VTBL struct __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

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

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo;

typedef struct __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfoVtbl;

interface __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver;

typedef struct __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuverVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        UINT32 index,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuverVtbl;

interface __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver
{
    CONST_VTBL struct __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuverVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment;

typedef struct __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        UINT32 index,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegmentVtbl;

interface __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment
{
    CONST_VTBL struct __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* sender,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* sender,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* sender,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition;

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopath_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapRoute_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapRoute_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapRoute __x_ABI_CWindows_CServices_CMaps_CIMapRoute;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapRoute_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

typedef enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceAudioMeasurementSystem __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceAudioMeasurementSystem;

typedef enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceAudioNotificationKind __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceAudioNotificationKind;

typedef enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceAudioNotifications __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceAudioNotifications;

typedef enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceLaneMarkers __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceLaneMarkers;

typedef enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceManeuverKind __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceManeuverKind;

typedef enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceMode __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceMode;

/*
 *
 * Struct Windows.Services.Maps.Guidance.GuidanceAudioMeasurementSystem
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceAudioMeasurementSystem
{
    GuidanceAudioMeasurementSystem_Meters = 0,
    GuidanceAudioMeasurementSystem_MilesAndYards = 1,
    GuidanceAudioMeasurementSystem_MilesAndFeet = 2,
};
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.Guidance.GuidanceAudioNotificationKind
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 2.0
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceAudioNotificationKind
{
    GuidanceAudioNotificationKind_Maneuver = 0,
    GuidanceAudioNotificationKind_Route = 1,
    GuidanceAudioNotificationKind_Gps = 2,
    GuidanceAudioNotificationKind_SpeedLimit = 3,
    GuidanceAudioNotificationKind_Traffic = 4,
    GuidanceAudioNotificationKind_TrafficCamera = 5,
};
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Services.Maps.Guidance.GuidanceAudioNotifications
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceAudioNotifications
{
    GuidanceAudioNotifications_None = 0,
    GuidanceAudioNotifications_Maneuver = 0x1,
    GuidanceAudioNotifications_Route = 0x2,
    GuidanceAudioNotifications_Gps = 0x4,
    GuidanceAudioNotifications_SpeedLimit = 0x8,
    GuidanceAudioNotifications_Traffic = 0x10,
    GuidanceAudioNotifications_TrafficCamera = 0x20,
};
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.Guidance.GuidanceLaneMarkers
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceLaneMarkers
{
    GuidanceLaneMarkers_None = 0,
    GuidanceLaneMarkers_LightRight = 0x1,
    GuidanceLaneMarkers_Right = 0x2,
    GuidanceLaneMarkers_HardRight = 0x4,
    GuidanceLaneMarkers_Straight = 0x8,
    GuidanceLaneMarkers_UTurnLeft = 0x10,
    GuidanceLaneMarkers_HardLeft = 0x20,
    GuidanceLaneMarkers_Left = 0x40,
    GuidanceLaneMarkers_LightLeft = 0x80,
    GuidanceLaneMarkers_UTurnRight = 0x100,
    GuidanceLaneMarkers_Unknown = 0xffffffff,
};
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.Guidance.GuidanceManeuverKind
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceManeuverKind
{
    GuidanceManeuverKind_None = 0,
    GuidanceManeuverKind_GoStraight = 1,
    GuidanceManeuverKind_UTurnRight = 2,
    GuidanceManeuverKind_UTurnLeft = 3,
    GuidanceManeuverKind_TurnKeepRight = 4,
    GuidanceManeuverKind_TurnLightRight = 5,
    GuidanceManeuverKind_TurnRight = 6,
    GuidanceManeuverKind_TurnHardRight = 7,
    GuidanceManeuverKind_KeepMiddle = 8,
    GuidanceManeuverKind_TurnKeepLeft = 9,
    GuidanceManeuverKind_TurnLightLeft = 10,
    GuidanceManeuverKind_TurnLeft = 11,
    GuidanceManeuverKind_TurnHardLeft = 12,
    GuidanceManeuverKind_FreewayEnterRight = 13,
    GuidanceManeuverKind_FreewayEnterLeft = 14,
    GuidanceManeuverKind_FreewayLeaveRight = 15,
    GuidanceManeuverKind_FreewayLeaveLeft = 16,
    GuidanceManeuverKind_FreewayKeepRight = 17,
    GuidanceManeuverKind_FreewayKeepLeft = 18,
    GuidanceManeuverKind_TrafficCircleRight1 = 19,
    GuidanceManeuverKind_TrafficCircleRight2 = 20,
    GuidanceManeuverKind_TrafficCircleRight3 = 21,
    GuidanceManeuverKind_TrafficCircleRight4 = 22,
    GuidanceManeuverKind_TrafficCircleRight5 = 23,
    GuidanceManeuverKind_TrafficCircleRight6 = 24,
    GuidanceManeuverKind_TrafficCircleRight7 = 25,
    GuidanceManeuverKind_TrafficCircleRight8 = 26,
    GuidanceManeuverKind_TrafficCircleRight9 = 27,
    GuidanceManeuverKind_TrafficCircleRight10 = 28,
    GuidanceManeuverKind_TrafficCircleRight11 = 29,
    GuidanceManeuverKind_TrafficCircleRight12 = 30,
    GuidanceManeuverKind_TrafficCircleLeft1 = 31,
    GuidanceManeuverKind_TrafficCircleLeft2 = 32,
    GuidanceManeuverKind_TrafficCircleLeft3 = 33,
    GuidanceManeuverKind_TrafficCircleLeft4 = 34,
    GuidanceManeuverKind_TrafficCircleLeft5 = 35,
    GuidanceManeuverKind_TrafficCircleLeft6 = 36,
    GuidanceManeuverKind_TrafficCircleLeft7 = 37,
    GuidanceManeuverKind_TrafficCircleLeft8 = 38,
    GuidanceManeuverKind_TrafficCircleLeft9 = 39,
    GuidanceManeuverKind_TrafficCircleLeft10 = 40,
    GuidanceManeuverKind_TrafficCircleLeft11 = 41,
    GuidanceManeuverKind_TrafficCircleLeft12 = 42,
    GuidanceManeuverKind_Start = 43,
    GuidanceManeuverKind_End = 44,
    GuidanceManeuverKind_TakeFerry = 45,
    GuidanceManeuverKind_PassTransitStation = 46,
    GuidanceManeuverKind_LeaveTransitStation = 47,
};
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.Maps.Guidance.GuidanceMode
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceMode
{
    GuidanceMode_None = 0,
    GuidanceMode_Simulation = 1,
    GuidanceMode_Navigation = 2,
    GuidanceMode_Tracking = 3,
};
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceAudioNotificationRequestedEventArgs
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceAudioNotificationRequestedEventArgs
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceAudioNotificationRequestedEventArgs[] = L"Windows.Services.Maps.Guidance.IGuidanceAudioNotificationRequestedEventArgs";
typedef struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AudioNotification)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs* This,
        enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceAudioNotificationKind* value);
    HRESULT (STDMETHODCALLTYPE* get_AudioFilePaths)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_AudioText)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_get_AudioNotification(This, value) \
    ((This)->lpVtbl->get_AudioNotification(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_get_AudioFilePaths(This, value) \
    ((This)->lpVtbl->get_AudioFilePaths(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_get_AudioText(This, value) \
    ((This)->lpVtbl->get_AudioText(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceAudioNotificationRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceLaneInfo
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceLaneInfo
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceLaneInfo[] = L"Windows.Services.Maps.Guidance.IGuidanceLaneInfo";
typedef struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LaneMarkers)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo* This,
        enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceLaneMarkers* value);
    HRESULT (STDMETHODCALLTYPE* get_IsOnRoute)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfoVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_get_LaneMarkers(This, value) \
    ((This)->lpVtbl->get_LaneMarkers(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_get_IsOnRoute(This, value) \
    ((This)->lpVtbl->get_IsOnRoute(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceLaneInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceManeuver
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceManeuver
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceManeuver[] = L"Windows.Services.Maps.Guidance.IGuidanceManeuver";
typedef struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuverVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_StartLocation)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint** value);
    HRESULT (STDMETHODCALLTYPE* get_DistanceFromRouteStart)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_DistanceFromPreviousManeuver)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_DepartureRoadName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NextRoadName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DepartureShortRoadName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NextShortRoadName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* This,
        enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceManeuverKind* value);
    HRESULT (STDMETHODCALLTYPE* get_StartAngle)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_EndAngle)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_RoadSignpost)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost** value);
    HRESULT (STDMETHODCALLTYPE* get_InstructionText)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuverVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuverVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_get_StartLocation(This, value) \
    ((This)->lpVtbl->get_StartLocation(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_get_DistanceFromRouteStart(This, value) \
    ((This)->lpVtbl->get_DistanceFromRouteStart(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_get_DistanceFromPreviousManeuver(This, value) \
    ((This)->lpVtbl->get_DistanceFromPreviousManeuver(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_get_DepartureRoadName(This, value) \
    ((This)->lpVtbl->get_DepartureRoadName(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_get_NextRoadName(This, value) \
    ((This)->lpVtbl->get_NextRoadName(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_get_DepartureShortRoadName(This, value) \
    ((This)->lpVtbl->get_DepartureShortRoadName(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_get_NextShortRoadName(This, value) \
    ((This)->lpVtbl->get_NextShortRoadName(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_get_StartAngle(This, value) \
    ((This)->lpVtbl->get_StartAngle(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_get_EndAngle(This, value) \
    ((This)->lpVtbl->get_EndAngle(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_get_RoadSignpost(This, value) \
    ((This)->lpVtbl->get_RoadSignpost(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_get_InstructionText(This, value) \
    ((This)->lpVtbl->get_InstructionText(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceMapMatchedCoordinate
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceMapMatchedCoordinate
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceMapMatchedCoordinate[] = L"Windows.Services.Maps.Guidance.IGuidanceMapMatchedCoordinate";
typedef struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Location)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint** value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentHeading)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentSpeed)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_IsOnStreet)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Road)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinateVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_get_Location(This, value) \
    ((This)->lpVtbl->get_Location(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_get_CurrentHeading(This, value) \
    ((This)->lpVtbl->get_CurrentHeading(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_get_CurrentSpeed(This, value) \
    ((This)->lpVtbl->get_CurrentSpeed(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_get_IsOnStreet(This, value) \
    ((This)->lpVtbl->get_IsOnStreet(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_get_Road(This, value) \
    ((This)->lpVtbl->get_Road(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceNavigator
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceNavigator
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceNavigator[] = L"Windows.Services.Maps.Guidance.IGuidanceNavigator";
typedef struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* StartNavigating)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute* route);
    HRESULT (STDMETHODCALLTYPE* StartSimulating)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute* route,
        INT32 speedInMetersPerSecond);
    HRESULT (STDMETHODCALLTYPE* StartTracking)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This);
    HRESULT (STDMETHODCALLTYPE* Pause)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This);
    HRESULT (STDMETHODCALLTYPE* Resume)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This);
    HRESULT (STDMETHODCALLTYPE* RepeatLastAudioNotification)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This);
    HRESULT (STDMETHODCALLTYPE* get_AudioMeasurementSystem)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceAudioMeasurementSystem* value);
    HRESULT (STDMETHODCALLTYPE* put_AudioMeasurementSystem)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceAudioMeasurementSystem value);
    HRESULT (STDMETHODCALLTYPE* get_AudioNotifications)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceAudioNotifications* value);
    HRESULT (STDMETHODCALLTYPE* put_AudioNotifications)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceAudioNotifications value);
    HRESULT (STDMETHODCALLTYPE* add_GuidanceUpdated)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceUpdatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_GuidanceUpdated)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_DestinationReached)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DestinationReached)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Rerouting)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Rerouting)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Rerouted)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceReroutedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Rerouted)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_RerouteFailed)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RerouteFailed)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_UserLocationLost)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_UserLocationLost)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_UserLocationRestored)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_UserLocationRestored)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* SetGuidanceVoice)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        INT32 voiceId,
        HSTRING voiceFolder);
    HRESULT (STDMETHODCALLTYPE* UpdateUserLocation)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate* userLocation);
    HRESULT (STDMETHODCALLTYPE* UpdateUserLocationWithPositionOverride)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeocoordinate* userLocation,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition positionOverride);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_StartNavigating(This, route) \
    ((This)->lpVtbl->StartNavigating(This, route))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_StartSimulating(This, route, speedInMetersPerSecond) \
    ((This)->lpVtbl->StartSimulating(This, route, speedInMetersPerSecond))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_StartTracking(This) \
    ((This)->lpVtbl->StartTracking(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_Pause(This) \
    ((This)->lpVtbl->Pause(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_Resume(This) \
    ((This)->lpVtbl->Resume(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_RepeatLastAudioNotification(This) \
    ((This)->lpVtbl->RepeatLastAudioNotification(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_get_AudioMeasurementSystem(This, value) \
    ((This)->lpVtbl->get_AudioMeasurementSystem(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_put_AudioMeasurementSystem(This, value) \
    ((This)->lpVtbl->put_AudioMeasurementSystem(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_get_AudioNotifications(This, value) \
    ((This)->lpVtbl->get_AudioNotifications(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_put_AudioNotifications(This, value) \
    ((This)->lpVtbl->put_AudioNotifications(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_add_GuidanceUpdated(This, handler, token) \
    ((This)->lpVtbl->add_GuidanceUpdated(This, handler, token))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_remove_GuidanceUpdated(This, token) \
    ((This)->lpVtbl->remove_GuidanceUpdated(This, token))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_add_DestinationReached(This, handler, token) \
    ((This)->lpVtbl->add_DestinationReached(This, handler, token))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_remove_DestinationReached(This, token) \
    ((This)->lpVtbl->remove_DestinationReached(This, token))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_add_Rerouting(This, handler, token) \
    ((This)->lpVtbl->add_Rerouting(This, handler, token))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_remove_Rerouting(This, token) \
    ((This)->lpVtbl->remove_Rerouting(This, token))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_add_Rerouted(This, handler, token) \
    ((This)->lpVtbl->add_Rerouted(This, handler, token))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_remove_Rerouted(This, token) \
    ((This)->lpVtbl->remove_Rerouted(This, token))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_add_RerouteFailed(This, handler, token) \
    ((This)->lpVtbl->add_RerouteFailed(This, handler, token))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_remove_RerouteFailed(This, token) \
    ((This)->lpVtbl->remove_RerouteFailed(This, token))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_add_UserLocationLost(This, handler, token) \
    ((This)->lpVtbl->add_UserLocationLost(This, handler, token))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_remove_UserLocationLost(This, token) \
    ((This)->lpVtbl->remove_UserLocationLost(This, token))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_add_UserLocationRestored(This, handler, token) \
    ((This)->lpVtbl->add_UserLocationRestored(This, handler, token))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_remove_UserLocationRestored(This, token) \
    ((This)->lpVtbl->remove_UserLocationRestored(This, token))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_SetGuidanceVoice(This, voiceId, voiceFolder) \
    ((This)->lpVtbl->SetGuidanceVoice(This, voiceId, voiceFolder))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_UpdateUserLocation(This, userLocation) \
    ((This)->lpVtbl->UpdateUserLocation(This, userLocation))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_UpdateUserLocationWithPositionOverride(This, userLocation, positionOverride) \
    ((This)->lpVtbl->UpdateUserLocationWithPositionOverride(This, userLocation, positionOverride))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceNavigator2
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceNavigator
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceNavigator2[] = L"Windows.Services.Maps.Guidance.IGuidanceNavigator2";
typedef struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_AudioNotificationRequested)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2* This,
        __FITypedEventHandler_2_Windows__CServices__CMaps__CGuidance__CGuidanceNavigator_Windows__CServices__CMaps__CGuidance__CGuidanceAudioNotificationRequestedEventArgs* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AudioNotificationRequested)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_IsGuidanceAudioMuted)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsGuidanceAudioMuted)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_add_AudioNotificationRequested(This, value, token) \
    ((This)->lpVtbl->add_AudioNotificationRequested(This, value, token))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_remove_AudioNotificationRequested(This, token) \
    ((This)->lpVtbl->remove_AudioNotificationRequested(This, token))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_get_IsGuidanceAudioMuted(This, value) \
    ((This)->lpVtbl->get_IsGuidanceAudioMuted(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_put_IsGuidanceAudioMuted(This, value) \
    ((This)->lpVtbl->put_IsGuidanceAudioMuted(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceNavigatorStatics
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceNavigator
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceNavigatorStatics[] = L"Windows.Services.Maps.Guidance.IGuidanceNavigatorStatics";
typedef struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCurrent)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigator** result);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStaticsVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_GetCurrent(This, result) \
    ((This)->lpVtbl->GetCurrent(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceNavigatorStatics2
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceNavigator
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceNavigatorStatics2[] = L"Windows.Services.Maps.Guidance.IGuidanceNavigatorStatics2";
typedef struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UseAppProvidedVoice)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_get_UseAppProvidedVoice(This, value) \
    ((This)->lpVtbl->get_UseAppProvidedVoice(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceNavigatorStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceReroutedEventArgs
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceReroutedEventArgs
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceReroutedEventArgs[] = L"Windows.Services.Maps.Guidance.IGuidanceReroutedEventArgs";
typedef struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Route)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute** result);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgsVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_get_Route(This, result) \
    ((This)->lpVtbl->get_Route(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceReroutedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceRoadSegment
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceRoadSegment
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceRoadSegment[] = L"Windows.Services.Maps.Guidance.IGuidanceRoadSegment";
typedef struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RoadName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ShortRoadName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SpeedLimit)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_TravelTime)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Path)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath** value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsHighway)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsTunnel)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsTollRoad)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegmentVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_get_RoadName(This, value) \
    ((This)->lpVtbl->get_RoadName(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_get_ShortRoadName(This, value) \
    ((This)->lpVtbl->get_ShortRoadName(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_get_SpeedLimit(This, value) \
    ((This)->lpVtbl->get_SpeedLimit(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_get_TravelTime(This, value) \
    ((This)->lpVtbl->get_TravelTime(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_get_Path(This, value) \
    ((This)->lpVtbl->get_Path(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_get_IsHighway(This, value) \
    ((This)->lpVtbl->get_IsHighway(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_get_IsTunnel(This, value) \
    ((This)->lpVtbl->get_IsTunnel(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_get_IsTollRoad(This, value) \
    ((This)->lpVtbl->get_IsTollRoad(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceRoadSegment2
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceRoadSegment
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceRoadSegment2[] = L"Windows.Services.Maps.Guidance.IGuidanceRoadSegment2";
typedef struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsScenic)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_get_IsScenic(This, value) \
    ((This)->lpVtbl->get_IsScenic(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSegment2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceRoadSignpost
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceRoadSignpost
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceRoadSignpost[] = L"Windows.Services.Maps.Guidance.IGuidanceRoadSignpost";
typedef struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpostVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExitNumber)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Exit)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BackgroundColor)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ForegroundColor)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ExitDirections)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost* This,
        __FIVectorView_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpostVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpostVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_get_ExitNumber(This, value) \
    ((This)->lpVtbl->get_ExitNumber(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_get_Exit(This, value) \
    ((This)->lpVtbl->get_Exit(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_get_BackgroundColor(This, value) \
    ((This)->lpVtbl->get_BackgroundColor(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_get_ForegroundColor(This, value) \
    ((This)->lpVtbl->get_ForegroundColor(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_get_ExitDirections(This, value) \
    ((This)->lpVtbl->get_ExitDirections(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoadSignpost_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceRoute
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceRoute
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceRoute[] = L"Windows.Services.Maps.Guidance.IGuidanceRoute";
typedef struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Distance)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Maneuvers)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute* This,
        __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceManeuver** value);
    HRESULT (STDMETHODCALLTYPE* get_BoundingBox)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox** value);
    HRESULT (STDMETHODCALLTYPE* get_Path)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopath** value);
    HRESULT (STDMETHODCALLTYPE* get_RoadSegments)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute* This,
        __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceRoadSegment** value);
    HRESULT (STDMETHODCALLTYPE* ConvertToMapRoute)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute* This,
        __x_ABI_CWindows_CServices_CMaps_CIMapRoute** result);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_get_Distance(This, value) \
    ((This)->lpVtbl->get_Distance(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_get_Maneuvers(This, value) \
    ((This)->lpVtbl->get_Maneuvers(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_get_BoundingBox(This, value) \
    ((This)->lpVtbl->get_BoundingBox(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_get_Path(This, value) \
    ((This)->lpVtbl->get_Path(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_get_RoadSegments(This, value) \
    ((This)->lpVtbl->get_RoadSegments(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_ConvertToMapRoute(This, result) \
    ((This)->lpVtbl->ConvertToMapRoute(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceRouteStatics
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceRoute
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceRouteStatics[] = L"Windows.Services.Maps.Guidance.IGuidanceRouteStatics";
typedef struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CanCreateFromMapRoute)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics* This,
        __x_ABI_CWindows_CServices_CMaps_CIMapRoute* mapRoute,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* TryCreateFromMapRoute)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics* This,
        __x_ABI_CWindows_CServices_CMaps_CIMapRoute* mapRoute,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute** result);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStaticsVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_CanCreateFromMapRoute(This, mapRoute, result) \
    ((This)->lpVtbl->CanCreateFromMapRoute(This, mapRoute, result))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_TryCreateFromMapRoute(This, mapRoute, result) \
    ((This)->lpVtbl->TryCreateFromMapRoute(This, mapRoute, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRouteStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceTelemetryCollector
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceTelemetryCollector
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceTelemetryCollector[] = L"Windows.Services.Maps.Guidance.IGuidanceTelemetryCollector";
typedef struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Enabled)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Enabled)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* ClearLocalData)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector* This);
    HRESULT (STDMETHODCALLTYPE* get_SpeedTrigger)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_SpeedTrigger)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_UploadFrequency)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_UploadFrequency)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector* This,
        INT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_get_Enabled(This, value) \
    ((This)->lpVtbl->get_Enabled(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_put_Enabled(This, value) \
    ((This)->lpVtbl->put_Enabled(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_ClearLocalData(This) \
    ((This)->lpVtbl->ClearLocalData(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_get_SpeedTrigger(This, value) \
    ((This)->lpVtbl->get_SpeedTrigger(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_put_SpeedTrigger(This, value) \
    ((This)->lpVtbl->put_SpeedTrigger(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_get_UploadFrequency(This, value) \
    ((This)->lpVtbl->get_UploadFrequency(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_put_UploadFrequency(This, value) \
    ((This)->lpVtbl->put_UploadFrequency(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceTelemetryCollectorStatics
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceTelemetryCollector
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceTelemetryCollectorStatics[] = L"Windows.Services.Maps.Guidance.IGuidanceTelemetryCollectorStatics";
typedef struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCurrent)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollector** result);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStaticsVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_GetCurrent(This, result) \
    ((This)->lpVtbl->GetCurrent(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceTelemetryCollectorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.Guidance.IGuidanceUpdatedEventArgs
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.Guidance.GuidanceUpdatedEventArgs
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_Guidance_IGuidanceUpdatedEventArgs[] = L"Windows.Services.Maps.Guidance.IGuidanceUpdatedEventArgs";
typedef struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Mode)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This,
        enum __x_ABI_CWindows_CServices_CMaps_CGuidance_CGuidanceMode* value);
    HRESULT (STDMETHODCALLTYPE* get_NextManeuver)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver** value);
    HRESULT (STDMETHODCALLTYPE* get_NextManeuverDistance)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_AfterNextManeuver)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceManeuver** value);
    HRESULT (STDMETHODCALLTYPE* get_AfterNextManeuverDistance)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_DistanceToDestination)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ElapsedDistance)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ElapsedTime)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_TimeToDestination)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_RoadName)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Route)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceRoute** result);
    HRESULT (STDMETHODCALLTYPE* get_CurrentLocation)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This,
        __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceMapMatchedCoordinate** result);
    HRESULT (STDMETHODCALLTYPE* get_IsNewManeuver)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_LaneInfo)(__x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs* This,
        __FIVectorView_1_Windows__CServices__CMaps__CGuidance__CGuidanceLaneInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgsVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_get_Mode(This, value) \
    ((This)->lpVtbl->get_Mode(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_get_NextManeuver(This, value) \
    ((This)->lpVtbl->get_NextManeuver(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_get_NextManeuverDistance(This, value) \
    ((This)->lpVtbl->get_NextManeuverDistance(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_get_AfterNextManeuver(This, value) \
    ((This)->lpVtbl->get_AfterNextManeuver(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_get_AfterNextManeuverDistance(This, value) \
    ((This)->lpVtbl->get_AfterNextManeuverDistance(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_get_DistanceToDestination(This, value) \
    ((This)->lpVtbl->get_DistanceToDestination(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_get_ElapsedDistance(This, value) \
    ((This)->lpVtbl->get_ElapsedDistance(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_get_ElapsedTime(This, value) \
    ((This)->lpVtbl->get_ElapsedTime(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_get_TimeToDestination(This, value) \
    ((This)->lpVtbl->get_TimeToDestination(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_get_RoadName(This, value) \
    ((This)->lpVtbl->get_RoadName(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_get_Route(This, result) \
    ((This)->lpVtbl->get_Route(This, result))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_get_CurrentLocation(This, result) \
    ((This)->lpVtbl->get_CurrentLocation(This, result))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_get_IsNewManeuver(This, value) \
    ((This)->lpVtbl->get_IsNewManeuver(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_get_LaneInfo(This, value) \
    ((This)->lpVtbl->get_LaneInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CGuidance_CIGuidanceUpdatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceAudioNotificationRequestedEventArgs
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceAudioNotificationRequestedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceAudioNotificationRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceAudioNotificationRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceAudioNotificationRequestedEventArgs[] = L"Windows.Services.Maps.Guidance.GuidanceAudioNotificationRequestedEventArgs";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceLaneInfo
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceLaneInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceLaneInfo_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceLaneInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceLaneInfo[] = L"Windows.Services.Maps.Guidance.GuidanceLaneInfo";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceManeuver
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceManeuver ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceManeuver_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceManeuver_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceManeuver[] = L"Windows.Services.Maps.Guidance.GuidanceManeuver";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceMapMatchedCoordinate
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceMapMatchedCoordinate ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceMapMatchedCoordinate_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceMapMatchedCoordinate_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceMapMatchedCoordinate[] = L"Windows.Services.Maps.Guidance.GuidanceMapMatchedCoordinate";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceNavigator
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.Guidance.IGuidanceNavigatorStatics interface starting with version 1.0 of the Windows.Services.Maps.GuidanceContract API contract
 *   Static Methods exist on the Windows.Services.Maps.Guidance.IGuidanceNavigatorStatics2 interface starting with version 2.0 of the Windows.Services.Maps.GuidanceContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceNavigator ** Default Interface **
 *    Windows.Services.Maps.Guidance.IGuidanceNavigator2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceNavigator_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceNavigator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceNavigator[] = L"Windows.Services.Maps.Guidance.GuidanceNavigator";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceReroutedEventArgs
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceReroutedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceReroutedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceReroutedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceReroutedEventArgs[] = L"Windows.Services.Maps.Guidance.GuidanceReroutedEventArgs";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceRoadSegment
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceRoadSegment ** Default Interface **
 *    Windows.Services.Maps.Guidance.IGuidanceRoadSegment2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceRoadSegment_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceRoadSegment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceRoadSegment[] = L"Windows.Services.Maps.Guidance.GuidanceRoadSegment";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceRoadSignpost
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceRoadSignpost ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceRoadSignpost_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceRoadSignpost_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceRoadSignpost[] = L"Windows.Services.Maps.Guidance.GuidanceRoadSignpost";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceRoute
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.Guidance.IGuidanceRouteStatics interface starting with version 1.0 of the Windows.Services.Maps.GuidanceContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceRoute ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceRoute_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceRoute_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceRoute[] = L"Windows.Services.Maps.Guidance.GuidanceRoute";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceTelemetryCollector
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.Guidance.IGuidanceTelemetryCollectorStatics interface starting with version 1.0 of the Windows.Services.Maps.GuidanceContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceTelemetryCollector ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceTelemetryCollector_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceTelemetryCollector_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceTelemetryCollector[] = L"Windows.Services.Maps.Guidance.GuidanceTelemetryCollector";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.Guidance.GuidanceUpdatedEventArgs
 *
 * Introduced to Windows.Services.Maps.GuidanceContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.Guidance.IGuidanceUpdatedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceUpdatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_Guidance_GuidanceUpdatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_Guidance_GuidanceUpdatedEventArgs[] = L"Windows.Services.Maps.Guidance.GuidanceUpdatedEventArgs";
#endif
#endif // WINDOWS_SERVICES_MAPS_GUIDANCECONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eservices2Emaps2Eguidance_p_h__

#endif // __windows2Eservices2Emaps2Eguidance_h__
