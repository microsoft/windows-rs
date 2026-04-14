
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
#ifndef __windows2Eservices2Emaps2Elocalsearch_h__
#define __windows2Eservices2Emaps2Elocalsearch_h__
#ifndef __windows2Eservices2Emaps2Elocalsearch_p_h__
#define __windows2Eservices2Emaps2Elocalsearch_p_h__


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

#if !defined(WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION)
#define WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION)

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
#include "Windows.Globalization.h"
#include "Windows.Services.Maps.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    interface ILocalCategoriesStatics;
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics ABI::Windows::Services::Maps::LocalSearch::ILocalCategoriesStatics

#endif // ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    interface ILocalLocation;
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation ABI::Windows::Services::Maps::LocalSearch::ILocalLocation

#endif // ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    interface ILocalLocation2;
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2 ABI::Windows::Services::Maps::LocalSearch::ILocalLocation2

#endif // ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    interface ILocalLocationFinderResult;
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult ABI::Windows::Services::Maps::LocalSearch::ILocalLocationFinderResult

#endif // ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    interface ILocalLocationFinderStatics;
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics ABI::Windows::Services::Maps::LocalSearch::ILocalLocationFinderStatics

#endif // ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    interface ILocalLocationHoursOfOperationItem;
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem ABI::Windows::Services::Maps::LocalSearch::ILocalLocationHoursOfOperationItem

#endif // ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    interface ILocalLocationRatingInfo;
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo ABI::Windows::Services::Maps::LocalSearch::ILocalLocationRatingInfo

#endif // ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    interface IPlaceInfoHelperStatics;
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics ABI::Windows::Services::Maps::LocalSearch::IPlaceInfoHelperStatics

#endif // ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    class LocalLocationFinderResult;
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_USE
#define DEF___FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("08e69b32-f420-5280-a721-075b4fd03d94"))
IAsyncOperation<ABI::Windows::Services::Maps::LocalSearch::LocalLocationFinderResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::LocalSearch::LocalLocationFinderResult*, ABI::Windows::Services::Maps::LocalSearch::ILocalLocationFinderResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Services.Maps.LocalSearch.LocalLocationFinderResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Services::Maps::LocalSearch::LocalLocationFinderResult*> __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_t;
#define __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_USE */

#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7b4a1b93-2943-5e50-a010-ee9aec1bbfe7"))
IAsyncOperationCompletedHandler<ABI::Windows::Services::Maps::LocalSearch::LocalLocationFinderResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::LocalSearch::LocalLocationFinderResult*, ABI::Windows::Services::Maps::LocalSearch::ILocalLocationFinderResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Services.Maps.LocalSearch.LocalLocationFinderResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Services::Maps::LocalSearch::LocalLocationFinderResult*> __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_USE */

#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    class LocalLocation;
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_USE
#define DEF___FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a23c2e87-a8e7-568f-96a1-69e98f86b9d3"))
IIterator<ABI::Windows::Services::Maps::LocalSearch::LocalLocation*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::LocalSearch::LocalLocation*, ABI::Windows::Services::Maps::LocalSearch::ILocalLocation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Services.Maps.LocalSearch.LocalLocation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Services::Maps::LocalSearch::LocalLocation*> __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_t;
#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_USE */

#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_USE
#define DEF___FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5474eefb-60f4-58ac-89e2-6e83f79fa76c"))
IIterable<ABI::Windows::Services::Maps::LocalSearch::LocalLocation*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::LocalSearch::LocalLocation*, ABI::Windows::Services::Maps::LocalSearch::ILocalLocation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Services.Maps.LocalSearch.LocalLocation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Services::Maps::LocalSearch::LocalLocation*> __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_t;
#define __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_USE */

#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    class LocalLocationHoursOfOperationItem;
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_USE
#define DEF___FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3f9ba5c0-b2dc-5777-a81e-b0a2839b152b"))
IIterator<ABI::Windows::Services::Maps::LocalSearch::LocalLocationHoursOfOperationItem*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::LocalSearch::LocalLocationHoursOfOperationItem*, ABI::Windows::Services::Maps::LocalSearch::ILocalLocationHoursOfOperationItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Services.Maps.LocalSearch.LocalLocationHoursOfOperationItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Services::Maps::LocalSearch::LocalLocationHoursOfOperationItem*> __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_t;
#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_USE */

#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000

#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_USE
#define DEF___FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f298f515-b9bd-5297-b8bd-1c0040daac76"))
IIterable<ABI::Windows::Services::Maps::LocalSearch::LocalLocationHoursOfOperationItem*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::LocalSearch::LocalLocationHoursOfOperationItem*, ABI::Windows::Services::Maps::LocalSearch::ILocalLocationHoursOfOperationItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Services.Maps.LocalSearch.LocalLocationHoursOfOperationItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Services::Maps::LocalSearch::LocalLocationHoursOfOperationItem*> __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_t;
#define __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_USE */

#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000

#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_USE
#define DEF___FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("619192f2-4f9d-5629-ab01-b18512503d73"))
IVectorView<ABI::Windows::Services::Maps::LocalSearch::LocalLocation*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::LocalSearch::LocalLocation*, ABI::Windows::Services::Maps::LocalSearch::ILocalLocation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Services.Maps.LocalSearch.LocalLocation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Services::Maps::LocalSearch::LocalLocation*> __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_t;
#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_USE */

#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_USE
#define DEF___FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("65535172-cd91-5b4c-aa60-dab146301284"))
IVectorView<ABI::Windows::Services::Maps::LocalSearch::LocalLocationHoursOfOperationItem*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::Maps::LocalSearch::LocalLocationHoursOfOperationItem*, ABI::Windows::Services::Maps::LocalSearch::ILocalLocationHoursOfOperationItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Services.Maps.LocalSearch.LocalLocationHoursOfOperationItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Services::Maps::LocalSearch::LocalLocationHoursOfOperationItem*> __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_t;
#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_USE */

#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000


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



#ifndef DEF___FIReference_1_int_USE
#define DEF___FIReference_1_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("548cefbd-bc8a-5fa0-8df2-957440fc8bf4"))
IReference<int> : IReference_impl<int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<int> __FIReference_1_int_t;
#define __FIReference_1_int ABI::Windows::Foundation::__FIReference_1_int_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_int_USE */


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
        namespace Globalization {
            typedef enum DayOfWeek : int DayOfWeek;
        } /* Globalization */
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

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                class PlaceInfo;
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    typedef enum LocalLocationFinderStatus : int LocalLocationFinderStatus;
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    class LocalLocationRatingInfo;
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Services.Maps.LocalSearch.LocalLocationFinderStatus
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    enum LocalLocationFinderStatus : int
                    {
                        LocalLocationFinderStatus_Success = 0,
                        LocalLocationFinderStatus_UnknownError = 1,
                        LocalLocationFinderStatus_InvalidCredentials = 2,
                        LocalLocationFinderStatus_InvalidCategory = 3,
                        LocalLocationFinderStatus_InvalidSearchTerm = 4,
                        LocalLocationFinderStatus_InvalidSearchArea = 5,
                        LocalLocationFinderStatus_NetworkFailure = 6,
                        LocalLocationFinderStatus_NotSupported = 7,
                    };
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.LocalSearch.ILocalCategoriesStatics
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.LocalSearch.LocalCategories
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_LocalSearch_ILocalCategoriesStatics[] = L"Windows.Services.Maps.LocalSearch.ILocalCategoriesStatics";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    MIDL_INTERFACE("f49399f5-8261-4321-9974-ef92d49a8dca")
                    ILocalCategoriesStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_BankAndCreditUnions(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EatDrink(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Hospitals(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HotelsAndMotels(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_All(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Parking(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SeeDo(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Shop(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILocalCategoriesStatics = __uuidof(ILocalCategoriesStatics);
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.LocalSearch.ILocalLocation
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.LocalSearch.LocalLocation
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_LocalSearch_ILocalLocation[] = L"Windows.Services.Maps.LocalSearch.ILocalLocation";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    MIDL_INTERFACE("bb0fe9ab-4502-4f2c-94a9-0d60de0e2163")
                    ILocalLocation : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Address(
                            ABI::Windows::Services::Maps::IMapAddress** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Identifier(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Point(
                            ABI::Windows::Devices::Geolocation::IGeopoint** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PhoneNumber(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DataAttribution(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILocalLocation = __uuidof(ILocalLocation);
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.LocalSearch.ILocalLocation2
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.LocalSearch.LocalLocation
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_LocalSearch_ILocalLocation2[] = L"Windows.Services.Maps.LocalSearch.ILocalLocation2";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    MIDL_INTERFACE("6e9e307c-ecb5-4ffc-bb8c-ba50ba8c2dc6")
                    ILocalLocation2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Category(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RatingInfo(
                            ABI::Windows::Services::Maps::LocalSearch::ILocalLocationRatingInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HoursOfOperation(
                            __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILocalLocation2 = __uuidof(ILocalLocation2);
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Services.Maps.LocalSearch.ILocalLocationFinderResult
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.LocalSearch.LocalLocationFinderResult
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_LocalSearch_ILocalLocationFinderResult[] = L"Windows.Services.Maps.LocalSearch.ILocalLocationFinderResult";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    MIDL_INTERFACE("d09b6cc6-f338-4191-9fd8-5440b9a68f52")
                    ILocalLocationFinderResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_LocalLocations(
                            __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Services::Maps::LocalSearch::LocalLocationFinderStatus* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILocalLocationFinderResult = __uuidof(ILocalLocationFinderResult);
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.LocalSearch.ILocalLocationFinderStatics
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.LocalSearch.LocalLocationFinder
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_LocalSearch_ILocalLocationFinderStatics[] = L"Windows.Services.Maps.LocalSearch.ILocalLocationFinderStatics";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    MIDL_INTERFACE("d2ef7344-a0de-48ca-81a8-07c7dcfd37ab")
                    ILocalLocationFinderStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE FindLocalLocationsAsync(
                            HSTRING searchTerm,
                            ABI::Windows::Devices::Geolocation::IGeocircle* searchArea,
                            HSTRING localCategory,
                            UINT32 maxResults,
                            __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILocalLocationFinderStatics = __uuidof(ILocalLocationFinderStatics);
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.LocalSearch.ILocalLocationHoursOfOperationItem
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.LocalSearch.LocalLocationHoursOfOperationItem
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_LocalSearch_ILocalLocationHoursOfOperationItem[] = L"Windows.Services.Maps.LocalSearch.ILocalLocationHoursOfOperationItem";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    MIDL_INTERFACE("23548c72-a1c7-43f1-a4f0-1091c39ec640")
                    ILocalLocationHoursOfOperationItem : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Day(
                            ABI::Windows::Globalization::DayOfWeek* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Start(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Span(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILocalLocationHoursOfOperationItem = __uuidof(ILocalLocationHoursOfOperationItem);
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Services.Maps.LocalSearch.ILocalLocationRatingInfo
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.LocalSearch.LocalLocationRatingInfo
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_LocalSearch_ILocalLocationRatingInfo[] = L"Windows.Services.Maps.LocalSearch.ILocalLocationRatingInfo";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    MIDL_INTERFACE("cb1dab56-3354-4311-8bc0-a2d4d5eb806e")
                    ILocalLocationRatingInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AggregateRating(
                            __FIReference_1_double** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RatingCount(
                            __FIReference_1_int** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ProviderIdentifier(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILocalLocationRatingInfo = __uuidof(ILocalLocationRatingInfo);
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Services.Maps.LocalSearch.IPlaceInfoHelperStatics
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.LocalSearch.PlaceInfoHelper
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_LocalSearch_IPlaceInfoHelperStatics[] = L"Windows.Services.Maps.LocalSearch.IPlaceInfoHelperStatics";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Maps {
                namespace LocalSearch {
                    MIDL_INTERFACE("dd1ca9a7-a9c6-491b-bc09-e80fcea48ee6")
                    IPlaceInfoHelperStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateFromLocalLocation(
                            ABI::Windows::Services::Maps::LocalSearch::ILocalLocation* location,
                            ABI::Windows::Services::Maps::IPlaceInfo** resultValue
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlaceInfoHelperStatics = __uuidof(IPlaceInfoHelperStatics);
                } /* LocalSearch */
            } /* Maps */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Services.Maps.LocalSearch.LocalCategories
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.LocalSearch.ILocalCategoriesStatics interface starting with version 1.0 of the Windows.Services.Maps.LocalSearchContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalCategories_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalCategories_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_LocalSearch_LocalCategories[] = L"Windows.Services.Maps.LocalSearch.LocalCategories";
#endif
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.LocalSearch.LocalLocation
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.LocalSearch.ILocalLocation ** Default Interface **
 *    Windows.Services.Maps.LocalSearch.ILocalLocation2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocation_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_LocalSearch_LocalLocation[] = L"Windows.Services.Maps.LocalSearch.LocalLocation";
#endif
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.LocalSearch.LocalLocationFinder
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.LocalSearch.ILocalLocationFinderStatics interface starting with version 1.0 of the Windows.Services.Maps.LocalSearchContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocationFinder_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocationFinder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_LocalSearch_LocalLocationFinder[] = L"Windows.Services.Maps.LocalSearch.LocalLocationFinder";
#endif
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.LocalSearch.LocalLocationFinderResult
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.LocalSearch.ILocalLocationFinderResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocationFinderResult_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocationFinderResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_LocalSearch_LocalLocationFinderResult[] = L"Windows.Services.Maps.LocalSearch.LocalLocationFinderResult";
#endif
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.LocalSearch.LocalLocationHoursOfOperationItem
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.LocalSearch.ILocalLocationHoursOfOperationItem ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocationHoursOfOperationItem_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocationHoursOfOperationItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_LocalSearch_LocalLocationHoursOfOperationItem[] = L"Windows.Services.Maps.LocalSearch.LocalLocationHoursOfOperationItem";
#endif
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Services.Maps.LocalSearch.LocalLocationRatingInfo
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.LocalSearch.ILocalLocationRatingInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocationRatingInfo_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocationRatingInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_LocalSearch_LocalLocationRatingInfo[] = L"Windows.Services.Maps.LocalSearch.LocalLocationRatingInfo";
#endif
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Services.Maps.LocalSearch.PlaceInfoHelper
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.LocalSearch.IPlaceInfoHelperStatics interface starting with version 4.0 of the Windows.Services.Maps.LocalSearchContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Services_Maps_LocalSearch_PlaceInfoHelper_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_LocalSearch_PlaceInfoHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_LocalSearch_PlaceInfoHelper[] = L"Windows.Services.Maps.LocalSearch.PlaceInfoHelper";
#endif
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics;

#endif // ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation;

#endif // ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2 __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2;

#endif // ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult;

#endif // ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics;

#endif // ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem;

#endif // ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo;

#endif // ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics;

#endif // ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult;

#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult;

typedef struct __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult* This,
        __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResultVtbl;

interface __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult* This,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation;

typedef struct __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationVtbl;

interface __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation
{
    CONST_VTBL struct __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation;

typedef struct __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation** result);

    END_INTERFACE
} __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationVtbl;

interface __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation
{
    CONST_VTBL struct __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem;

typedef struct __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItemVtbl;

interface __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem
{
    CONST_VTBL struct __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000

#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem;

typedef struct __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        __FIIterator_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem** result);

    END_INTERFACE
} __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItemVtbl;

interface __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem
{
    CONST_VTBL struct __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000

#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation;

typedef struct __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        UINT32 index,
        __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationVtbl;

interface __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation
{
    CONST_VTBL struct __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem;

typedef struct __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        UINT32 index,
        __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItemVtbl;

interface __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem
{
    CONST_VTBL struct __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000

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

#if !defined(____FIReference_1_int_INTERFACE_DEFINED__)
#define ____FIReference_1_int_INTERFACE_DEFINED__

typedef interface __FIReference_1_int __FIReference_1_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_int;

typedef struct __FIReference_1_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_int* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_int* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_int* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_int* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_int* This,
        INT32* result);

    END_INTERFACE
} __FIReference_1_intVtbl;

interface __FIReference_1_int
{
    CONST_VTBL struct __FIReference_1_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_int_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_int_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_int_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_int_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_int_INTERFACE_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef enum __x_ABI_CWindows_CGlobalization_CDayOfWeek __x_ABI_CWindows_CGlobalization_CDayOfWeek;

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIMapAddress_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIMapAddress_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIMapAddress __x_ABI_CWindows_CServices_CMaps_CIMapAddress;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIMapAddress_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo;

#endif // ____x_ABI_CWindows_CServices_CMaps_CIPlaceInfo_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CLocalLocationFinderStatus __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CLocalLocationFinderStatus;

/*
 *
 * Struct Windows.Services.Maps.LocalSearch.LocalLocationFinderStatus
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CLocalLocationFinderStatus
{
    LocalLocationFinderStatus_Success = 0,
    LocalLocationFinderStatus_UnknownError = 1,
    LocalLocationFinderStatus_InvalidCredentials = 2,
    LocalLocationFinderStatus_InvalidCategory = 3,
    LocalLocationFinderStatus_InvalidSearchTerm = 4,
    LocalLocationFinderStatus_InvalidSearchArea = 5,
    LocalLocationFinderStatus_NetworkFailure = 6,
    LocalLocationFinderStatus_NotSupported = 7,
};
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.LocalSearch.ILocalCategoriesStatics
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.LocalSearch.LocalCategories
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_LocalSearch_ILocalCategoriesStatics[] = L"Windows.Services.Maps.LocalSearch.ILocalCategoriesStatics";
typedef struct __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BankAndCreditUnions)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EatDrink)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Hospitals)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HotelsAndMotels)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_All)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Parking)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SeeDo)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Shop)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStaticsVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_get_BankAndCreditUnions(This, value) \
    ((This)->lpVtbl->get_BankAndCreditUnions(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_get_EatDrink(This, value) \
    ((This)->lpVtbl->get_EatDrink(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_get_Hospitals(This, value) \
    ((This)->lpVtbl->get_Hospitals(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_get_HotelsAndMotels(This, value) \
    ((This)->lpVtbl->get_HotelsAndMotels(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_get_All(This, value) \
    ((This)->lpVtbl->get_All(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_get_Parking(This, value) \
    ((This)->lpVtbl->get_Parking(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_get_SeeDo(This, value) \
    ((This)->lpVtbl->get_SeeDo(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_get_Shop(This, value) \
    ((This)->lpVtbl->get_Shop(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalCategoriesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.LocalSearch.ILocalLocation
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.LocalSearch.LocalLocation
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_LocalSearch_ILocalLocation[] = L"Windows.Services.Maps.LocalSearch.ILocalLocation";
typedef struct __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Address)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation* This,
        __x_ABI_CWindows_CServices_CMaps_CIMapAddress** value);
    HRESULT (STDMETHODCALLTYPE* get_Identifier)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Point)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint** value);
    HRESULT (STDMETHODCALLTYPE* get_PhoneNumber)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DataAttribution)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_get_Address(This, value) \
    ((This)->lpVtbl->get_Address(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_get_Identifier(This, value) \
    ((This)->lpVtbl->get_Identifier(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_get_Point(This, value) \
    ((This)->lpVtbl->get_Point(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_get_PhoneNumber(This, value) \
    ((This)->lpVtbl->get_PhoneNumber(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_get_DataAttribution(This, value) \
    ((This)->lpVtbl->get_DataAttribution(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.LocalSearch.ILocalLocation2
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.LocalSearch.LocalLocation
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_LocalSearch_ILocalLocation2[] = L"Windows.Services.Maps.LocalSearch.ILocalLocation2";
typedef struct __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Category)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RatingInfo)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2* This,
        __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_HoursOfOperation)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2* This,
        __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationHoursOfOperationItem** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2Vtbl;

interface __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_get_Category(This, value) \
    ((This)->lpVtbl->get_Category(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_get_RatingInfo(This, value) \
    ((This)->lpVtbl->get_RatingInfo(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_get_HoursOfOperation(This, value) \
    ((This)->lpVtbl->get_HoursOfOperation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Services.Maps.LocalSearch.ILocalLocationFinderResult
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.LocalSearch.LocalLocationFinderResult
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_LocalSearch_ILocalLocationFinderResult[] = L"Windows.Services.Maps.LocalSearch.ILocalLocationFinderResult";
typedef struct __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LocalLocations)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult* This,
        __FIVectorView_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocation** value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult* This,
        enum __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CLocalLocationFinderStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResultVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_get_LocalLocations(This, value) \
    ((This)->lpVtbl->get_LocalLocations(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.LocalSearch.ILocalLocationFinderStatics
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.LocalSearch.LocalLocationFinder
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_LocalSearch_ILocalLocationFinderStatics[] = L"Windows.Services.Maps.LocalSearch.ILocalLocationFinderStatics";
typedef struct __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindLocalLocationsAsync)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics* This,
        HSTRING searchTerm,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeocircle* searchArea,
        HSTRING localCategory,
        UINT32 maxResults,
        __FIAsyncOperation_1_Windows__CServices__CMaps__CLocalSearch__CLocalLocationFinderResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStaticsVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_FindLocalLocationsAsync(This, searchTerm, searchArea, localCategory, maxResults, result) \
    ((This)->lpVtbl->FindLocalLocationsAsync(This, searchTerm, searchArea, localCategory, maxResults, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationFinderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.Maps.LocalSearch.ILocalLocationHoursOfOperationItem
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.LocalSearch.LocalLocationHoursOfOperationItem
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_LocalSearch_ILocalLocationHoursOfOperationItem[] = L"Windows.Services.Maps.LocalSearch.ILocalLocationHoursOfOperationItem";
typedef struct __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Day)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem* This,
        enum __x_ABI_CWindows_CGlobalization_CDayOfWeek* value);
    HRESULT (STDMETHODCALLTYPE* get_Start)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Span)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItemVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_get_Day(This, value) \
    ((This)->lpVtbl->get_Day(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_get_Start(This, value) \
    ((This)->lpVtbl->get_Start(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_get_Span(This, value) \
    ((This)->lpVtbl->get_Span(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationHoursOfOperationItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Services.Maps.LocalSearch.ILocalLocationRatingInfo
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.LocalSearch.LocalLocationRatingInfo
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_LocalSearch_ILocalLocationRatingInfo[] = L"Windows.Services.Maps.LocalSearch.ILocalLocationRatingInfo";
typedef struct __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AggregateRating)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo* This,
        __FIReference_1_double** value);
    HRESULT (STDMETHODCALLTYPE* get_RatingCount)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* get_ProviderIdentifier)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfoVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_get_AggregateRating(This, value) \
    ((This)->lpVtbl->get_AggregateRating(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_get_RatingCount(This, value) \
    ((This)->lpVtbl->get_RatingCount(This, value))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_get_ProviderIdentifier(This, value) \
    ((This)->lpVtbl->get_ProviderIdentifier(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocationRatingInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Services.Maps.LocalSearch.IPlaceInfoHelperStatics
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Maps.LocalSearch.PlaceInfoHelper
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Maps_LocalSearch_IPlaceInfoHelperStatics[] = L"Windows.Services.Maps.LocalSearch.IPlaceInfoHelperStatics";
typedef struct __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromLocalLocation)(__x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics* This,
        __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CILocalLocation* location,
        __x_ABI_CWindows_CServices_CMaps_CIPlaceInfo** resultValue);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStaticsVtbl;

interface __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_CreateFromLocalLocation(This, location, resultValue) \
    ((This)->lpVtbl->CreateFromLocalLocation(This, location, resultValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CMaps_CLocalSearch_CIPlaceInfoHelperStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Services.Maps.LocalSearch.LocalCategories
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.LocalSearch.ILocalCategoriesStatics interface starting with version 1.0 of the Windows.Services.Maps.LocalSearchContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalCategories_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalCategories_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_LocalSearch_LocalCategories[] = L"Windows.Services.Maps.LocalSearch.LocalCategories";
#endif
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.LocalSearch.LocalLocation
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.LocalSearch.ILocalLocation ** Default Interface **
 *    Windows.Services.Maps.LocalSearch.ILocalLocation2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocation_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_LocalSearch_LocalLocation[] = L"Windows.Services.Maps.LocalSearch.LocalLocation";
#endif
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.LocalSearch.LocalLocationFinder
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.LocalSearch.ILocalLocationFinderStatics interface starting with version 1.0 of the Windows.Services.Maps.LocalSearchContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocationFinder_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocationFinder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_LocalSearch_LocalLocationFinder[] = L"Windows.Services.Maps.LocalSearch.LocalLocationFinder";
#endif
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.LocalSearch.LocalLocationFinderResult
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.LocalSearch.ILocalLocationFinderResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocationFinderResult_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocationFinderResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_LocalSearch_LocalLocationFinderResult[] = L"Windows.Services.Maps.LocalSearch.LocalLocationFinderResult";
#endif
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.Maps.LocalSearch.LocalLocationHoursOfOperationItem
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.LocalSearch.ILocalLocationHoursOfOperationItem ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocationHoursOfOperationItem_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocationHoursOfOperationItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_LocalSearch_LocalLocationHoursOfOperationItem[] = L"Windows.Services.Maps.LocalSearch.LocalLocationHoursOfOperationItem";
#endif
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Services.Maps.LocalSearch.LocalLocationRatingInfo
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.Maps.LocalSearch.ILocalLocationRatingInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocationRatingInfo_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_LocalSearch_LocalLocationRatingInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_LocalSearch_LocalLocationRatingInfo[] = L"Windows.Services.Maps.LocalSearch.LocalLocationRatingInfo";
#endif
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Services.Maps.LocalSearch.PlaceInfoHelper
 *
 * Introduced to Windows.Services.Maps.LocalSearchContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Maps.LocalSearch.IPlaceInfoHelperStatics interface starting with version 4.0 of the Windows.Services.Maps.LocalSearchContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Services_Maps_LocalSearch_PlaceInfoHelper_DEFINED
#define RUNTIMECLASS_Windows_Services_Maps_LocalSearch_PlaceInfoHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Maps_LocalSearch_PlaceInfoHelper[] = L"Windows.Services.Maps.LocalSearch.PlaceInfoHelper";
#endif
#endif // WINDOWS_SERVICES_MAPS_LOCALSEARCHCONTRACT_VERSION >= 0x40000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eservices2Emaps2Elocalsearch_p_h__

#endif // __windows2Eservices2Emaps2Elocalsearch_h__
