
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
#ifndef __windows2Eapplicationmodel2Estore2Epreview_h__
#define __windows2Eapplicationmodel2Estore2Epreview_h__
#ifndef __windows2Eapplicationmodel2Estore2Epreview_p_h__
#define __windows2Eapplicationmodel2Estore2Epreview_p_h__


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
#include "Windows.Security.Authentication.Web.Core.h"
#include "Windows.Security.Credentials.h"
#include "Windows.Storage.Streams.h"
#include "Windows.System.h"
#include "Windows.UI.Xaml.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    interface IDeliveryOptimizationSettings;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings ABI::Windows::ApplicationModel::Store::Preview::IDeliveryOptimizationSettings

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    interface IDeliveryOptimizationSettingsStatics;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics ABI::Windows::ApplicationModel::Store::Preview::IDeliveryOptimizationSettingsStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    interface IStoreConfigurationStatics;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics ABI::Windows::ApplicationModel::Store::Preview::IStoreConfigurationStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    interface IStoreConfigurationStatics2;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2 ABI::Windows::ApplicationModel::Store::Preview::IStoreConfigurationStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    interface IStoreConfigurationStatics3;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3 ABI::Windows::ApplicationModel::Store::Preview::IStoreConfigurationStatics3

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    interface IStoreConfigurationStatics4;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4 ABI::Windows::ApplicationModel::Store::Preview::IStoreConfigurationStatics4

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    interface IStoreConfigurationStatics5;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5 ABI::Windows::ApplicationModel::Store::Preview::IStoreConfigurationStatics5

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    interface IStoreHardwareManufacturerInfo;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo ABI::Windows::ApplicationModel::Store::Preview::IStoreHardwareManufacturerInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    interface IStorePreview;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview ABI::Windows::ApplicationModel::Store::Preview::IStorePreview

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    interface IStorePreviewProductInfo;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo ABI::Windows::ApplicationModel::Store::Preview::IStorePreviewProductInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    interface IStorePreviewPurchaseResults;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults ABI::Windows::ApplicationModel::Store::Preview::IStorePreviewPurchaseResults

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    interface IStorePreviewSkuInfo;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo ABI::Windows::ApplicationModel::Store::Preview::IStorePreviewSkuInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    interface IWebAuthenticationCoreManagerHelper;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper ABI::Windows::ApplicationModel::Store::Preview::IWebAuthenticationCoreManagerHelper

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    class StorePreviewPurchaseResults;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9aa2af80-0dcb-5ec1-8435-0b687ed374a5"))
IAsyncOperation<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewPurchaseResults*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewPurchaseResults*, ABI::Windows::ApplicationModel::Store::Preview::IStorePreviewPurchaseResults*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Store.Preview.StorePreviewPurchaseResults>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewPurchaseResults*> __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b1ea16e7-8268-51ff-8129-dcefd493f35f"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewPurchaseResults*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewPurchaseResults*, ABI::Windows::ApplicationModel::Store::Preview::IStorePreviewPurchaseResults*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Store.Preview.StorePreviewPurchaseResults>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewPurchaseResults*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    class StorePreviewProductInfo;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3fb16dec-73f1-5cfa-80e6-67fa232d1bef"))
IIterator<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewProductInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewProductInfo*, ABI::Windows::ApplicationModel::Store::Preview::IStorePreviewProductInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewProductInfo*> __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_t;
#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a9cb4860-67d1-53c2-a621-3074b0344d49"))
IIterable<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewProductInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewProductInfo*, ABI::Windows::ApplicationModel::Store::Preview::IStorePreviewProductInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewProductInfo*> __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_t;
#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fcc6add5-dc28-500f-8e24-b22d1ab56aad"))
IVectorView<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewProductInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewProductInfo*, ABI::Windows::ApplicationModel::Store::Preview::IStorePreviewProductInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewProductInfo*> __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_t;
#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("49c36a66-3908-51b3-8402-eb8e94c68864"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("167564b0-c6f5-5143-b66f-a6f9ca69e7a2"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    typedef enum StoreSystemFeature : int StoreSystemFeature;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d0455b2d-d8aa-557e-89a3-63c33e8cee99"))
IIterator<enum ABI::Windows::ApplicationModel::Store::Preview::StoreSystemFeature> : IIterator_impl<enum ABI::Windows::ApplicationModel::Store::Preview::StoreSystemFeature>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Store.Preview.StoreSystemFeature>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::ApplicationModel::Store::Preview::StoreSystemFeature> __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_t;
#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("98a8577a-b128-5400-8d3d-58654eaaf957"))
IIterable<enum ABI::Windows::ApplicationModel::Store::Preview::StoreSystemFeature> : IIterable_impl<enum ABI::Windows::ApplicationModel::Store::Preview::StoreSystemFeature>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Store.Preview.StoreSystemFeature>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::ApplicationModel::Store::Preview::StoreSystemFeature> __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_t;
#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c8f90757-ebcf-5f2c-b918-6487105b0ca1"))
IVectorView<enum ABI::Windows::ApplicationModel::Store::Preview::StoreSystemFeature> : IVectorView_impl<enum ABI::Windows::ApplicationModel::Store::Preview::StoreSystemFeature>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Store.Preview.StoreSystemFeature>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::ApplicationModel::Store::Preview::StoreSystemFeature> __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_t;
#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b1713163-ee1b-5290-8316-f7ebb9d53163"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Store.Preview.StoreSystemFeature>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7e7946ef-f8f0-53fd-9613-7261cb35daf4"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Store.Preview.StoreSystemFeature>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        class WebTokenRequestResult;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebTokenRequestResult;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequestResult

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0a815852-7c44-5674-b3d2-fa2e4c1e46c9"))
IAsyncOperation<ABI::Windows::Security::Authentication::Web::Core::WebTokenRequestResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Core::WebTokenRequestResult*, ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequestResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Authentication.Web.Core.WebTokenRequestResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Authentication::Web::Core::WebTokenRequestResult*> __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("deb54b22-70f2-55ab-97c0-6cbdc5ddb6f0"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::Web::Core::WebTokenRequestResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Core::WebTokenRequestResult*, ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequestResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Authentication.Web.Core.WebTokenRequestResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::Web::Core::WebTokenRequestResult*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("65178d50-e6a2-5d16-b244-65e9725e5a0c"))
IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStreamReference*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IRandomAccessStreamReference*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IRandomAccessStreamReference>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStreamReference*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("60847289-ea0b-5df6-89df-f2c62cba9693"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStreamReference*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IRandomAccessStreamReference*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IRandomAccessStreamReference>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStreamReference*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    class StorePreviewSkuInfo;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b6f9b421-7f54-5d26-9c37-9f9d7ac893eb"))
IIterator<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewSkuInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewSkuInfo*, ABI::Windows::ApplicationModel::Store::Preview::IStorePreviewSkuInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Store.Preview.StorePreviewSkuInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewSkuInfo*> __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_t;
#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2c0d85d9-0df7-5de6-962e-bc8f149faf19"))
IIterable<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewSkuInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewSkuInfo*, ABI::Windows::ApplicationModel::Store::Preview::IStorePreviewSkuInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Store.Preview.StorePreviewSkuInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewSkuInfo*> __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_t;
#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("47418723-a671-5fdc-8647-68f7d8c31416"))
IVectorView<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewSkuInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewSkuInfo*, ABI::Windows::ApplicationModel::Store::Preview::IStorePreviewSkuInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Store.Preview.StorePreviewSkuInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Store::Preview::StorePreviewSkuInfo*> __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_t;
#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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


namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        class WebTokenRequest;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebTokenRequest;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequest

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_FWD_DEFINED__

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
        namespace UI {
            namespace Xaml {
                class UIElement;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                interface IUIElement;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CIUIElement ABI::Windows::UI::Xaml::IUIElement

#endif // ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    typedef enum DeliveryOptimizationDownloadMode : int DeliveryOptimizationDownloadMode;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    typedef enum DeliveryOptimizationDownloadModeSource : int DeliveryOptimizationDownloadModeSource;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    typedef enum StoreLogOptions : unsigned int StoreLogOptions;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    typedef enum StorePreviewProductPurchaseStatus : int StorePreviewProductPurchaseStatus;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    class DeliveryOptimizationSettings;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    class StoreHardwareManufacturerInfo;
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.DeliveryOptimizationDownloadMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    enum DeliveryOptimizationDownloadMode : int
                    {
                        DeliveryOptimizationDownloadMode_Simple = 0,
                        DeliveryOptimizationDownloadMode_HttpOnly = 1,
                        DeliveryOptimizationDownloadMode_Lan = 2,
                        DeliveryOptimizationDownloadMode_Group = 3,
                        DeliveryOptimizationDownloadMode_Internet = 4,
                        DeliveryOptimizationDownloadMode_Bypass = 5,
                    };
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.DeliveryOptimizationDownloadModeSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    enum DeliveryOptimizationDownloadModeSource : int
                    {
                        DeliveryOptimizationDownloadModeSource_Default = 0,
                        DeliveryOptimizationDownloadModeSource_Policy = 1,
                    };
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.StoreLogOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    enum StoreLogOptions : unsigned int
                    {
                        StoreLogOptions_None = 0,
                        StoreLogOptions_TryElevate = 0x1,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(StoreLogOptions)
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.StorePreviewProductPurchaseStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    enum StorePreviewProductPurchaseStatus : int
                    {
                        StorePreviewProductPurchaseStatus_Succeeded = 0,
                        StorePreviewProductPurchaseStatus_AlreadyPurchased = 1,
                        StorePreviewProductPurchaseStatus_NotFulfilled = 2,
                        StorePreviewProductPurchaseStatus_NotPurchased = 3,
                    };
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.StoreSystemFeature
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    enum StoreSystemFeature : int
                    {
                        StoreSystemFeature_ArchitectureX86 = 0,
                        StoreSystemFeature_ArchitectureX64 = 1,
                        StoreSystemFeature_ArchitectureArm = 2,
                        StoreSystemFeature_DirectX9 = 3,
                        StoreSystemFeature_DirectX10 = 4,
                        StoreSystemFeature_DirectX11 = 5,
                        StoreSystemFeature_D3D12HardwareFL11 = 6,
                        StoreSystemFeature_D3D12HardwareFL12 = 7,
                        StoreSystemFeature_Memory300MB = 8,
                        StoreSystemFeature_Memory750MB = 9,
                        StoreSystemFeature_Memory1GB = 10,
                        StoreSystemFeature_Memory2GB = 11,
                        StoreSystemFeature_CameraFront = 12,
                        StoreSystemFeature_CameraRear = 13,
                        StoreSystemFeature_Gyroscope = 14,
                        StoreSystemFeature_Hover = 15,
                        StoreSystemFeature_Magnetometer = 16,
                        StoreSystemFeature_Nfc = 17,
                        StoreSystemFeature_Resolution720P = 18,
                        StoreSystemFeature_ResolutionWvga = 19,
                        StoreSystemFeature_ResolutionWvgaOr720P = 20,
                        StoreSystemFeature_ResolutionWxga = 21,
                        StoreSystemFeature_ResolutionWvgaOrWxga = 22,
                        StoreSystemFeature_ResolutionWxgaOr720P = 23,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        StoreSystemFeature_Memory4GB = 24,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        StoreSystemFeature_Memory6GB = 25,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        StoreSystemFeature_Memory8GB = 26,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        StoreSystemFeature_Memory12GB = 27,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        StoreSystemFeature_Memory16GB = 28,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        StoreSystemFeature_Memory20GB = 29,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        StoreSystemFeature_VideoMemory2GB = 30,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        StoreSystemFeature_VideoMemory4GB = 31,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        StoreSystemFeature_VideoMemory6GB = 32,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        StoreSystemFeature_VideoMemory1GB = 33,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                        StoreSystemFeature_ArchitectureArm64 = 34,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    };
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IDeliveryOptimizationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.DeliveryOptimizationSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IDeliveryOptimizationSettings[] = L"Windows.ApplicationModel.Store.Preview.IDeliveryOptimizationSettings";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    MIDL_INTERFACE("1810fda0-e853-565e-b874-7a8a7b9a0e0f")
                    IDeliveryOptimizationSettings : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DownloadMode(
                            ABI::Windows::ApplicationModel::Store::Preview::DeliveryOptimizationDownloadMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DownloadModeSource(
                            ABI::Windows::ApplicationModel::Store::Preview::DeliveryOptimizationDownloadModeSource* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDeliveryOptimizationSettings = __uuidof(IDeliveryOptimizationSettings);
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IDeliveryOptimizationSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.DeliveryOptimizationSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IDeliveryOptimizationSettingsStatics[] = L"Windows.ApplicationModel.Store.Preview.IDeliveryOptimizationSettingsStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    MIDL_INTERFACE("5c817caf-aed5-5999-b4c9-8c60898bc4f3")
                    IDeliveryOptimizationSettingsStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetCurrentSettings(
                            ABI::Windows::ApplicationModel::Store::Preview::IDeliveryOptimizationSettings** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDeliveryOptimizationSettingsStatics = __uuidof(IDeliveryOptimizationSettingsStatics);
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StoreConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStoreConfigurationStatics[] = L"Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    MIDL_INTERFACE("728f7fc0-8628-42ec-84a2-07780eb44d8b")
                    IStoreConfigurationStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetSystemConfiguration(
                            HSTRING catalogHardwareManufacturerId,
                            HSTRING catalogStoreContentModifierId,
                            ABI::Windows::Foundation::DateTime systemConfigurationExpiration,
                            HSTRING catalogHardwareDescriptor
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetMobileOperatorConfiguration(
                            HSTRING mobileOperatorId,
                            UINT32 appDownloadLimitInMegabytes,
                            UINT32 updateDownloadLimitInMegabytes
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStoreWebAccountId(
                            HSTRING webAccountId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsStoreWebAccountId(
                            HSTRING webAccountId,
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HardwareManufacturerInfo(
                            ABI::Windows::ApplicationModel::Store::Preview::IStoreHardwareManufacturerInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE FilterUnsupportedSystemFeaturesAsync(
                            __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* systemFeatures,
                            __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IStoreConfigurationStatics = __uuidof(IStoreConfigurationStatics);
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StoreConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStoreConfigurationStatics2[] = L"Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    MIDL_INTERFACE("657c4595-c8b7-4fe9-9f4c-4d71027d347e")
                    IStoreConfigurationStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PurchasePromptingPolicy(
                            __FIReference_1_UINT32** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PurchasePromptingPolicy(
                            __FIReference_1_UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IStoreConfigurationStatics2 = __uuidof(IStoreConfigurationStatics2);
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StoreConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStoreConfigurationStatics3[] = L"Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    MIDL_INTERFACE("6d45f57c-f144-4cb5-9d3f-4eb05e30b6d3")
                    IStoreConfigurationStatics3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE HasStoreWebAccount(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE HasStoreWebAccountForUser(
                            ABI::Windows::System::IUser* user,
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStoreLogDataAsync(
                            ABI::Windows::ApplicationModel::Store::Preview::StoreLogOptions options,
                            __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStoreWebAccountIdForUser(
                            ABI::Windows::System::IUser* user,
                            HSTRING webAccountId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsStoreWebAccountIdForUser(
                            ABI::Windows::System::IUser* user,
                            HSTRING webAccountId,
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetPurchasePromptingPolicyForUser(
                            ABI::Windows::System::IUser* user,
                            __FIReference_1_UINT32** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPurchasePromptingPolicyForUser(
                            ABI::Windows::System::IUser* user,
                            __FIReference_1_UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IStoreConfigurationStatics3 = __uuidof(IStoreConfigurationStatics3);
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StoreConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStoreConfigurationStatics4[] = L"Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics4";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    MIDL_INTERFACE("20ff56d2-4ee3-4cf0-9b12-552c03310f75")
                    IStoreConfigurationStatics4 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetStoreWebAccountId(
                            HSTRING* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStoreWebAccountIdForUser(
                            ABI::Windows::System::IUser* user,
                            HSTRING* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetEnterpriseStoreWebAccountId(
                            HSTRING webAccountId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetEnterpriseStoreWebAccountIdForUser(
                            ABI::Windows::System::IUser* user,
                            HSTRING webAccountId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetEnterpriseStoreWebAccountId(
                            HSTRING* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetEnterpriseStoreWebAccountIdForUser(
                            ABI::Windows::System::IUser* user,
                            HSTRING* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ShouldRestrictToEnterpriseStoreOnly(
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ShouldRestrictToEnterpriseStoreOnlyForUser(
                            ABI::Windows::System::IUser* user,
                            boolean* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IStoreConfigurationStatics4 = __uuidof(IStoreConfigurationStatics4);
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StoreConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStoreConfigurationStatics5[] = L"Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics5";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    MIDL_INTERFACE("f7613191-8fa9-49db-822b-0160e7e4e5c5")
                    IStoreConfigurationStatics5 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE IsPinToDesktopSupported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsPinToTaskbarSupported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsPinToStartSupported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE PinToDesktop(
                            HSTRING appPackageFamilyName
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE PinToDesktopForUser(
                            ABI::Windows::System::IUser* user,
                            HSTRING appPackageFamilyName
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IStoreConfigurationStatics5 = __uuidof(IStoreConfigurationStatics5);
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStoreHardwareManufacturerInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StoreHardwareManufacturerInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStoreHardwareManufacturerInfo[] = L"Windows.ApplicationModel.Store.Preview.IStoreHardwareManufacturerInfo";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    MIDL_INTERFACE("f292dc08-c654-43ac-a21f-34801c9d3388")
                    IStoreHardwareManufacturerInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_HardwareManufacturerId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StoreContentModifierId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ModelName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ManufacturerName(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IStoreHardwareManufacturerInfo = __uuidof(IStoreHardwareManufacturerInfo);
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStorePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StorePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStorePreview[] = L"Windows.ApplicationModel.Store.Preview.IStorePreview";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    MIDL_INTERFACE("8a157241-840e-49a9-bc01-5d5b01fbc8e9")
                    IStorePreview : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE RequestProductPurchaseByProductIdAndSkuIdAsync(
                            HSTRING productId,
                            HSTRING skuId,
                            __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults** requestPurchaseBySkuIdOperation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE LoadAddOnProductInfosAsync(
                            __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo** loadAddOnProductInfosOperation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IStorePreview = __uuidof(IStorePreview);
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStorePreviewProductInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStorePreviewProductInfo[] = L"Windows.ApplicationModel.Store.Preview.IStorePreviewProductInfo";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    MIDL_INTERFACE("1937dbb3-6c01-4c9d-85cd-5babaac2b351")
                    IStorePreviewProductInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ProductId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ProductType(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Title(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SkuInfoList(
                            __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IStorePreviewProductInfo = __uuidof(IStorePreviewProductInfo);
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStorePreviewPurchaseResults
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StorePreviewPurchaseResults
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStorePreviewPurchaseResults[] = L"Windows.ApplicationModel.Store.Preview.IStorePreviewPurchaseResults";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    MIDL_INTERFACE("b0daaed1-d6c5-4e53-a043-fba0d8e61231")
                    IStorePreviewPurchaseResults : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ProductPurchaseStatus(
                            ABI::Windows::ApplicationModel::Store::Preview::StorePreviewProductPurchaseStatus* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IStorePreviewPurchaseResults = __uuidof(IStorePreviewPurchaseResults);
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStorePreviewSkuInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StorePreviewSkuInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStorePreviewSkuInfo[] = L"Windows.ApplicationModel.Store.Preview.IStorePreviewSkuInfo";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    MIDL_INTERFACE("81fd76e2-0b26-48d9-98ce-27461c669d6c")
                    IStorePreviewSkuInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ProductId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SkuId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SkuType(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Title(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CustomDeveloperData(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CurrencyCode(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FormattedListPrice(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExtendedData(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IStorePreviewSkuInfo = __uuidof(IStorePreviewSkuInfo);
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IWebAuthenticationCoreManagerHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.WebAuthenticationCoreManagerHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IWebAuthenticationCoreManagerHelper[] = L"Windows.ApplicationModel.Store.Preview.IWebAuthenticationCoreManagerHelper";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    MIDL_INTERFACE("06a50525-e715-4123-9276-9d6f865ba55f")
                    IWebAuthenticationCoreManagerHelper : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE RequestTokenWithUIElementHostingAsync(
                            ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequest* request,
                            ABI::Windows::UI::Xaml::IUIElement* uiElement,
                            __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult** asyncInfo
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RequestTokenWithUIElementHostingAndWebAccountAsync(
                            ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequest* request,
                            ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                            ABI::Windows::UI::Xaml::IUIElement* uiElement,
                            __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult** asyncInfo
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWebAuthenticationCoreManagerHelper = __uuidof(IWebAuthenticationCoreManagerHelper);
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.DeliveryOptimizationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Store.Preview.IDeliveryOptimizationSettingsStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.IDeliveryOptimizationSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_DeliveryOptimizationSettings_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_DeliveryOptimizationSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_DeliveryOptimizationSettings[] = L"Windows.ApplicationModel.Store.Preview.DeliveryOptimizationSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.StoreConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics5 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics3 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics4 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics2 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StoreConfiguration_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StoreConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_StoreConfiguration[] = L"Windows.ApplicationModel.Store.Preview.StoreConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.StoreHardwareManufacturerInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.IStoreHardwareManufacturerInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StoreHardwareManufacturerInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StoreHardwareManufacturerInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_StoreHardwareManufacturerInfo[] = L"Windows.ApplicationModel.Store.Preview.StoreHardwareManufacturerInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.StorePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Store.Preview.IStorePreview interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StorePreview_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StorePreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_StorePreview[] = L"Windows.ApplicationModel.Store.Preview.StorePreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.IStorePreviewProductInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StorePreviewProductInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StorePreviewProductInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_StorePreviewProductInfo[] = L"Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.StorePreviewPurchaseResults
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.IStorePreviewPurchaseResults ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StorePreviewPurchaseResults_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StorePreviewPurchaseResults_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_StorePreviewPurchaseResults[] = L"Windows.ApplicationModel.Store.Preview.StorePreviewPurchaseResults";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.StorePreviewSkuInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.IStorePreviewSkuInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StorePreviewSkuInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StorePreviewSkuInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_StorePreviewSkuInfo[] = L"Windows.ApplicationModel.Store.Preview.StorePreviewSkuInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.WebAuthenticationCoreManagerHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Store.Preview.IWebAuthenticationCoreManagerHelper interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_WebAuthenticationCoreManagerHelper_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_WebAuthenticationCoreManagerHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_WebAuthenticationCoreManagerHelper[] = L"Windows.ApplicationModel.Store.Preview.WebAuthenticationCoreManagerHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResultsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResultsVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResultsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResultsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResultsVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResultsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo;

typedef struct __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfoVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo;

typedef struct __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfoVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfoVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfoVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfoVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CStoreSystemFeature __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CStoreSystemFeature;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature;

typedef struct __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeatureVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CStoreSystemFeature* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CStoreSystemFeature* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeatureVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeatureVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature;

typedef struct __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeatureVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeatureVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeatureVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeatureVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        UINT32 index,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CStoreSystemFeature* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CStoreSystemFeature value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CStoreSystemFeature* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeatureVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeatureVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeatureVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeatureVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeatureVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeatureVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeatureVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeatureVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReferenceVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReferenceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo;

typedef struct __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfoVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo;

typedef struct __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfoVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfoVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIUIElement __x_ABI_CWindows_CUI_CXaml_CIUIElement;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CDeliveryOptimizationDownloadMode __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CDeliveryOptimizationDownloadMode;

typedef enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CDeliveryOptimizationDownloadModeSource __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CDeliveryOptimizationDownloadModeSource;

typedef enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CStoreLogOptions __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CStoreLogOptions;

typedef enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CStorePreviewProductPurchaseStatus __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CStorePreviewProductPurchaseStatus;

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.DeliveryOptimizationDownloadMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CDeliveryOptimizationDownloadMode
{
    DeliveryOptimizationDownloadMode_Simple = 0,
    DeliveryOptimizationDownloadMode_HttpOnly = 1,
    DeliveryOptimizationDownloadMode_Lan = 2,
    DeliveryOptimizationDownloadMode_Group = 3,
    DeliveryOptimizationDownloadMode_Internet = 4,
    DeliveryOptimizationDownloadMode_Bypass = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.DeliveryOptimizationDownloadModeSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CDeliveryOptimizationDownloadModeSource
{
    DeliveryOptimizationDownloadModeSource_Default = 0,
    DeliveryOptimizationDownloadModeSource_Policy = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.StoreLogOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CStoreLogOptions
{
    StoreLogOptions_None = 0,
    StoreLogOptions_TryElevate = 0x1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.StorePreviewProductPurchaseStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CStorePreviewProductPurchaseStatus
{
    StorePreviewProductPurchaseStatus_Succeeded = 0,
    StorePreviewProductPurchaseStatus_AlreadyPurchased = 1,
    StorePreviewProductPurchaseStatus_NotFulfilled = 2,
    StorePreviewProductPurchaseStatus_NotPurchased = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.StoreSystemFeature
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CStoreSystemFeature
{
    StoreSystemFeature_ArchitectureX86 = 0,
    StoreSystemFeature_ArchitectureX64 = 1,
    StoreSystemFeature_ArchitectureArm = 2,
    StoreSystemFeature_DirectX9 = 3,
    StoreSystemFeature_DirectX10 = 4,
    StoreSystemFeature_DirectX11 = 5,
    StoreSystemFeature_D3D12HardwareFL11 = 6,
    StoreSystemFeature_D3D12HardwareFL12 = 7,
    StoreSystemFeature_Memory300MB = 8,
    StoreSystemFeature_Memory750MB = 9,
    StoreSystemFeature_Memory1GB = 10,
    StoreSystemFeature_Memory2GB = 11,
    StoreSystemFeature_CameraFront = 12,
    StoreSystemFeature_CameraRear = 13,
    StoreSystemFeature_Gyroscope = 14,
    StoreSystemFeature_Hover = 15,
    StoreSystemFeature_Magnetometer = 16,
    StoreSystemFeature_Nfc = 17,
    StoreSystemFeature_Resolution720P = 18,
    StoreSystemFeature_ResolutionWvga = 19,
    StoreSystemFeature_ResolutionWvgaOr720P = 20,
    StoreSystemFeature_ResolutionWxga = 21,
    StoreSystemFeature_ResolutionWvgaOrWxga = 22,
    StoreSystemFeature_ResolutionWxgaOr720P = 23,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    StoreSystemFeature_Memory4GB = 24,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    StoreSystemFeature_Memory6GB = 25,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    StoreSystemFeature_Memory8GB = 26,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    StoreSystemFeature_Memory12GB = 27,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    StoreSystemFeature_Memory16GB = 28,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    StoreSystemFeature_Memory20GB = 29,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    StoreSystemFeature_VideoMemory2GB = 30,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    StoreSystemFeature_VideoMemory4GB = 31,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    StoreSystemFeature_VideoMemory6GB = 32,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    StoreSystemFeature_VideoMemory1GB = 33,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    StoreSystemFeature_ArchitectureArm64 = 34,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IDeliveryOptimizationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.DeliveryOptimizationSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IDeliveryOptimizationSettings[] = L"Windows.ApplicationModel.Store.Preview.IDeliveryOptimizationSettings";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DownloadMode)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CDeliveryOptimizationDownloadMode* value);
    HRESULT (STDMETHODCALLTYPE* get_DownloadModeSource)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CDeliveryOptimizationDownloadModeSource* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_get_DownloadMode(This, value) \
    ((This)->lpVtbl->get_DownloadMode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_get_DownloadModeSource(This, value) \
    ((This)->lpVtbl->get_DownloadModeSource(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IDeliveryOptimizationSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.DeliveryOptimizationSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IDeliveryOptimizationSettingsStatics[] = L"Windows.ApplicationModel.Store.Preview.IDeliveryOptimizationSettingsStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCurrentSettings)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettings** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_GetCurrentSettings(This, result) \
    ((This)->lpVtbl->GetCurrentSettings(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIDeliveryOptimizationSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StoreConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStoreConfigurationStatics[] = L"Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetSystemConfiguration)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics* This,
        HSTRING catalogHardwareManufacturerId,
        HSTRING catalogStoreContentModifierId,
        struct __x_ABI_CWindows_CFoundation_CDateTime systemConfigurationExpiration,
        HSTRING catalogHardwareDescriptor);
    HRESULT (STDMETHODCALLTYPE* SetMobileOperatorConfiguration)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics* This,
        HSTRING mobileOperatorId,
        UINT32 appDownloadLimitInMegabytes,
        UINT32 updateDownloadLimitInMegabytes);
    HRESULT (STDMETHODCALLTYPE* SetStoreWebAccountId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics* This,
        HSTRING webAccountId);
    HRESULT (STDMETHODCALLTYPE* IsStoreWebAccountId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics* This,
        HSTRING webAccountId,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_HardwareManufacturerInfo)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo** value);
    HRESULT (STDMETHODCALLTYPE* FilterUnsupportedSystemFeaturesAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics* This,
        __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature* systemFeatures,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStoreSystemFeature** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_SetSystemConfiguration(This, catalogHardwareManufacturerId, catalogStoreContentModifierId, systemConfigurationExpiration, catalogHardwareDescriptor) \
    ((This)->lpVtbl->SetSystemConfiguration(This, catalogHardwareManufacturerId, catalogStoreContentModifierId, systemConfigurationExpiration, catalogHardwareDescriptor))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_SetMobileOperatorConfiguration(This, mobileOperatorId, appDownloadLimitInMegabytes, updateDownloadLimitInMegabytes) \
    ((This)->lpVtbl->SetMobileOperatorConfiguration(This, mobileOperatorId, appDownloadLimitInMegabytes, updateDownloadLimitInMegabytes))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_SetStoreWebAccountId(This, webAccountId) \
    ((This)->lpVtbl->SetStoreWebAccountId(This, webAccountId))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_IsStoreWebAccountId(This, webAccountId, value) \
    ((This)->lpVtbl->IsStoreWebAccountId(This, webAccountId, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_get_HardwareManufacturerInfo(This, value) \
    ((This)->lpVtbl->get_HardwareManufacturerInfo(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_FilterUnsupportedSystemFeaturesAsync(This, systemFeatures, operation) \
    ((This)->lpVtbl->FilterUnsupportedSystemFeaturesAsync(This, systemFeatures, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StoreConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStoreConfigurationStatics2[] = L"Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PurchasePromptingPolicy)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2* This,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* put_PurchasePromptingPolicy)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2* This,
        __FIReference_1_UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_get_PurchasePromptingPolicy(This, value) \
    ((This)->lpVtbl->get_PurchasePromptingPolicy(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_put_PurchasePromptingPolicy(This, value) \
    ((This)->lpVtbl->put_PurchasePromptingPolicy(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StoreConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStoreConfigurationStatics3[] = L"Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics3";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* HasStoreWebAccount)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* HasStoreWebAccountForUser)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetStoreLogDataAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CStoreLogOptions options,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference** operation);
    HRESULT (STDMETHODCALLTYPE* SetStoreWebAccountIdForUser)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING webAccountId);
    HRESULT (STDMETHODCALLTYPE* IsStoreWebAccountIdForUser)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING webAccountId,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetPurchasePromptingPolicyForUser)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* SetPurchasePromptingPolicyForUser)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __FIReference_1_UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_HasStoreWebAccount(This, value) \
    ((This)->lpVtbl->HasStoreWebAccount(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_HasStoreWebAccountForUser(This, user, value) \
    ((This)->lpVtbl->HasStoreWebAccountForUser(This, user, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_GetStoreLogDataAsync(This, options, operation) \
    ((This)->lpVtbl->GetStoreLogDataAsync(This, options, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_SetStoreWebAccountIdForUser(This, user, webAccountId) \
    ((This)->lpVtbl->SetStoreWebAccountIdForUser(This, user, webAccountId))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_IsStoreWebAccountIdForUser(This, user, webAccountId, value) \
    ((This)->lpVtbl->IsStoreWebAccountIdForUser(This, user, webAccountId, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_GetPurchasePromptingPolicyForUser(This, user, value) \
    ((This)->lpVtbl->GetPurchasePromptingPolicyForUser(This, user, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_SetPurchasePromptingPolicyForUser(This, user, value) \
    ((This)->lpVtbl->SetPurchasePromptingPolicyForUser(This, user, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StoreConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStoreConfigurationStatics4[] = L"Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics4";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetStoreWebAccountId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* GetStoreWebAccountIdForUser)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* SetEnterpriseStoreWebAccountId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4* This,
        HSTRING webAccountId);
    HRESULT (STDMETHODCALLTYPE* SetEnterpriseStoreWebAccountIdForUser)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING webAccountId);
    HRESULT (STDMETHODCALLTYPE* GetEnterpriseStoreWebAccountId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* GetEnterpriseStoreWebAccountIdForUser)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* ShouldRestrictToEnterpriseStoreOnly)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* ShouldRestrictToEnterpriseStoreOnlyForUser)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_GetStoreWebAccountId(This, result) \
    ((This)->lpVtbl->GetStoreWebAccountId(This, result))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_GetStoreWebAccountIdForUser(This, user, result) \
    ((This)->lpVtbl->GetStoreWebAccountIdForUser(This, user, result))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_SetEnterpriseStoreWebAccountId(This, webAccountId) \
    ((This)->lpVtbl->SetEnterpriseStoreWebAccountId(This, webAccountId))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_SetEnterpriseStoreWebAccountIdForUser(This, user, webAccountId) \
    ((This)->lpVtbl->SetEnterpriseStoreWebAccountIdForUser(This, user, webAccountId))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_GetEnterpriseStoreWebAccountId(This, result) \
    ((This)->lpVtbl->GetEnterpriseStoreWebAccountId(This, result))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_GetEnterpriseStoreWebAccountIdForUser(This, user, result) \
    ((This)->lpVtbl->GetEnterpriseStoreWebAccountIdForUser(This, user, result))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_ShouldRestrictToEnterpriseStoreOnly(This, result) \
    ((This)->lpVtbl->ShouldRestrictToEnterpriseStoreOnly(This, result))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_ShouldRestrictToEnterpriseStoreOnlyForUser(This, user, result) \
    ((This)->lpVtbl->ShouldRestrictToEnterpriseStoreOnlyForUser(This, user, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StoreConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStoreConfigurationStatics5[] = L"Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics5";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsPinToDesktopSupported)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsPinToTaskbarSupported)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsPinToStartSupported)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* PinToDesktop)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5* This,
        HSTRING appPackageFamilyName);
    HRESULT (STDMETHODCALLTYPE* PinToDesktopForUser)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING appPackageFamilyName);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_IsPinToDesktopSupported(This, value) \
    ((This)->lpVtbl->IsPinToDesktopSupported(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_IsPinToTaskbarSupported(This, value) \
    ((This)->lpVtbl->IsPinToTaskbarSupported(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_IsPinToStartSupported(This, value) \
    ((This)->lpVtbl->IsPinToStartSupported(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_PinToDesktop(This, appPackageFamilyName) \
    ((This)->lpVtbl->PinToDesktop(This, appPackageFamilyName))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_PinToDesktopForUser(This, user, appPackageFamilyName) \
    ((This)->lpVtbl->PinToDesktopForUser(This, user, appPackageFamilyName))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreConfigurationStatics5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStoreHardwareManufacturerInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StoreHardwareManufacturerInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStoreHardwareManufacturerInfo[] = L"Windows.ApplicationModel.Store.Preview.IStoreHardwareManufacturerInfo";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HardwareManufacturerId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_StoreContentModifierId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ModelName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ManufacturerName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfoVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_get_HardwareManufacturerId(This, value) \
    ((This)->lpVtbl->get_HardwareManufacturerId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_get_StoreContentModifierId(This, value) \
    ((This)->lpVtbl->get_StoreContentModifierId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_get_ModelName(This, value) \
    ((This)->lpVtbl->get_ModelName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_get_ManufacturerName(This, value) \
    ((This)->lpVtbl->get_ManufacturerName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStoreHardwareManufacturerInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStorePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StorePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStorePreview[] = L"Windows.ApplicationModel.Store.Preview.IStorePreview";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestProductPurchaseByProductIdAndSkuIdAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview* This,
        HSTRING productId,
        HSTRING skuId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewPurchaseResults** requestPurchaseBySkuIdOperation);
    HRESULT (STDMETHODCALLTYPE* LoadAddOnProductInfosAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewProductInfo** loadAddOnProductInfosOperation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_RequestProductPurchaseByProductIdAndSkuIdAsync(This, productId, skuId, requestPurchaseBySkuIdOperation) \
    ((This)->lpVtbl->RequestProductPurchaseByProductIdAndSkuIdAsync(This, productId, skuId, requestPurchaseBySkuIdOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_LoadAddOnProductInfosAsync(This, loadAddOnProductInfosOperation) \
    ((This)->lpVtbl->LoadAddOnProductInfosAsync(This, loadAddOnProductInfosOperation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStorePreviewProductInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStorePreviewProductInfo[] = L"Windows.ApplicationModel.Store.Preview.IStorePreviewProductInfo";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProductId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ProductType)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SkuInfoList)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo* This,
        __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CStorePreviewSkuInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfoVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_get_ProductId(This, value) \
    ((This)->lpVtbl->get_ProductId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_get_ProductType(This, value) \
    ((This)->lpVtbl->get_ProductType(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_get_SkuInfoList(This, value) \
    ((This)->lpVtbl->get_SkuInfoList(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewProductInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStorePreviewPurchaseResults
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StorePreviewPurchaseResults
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStorePreviewPurchaseResults[] = L"Windows.ApplicationModel.Store.Preview.IStorePreviewPurchaseResults";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResultsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProductPurchaseStatus)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CStorePreviewProductPurchaseStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResultsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResultsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_get_ProductPurchaseStatus(This, value) \
    ((This)->lpVtbl->get_ProductPurchaseStatus(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewPurchaseResults_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IStorePreviewSkuInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.StorePreviewSkuInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IStorePreviewSkuInfo[] = L"Windows.ApplicationModel.Store.Preview.IStorePreviewSkuInfo";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProductId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SkuId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SkuType)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CustomDeveloperData)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CurrencyCode)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FormattedListPrice)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedData)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfoVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_get_ProductId(This, value) \
    ((This)->lpVtbl->get_ProductId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_get_SkuId(This, value) \
    ((This)->lpVtbl->get_SkuId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_get_SkuType(This, value) \
    ((This)->lpVtbl->get_SkuType(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_get_CustomDeveloperData(This, value) \
    ((This)->lpVtbl->get_CustomDeveloperData(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_get_CurrencyCode(This, value) \
    ((This)->lpVtbl->get_CurrencyCode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_get_FormattedListPrice(This, value) \
    ((This)->lpVtbl->get_FormattedListPrice(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_get_ExtendedData(This, value) \
    ((This)->lpVtbl->get_ExtendedData(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIStorePreviewSkuInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.IWebAuthenticationCoreManagerHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.WebAuthenticationCoreManagerHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_IWebAuthenticationCoreManagerHelper[] = L"Windows.ApplicationModel.Store.Preview.IWebAuthenticationCoreManagerHelper";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelperVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestTokenWithUIElementHostingAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest* request,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* uiElement,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* RequestTokenWithUIElementHostingAndWebAccountAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest* request,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* uiElement,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelperVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelperVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_RequestTokenWithUIElementHostingAsync(This, request, uiElement, asyncInfo) \
    ((This)->lpVtbl->RequestTokenWithUIElementHostingAsync(This, request, uiElement, asyncInfo))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_RequestTokenWithUIElementHostingAndWebAccountAsync(This, request, webAccount, uiElement, asyncInfo) \
    ((This)->lpVtbl->RequestTokenWithUIElementHostingAndWebAccountAsync(This, request, webAccount, uiElement, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CIWebAuthenticationCoreManagerHelper_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.DeliveryOptimizationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Store.Preview.IDeliveryOptimizationSettingsStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.IDeliveryOptimizationSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_DeliveryOptimizationSettings_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_DeliveryOptimizationSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_DeliveryOptimizationSettings[] = L"Windows.ApplicationModel.Store.Preview.DeliveryOptimizationSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.StoreConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics5 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics3 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics4 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics2 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StoreConfiguration_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StoreConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_StoreConfiguration[] = L"Windows.ApplicationModel.Store.Preview.StoreConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.StoreHardwareManufacturerInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.IStoreHardwareManufacturerInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StoreHardwareManufacturerInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StoreHardwareManufacturerInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_StoreHardwareManufacturerInfo[] = L"Windows.ApplicationModel.Store.Preview.StoreHardwareManufacturerInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.StorePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Store.Preview.IStorePreview interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StorePreview_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StorePreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_StorePreview[] = L"Windows.ApplicationModel.Store.Preview.StorePreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.IStorePreviewProductInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StorePreviewProductInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StorePreviewProductInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_StorePreviewProductInfo[] = L"Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.StorePreviewPurchaseResults
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.IStorePreviewPurchaseResults ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StorePreviewPurchaseResults_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StorePreviewPurchaseResults_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_StorePreviewPurchaseResults[] = L"Windows.ApplicationModel.Store.Preview.StorePreviewPurchaseResults";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.StorePreviewSkuInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.IStorePreviewSkuInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StorePreviewSkuInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_StorePreviewSkuInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_StorePreviewSkuInfo[] = L"Windows.ApplicationModel.Store.Preview.StorePreviewSkuInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.WebAuthenticationCoreManagerHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Store.Preview.IWebAuthenticationCoreManagerHelper interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_WebAuthenticationCoreManagerHelper_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_WebAuthenticationCoreManagerHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_WebAuthenticationCoreManagerHelper[] = L"Windows.ApplicationModel.Store.Preview.WebAuthenticationCoreManagerHelper";
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
#endif // __windows2Eapplicationmodel2Estore2Epreview_p_h__

#endif // __windows2Eapplicationmodel2Estore2Epreview_h__
