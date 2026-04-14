
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
#ifndef __windows2Edevices2Egeolocation2Egeofencing_h__
#define __windows2Edevices2Egeolocation2Egeofencing_h__
#ifndef __windows2Edevices2Egeolocation2Egeofencing_p_h__
#define __windows2Edevices2Egeolocation2Egeofencing_p_h__


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
#include "Windows.Devices.Geolocation.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    interface IGeofence;
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence ABI::Windows::Devices::Geolocation::Geofencing::IGeofence

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    interface IGeofenceFactory;
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory ABI::Windows::Devices::Geolocation::Geofencing::IGeofenceFactory

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    interface IGeofenceMonitor;
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor ABI::Windows::Devices::Geolocation::Geofencing::IGeofenceMonitor

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    interface IGeofenceMonitorStatics;
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics ABI::Windows::Devices::Geolocation::Geofencing::IGeofenceMonitorStatics

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    interface IGeofenceStateChangeReport;
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport ABI::Windows::Devices::Geolocation::Geofencing::IGeofenceStateChangeReport

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    class Geofence;
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_USE
#define DEF___FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e7a9e144-202d-5708-a9bd-e3dc0e14df46"))
IIterator<ABI::Windows::Devices::Geolocation::Geofencing::Geofence*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geofencing::Geofence*, ABI::Windows::Devices::Geolocation::Geofencing::IGeofence*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Geolocation.Geofencing.Geofence>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Geolocation::Geofencing::Geofence*> __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_t;
#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_USE
#define DEF___FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ce697733-595c-51c0-ad5f-324af5cdf2dd"))
IIterable<ABI::Windows::Devices::Geolocation::Geofencing::Geofence*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geofencing::Geofence*, ABI::Windows::Devices::Geolocation::Geofencing::IGeofence*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Geolocation.Geofencing.Geofence>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Geolocation::Geofencing::Geofence*> __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_t;
#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    class GeofenceStateChangeReport;
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_USE
#define DEF___FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("eaff2de4-6650-544a-b7ac-6d5b819d4698"))
IIterator<ABI::Windows::Devices::Geolocation::Geofencing::GeofenceStateChangeReport*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geofencing::GeofenceStateChangeReport*, ABI::Windows::Devices::Geolocation::Geofencing::IGeofenceStateChangeReport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Geolocation.Geofencing.GeofenceStateChangeReport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Geolocation::Geofencing::GeofenceStateChangeReport*> __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_t;
#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_USE
#define DEF___FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("76f50b4e-7aa7-565b-aada-b0c1cc144ed0"))
IIterable<ABI::Windows::Devices::Geolocation::Geofencing::GeofenceStateChangeReport*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geofencing::GeofenceStateChangeReport*, ABI::Windows::Devices::Geolocation::Geofencing::IGeofenceStateChangeReport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Geolocation.Geofencing.GeofenceStateChangeReport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Geolocation::Geofencing::GeofenceStateChangeReport*> __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_t;
#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_USE
#define DEF___FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d8039aca-1a45-5d13-8cfd-4900c22b8ef1"))
IVectorView<ABI::Windows::Devices::Geolocation::Geofencing::Geofence*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geofencing::Geofence*, ABI::Windows::Devices::Geolocation::Geofencing::IGeofence*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Geolocation.Geofencing.Geofence>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Geolocation::Geofencing::Geofence*> __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_t;
#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_USE
#define DEF___FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ea91593d-ecf4-5041-86f2-837a282c4d94"))
IVectorView<ABI::Windows::Devices::Geolocation::Geofencing::GeofenceStateChangeReport*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geofencing::GeofenceStateChangeReport*, ABI::Windows::Devices::Geolocation::Geofencing::IGeofenceStateChangeReport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Geolocation.Geofencing.GeofenceStateChangeReport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Geolocation::Geofencing::GeofenceStateChangeReport*> __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_t;
#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_USE
#define DEF___FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("096dc936-5f66-5c6e-95ce-ef5541fbf4c4"))
IVector<ABI::Windows::Devices::Geolocation::Geofencing::Geofence*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geofencing::Geofence*, ABI::Windows::Devices::Geolocation::Geofencing::IGeofence*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Devices.Geolocation.Geofencing.Geofence>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Devices::Geolocation::Geofencing::Geofence*> __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_t;
#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    class GeofenceMonitor;
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ecc5af2c-e47a-59ce-86be-9c3066fe26f7"))
ITypedEventHandler<ABI::Windows::Devices::Geolocation::Geofencing::GeofenceMonitor*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geofencing::GeofenceMonitor*, ABI::Windows::Devices::Geolocation::Geofencing::IGeofenceMonitor*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Geolocation.Geofencing.GeofenceMonitor, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Geolocation::Geofencing::GeofenceMonitor*, IInspectable*> __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable_USE */

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

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
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
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    typedef enum GeofenceMonitorStatus : int GeofenceMonitorStatus;
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    typedef enum GeofenceRemovalReason : int GeofenceRemovalReason;
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    typedef enum GeofenceState : unsigned int GeofenceState;
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    typedef enum MonitoredGeofenceStates : unsigned int MonitoredGeofenceStates;
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Geolocation.Geofencing.GeofenceMonitorStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    enum GeofenceMonitorStatus : int
                    {
                        GeofenceMonitorStatus_Ready = 0,
                        GeofenceMonitorStatus_Initializing = 1,
                        GeofenceMonitorStatus_NoData = 2,
                        GeofenceMonitorStatus_Disabled = 3,
                        GeofenceMonitorStatus_NotInitialized = 4,
                        GeofenceMonitorStatus_NotAvailable = 5,
                    };
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Geolocation.Geofencing.GeofenceRemovalReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    enum GeofenceRemovalReason : int
                    {
                        GeofenceRemovalReason_Used = 0,
                        GeofenceRemovalReason_Expired = 1,
                    };
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Geolocation.Geofencing.GeofenceState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    enum GeofenceState : unsigned int
                    {
                        GeofenceState_None = 0,
                        GeofenceState_Entered = 0x1,
                        GeofenceState_Exited = 0x2,
                        GeofenceState_Removed = 0x4,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(GeofenceState)
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Geolocation.Geofencing.MonitoredGeofenceStates
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    enum MonitoredGeofenceStates : unsigned int
                    {
                        MonitoredGeofenceStates_None = 0,
                        MonitoredGeofenceStates_Entered = 0x1,
                        MonitoredGeofenceStates_Exited = 0x2,
                        MonitoredGeofenceStates_Removed = 0x4,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(MonitoredGeofenceStates)
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.Geofencing.IGeofence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geofencing.Geofence
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_Geofencing_IGeofence[] = L"Windows.Devices.Geolocation.Geofencing.IGeofence";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    MIDL_INTERFACE("9c090823-edb8-47e0-8245-5bf61d321f2d")
                    IGeofence : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_StartTime(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Duration(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DwellTime(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MonitoredStates(
                            ABI::Windows::Devices::Geolocation::Geofencing::MonitoredGeofenceStates* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Geoshape(
                            ABI::Windows::Devices::Geolocation::IGeoshape** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SingleUse(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGeofence = __uuidof(IGeofence);
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.Geofencing.IGeofenceFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geofencing.Geofence
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_Geofencing_IGeofenceFactory[] = L"Windows.Devices.Geolocation.Geofencing.IGeofenceFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    MIDL_INTERFACE("841f624b-325f-4b90-bca7-2b8022a93796")
                    IGeofenceFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            HSTRING id,
                            ABI::Windows::Devices::Geolocation::IGeoshape* geoshape,
                            ABI::Windows::Devices::Geolocation::Geofencing::IGeofence** geofence
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateWithMonitorStates(
                            HSTRING id,
                            ABI::Windows::Devices::Geolocation::IGeoshape* geoshape,
                            ABI::Windows::Devices::Geolocation::Geofencing::MonitoredGeofenceStates monitoredStates,
                            boolean singleUse,
                            ABI::Windows::Devices::Geolocation::Geofencing::IGeofence** geofence
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateWithMonitorStatesAndDwellTime(
                            HSTRING id,
                            ABI::Windows::Devices::Geolocation::IGeoshape* geoshape,
                            ABI::Windows::Devices::Geolocation::Geofencing::MonitoredGeofenceStates monitoredStates,
                            boolean singleUse,
                            ABI::Windows::Foundation::TimeSpan dwellTime,
                            ABI::Windows::Devices::Geolocation::Geofencing::IGeofence** geofence
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateWithMonitorStatesDwellTimeStartTimeAndDuration(
                            HSTRING id,
                            ABI::Windows::Devices::Geolocation::IGeoshape* geoshape,
                            ABI::Windows::Devices::Geolocation::Geofencing::MonitoredGeofenceStates monitoredStates,
                            boolean singleUse,
                            ABI::Windows::Foundation::TimeSpan dwellTime,
                            ABI::Windows::Foundation::DateTime startTime,
                            ABI::Windows::Foundation::TimeSpan duration,
                            ABI::Windows::Devices::Geolocation::Geofencing::IGeofence** geofence
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGeofenceFactory = __uuidof(IGeofenceFactory);
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.Geofencing.IGeofenceMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geofencing.GeofenceMonitor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_Geofencing_IGeofenceMonitor[] = L"Windows.Devices.Geolocation.Geofencing.IGeofenceMonitor";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    MIDL_INTERFACE("4c0f5f78-1c1f-4621-bbbd-833b92247226")
                    IGeofenceMonitor : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Devices::Geolocation::Geofencing::GeofenceMonitorStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Geofences(
                            __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LastKnownGeoposition(
                            ABI::Windows::Devices::Geolocation::IGeoposition** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_GeofenceStateChanged(
                            __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable* eventHandler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_GeofenceStateChanged(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReadReports(
                            __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_StatusChanged(
                            __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable* eventHandler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_StatusChanged(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGeofenceMonitor = __uuidof(IGeofenceMonitor);
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.Geofencing.IGeofenceMonitorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geofencing.GeofenceMonitor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_Geofencing_IGeofenceMonitorStatics[] = L"Windows.Devices.Geolocation.Geofencing.IGeofenceMonitorStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    MIDL_INTERFACE("2dd32fcf-7e75-4899-ace3-2bd0a65cce06")
                    IGeofenceMonitorStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Current(
                            ABI::Windows::Devices::Geolocation::Geofencing::IGeofenceMonitor** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGeofenceMonitorStatics = __uuidof(IGeofenceMonitorStatics);
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.Geofencing.IGeofenceStateChangeReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geofencing.GeofenceStateChangeReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_Geofencing_IGeofenceStateChangeReport[] = L"Windows.Devices.Geolocation.Geofencing.IGeofenceStateChangeReport";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Geofencing {
                    MIDL_INTERFACE("9a243c18-2464-4c89-be05-b3ffff5babc5")
                    IGeofenceStateChangeReport : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_NewState(
                            ABI::Windows::Devices::Geolocation::Geofencing::GeofenceState* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Geofence(
                            ABI::Windows::Devices::Geolocation::Geofencing::IGeofence** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Geoposition(
                            ABI::Windows::Devices::Geolocation::IGeoposition** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RemovalReason(
                            ABI::Windows::Devices::Geolocation::Geofencing::GeofenceRemovalReason* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGeofenceStateChangeReport = __uuidof(IGeofenceStateChangeReport);
                } /* Geofencing */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geofencing.Geofence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Geolocation.Geofencing.IGeofenceFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.Geofencing.IGeofence ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geofencing_Geofence_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geofencing_Geofence_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geofencing_Geofence[] = L"Windows.Devices.Geolocation.Geofencing.Geofence";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geofencing.GeofenceMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Geolocation.Geofencing.IGeofenceMonitorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.Geofencing.IGeofenceMonitor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geofencing_GeofenceMonitor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geofencing_GeofenceMonitor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geofencing_GeofenceMonitor[] = L"Windows.Devices.Geolocation.Geofencing.GeofenceMonitor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geofencing.GeofenceStateChangeReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.Geofencing.IGeofenceStateChangeReport ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geofencing_GeofenceStateChangeReport_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geofencing_GeofenceStateChangeReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geofencing_GeofenceStateChangeReport[] = L"Windows.Devices.Geolocation.Geofencing.GeofenceStateChangeReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence;

typedef struct __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceVtbl;

interface __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence;

typedef struct __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceVtbl;

interface __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport;

typedef struct __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReportVtbl;

interface __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport;

typedef struct __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        __FIIterator_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReportVtbl;

interface __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence;

typedef struct __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceVtbl;

interface __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport;

typedef struct __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReportVtbl;

interface __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence;

typedef struct __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence** items);

    END_INTERFACE
} __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceVtbl;

interface __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence
{
    CONST_VTBL struct __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef enum __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CGeofenceMonitorStatus __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CGeofenceMonitorStatus;

typedef enum __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CGeofenceRemovalReason __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CGeofenceRemovalReason;

typedef enum __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CGeofenceState __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CGeofenceState;

typedef enum __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CMonitoredGeofenceStates __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CMonitoredGeofenceStates;

/*
 *
 * Struct Windows.Devices.Geolocation.Geofencing.GeofenceMonitorStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CGeofenceMonitorStatus
{
    GeofenceMonitorStatus_Ready = 0,
    GeofenceMonitorStatus_Initializing = 1,
    GeofenceMonitorStatus_NoData = 2,
    GeofenceMonitorStatus_Disabled = 3,
    GeofenceMonitorStatus_NotInitialized = 4,
    GeofenceMonitorStatus_NotAvailable = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Geolocation.Geofencing.GeofenceRemovalReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CGeofenceRemovalReason
{
    GeofenceRemovalReason_Used = 0,
    GeofenceRemovalReason_Expired = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Geolocation.Geofencing.GeofenceState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CGeofenceState
{
    GeofenceState_None = 0,
    GeofenceState_Entered = 0x1,
    GeofenceState_Exited = 0x2,
    GeofenceState_Removed = 0x4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Geolocation.Geofencing.MonitoredGeofenceStates
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CMonitoredGeofenceStates
{
    MonitoredGeofenceStates_None = 0,
    MonitoredGeofenceStates_Entered = 0x1,
    MonitoredGeofenceStates_Exited = 0x2,
    MonitoredGeofenceStates_Removed = 0x4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.Geofencing.IGeofence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geofencing.Geofence
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_Geofencing_IGeofence[] = L"Windows.Devices.Geolocation.Geofencing.IGeofence";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_StartTime)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_DwellTime)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MonitoredStates)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence* This,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CMonitoredGeofenceStates* value);
    HRESULT (STDMETHODCALLTYPE* get_Geoshape)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape** value);
    HRESULT (STDMETHODCALLTYPE* get_SingleUse)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_get_StartTime(This, value) \
    ((This)->lpVtbl->get_StartTime(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_get_DwellTime(This, value) \
    ((This)->lpVtbl->get_DwellTime(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_get_MonitoredStates(This, value) \
    ((This)->lpVtbl->get_MonitoredStates(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_get_Geoshape(This, value) \
    ((This)->lpVtbl->get_Geoshape(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_get_SingleUse(This, value) \
    ((This)->lpVtbl->get_SingleUse(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.Geofencing.IGeofenceFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geofencing.Geofence
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_Geofencing_IGeofenceFactory[] = L"Windows.Devices.Geolocation.Geofencing.IGeofenceFactory";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory* This,
        HSTRING id,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape* geoshape,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence** geofence);
    HRESULT (STDMETHODCALLTYPE* CreateWithMonitorStates)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory* This,
        HSTRING id,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape* geoshape,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CMonitoredGeofenceStates monitoredStates,
        boolean singleUse,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence** geofence);
    HRESULT (STDMETHODCALLTYPE* CreateWithMonitorStatesAndDwellTime)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory* This,
        HSTRING id,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape* geoshape,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CMonitoredGeofenceStates monitoredStates,
        boolean singleUse,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan dwellTime,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence** geofence);
    HRESULT (STDMETHODCALLTYPE* CreateWithMonitorStatesDwellTimeStartTimeAndDuration)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory* This,
        HSTRING id,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoshape* geoshape,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CMonitoredGeofenceStates monitoredStates,
        boolean singleUse,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan dwellTime,
        struct __x_ABI_CWindows_CFoundation_CDateTime startTime,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan duration,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence** geofence);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_Create(This, id, geoshape, geofence) \
    ((This)->lpVtbl->Create(This, id, geoshape, geofence))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_CreateWithMonitorStates(This, id, geoshape, monitoredStates, singleUse, geofence) \
    ((This)->lpVtbl->CreateWithMonitorStates(This, id, geoshape, monitoredStates, singleUse, geofence))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_CreateWithMonitorStatesAndDwellTime(This, id, geoshape, monitoredStates, singleUse, dwellTime, geofence) \
    ((This)->lpVtbl->CreateWithMonitorStatesAndDwellTime(This, id, geoshape, monitoredStates, singleUse, dwellTime, geofence))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_CreateWithMonitorStatesDwellTimeStartTimeAndDuration(This, id, geoshape, monitoredStates, singleUse, dwellTime, startTime, duration, geofence) \
    ((This)->lpVtbl->CreateWithMonitorStatesDwellTimeStartTimeAndDuration(This, id, geoshape, monitoredStates, singleUse, dwellTime, startTime, duration, geofence))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.Geofencing.IGeofenceMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geofencing.GeofenceMonitor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_Geofencing_IGeofenceMonitor[] = L"Windows.Devices.Geolocation.Geofencing.IGeofenceMonitor";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor* This,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CGeofenceMonitorStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Geofences)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor* This,
        __FIVector_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofence** value);
    HRESULT (STDMETHODCALLTYPE* get_LastKnownGeoposition)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition** value);
    HRESULT (STDMETHODCALLTYPE* add_GeofenceStateChanged)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor* This,
        __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable* eventHandler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_GeofenceStateChanged)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* ReadReports)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor* This,
        __FIVectorView_1_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceStateChangeReport** value);
    HRESULT (STDMETHODCALLTYPE* add_StatusChanged)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor* This,
        __FITypedEventHandler_2_Windows__CDevices__CGeolocation__CGeofencing__CGeofenceMonitor_IInspectable* eventHandler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StatusChanged)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_get_Geofences(This, value) \
    ((This)->lpVtbl->get_Geofences(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_get_LastKnownGeoposition(This, value) \
    ((This)->lpVtbl->get_LastKnownGeoposition(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_add_GeofenceStateChanged(This, eventHandler, token) \
    ((This)->lpVtbl->add_GeofenceStateChanged(This, eventHandler, token))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_remove_GeofenceStateChanged(This, token) \
    ((This)->lpVtbl->remove_GeofenceStateChanged(This, token))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_ReadReports(This, value) \
    ((This)->lpVtbl->ReadReports(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_add_StatusChanged(This, eventHandler, token) \
    ((This)->lpVtbl->add_StatusChanged(This, eventHandler, token))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_remove_StatusChanged(This, token) \
    ((This)->lpVtbl->remove_StatusChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.Geofencing.IGeofenceMonitorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geofencing.GeofenceMonitor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_Geofencing_IGeofenceMonitorStatics[] = L"Windows.Devices.Geolocation.Geofencing.IGeofenceMonitorStatics";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitor** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_get_Current(This, value) \
    ((This)->lpVtbl->get_Current(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceMonitorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Geolocation.Geofencing.IGeofenceStateChangeReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Geofencing.GeofenceStateChangeReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_Geofencing_IGeofenceStateChangeReport[] = L"Windows.Devices.Geolocation.Geofencing.IGeofenceStateChangeReport";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NewState)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport* This,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CGeofenceState* value);
    HRESULT (STDMETHODCALLTYPE* get_Geofence)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofence** value);
    HRESULT (STDMETHODCALLTYPE* get_Geoposition)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoposition** value);
    HRESULT (STDMETHODCALLTYPE* get_RemovalReason)(__x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport* This,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CGeofenceRemovalReason* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReportVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_get_NewState(This, value) \
    ((This)->lpVtbl->get_NewState(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_get_Geofence(This, value) \
    ((This)->lpVtbl->get_Geofence(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_get_Geoposition(This, value) \
    ((This)->lpVtbl->get_Geoposition(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_get_RemovalReason(This, value) \
    ((This)->lpVtbl->get_RemovalReason(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CGeofencing_CIGeofenceStateChangeReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geofencing.Geofence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Geolocation.Geofencing.IGeofenceFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.Geofencing.IGeofence ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geofencing_Geofence_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geofencing_Geofence_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geofencing_Geofence[] = L"Windows.Devices.Geolocation.Geofencing.Geofence";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geofencing.GeofenceMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Geolocation.Geofencing.IGeofenceMonitorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.Geofencing.IGeofenceMonitor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geofencing_GeofenceMonitor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geofencing_GeofenceMonitor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geofencing_GeofenceMonitor[] = L"Windows.Devices.Geolocation.Geofencing.GeofenceMonitor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Geolocation.Geofencing.GeofenceStateChangeReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.Geofencing.IGeofenceStateChangeReport ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Geofencing_GeofenceStateChangeReport_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Geofencing_GeofenceStateChangeReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Geofencing_GeofenceStateChangeReport[] = L"Windows.Devices.Geolocation.Geofencing.GeofenceStateChangeReport";
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
#endif // __windows2Edevices2Egeolocation2Egeofencing_p_h__

#endif // __windows2Edevices2Egeolocation2Egeofencing_h__
