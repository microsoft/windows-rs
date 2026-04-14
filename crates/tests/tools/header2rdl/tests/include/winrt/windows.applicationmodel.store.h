
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
#ifndef __windows2Eapplicationmodel2Estore_h__
#define __windows2Eapplicationmodel2Estore_h__
#ifndef __windows2Eapplicationmodel2Estore_p_h__
#define __windows2Eapplicationmodel2Estore_p_h__


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
#include "Windows.Storage.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface ILicenseChangedEventHandler;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler ABI::Windows::ApplicationModel::Store::ILicenseChangedEventHandler

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface ICurrentApp;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp ABI::Windows::ApplicationModel::Store::ICurrentApp

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface ICurrentApp2Statics;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics ABI::Windows::ApplicationModel::Store::ICurrentApp2Statics

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface ICurrentAppSimulator;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator ABI::Windows::ApplicationModel::Store::ICurrentAppSimulator

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface ICurrentAppSimulatorStaticsWithFiltering;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering ABI::Windows::ApplicationModel::Store::ICurrentAppSimulatorStaticsWithFiltering

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface ICurrentAppSimulatorWithCampaignId;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId ABI::Windows::ApplicationModel::Store::ICurrentAppSimulatorWithCampaignId

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface ICurrentAppSimulatorWithConsumables;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables ABI::Windows::ApplicationModel::Store::ICurrentAppSimulatorWithConsumables

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface ICurrentAppStaticsWithFiltering;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering ABI::Windows::ApplicationModel::Store::ICurrentAppStaticsWithFiltering

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface ICurrentAppWithCampaignId;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId ABI::Windows::ApplicationModel::Store::ICurrentAppWithCampaignId

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface ICurrentAppWithConsumables;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables ABI::Windows::ApplicationModel::Store::ICurrentAppWithConsumables

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface ILicenseInformation;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation ABI::Windows::ApplicationModel::Store::ILicenseInformation

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface IListingInformation;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation ABI::Windows::ApplicationModel::Store::IListingInformation

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface IListingInformation2;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2 ABI::Windows::ApplicationModel::Store::IListingInformation2

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface IProductLicense;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense ABI::Windows::ApplicationModel::Store::IProductLicense

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface IProductLicenseWithFulfillment;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment ABI::Windows::ApplicationModel::Store::IProductLicenseWithFulfillment

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface IProductListing;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing ABI::Windows::ApplicationModel::Store::IProductListing

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface IProductListing2;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2 ABI::Windows::ApplicationModel::Store::IProductListing2

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface IProductListingWithConsumables;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables ABI::Windows::ApplicationModel::Store::IProductListingWithConsumables

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface IProductListingWithMetadata;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata ABI::Windows::ApplicationModel::Store::IProductListingWithMetadata

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface IProductPurchaseDisplayProperties;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties ABI::Windows::ApplicationModel::Store::IProductPurchaseDisplayProperties

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface IProductPurchaseDisplayPropertiesFactory;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory ABI::Windows::ApplicationModel::Store::IProductPurchaseDisplayPropertiesFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface IPurchaseResults;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults ABI::Windows::ApplicationModel::Store::IPurchaseResults

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                interface IUnfulfilledConsumable;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable ABI::Windows::ApplicationModel::Store::IUnfulfilledConsumable

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIAsyncOperation_1_HSTRING_USE
#define DEF___FIAsyncOperation_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3e1fe603-f897-5263-b328-0806426b8a79"))
IAsyncOperation<HSTRING> : IAsyncOperation_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<HSTRING> __FIAsyncOperation_1_HSTRING_t;
#define __FIAsyncOperation_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperation_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE
#define DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b79a741f-7fb5-50ae-9e99-911201ec3d41"))
IAsyncOperationCompletedHandler<HSTRING> : IAsyncOperationCompletedHandler_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<HSTRING> __FIAsyncOperationCompletedHandler_1_HSTRING_t;
#define __FIAsyncOperationCompletedHandler_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                typedef enum FulfillmentResult : int FulfillmentResult;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5c8531ac-5d8d-5e07-b6ee-7cab96930e8a"))
IAsyncOperation<enum ABI::Windows::ApplicationModel::Store::FulfillmentResult> : IAsyncOperation_impl<enum ABI::Windows::ApplicationModel::Store::FulfillmentResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Store.FulfillmentResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::ApplicationModel::Store::FulfillmentResult> __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8775acc9-b9ae-5cce-895c-971bf9270892"))
IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::Store::FulfillmentResult> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::ApplicationModel::Store::FulfillmentResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Store.FulfillmentResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::Store::FulfillmentResult> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                class ListingInformation;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("161600f7-6d4f-500d-93a8-09ad6b5ac4ab"))
IAsyncOperation<ABI::Windows::ApplicationModel::Store::ListingInformation*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::ListingInformation*, ABI::Windows::ApplicationModel::Store::IListingInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Store.ListingInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Store::ListingInformation*> __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fdf83922-762e-57dc-b721-c72ee568fd96"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Store::ListingInformation*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::ListingInformation*, ABI::Windows::ApplicationModel::Store::IListingInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Store.ListingInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Store::ListingInformation*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                class PurchaseResults;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("241f6b10-6af6-5164-85eb-bae6bdae0be8"))
IAsyncOperation<ABI::Windows::ApplicationModel::Store::PurchaseResults*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::PurchaseResults*, ABI::Windows::ApplicationModel::Store::IPurchaseResults*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Store.PurchaseResults>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Store::PurchaseResults*> __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("24b6922a-fdb1-5003-ae89-c8bf16ca0143"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Store::PurchaseResults*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::PurchaseResults*, ABI::Windows::ApplicationModel::Store::IPurchaseResults*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Store.PurchaseResults>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Store::PurchaseResults*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                class UnfulfilledConsumable;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cb77cf2e-ef57-5256-9753-214baada2301"))
IIterator<ABI::Windows::ApplicationModel::Store::UnfulfilledConsumable*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::UnfulfilledConsumable*, ABI::Windows::ApplicationModel::Store::IUnfulfilledConsumable*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Store.UnfulfilledConsumable>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Store::UnfulfilledConsumable*> __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_t;
#define __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2f4d1483-dd86-5fdb-8c44-06c98844bf3d"))
IIterable<ABI::Windows::ApplicationModel::Store::UnfulfilledConsumable*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::UnfulfilledConsumable*, ABI::Windows::ApplicationModel::Store::IUnfulfilledConsumable*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Store.UnfulfilledConsumable>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Store::UnfulfilledConsumable*> __FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_t;
#define __FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f0ae5962-e039-5105-bcc5-9b552f13b102"))
IVectorView<ABI::Windows::ApplicationModel::Store::UnfulfilledConsumable*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::UnfulfilledConsumable*, ABI::Windows::ApplicationModel::Store::IUnfulfilledConsumable*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Store.UnfulfilledConsumable>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Store::UnfulfilledConsumable*> __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_t;
#define __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f1cd679a-3a8f-5e1b-82f1-f2fd0916ca3f"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Store.UnfulfilledConsumable>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0451fe11-6b50-54c1-b765-d946b1d5c88b"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Store.UnfulfilledConsumable>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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
        namespace ApplicationModel {
            namespace Store {
                class ProductLicense;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ff71c38c-3e6e-5e60-994b-9201436deed1"))
IKeyValuePair<HSTRING, ABI::Windows::ApplicationModel::Store::ProductLicense*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::ProductLicense*, ABI::Windows::ApplicationModel::Store::IProductLicense*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Store.ProductLicense>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::ApplicationModel::Store::ProductLicense*> __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3f3c06e6-593f-557a-8e3c-45513ac01f7c"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Store.ProductLicense>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ffe05002-6e65-5001-a752-9b27ed3e2839"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Store.ProductLicense>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                class ProductListing;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("61a02c5d-4007-573e-8a01-0259714927df"))
IKeyValuePair<HSTRING, ABI::Windows::ApplicationModel::Store::ProductListing*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::ProductListing*, ABI::Windows::ApplicationModel::Store::IProductListing*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Store.ProductListing>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::ApplicationModel::Store::ProductListing*> __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("142a6937-1ae1-5aad-8ada-9a8ee034e4eb"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Store.ProductListing>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3a7d9a2f-2a82-59f8-bd7c-d691ca169863"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Store.ProductListing>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_USE
#define DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d6434f65-dd84-534c-b579-cc10d69ec30a"))
IMapView<HSTRING, ABI::Windows::ApplicationModel::Store::ProductLicense*> : IMapView_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::ProductLicense*, ABI::Windows::ApplicationModel::Store::IProductLicense*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.ApplicationModel.Store.ProductLicense>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::ApplicationModel::Store::ProductLicense*> __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_t;
#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_USE
#define DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3206de83-efbe-5791-a487-2ac974206ec6"))
IMapView<HSTRING, ABI::Windows::ApplicationModel::Store::ProductListing*> : IMapView_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::ProductListing*, ABI::Windows::ApplicationModel::Store::IProductListing*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.ApplicationModel.Store.ProductListing>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::ApplicationModel::Store::ProductListing*> __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_t;
#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
        namespace Storage {
            class StorageFile;
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFile;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFile ABI::Windows::Storage::IStorageFile

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                typedef enum ProductPurchaseStatus : int ProductPurchaseStatus;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                typedef enum ProductType : int ProductType;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                class LicenseInformation;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                class ProductPurchaseDisplayProperties;
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.Store.FulfillmentResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                enum FulfillmentResult : int
                {
                    FulfillmentResult_Succeeded = 0,
                    FulfillmentResult_NothingToFulfill = 1,
                    FulfillmentResult_PurchasePending = 2,
                    FulfillmentResult_PurchaseReverted = 3,
                    FulfillmentResult_ServerError = 4,
                };
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Store.ProductPurchaseStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                enum ProductPurchaseStatus : int
                {
                    ProductPurchaseStatus_Succeeded = 0,
                    ProductPurchaseStatus_AlreadyPurchased = 1,
                    ProductPurchaseStatus_NotFulfilled = 2,
                    ProductPurchaseStatus_NotPurchased = 3,
                };
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Store.ProductType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                enum ProductType : int
                {
                    ProductType_Unknown = 0,
                    ProductType_Durable = 1,
                    ProductType_Consumable = 2,
                };
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.ApplicationModel.Store.LicenseChangedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("d4a50255-1369-4c36-832f-6f2d88e3659b")
                ILicenseChangedEventHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ILicenseChangedEventHandler = __uuidof(ILicenseChangedEventHandler);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.ICurrentApp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.CurrentApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ICurrentApp[] = L"Windows.ApplicationModel.Store.ICurrentApp";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("d52dc065-da3f-4685-995e-9b482eb5e603")
                ICurrentApp : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LicenseInformation(
                        ABI::Windows::ApplicationModel::Store::ILicenseInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LinkUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AppId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestAppPurchaseAsync(
                        boolean includeReceipt,
                        __FIAsyncOperation_1_HSTRING** requestAppPurchaseOperation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("RequestProductPurchaseAsync(productId, includeReceipt) may be altered or unavailable for releases after Windows 8.1. Instead, use RequestProductPurchaseAsync(productId).")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE RequestProductPurchaseAsync(
                        HSTRING productId,
                        boolean includeReceipt,
                        __FIAsyncOperation_1_HSTRING** requestProductPurchaseOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LoadListingInformationAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation** loadListingOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAppReceiptAsync(
                        __FIAsyncOperation_1_HSTRING** appReceiptOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetProductReceiptAsync(
                        HSTRING productId,
                        __FIAsyncOperation_1_HSTRING** getProductReceiptOperation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICurrentApp = __uuidof(ICurrentApp);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.ICurrentApp2Statics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.CurrentApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ICurrentApp2Statics[] = L"Windows.ApplicationModel.Store.ICurrentApp2Statics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("df4e6e2d-3171-4ad3-8614-2c61244373cb")
                ICurrentApp2Statics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetCustomerPurchaseIdAsync(
                        HSTRING serviceTicket,
                        HSTRING publisherUserId,
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCustomerCollectionsIdAsync(
                        HSTRING serviceTicket,
                        HSTRING publisherUserId,
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICurrentApp2Statics = __uuidof(ICurrentApp2Statics);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Store.ICurrentAppSimulator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.CurrentAppSimulator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ICurrentAppSimulator[] = L"Windows.ApplicationModel.Store.ICurrentAppSimulator";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("f17f9db1-74cd-4787-9787-19866e9a5559")
                ICurrentAppSimulator : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LicenseInformation(
                        ABI::Windows::ApplicationModel::Store::ILicenseInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LinkUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AppId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestAppPurchaseAsync(
                        boolean includeReceipt,
                        __FIAsyncOperation_1_HSTRING** requestAppPurchaseOperation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("RequestProductPurchaseAsync(productId, includeReceipt) may be altered or unavailable for releases after Windows 8.1. Instead, use RequestProductPurchaseAsync(productId).")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE RequestProductPurchaseAsync(
                        HSTRING productId,
                        boolean includeReceipt,
                        __FIAsyncOperation_1_HSTRING** requestProductPurchaseOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LoadListingInformationAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation** loadListingOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAppReceiptAsync(
                        __FIAsyncOperation_1_HSTRING** appReceiptOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetProductReceiptAsync(
                        HSTRING productId,
                        __FIAsyncOperation_1_HSTRING** getProductReceiptOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReloadSimulatorAsync(
                        ABI::Windows::Storage::IStorageFile* simulatorSettingsFile,
                        ABI::Windows::Foundation::IAsyncAction** reloadSimulatorOperation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICurrentAppSimulator = __uuidof(ICurrentAppSimulator);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.ICurrentAppSimulatorStaticsWithFiltering
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.CurrentAppSimulator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ICurrentAppSimulatorStaticsWithFiltering[] = L"Windows.ApplicationModel.Store.ICurrentAppSimulatorStaticsWithFiltering";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("617e70e2-f86f-4b54-9666-dde285092c68")
                ICurrentAppSimulatorStaticsWithFiltering : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE LoadListingInformationByProductIdsAsync(
                        __FIIterable_1_HSTRING* productIds,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation** loadListingOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LoadListingInformationByKeywordsAsync(
                        __FIIterable_1_HSTRING* keywords,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation** loadListingOperation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICurrentAppSimulatorStaticsWithFiltering = __uuidof(ICurrentAppSimulatorStaticsWithFiltering);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.ICurrentAppSimulatorWithCampaignId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.CurrentAppSimulator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ICurrentAppSimulatorWithCampaignId[] = L"Windows.ApplicationModel.Store.ICurrentAppSimulatorWithCampaignId";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("84678a43-df00-4672-a43f-b25b1441cfcf")
                ICurrentAppSimulatorWithCampaignId : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAppPurchaseCampaignIdAsync(
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICurrentAppSimulatorWithCampaignId = __uuidof(ICurrentAppSimulatorWithCampaignId);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.ICurrentAppSimulatorWithConsumables
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.CurrentAppSimulator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ICurrentAppSimulatorWithConsumables[] = L"Windows.ApplicationModel.Store.ICurrentAppSimulatorWithConsumables";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("4e51f0ab-20e7-4412-9b85-59bb78388667")
                ICurrentAppSimulatorWithConsumables : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ReportConsumableFulfillmentAsync(
                        HSTRING productId,
                        GUID transactionId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult** reportConsumableFulfillmentOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestProductPurchaseWithResultsAsync(
                        HSTRING productId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults** requestProductPurchaseWithResultsOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestProductPurchaseWithDisplayPropertiesAsync(
                        HSTRING productId,
                        HSTRING offerId,
                        ABI::Windows::ApplicationModel::Store::IProductPurchaseDisplayProperties* displayProperties,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults** requestProductPurchaseWithDisplayPropertiesOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetUnfulfilledConsumablesAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable** getUnfulfilledConsumablesOperation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICurrentAppSimulatorWithConsumables = __uuidof(ICurrentAppSimulatorWithConsumables);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.ICurrentAppStaticsWithFiltering
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.CurrentApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ICurrentAppStaticsWithFiltering[] = L"Windows.ApplicationModel.Store.ICurrentAppStaticsWithFiltering";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("d36d6542-9085-438e-97ba-a25c976be2fd")
                ICurrentAppStaticsWithFiltering : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE LoadListingInformationByProductIdsAsync(
                        __FIIterable_1_HSTRING* productIds,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation** loadListingOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LoadListingInformationByKeywordsAsync(
                        __FIIterable_1_HSTRING* keywords,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation** loadListingOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReportProductFulfillment(
                        HSTRING productId
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICurrentAppStaticsWithFiltering = __uuidof(ICurrentAppStaticsWithFiltering);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.ICurrentAppWithCampaignId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.CurrentApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ICurrentAppWithCampaignId[] = L"Windows.ApplicationModel.Store.ICurrentAppWithCampaignId";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("312f4cd0-36c1-44a6-b32b-432d608e4dd6")
                ICurrentAppWithCampaignId : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAppPurchaseCampaignIdAsync(
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICurrentAppWithCampaignId = __uuidof(ICurrentAppWithCampaignId);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.ICurrentAppWithConsumables
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.CurrentApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ICurrentAppWithConsumables[] = L"Windows.ApplicationModel.Store.ICurrentAppWithConsumables";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("844e0071-9e4f-4f79-995a-5f91172e6cef")
                ICurrentAppWithConsumables : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ReportConsumableFulfillmentAsync(
                        HSTRING productId,
                        GUID transactionId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult** reportConsumableFulfillmentOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestProductPurchaseWithResultsAsync(
                        HSTRING productId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults** requestProductPurchaseWithResultsOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestProductPurchaseWithDisplayPropertiesAsync(
                        HSTRING productId,
                        HSTRING offerId,
                        ABI::Windows::ApplicationModel::Store::IProductPurchaseDisplayProperties* displayProperties,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults** requestProductPurchaseWithDisplayPropertiesOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetUnfulfilledConsumablesAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable** getUnfulfilledConsumablesOperation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICurrentAppWithConsumables = __uuidof(ICurrentAppWithConsumables);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.ILicenseInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.LicenseInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ILicenseInformation[] = L"Windows.ApplicationModel.Store.ILicenseInformation";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("8eb7dc30-f170-4ed5-8e21-1516da3fd367")
                ILicenseInformation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProductLicenses(
                        __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsActive(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsTrial(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExpirationDate(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_LicenseChanged(
                        ABI::Windows::ApplicationModel::Store::ILicenseChangedEventHandler* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_LicenseChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILicenseInformation = __uuidof(ILicenseInformation);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IListingInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ListingInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IListingInformation[] = L"Windows.ApplicationModel.Store.IListingInformation";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("588b4abf-bc74-4383-b78c-99606323dece")
                IListingInformation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentMarket(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProductListings(
                        __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FormattedPrice(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AgeRating(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IListingInformation = __uuidof(IListingInformation);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IListingInformation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ListingInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IListingInformation2[] = L"Windows.ApplicationModel.Store.IListingInformation2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("c0fd2c1d-b30e-4384-84ea-72fefa82223e")
                IListingInformation2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_FormattedBasePrice(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SaleEndDate(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsOnSale(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrencyCode(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IListingInformation2 = __uuidof(IListingInformation2);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Store.IProductLicense
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ProductLicense
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IProductLicense[] = L"Windows.ApplicationModel.Store.IProductLicense";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("363308c7-2bcf-4c0e-8f2f-e808aaa8f99d")
                IProductLicense : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProductId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsActive(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExpirationDate(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProductLicense = __uuidof(IProductLicense);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IProductLicenseWithFulfillment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ProductLicense
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Store.IProductLicense
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IProductLicenseWithFulfillment[] = L"Windows.ApplicationModel.Store.IProductLicenseWithFulfillment";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("fc535c8a-f667-40f3-ba3c-045a63abb3ac")
                IProductLicenseWithFulfillment : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsConsumable(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProductLicenseWithFulfillment = __uuidof(IProductLicenseWithFulfillment);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IProductListing
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ProductListing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IProductListing[] = L"Windows.ApplicationModel.Store.IProductListing";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("45a7d6ad-c750-4d9c-947c-b00dcbf9e9c2")
                IProductListing : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProductId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FormattedPrice(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProductListing = __uuidof(IProductListing);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIProductListing;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IProductListing2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ProductListing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IProductListing2[] = L"Windows.ApplicationModel.Store.IProductListing2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("f89e290f-73fe-494d-a939-08a9b2495abe")
                IProductListing2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_FormattedBasePrice(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SaleEndDate(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsOnSale(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrencyCode(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProductListing2 = __uuidof(IProductListing2);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Store.IProductListingWithConsumables
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ProductListing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IProductListingWithConsumables[] = L"Windows.ApplicationModel.Store.IProductListingWithConsumables";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("eb9e9790-8f6b-481f-93a7-5c3a63068149")
                IProductListingWithConsumables : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProductType(
                        ABI::Windows::ApplicationModel::Store::ProductType* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProductListingWithConsumables = __uuidof(IProductListingWithConsumables);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IProductListingWithMetadata
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ProductListing
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Store.IProductListing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IProductListingWithMetadata[] = L"Windows.ApplicationModel.Store.IProductListingWithMetadata";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("124da567-23f8-423e-9532-189943c40ace")
                IProductListingWithMetadata : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Keywords(
                        __FIIterable_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProductType(
                        ABI::Windows::ApplicationModel::Store::ProductType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Tag(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ImageUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProductListingWithMetadata = __uuidof(IProductListingWithMetadata);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IProductPurchaseDisplayProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ProductPurchaseDisplayProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IProductPurchaseDisplayProperties[] = L"Windows.ApplicationModel.Store.IProductPurchaseDisplayProperties";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("d70b7420-bc92-401b-a809-c9b2e5dbbdaf")
                IProductPurchaseDisplayProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Name(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Description(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Image(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Image(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProductPurchaseDisplayProperties = __uuidof(IProductPurchaseDisplayProperties);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IProductPurchaseDisplayPropertiesFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ProductPurchaseDisplayProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IProductPurchaseDisplayPropertiesFactory[] = L"Windows.ApplicationModel.Store.IProductPurchaseDisplayPropertiesFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("6f491df4-32d6-4b40-b474-b83038a4d9cf")
                IProductPurchaseDisplayPropertiesFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateProductPurchaseDisplayProperties(
                        HSTRING name,
                        ABI::Windows::ApplicationModel::Store::IProductPurchaseDisplayProperties** displayProperties
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProductPurchaseDisplayPropertiesFactory = __uuidof(IProductPurchaseDisplayPropertiesFactory);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IPurchaseResults
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.PurchaseResults
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IPurchaseResults[] = L"Windows.ApplicationModel.Store.IPurchaseResults";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("ed50b37e-8656-4f65-b8c8-ac7e0cb1a1c2")
                IPurchaseResults : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::ApplicationModel::Store::ProductPurchaseStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransactionId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReceiptXml(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OfferId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPurchaseResults = __uuidof(IPurchaseResults);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IUnfulfilledConsumable
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.UnfulfilledConsumable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IUnfulfilledConsumable[] = L"Windows.ApplicationModel.Store.IUnfulfilledConsumable";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                MIDL_INTERFACE("2df7fbbb-1cdd-4cb8-a014-7b9cf8986927")
                IUnfulfilledConsumable : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProductId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransactionId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OfferId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUnfulfilledConsumable = __uuidof(IUnfulfilledConsumable);
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.CurrentApp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Store.ICurrentAppWithCampaignId interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.ICurrentApp interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.ICurrentAppWithConsumables interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.ICurrentApp2Statics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.ICurrentAppStaticsWithFiltering interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_CurrentApp_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_CurrentApp_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_CurrentApp[] = L"Windows.ApplicationModel.Store.CurrentApp";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.CurrentAppSimulator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Store.ICurrentAppSimulatorWithCampaignId interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.ICurrentAppSimulatorWithConsumables interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.ICurrentAppSimulatorStaticsWithFiltering interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.ICurrentAppSimulator interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_CurrentAppSimulator_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_CurrentAppSimulator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_CurrentAppSimulator[] = L"Windows.ApplicationModel.Store.CurrentAppSimulator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.LicenseInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.ILicenseInformation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_LicenseInformation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_LicenseInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_LicenseInformation[] = L"Windows.ApplicationModel.Store.LicenseInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.ListingInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.IListingInformation ** Default Interface **
 *    Windows.ApplicationModel.Store.IListingInformation2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_ListingInformation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_ListingInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_ListingInformation[] = L"Windows.ApplicationModel.Store.ListingInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.ProductLicense
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.IProductLicense ** Default Interface **
 *    Windows.ApplicationModel.Store.IProductLicenseWithFulfillment
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_ProductLicense_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_ProductLicense_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_ProductLicense[] = L"Windows.ApplicationModel.Store.ProductLicense";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.ProductListing
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.IProductListing ** Default Interface **
 *    Windows.ApplicationModel.Store.IProductListingWithMetadata
 *    Windows.ApplicationModel.Store.IProductListing2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_ProductListing_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_ProductListing_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_ProductListing[] = L"Windows.ApplicationModel.Store.ProductListing";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.ProductPurchaseDisplayProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Store.IProductPurchaseDisplayPropertiesFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.IProductPurchaseDisplayProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_ProductPurchaseDisplayProperties_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_ProductPurchaseDisplayProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_ProductPurchaseDisplayProperties[] = L"Windows.ApplicationModel.Store.ProductPurchaseDisplayProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.PurchaseResults
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.IPurchaseResults ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_PurchaseResults_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_PurchaseResults_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_PurchaseResults[] = L"Windows.ApplicationModel.Store.PurchaseResults";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.UnfulfilledConsumable
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.IUnfulfilledConsumable ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_UnfulfilledConsumable_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_UnfulfilledConsumable_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_UnfulfilledConsumable[] = L"Windows.ApplicationModel.Store.UnfulfilledConsumable";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler __x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2 __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2 __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults __x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_HSTRING __FIAsyncOperationCompletedHandler_1_HSTRING;

#if !defined(____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_HSTRING __FIAsyncOperation_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_HSTRING;

typedef struct __FIAsyncOperation_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1_HSTRING* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIAsyncOperation_1_HSTRINGVtbl;

interface __FIAsyncOperation_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperation_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_HSTRING_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_HSTRING_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_HSTRING_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_HSTRING __FIAsyncOperationCompletedHandler_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_HSTRING;

typedef struct __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_HSTRING* This,
        __FIAsyncOperation_1_HSTRING* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl;

interface __FIAsyncOperationCompletedHandler_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CStore_CFulfillmentResult __x_ABI_CWindows_CApplicationModel_CStore_CFulfillmentResult;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CFulfillmentResult* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CFulfillmentResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformationVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformationVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CListingInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResultsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResultsVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResultsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResultsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResultsVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResultsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPurchaseResults_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable;

typedef struct __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumableVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable __FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable;

typedef struct __FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        __FIIterator_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumableVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumableVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        __FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumableVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumableVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicenseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicenseVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicenseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicenseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicenseVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicenseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicenseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicenseVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicenseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListingVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListingVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListingVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense;

typedef struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicenseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        HSTRING key,
        __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense** first,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicenseVtbl;

interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicenseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing;

typedef struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        HSTRING key,
        __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing** first,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListingVtbl;

interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CStore_CProductPurchaseStatus __x_ABI_CWindows_CApplicationModel_CStore_CProductPurchaseStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CStore_CProductType __x_ABI_CWindows_CApplicationModel_CStore_CProductType;

/*
 *
 * Struct Windows.ApplicationModel.Store.FulfillmentResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CStore_CFulfillmentResult
{
    FulfillmentResult_Succeeded = 0,
    FulfillmentResult_NothingToFulfill = 1,
    FulfillmentResult_PurchasePending = 2,
    FulfillmentResult_PurchaseReverted = 3,
    FulfillmentResult_ServerError = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Store.ProductPurchaseStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CStore_CProductPurchaseStatus
{
    ProductPurchaseStatus_Succeeded = 0,
    ProductPurchaseStatus_AlreadyPurchased = 1,
    ProductPurchaseStatus_NotFulfilled = 2,
    ProductPurchaseStatus_NotPurchased = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Store.ProductType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CStore_CProductType
{
    ProductType_Unknown = 0,
    ProductType_Durable = 1,
    ProductType_Consumable = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.ApplicationModel.Store.LicenseChangedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandlerVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler_Invoke(This) \
    ((This)->lpVtbl->Invoke(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.ICurrentApp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.CurrentApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ICurrentApp[] = L"Windows.ApplicationModel.Store.ICurrentApp";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LicenseInformation)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation** value);
    HRESULT (STDMETHODCALLTYPE* get_LinkUri)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_AppId)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* RequestAppPurchaseAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp* This,
        boolean includeReceipt,
        __FIAsyncOperation_1_HSTRING** requestAppPurchaseOperation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("RequestProductPurchaseAsync(productId, includeReceipt) may be altered or unavailable for releases after Windows 8.1. Instead, use RequestProductPurchaseAsync(productId).")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* RequestProductPurchaseAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp* This,
        HSTRING productId,
        boolean includeReceipt,
        __FIAsyncOperation_1_HSTRING** requestProductPurchaseOperation);
    HRESULT (STDMETHODCALLTYPE* LoadListingInformationAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation** loadListingOperation);
    HRESULT (STDMETHODCALLTYPE* GetAppReceiptAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp* This,
        __FIAsyncOperation_1_HSTRING** appReceiptOperation);
    HRESULT (STDMETHODCALLTYPE* GetProductReceiptAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp* This,
        HSTRING productId,
        __FIAsyncOperation_1_HSTRING** getProductReceiptOperation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_get_LicenseInformation(This, value) \
    ((This)->lpVtbl->get_LicenseInformation(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_get_LinkUri(This, value) \
    ((This)->lpVtbl->get_LinkUri(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_get_AppId(This, value) \
    ((This)->lpVtbl->get_AppId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_RequestAppPurchaseAsync(This, includeReceipt, requestAppPurchaseOperation) \
    ((This)->lpVtbl->RequestAppPurchaseAsync(This, includeReceipt, requestAppPurchaseOperation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("RequestProductPurchaseAsync(productId, includeReceipt) may be altered or unavailable for releases after Windows 8.1. Instead, use RequestProductPurchaseAsync(productId).")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_RequestProductPurchaseAsync(This, productId, includeReceipt, requestProductPurchaseOperation) \
    ((This)->lpVtbl->RequestProductPurchaseAsync(This, productId, includeReceipt, requestProductPurchaseOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_LoadListingInformationAsync(This, loadListingOperation) \
    ((This)->lpVtbl->LoadListingInformationAsync(This, loadListingOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_GetAppReceiptAsync(This, appReceiptOperation) \
    ((This)->lpVtbl->GetAppReceiptAsync(This, appReceiptOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_GetProductReceiptAsync(This, productId, getProductReceiptOperation) \
    ((This)->lpVtbl->GetProductReceiptAsync(This, productId, getProductReceiptOperation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.ICurrentApp2Statics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.CurrentApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ICurrentApp2Statics[] = L"Windows.ApplicationModel.Store.ICurrentApp2Statics";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2StaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCustomerPurchaseIdAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics* This,
        HSTRING serviceTicket,
        HSTRING publisherUserId,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* GetCustomerCollectionsIdAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics* This,
        HSTRING serviceTicket,
        HSTRING publisherUserId,
        __FIAsyncOperation_1_HSTRING** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2StaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2StaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_GetCustomerPurchaseIdAsync(This, serviceTicket, publisherUserId, operation) \
    ((This)->lpVtbl->GetCustomerPurchaseIdAsync(This, serviceTicket, publisherUserId, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_GetCustomerCollectionsIdAsync(This, serviceTicket, publisherUserId, operation) \
    ((This)->lpVtbl->GetCustomerCollectionsIdAsync(This, serviceTicket, publisherUserId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentApp2Statics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Store.ICurrentAppSimulator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.CurrentAppSimulator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ICurrentAppSimulator[] = L"Windows.ApplicationModel.Store.ICurrentAppSimulator";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LicenseInformation)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation** value);
    HRESULT (STDMETHODCALLTYPE* get_LinkUri)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_AppId)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* RequestAppPurchaseAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator* This,
        boolean includeReceipt,
        __FIAsyncOperation_1_HSTRING** requestAppPurchaseOperation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("RequestProductPurchaseAsync(productId, includeReceipt) may be altered or unavailable for releases after Windows 8.1. Instead, use RequestProductPurchaseAsync(productId).")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* RequestProductPurchaseAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator* This,
        HSTRING productId,
        boolean includeReceipt,
        __FIAsyncOperation_1_HSTRING** requestProductPurchaseOperation);
    HRESULT (STDMETHODCALLTYPE* LoadListingInformationAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation** loadListingOperation);
    HRESULT (STDMETHODCALLTYPE* GetAppReceiptAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator* This,
        __FIAsyncOperation_1_HSTRING** appReceiptOperation);
    HRESULT (STDMETHODCALLTYPE* GetProductReceiptAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator* This,
        HSTRING productId,
        __FIAsyncOperation_1_HSTRING** getProductReceiptOperation);
    HRESULT (STDMETHODCALLTYPE* ReloadSimulatorAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* simulatorSettingsFile,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** reloadSimulatorOperation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_get_LicenseInformation(This, value) \
    ((This)->lpVtbl->get_LicenseInformation(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_get_LinkUri(This, value) \
    ((This)->lpVtbl->get_LinkUri(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_get_AppId(This, value) \
    ((This)->lpVtbl->get_AppId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_RequestAppPurchaseAsync(This, includeReceipt, requestAppPurchaseOperation) \
    ((This)->lpVtbl->RequestAppPurchaseAsync(This, includeReceipt, requestAppPurchaseOperation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("RequestProductPurchaseAsync(productId, includeReceipt) may be altered or unavailable for releases after Windows 8.1. Instead, use RequestProductPurchaseAsync(productId).")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_RequestProductPurchaseAsync(This, productId, includeReceipt, requestProductPurchaseOperation) \
    ((This)->lpVtbl->RequestProductPurchaseAsync(This, productId, includeReceipt, requestProductPurchaseOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_LoadListingInformationAsync(This, loadListingOperation) \
    ((This)->lpVtbl->LoadListingInformationAsync(This, loadListingOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_GetAppReceiptAsync(This, appReceiptOperation) \
    ((This)->lpVtbl->GetAppReceiptAsync(This, appReceiptOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_GetProductReceiptAsync(This, productId, getProductReceiptOperation) \
    ((This)->lpVtbl->GetProductReceiptAsync(This, productId, getProductReceiptOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_ReloadSimulatorAsync(This, simulatorSettingsFile, reloadSimulatorOperation) \
    ((This)->lpVtbl->ReloadSimulatorAsync(This, simulatorSettingsFile, reloadSimulatorOperation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.ICurrentAppSimulatorStaticsWithFiltering
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.CurrentAppSimulator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ICurrentAppSimulatorStaticsWithFiltering[] = L"Windows.ApplicationModel.Store.ICurrentAppSimulatorStaticsWithFiltering";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFilteringVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* LoadListingInformationByProductIdsAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering* This,
        __FIIterable_1_HSTRING* productIds,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation** loadListingOperation);
    HRESULT (STDMETHODCALLTYPE* LoadListingInformationByKeywordsAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering* This,
        __FIIterable_1_HSTRING* keywords,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation** loadListingOperation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFilteringVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFilteringVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_LoadListingInformationByProductIdsAsync(This, productIds, loadListingOperation) \
    ((This)->lpVtbl->LoadListingInformationByProductIdsAsync(This, productIds, loadListingOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_LoadListingInformationByKeywordsAsync(This, keywords, loadListingOperation) \
    ((This)->lpVtbl->LoadListingInformationByKeywordsAsync(This, keywords, loadListingOperation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorStaticsWithFiltering_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.ICurrentAppSimulatorWithCampaignId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.CurrentAppSimulator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ICurrentAppSimulatorWithCampaignId[] = L"Windows.ApplicationModel.Store.ICurrentAppSimulatorWithCampaignId";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignIdVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAppPurchaseCampaignIdAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId* This,
        __FIAsyncOperation_1_HSTRING** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignIdVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignIdVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_GetAppPurchaseCampaignIdAsync(This, operation) \
    ((This)->lpVtbl->GetAppPurchaseCampaignIdAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithCampaignId_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.ICurrentAppSimulatorWithConsumables
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.CurrentAppSimulator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ICurrentAppSimulatorWithConsumables[] = L"Windows.ApplicationModel.Store.ICurrentAppSimulatorWithConsumables";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumablesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportConsumableFulfillmentAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables* This,
        HSTRING productId,
        GUID transactionId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult** reportConsumableFulfillmentOperation);
    HRESULT (STDMETHODCALLTYPE* RequestProductPurchaseWithResultsAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables* This,
        HSTRING productId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults** requestProductPurchaseWithResultsOperation);
    HRESULT (STDMETHODCALLTYPE* RequestProductPurchaseWithDisplayPropertiesAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables* This,
        HSTRING productId,
        HSTRING offerId,
        __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties* displayProperties,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults** requestProductPurchaseWithDisplayPropertiesOperation);
    HRESULT (STDMETHODCALLTYPE* GetUnfulfilledConsumablesAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable** getUnfulfilledConsumablesOperation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumablesVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumablesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_ReportConsumableFulfillmentAsync(This, productId, transactionId, reportConsumableFulfillmentOperation) \
    ((This)->lpVtbl->ReportConsumableFulfillmentAsync(This, productId, transactionId, reportConsumableFulfillmentOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_RequestProductPurchaseWithResultsAsync(This, productId, requestProductPurchaseWithResultsOperation) \
    ((This)->lpVtbl->RequestProductPurchaseWithResultsAsync(This, productId, requestProductPurchaseWithResultsOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_RequestProductPurchaseWithDisplayPropertiesAsync(This, productId, offerId, displayProperties, requestProductPurchaseWithDisplayPropertiesOperation) \
    ((This)->lpVtbl->RequestProductPurchaseWithDisplayPropertiesAsync(This, productId, offerId, displayProperties, requestProductPurchaseWithDisplayPropertiesOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_GetUnfulfilledConsumablesAsync(This, getUnfulfilledConsumablesOperation) \
    ((This)->lpVtbl->GetUnfulfilledConsumablesAsync(This, getUnfulfilledConsumablesOperation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppSimulatorWithConsumables_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.ICurrentAppStaticsWithFiltering
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.CurrentApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ICurrentAppStaticsWithFiltering[] = L"Windows.ApplicationModel.Store.ICurrentAppStaticsWithFiltering";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFilteringVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* LoadListingInformationByProductIdsAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering* This,
        __FIIterable_1_HSTRING* productIds,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation** loadListingOperation);
    HRESULT (STDMETHODCALLTYPE* LoadListingInformationByKeywordsAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering* This,
        __FIIterable_1_HSTRING* keywords,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CListingInformation** loadListingOperation);
    HRESULT (STDMETHODCALLTYPE* ReportProductFulfillment)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering* This,
        HSTRING productId);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFilteringVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFilteringVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_LoadListingInformationByProductIdsAsync(This, productIds, loadListingOperation) \
    ((This)->lpVtbl->LoadListingInformationByProductIdsAsync(This, productIds, loadListingOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_LoadListingInformationByKeywordsAsync(This, keywords, loadListingOperation) \
    ((This)->lpVtbl->LoadListingInformationByKeywordsAsync(This, keywords, loadListingOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_ReportProductFulfillment(This, productId) \
    ((This)->lpVtbl->ReportProductFulfillment(This, productId))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppStaticsWithFiltering_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.ICurrentAppWithCampaignId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.CurrentApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ICurrentAppWithCampaignId[] = L"Windows.ApplicationModel.Store.ICurrentAppWithCampaignId";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignIdVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAppPurchaseCampaignIdAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId* This,
        __FIAsyncOperation_1_HSTRING** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignIdVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignIdVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_GetAppPurchaseCampaignIdAsync(This, operation) \
    ((This)->lpVtbl->GetAppPurchaseCampaignIdAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithCampaignId_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.ICurrentAppWithConsumables
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.CurrentApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ICurrentAppWithConsumables[] = L"Windows.ApplicationModel.Store.ICurrentAppWithConsumables";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumablesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportConsumableFulfillmentAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables* This,
        HSTRING productId,
        GUID transactionId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CFulfillmentResult** reportConsumableFulfillmentOperation);
    HRESULT (STDMETHODCALLTYPE* RequestProductPurchaseWithResultsAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables* This,
        HSTRING productId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults** requestProductPurchaseWithResultsOperation);
    HRESULT (STDMETHODCALLTYPE* RequestProductPurchaseWithDisplayPropertiesAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables* This,
        HSTRING productId,
        HSTRING offerId,
        __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties* displayProperties,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPurchaseResults** requestProductPurchaseWithDisplayPropertiesOperation);
    HRESULT (STDMETHODCALLTYPE* GetUnfulfilledConsumablesAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CUnfulfilledConsumable** getUnfulfilledConsumablesOperation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumablesVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumablesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_ReportConsumableFulfillmentAsync(This, productId, transactionId, reportConsumableFulfillmentOperation) \
    ((This)->lpVtbl->ReportConsumableFulfillmentAsync(This, productId, transactionId, reportConsumableFulfillmentOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_RequestProductPurchaseWithResultsAsync(This, productId, requestProductPurchaseWithResultsOperation) \
    ((This)->lpVtbl->RequestProductPurchaseWithResultsAsync(This, productId, requestProductPurchaseWithResultsOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_RequestProductPurchaseWithDisplayPropertiesAsync(This, productId, offerId, displayProperties, requestProductPurchaseWithDisplayPropertiesOperation) \
    ((This)->lpVtbl->RequestProductPurchaseWithDisplayPropertiesAsync(This, productId, offerId, displayProperties, requestProductPurchaseWithDisplayPropertiesOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_GetUnfulfilledConsumablesAsync(This, getUnfulfilledConsumablesOperation) \
    ((This)->lpVtbl->GetUnfulfilledConsumablesAsync(This, getUnfulfilledConsumablesOperation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CICurrentAppWithConsumables_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.ILicenseInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.LicenseInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_ILicenseInformation[] = L"Windows.ApplicationModel.Store.ILicenseInformation";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProductLicenses)(__x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductLicense** value);
    HRESULT (STDMETHODCALLTYPE* get_IsActive)(__x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsTrial)(__x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ExpirationDate)(__x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* add_LicenseChanged)(__x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CILicenseChangedEventHandler* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_LicenseChanged)(__x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_get_ProductLicenses(This, value) \
    ((This)->lpVtbl->get_ProductLicenses(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_get_IsActive(This, value) \
    ((This)->lpVtbl->get_IsActive(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_get_IsTrial(This, value) \
    ((This)->lpVtbl->get_IsTrial(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_get_ExpirationDate(This, value) \
    ((This)->lpVtbl->get_ExpirationDate(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_add_LicenseChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_LicenseChanged(This, handler, cookie))

#define __x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_remove_LicenseChanged(This, cookie) \
    ((This)->lpVtbl->remove_LicenseChanged(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CILicenseInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IListingInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ListingInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IListingInformation[] = L"Windows.ApplicationModel.Store.IListingInformation";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CurrentMarket)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ProductListings)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CProductListing** value);
    HRESULT (STDMETHODCALLTYPE* get_FormattedPrice)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AgeRating)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_get_CurrentMarket(This, value) \
    ((This)->lpVtbl->get_CurrentMarket(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_get_ProductListings(This, value) \
    ((This)->lpVtbl->get_ProductListings(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_get_FormattedPrice(This, value) \
    ((This)->lpVtbl->get_FormattedPrice(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_get_AgeRating(This, value) \
    ((This)->lpVtbl->get_AgeRating(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IListingInformation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ListingInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IListingInformation2[] = L"Windows.ApplicationModel.Store.IListingInformation2";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FormattedBasePrice)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SaleEndDate)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_IsOnSale)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_CurrencyCode)(__x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_get_FormattedBasePrice(This, value) \
    ((This)->lpVtbl->get_FormattedBasePrice(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_get_SaleEndDate(This, value) \
    ((This)->lpVtbl->get_SaleEndDate(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_get_IsOnSale(This, value) \
    ((This)->lpVtbl->get_IsOnSale(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_get_CurrencyCode(This, value) \
    ((This)->lpVtbl->get_CurrencyCode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIListingInformation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Store.IProductLicense
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ProductLicense
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IProductLicense[] = L"Windows.ApplicationModel.Store.IProductLicense";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProductId)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsActive)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ExpirationDate)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_get_ProductId(This, value) \
    ((This)->lpVtbl->get_ProductId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_get_IsActive(This, value) \
    ((This)->lpVtbl->get_IsActive(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_get_ExpirationDate(This, value) \
    ((This)->lpVtbl->get_ExpirationDate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicense_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IProductLicenseWithFulfillment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ProductLicense
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Store.IProductLicense
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IProductLicenseWithFulfillment[] = L"Windows.ApplicationModel.Store.IProductLicenseWithFulfillment";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsConsumable)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillmentVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_get_IsConsumable(This, value) \
    ((This)->lpVtbl->get_IsConsumable(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductLicenseWithFulfillment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IProductListing
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ProductListing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IProductListing[] = L"Windows.ApplicationModel.Store.IProductListing";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProductId)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FormattedPrice)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_get_ProductId(This, value) \
    ((This)->lpVtbl->get_ProductId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_get_FormattedPrice(This, value) \
    ((This)->lpVtbl->get_FormattedPrice(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIProductListing;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IProductListing2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ProductListing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IProductListing2[] = L"Windows.ApplicationModel.Store.IProductListing2";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FormattedBasePrice)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SaleEndDate)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_IsOnSale)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_CurrencyCode)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_get_FormattedBasePrice(This, value) \
    ((This)->lpVtbl->get_FormattedBasePrice(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_get_SaleEndDate(This, value) \
    ((This)->lpVtbl->get_SaleEndDate(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_get_IsOnSale(This, value) \
    ((This)->lpVtbl->get_IsOnSale(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_get_CurrencyCode(This, value) \
    ((This)->lpVtbl->get_CurrencyCode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductListing2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Store.IProductListingWithConsumables
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ProductListing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IProductListingWithConsumables[] = L"Windows.ApplicationModel.Store.IProductListingWithConsumables";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumablesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProductType)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CProductType* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumablesVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumablesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_get_ProductType(This, value) \
    ((This)->lpVtbl->get_ProductType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithConsumables_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IProductListingWithMetadata
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ProductListing
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Store.IProductListing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IProductListingWithMetadata[] = L"Windows.ApplicationModel.Store.IProductListingWithMetadata";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Keywords)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata* This,
        __FIIterable_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_ProductType)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CProductType* value);
    HRESULT (STDMETHODCALLTYPE* get_Tag)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ImageUri)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadataVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_get_Keywords(This, value) \
    ((This)->lpVtbl->get_Keywords(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_get_ProductType(This, value) \
    ((This)->lpVtbl->get_ProductType(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_get_Tag(This, value) \
    ((This)->lpVtbl->get_Tag(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_get_ImageUri(This, value) \
    ((This)->lpVtbl->get_ImageUri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductListingWithMetadata_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IProductPurchaseDisplayProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ProductPurchaseDisplayProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IProductPurchaseDisplayProperties[] = L"Windows.ApplicationModel.Store.IProductPurchaseDisplayProperties";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Name)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Image)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_Image)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_put_Name(This, value) \
    ((This)->lpVtbl->put_Name(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_get_Image(This, value) \
    ((This)->lpVtbl->get_Image(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_put_Image(This, value) \
    ((This)->lpVtbl->put_Image(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IProductPurchaseDisplayPropertiesFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.ProductPurchaseDisplayProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IProductPurchaseDisplayPropertiesFactory[] = L"Windows.ApplicationModel.Store.IProductPurchaseDisplayPropertiesFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateProductPurchaseDisplayProperties)(__x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory* This,
        HSTRING name,
        __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayProperties** displayProperties);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_CreateProductPurchaseDisplayProperties(This, name, displayProperties) \
    ((This)->lpVtbl->CreateProductPurchaseDisplayProperties(This, name, displayProperties))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIProductPurchaseDisplayPropertiesFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IPurchaseResults
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.PurchaseResults
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IPurchaseResults[] = L"Windows.ApplicationModel.Store.IPurchaseResults";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResultsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CProductPurchaseStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_TransactionId)(__x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_ReceiptXml)(__x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_OfferId)(__x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResultsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResultsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_get_TransactionId(This, value) \
    ((This)->lpVtbl->get_TransactionId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_get_ReceiptXml(This, value) \
    ((This)->lpVtbl->get_ReceiptXml(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_get_OfferId(This, value) \
    ((This)->lpVtbl->get_OfferId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIPurchaseResults_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.IUnfulfilledConsumable
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.UnfulfilledConsumable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_IUnfulfilledConsumable[] = L"Windows.ApplicationModel.Store.IUnfulfilledConsumable";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProductId)(__x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TransactionId)(__x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_OfferId)(__x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumableVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_get_ProductId(This, value) \
    ((This)->lpVtbl->get_ProductId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_get_TransactionId(This, value) \
    ((This)->lpVtbl->get_TransactionId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_get_OfferId(This, value) \
    ((This)->lpVtbl->get_OfferId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CIUnfulfilledConsumable_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.CurrentApp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Store.ICurrentAppWithCampaignId interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.ICurrentApp interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.ICurrentAppWithConsumables interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.ICurrentApp2Statics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.ICurrentAppStaticsWithFiltering interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_CurrentApp_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_CurrentApp_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_CurrentApp[] = L"Windows.ApplicationModel.Store.CurrentApp";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.CurrentAppSimulator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Store.ICurrentAppSimulatorWithCampaignId interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.ICurrentAppSimulatorWithConsumables interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.ICurrentAppSimulatorStaticsWithFiltering interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.ICurrentAppSimulator interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_CurrentAppSimulator_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_CurrentAppSimulator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_CurrentAppSimulator[] = L"Windows.ApplicationModel.Store.CurrentAppSimulator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.LicenseInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.ILicenseInformation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_LicenseInformation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_LicenseInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_LicenseInformation[] = L"Windows.ApplicationModel.Store.LicenseInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.ListingInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.IListingInformation ** Default Interface **
 *    Windows.ApplicationModel.Store.IListingInformation2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_ListingInformation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_ListingInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_ListingInformation[] = L"Windows.ApplicationModel.Store.ListingInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.ProductLicense
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.IProductLicense ** Default Interface **
 *    Windows.ApplicationModel.Store.IProductLicenseWithFulfillment
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_ProductLicense_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_ProductLicense_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_ProductLicense[] = L"Windows.ApplicationModel.Store.ProductLicense";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.ProductListing
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.IProductListing ** Default Interface **
 *    Windows.ApplicationModel.Store.IProductListingWithMetadata
 *    Windows.ApplicationModel.Store.IProductListing2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_ProductListing_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_ProductListing_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_ProductListing[] = L"Windows.ApplicationModel.Store.ProductListing";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.ProductPurchaseDisplayProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Store.IProductPurchaseDisplayPropertiesFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.IProductPurchaseDisplayProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_ProductPurchaseDisplayProperties_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_ProductPurchaseDisplayProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_ProductPurchaseDisplayProperties[] = L"Windows.ApplicationModel.Store.ProductPurchaseDisplayProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.PurchaseResults
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.IPurchaseResults ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_PurchaseResults_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_PurchaseResults_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_PurchaseResults[] = L"Windows.ApplicationModel.Store.PurchaseResults";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.UnfulfilledConsumable
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.IUnfulfilledConsumable ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_UnfulfilledConsumable_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_UnfulfilledConsumable_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_UnfulfilledConsumable[] = L"Windows.ApplicationModel.Store.UnfulfilledConsumable";
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
#endif // __windows2Eapplicationmodel2Estore_p_h__

#endif // __windows2Eapplicationmodel2Estore_h__
