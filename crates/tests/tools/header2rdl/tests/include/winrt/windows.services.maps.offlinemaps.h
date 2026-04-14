
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
#ifndef __windows2Eservices2Emaps2Eofflinemaps_h__
#define __windows2Eservices2Emaps2Eofflinemaps_h__
#ifndef __windows2Eservices2Emaps2Eofflinemaps_p_h__
#define __windows2Eservices2Emaps2Eofflinemaps_p_h__


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
#ifndef ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace OfflineMaps {
                    interface IOfflineMapPackage;
                } /* OfflineMaps */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage ABI::Windows::Services::Maps::OfflineMaps::IOfflineMapPackage

#endif // ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace OfflineMaps {
                    interface IOfflineMapPackageQueryResult;
                } /* OfflineMaps */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult ABI::Windows::Services::Maps::OfflineMaps::IOfflineMapPackageQueryResult

#endif // ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace OfflineMaps {
                    interface IOfflineMapPackageStartDownloadResult;
                } /* OfflineMaps */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult ABI::Windows::Services::Maps::OfflineMaps::IOfflineMapPackageStartDownloadResult

#endif // ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace OfflineMaps {
                    interface IOfflineMapPackageStatics;
                } /* OfflineMaps */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics ABI::Windows::Services::Maps::OfflineMaps::IOfflineMapPackageStatics

#endif // ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace OfflineMaps {
                    class OfflineMapPackageQueryResult;
                } /* OfflineMaps */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_USE
#define DEF___FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c01a663d-6d9b-5385-ae68-0a65a9544514"))
IAsyncOperation<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackageQueryResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackageQueryResult*, ABI::Windows::Services::Maps::OfflineMaps::IOfflineMapPackageQueryResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackageQueryResult*> __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_t;
#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8a4df3c9-2595-5bec-8ba1-c1d955f168c0"))
IAsyncOperationCompletedHandler<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackageQueryResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackageQueryResult*, ABI::Windows::Services::Maps::OfflineMaps::IOfflineMapPackageQueryResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackageQueryResult*> __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace OfflineMaps {
                    class OfflineMapPackageStartDownloadResult;
                } /* OfflineMaps */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_USE
#define DEF___FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("911272f7-f5aa-5393-94a1-e96adfad3da4"))
IAsyncOperation<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackageStartDownloadResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackageStartDownloadResult*, ABI::Windows::Services::Maps::OfflineMaps::IOfflineMapPackageStartDownloadResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackageStartDownloadResult*> __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_t;
#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8654a79e-c52f-5f98-af0a-522466c27246"))
IAsyncOperationCompletedHandler<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackageStartDownloadResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackageStartDownloadResult*, ABI::Windows::Services::Maps::OfflineMaps::IOfflineMapPackageStartDownloadResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackageStartDownloadResult*> __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace OfflineMaps {
                    class OfflineMapPackage;
                } /* OfflineMaps */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_USE
#define DEF___FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("750f4d37-3766-5842-a425-ccd9b8d80786"))
IIterator<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackage*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackage*, ABI::Windows::Services::Maps::OfflineMaps::IOfflineMapPackage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Services.Maps.OfflineMaps.OfflineMapPackage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackage*> __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_t;
#define __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_USE
#define DEF___FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7522287c-0af2-586c-bb3f-457c07984a6f"))
IIterable<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackage*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackage*, ABI::Windows::Services::Maps::OfflineMaps::IOfflineMapPackage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Services.Maps.OfflineMaps.OfflineMapPackage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackage*> __FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_t;
#define __FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_USE
#define DEF___FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2522ebc2-bd9f-551b-b90e-6c28152958a0"))
IVectorView<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackage*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackage*, ABI::Windows::Services::Maps::OfflineMaps::IOfflineMapPackage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Services.Maps.OfflineMaps.OfflineMapPackage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackage*> __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_t;
#define __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2d2e0d20-826f-560c-b1c1-b4bd6fbf329a"))
ITypedEventHandler<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackage*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackage*, ABI::Windows::Services::Maps::OfflineMaps::IOfflineMapPackage*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Services.Maps.OfflineMaps.OfflineMapPackage, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackage*, IInspectable*> __FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

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
                class Geocircle;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

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
        namespace Services {
            namespace Maps {
                namespace OfflineMaps {
                    typedef enum OfflineMapPackageQueryStatus : int OfflineMapPackageQueryStatus;
                } /* OfflineMaps */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace OfflineMaps {
                    typedef enum OfflineMapPackageStartDownloadStatus : int OfflineMapPackageStartDownloadStatus;
                } /* OfflineMaps */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace OfflineMaps {
                    typedef enum OfflineMapPackageStatus : int OfflineMapPackageStatus;
                } /* OfflineMaps */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace OfflineMaps {
                    enum OfflineMapPackageQueryStatus : int
                    {
                        OfflineMapPackageQueryStatus_Success = 0,
                        OfflineMapPackageQueryStatus_UnknownError = 1,
                        OfflineMapPackageQueryStatus_InvalidCredentials = 2,
                        OfflineMapPackageQueryStatus_NetworkFailure = 3,
                    };
                } /* OfflineMaps */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace OfflineMaps {
                    enum OfflineMapPackageStartDownloadStatus : int
                    {
                        OfflineMapPackageStartDownloadStatus_Success = 0,
                        OfflineMapPackageStartDownloadStatus_UnknownError = 1,
                        OfflineMapPackageStartDownloadStatus_InvalidCredentials = 2,
                        OfflineMapPackageStartDownloadStatus_DeniedWithoutCapability = 3,
                    };
                } /* OfflineMaps */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Services.Maps.OfflineMaps.OfflineMapPackageStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace OfflineMaps {
                    enum OfflineMapPackageStatus : int
                    {
                        OfflineMapPackageStatus_NotDownloaded = 0,
                        OfflineMapPackageStatus_Downloading = 1,
                        OfflineMapPackageStatus_Downloaded = 2,
                        OfflineMapPackageStatus_Deleting = 3,
                    };
                } /* OfflineMaps */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.OfflineMaps.IOfflineMapPackage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.OfflineMaps.OfflineMapPackage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_OfflineMaps_IOfflineMapPackage[] = L"Windows.Services.Maps.OfflineMaps.IOfflineMapPackage";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace OfflineMaps {
                    MIDL_INTERFACE("a797673b-a5b5-4144-b525-e68c8862664b")
                    IOfflineMapPackage : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackageStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EnclosingRegionName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EstimatedSizeInBytes(
                            UINT64* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_StatusChanged(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_StatusChanged(
                            __FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable* value,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RequestStartDownloadAsync(
                            __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IOfflineMapPackage = __uuidof(IOfflineMapPackage);
                } /* OfflineMaps */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.OfflineMaps.IOfflineMapPackageQueryResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_OfflineMaps_IOfflineMapPackageQueryResult[] = L"Windows.Services.Maps.OfflineMaps.IOfflineMapPackageQueryResult";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace OfflineMaps {
                    MIDL_INTERFACE("55585411-39e1-4e41-a4e1-5f4872bee199")
                    IOfflineMapPackageQueryResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackageQueryStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Packages(
                            __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IOfflineMapPackageQueryResult = __uuidof(IOfflineMapPackageQueryResult);
                } /* OfflineMaps */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.OfflineMaps.IOfflineMapPackageStartDownloadResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_OfflineMaps_IOfflineMapPackageStartDownloadResult[] = L"Windows.Services.Maps.OfflineMaps.IOfflineMapPackageStartDownloadResult";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace OfflineMaps {
                    MIDL_INTERFACE("d965b918-d4d6-4afe-9378-3ec71ef11c3d")
                    IOfflineMapPackageStartDownloadResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Services::Maps::OfflineMaps::OfflineMapPackageStartDownloadStatus* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IOfflineMapPackageStartDownloadResult = __uuidof(IOfflineMapPackageStartDownloadResult);
                } /* OfflineMaps */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.OfflineMaps.IOfflineMapPackageStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.OfflineMaps.OfflineMapPackage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_OfflineMaps_IOfflineMapPackageStatics[] = L"Windows.Services.Maps.OfflineMaps.IOfflineMapPackageStatics";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace OfflineMaps {
                    MIDL_INTERFACE("185e7922-a831-4ab0-941f-6998fa929285")
                    IOfflineMapPackageStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE FindPackagesAsync(
                            ABI::Windows::Devices::Geolocation::IGeopoint* queryPoint,
                            __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE FindPackagesInBoundingBoxAsync(
                            ABI::Windows::Devices::Geolocation::IGeoboundingBox* queryBoundingBox,
                            __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE FindPackagesInGeocircleAsync(
                            ABI::Windows::Devices::Geolocation::IGeocircle* queryCircle,
                            __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IOfflineMapPackageStatics = __uuidof(IOfflineMapPackageStatics);
                } /* OfflineMaps */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Services.Maps.OfflineMaps.OfflineMapPackage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.OfflineMaps.IOfflineMapPackageStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.OfflineMaps.IOfflineMapPackage ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Services_Maps_OfflineMaps_OfflineMapPackage_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_OfflineMaps_OfflineMapPackage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_OfflineMaps_OfflineMapPackage[] = L"Windows.Services.Maps.OfflineMaps.OfflineMapPackage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.OfflineMaps.IOfflineMapPackageQueryResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Services_Maps_OfflineMaps_OfflineMapPackageQueryResult_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_OfflineMaps_OfflineMapPackageQueryResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_OfflineMaps_OfflineMapPackageQueryResult[] = L"Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.OfflineMaps.IOfflineMapPackageStartDownloadResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Services_Maps_OfflineMaps_OfflineMapPackageStartDownloadResult_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_OfflineMaps_OfflineMapPackageStartDownloadResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_OfflineMaps_OfflineMapPackageStartDownloadResult[] = L"Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage;

#endif // ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult;

#endif // ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult;

#endif // ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics;

#endif // ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult;

typedef struct __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult* This,
        __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResultVtbl;

interface __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult* This,
        __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult;

typedef struct __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult* This,
        __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResultVtbl;

interface __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult* This,
        __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage;

typedef struct __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageVtbl;

interface __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage
{
    CONST_VTBL struct __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage __FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage;

typedef struct __FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        __FIIterator_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage** result);

    END_INTERFACE
} __FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageVtbl;

interface __FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage
{
    CONST_VTBL struct __FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage;

typedef struct __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        UINT32 index,
        __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageVtbl;

interface __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage
{
    CONST_VTBL struct __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable __FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable* This,
        __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CServices_CMaps_COfflineMaps_COfflineMapPackageQueryStatus __x_ABI_CWindows_CServices_CMaps_COfflineMaps_COfflineMapPackageQueryStatus;

typedef enum __x_ABI_CWindows_CServices_CMaps_COfflineMaps_COfflineMapPackageStartDownloadStatus __x_ABI_CWindows_CServices_CMaps_COfflineMaps_COfflineMapPackageStartDownloadStatus;

typedef enum __x_ABI_CWindows_CServices_CMaps_COfflineMaps_COfflineMapPackageStatus __x_ABI_CWindows_CServices_CMaps_COfflineMaps_COfflineMapPackageStatus;

/*
 *
 * Struct Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CServices_CMaps_COfflineMaps_COfflineMapPackageQueryStatus
{
    OfflineMapPackageQueryStatus_Success = 0,
    OfflineMapPackageQueryStatus_UnknownError = 1,
    OfflineMapPackageQueryStatus_InvalidCredentials = 2,
    OfflineMapPackageQueryStatus_NetworkFailure = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CServices_CMaps_COfflineMaps_COfflineMapPackageStartDownloadStatus
{
    OfflineMapPackageStartDownloadStatus_Success = 0,
    OfflineMapPackageStartDownloadStatus_UnknownError = 1,
    OfflineMapPackageStartDownloadStatus_InvalidCredentials = 2,
    OfflineMapPackageStartDownloadStatus_DeniedWithoutCapability = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Services.Maps.OfflineMaps.OfflineMapPackageStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CServices_CMaps_COfflineMaps_COfflineMapPackageStatus
{
    OfflineMapPackageStatus_NotDownloaded = 0,
    OfflineMapPackageStatus_Downloading = 1,
    OfflineMapPackageStatus_Downloaded = 2,
    OfflineMapPackageStatus_Deleting = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.OfflineMaps.IOfflineMapPackage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.OfflineMaps.OfflineMapPackage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_OfflineMaps_IOfflineMapPackage[] = L"Windows.Services.Maps.OfflineMaps.IOfflineMapPackage";
typedef struct __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage* This,
        enum __x_ABI_CWindows_CServices_CMaps_COfflineMaps_COfflineMapPackageStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EnclosingRegionName)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EstimatedSizeInBytes)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* remove_StatusChanged)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_StatusChanged)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage* This,
        __FITypedEventHandler_2_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage_IInspectable* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* RequestStartDownloadAsync)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage* This,
        __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageStartDownloadResult** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageVtbl;

interface __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_get_EnclosingRegionName(This, value) \
    ((This)->lpVtbl->get_EnclosingRegionName(This, value))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_get_EstimatedSizeInBytes(This, value) \
    ((This)->lpVtbl->get_EstimatedSizeInBytes(This, value))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_remove_StatusChanged(This, token) \
    ((This)->lpVtbl->remove_StatusChanged(This, token))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_add_StatusChanged(This, value, token) \
    ((This)->lpVtbl->add_StatusChanged(This, value, token))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_RequestStartDownloadAsync(This, value) \
    ((This)->lpVtbl->RequestStartDownloadAsync(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.OfflineMaps.IOfflineMapPackageQueryResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_OfflineMaps_IOfflineMapPackageQueryResult[] = L"Windows.Services.Maps.OfflineMaps.IOfflineMapPackageQueryResult";
typedef struct __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult* This,
        enum __x_ABI_CWindows_CServices_CMaps_COfflineMaps_COfflineMapPackageQueryStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Packages)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult* This,
        __FIVectorView_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackage** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResultVtbl;

interface __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_get_Packages(This, value) \
    ((This)->lpVtbl->get_Packages(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageQueryResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.OfflineMaps.IOfflineMapPackageStartDownloadResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_OfflineMaps_IOfflineMapPackageStartDownloadResult[] = L"Windows.Services.Maps.OfflineMaps.IOfflineMapPackageStartDownloadResult";
typedef struct __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult* This,
        enum __x_ABI_CWindows_CServices_CMaps_COfflineMaps_COfflineMapPackageStartDownloadStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResultVtbl;

interface __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStartDownloadResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Maps.OfflineMaps.IOfflineMapPackageStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.OfflineMaps.OfflineMapPackage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_OfflineMaps_IOfflineMapPackageStatics[] = L"Windows.Services.Maps.OfflineMaps.IOfflineMapPackageStatics";
typedef struct __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindPackagesAsync)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* queryPoint,
        __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult** result);
    HRESULT (STDMETHODCALLTYPE* FindPackagesInBoundingBoxAsync)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeoboundingBox* queryBoundingBox,
        __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult** result);
    HRESULT (STDMETHODCALLTYPE* FindPackagesInGeocircleAsync)(__x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle* queryCircle,
        __FIAsyncOperation_1_Windows__CServices__CMaps__COfflineMaps__COfflineMapPackageQueryResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStaticsVtbl;

interface __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_FindPackagesAsync(This, queryPoint, result) \
    ((This)->lpVtbl->FindPackagesAsync(This, queryPoint, result))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_FindPackagesInBoundingBoxAsync(This, queryBoundingBox, result) \
    ((This)->lpVtbl->FindPackagesInBoundingBoxAsync(This, queryBoundingBox, result))

#define __x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_FindPackagesInGeocircleAsync(This, queryCircle, result) \
    ((This)->lpVtbl->FindPackagesInGeocircleAsync(This, queryCircle, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_COfflineMaps_CIOfflineMapPackageStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Services.Maps.OfflineMaps.OfflineMapPackage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.OfflineMaps.IOfflineMapPackageStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.OfflineMaps.IOfflineMapPackage ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Services_Maps_OfflineMaps_OfflineMapPackage_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_OfflineMaps_OfflineMapPackage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_OfflineMaps_OfflineMapPackage[] = L"Windows.Services.Maps.OfflineMaps.OfflineMapPackage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.OfflineMaps.IOfflineMapPackageQueryResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Services_Maps_OfflineMaps_OfflineMapPackageQueryResult_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_OfflineMaps_OfflineMapPackageQueryResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_OfflineMaps_OfflineMapPackageQueryResult[] = L"Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.OfflineMaps.IOfflineMapPackageStartDownloadResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Services_Maps_OfflineMaps_OfflineMapPackageStartDownloadResult_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_OfflineMaps_OfflineMapPackageStartDownloadResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_OfflineMaps_OfflineMapPackageStartDownloadResult[] = L"Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadResult";
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
#endif // __windows2Eservices2Emaps2Eofflinemaps_p_h__

#endif // __windows2Eservices2Emaps2Eofflinemaps_h__
