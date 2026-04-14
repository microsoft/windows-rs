
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
#ifndef __windows2Eapplicationmodel2Eappextensions_h__
#define __windows2Eapplicationmodel2Eappextensions_h__
#ifndef __windows2Eapplicationmodel2Eappextensions_p_h__
#define __windows2Eapplicationmodel2Eappextensions_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION)

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
#include "Windows.ApplicationModel.h"
#include "Windows.Storage.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                interface IAppExtension;
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension ABI::Windows::ApplicationModel::AppExtensions::IAppExtension

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                interface IAppExtension2;
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2 ABI::Windows::ApplicationModel::AppExtensions::IAppExtension2

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                interface IAppExtension3;
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3 ABI::Windows::ApplicationModel::AppExtensions::IAppExtension3

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                interface IAppExtensionCatalog;
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionCatalog

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                interface IAppExtensionCatalog2;
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2 ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionCatalog2

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                interface IAppExtensionCatalogStatics;
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionCatalogStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                interface IAppExtensionPackageInstalledEventArgs;
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionPackageInstalledEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                interface IAppExtensionPackageStatusChangedEventArgs;
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionPackageStatusChangedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                interface IAppExtensionPackageUninstallingEventArgs;
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionPackageUninstallingEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                interface IAppExtensionPackageUpdatedEventArgs;
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionPackageUpdatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                interface IAppExtensionPackageUpdatingEventArgs;
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionPackageUpdatingEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_FWD_DEFINED__

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

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___F__CIPropertySet_USE
#define DEF___FIAsyncOperation_1___F__CIPropertySet_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("490b0686-afd7-5037-9647-d8fe248f182c"))
IAsyncOperation<ABI::Windows::Foundation::Collections::IPropertySet*> : IAsyncOperation_impl<ABI::Windows::Foundation::Collections::IPropertySet*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IPropertySet>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Foundation::Collections::IPropertySet*> __FIAsyncOperation_1___F__CIPropertySet_t;
#define __FIAsyncOperation_1___F__CIPropertySet ABI::Windows::Foundation::__FIAsyncOperation_1___F__CIPropertySet_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___F__CIPropertySet_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___F__CIPropertySet_USE
#define DEF___FIAsyncOperationCompletedHandler_1___F__CIPropertySet_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5075a55f-68ba-56f2-97e6-9b1cbfa2c5f2"))
IAsyncOperationCompletedHandler<ABI::Windows::Foundation::Collections::IPropertySet*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Collections::IPropertySet*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IPropertySet>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Foundation::Collections::IPropertySet*> __FIAsyncOperationCompletedHandler_1___F__CIPropertySet_t;
#define __FIAsyncOperationCompletedHandler_1___F__CIPropertySet ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___F__CIPropertySet_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___F__CIPropertySet_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                class AppExtension;
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8e80ca83-3cd3-5f9c-83e4-84347ca5498c"))
IIterator<ABI::Windows::ApplicationModel::AppExtensions::AppExtension*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppExtensions::AppExtension*, ABI::Windows::ApplicationModel::AppExtensions::IAppExtension*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.AppExtensions.AppExtension>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::AppExtensions::AppExtension*> __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_t;
#define __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3b4fe356-1b13-59cb-ab1f-c4667a74756b"))
IIterable<ABI::Windows::ApplicationModel::AppExtensions::AppExtension*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppExtensions::AppExtension*, ABI::Windows::ApplicationModel::AppExtensions::IAppExtension*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.AppExtensions.AppExtension>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::AppExtensions::AppExtension*> __FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_t;
#define __FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("94520810-7e9b-5efd-b74d-e9d4175fd94a"))
IVectorView<ABI::Windows::ApplicationModel::AppExtensions::AppExtension*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppExtensions::AppExtension*, ABI::Windows::ApplicationModel::AppExtensions::IAppExtension*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.AppExtensions.AppExtension>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::AppExtensions::AppExtension*> __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_t;
#define __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("83295bb9-10df-530f-a0d7-be05ba80cb18"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.AppExtensions.AppExtension>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cbd3ea3b-1275-5688-8610-0ab1f83442fc"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.AppExtensions.AppExtension>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageFolder;
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFolder;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFolder ABI::Windows::Storage::IStorageFolder

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStorageFolder_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStorageFolder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6be9e7d7-e83a-5cbc-802c-1768960b52c3"))
IAsyncOperation<ABI::Windows::Storage::StorageFolder*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFolder*, ABI::Windows::Storage::IStorageFolder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.StorageFolder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::StorageFolder*> __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStorageFolder_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStorageFolder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c211026e-9e63-5452-ba54-3a07d6a96874"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::StorageFolder*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFolder*, ABI::Windows::Storage::IStorageFolder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.StorageFolder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::StorageFolder*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                class AppExtensionCatalog;
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                class AppExtensionPackageInstalledEventArgs;
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("26460556-9f0a-562e-9165-9eb9e1898b1e"))
ITypedEventHandler<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionCatalog*, ABI::Windows::ApplicationModel::AppExtensions::AppExtensionPackageInstalledEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionCatalog*, ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionCatalog*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionPackageInstalledEventArgs*, ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionPackageInstalledEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.AppExtensions.AppExtensionCatalog, Windows.ApplicationModel.AppExtensions.AppExtensionPackageInstalledEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionCatalog*, ABI::Windows::ApplicationModel::AppExtensions::AppExtensionPackageInstalledEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                class AppExtensionPackageStatusChangedEventArgs;
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("61a2a9c0-d3bb-5953-8df7-591fdd5bd74a"))
ITypedEventHandler<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionCatalog*, ABI::Windows::ApplicationModel::AppExtensions::AppExtensionPackageStatusChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionCatalog*, ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionCatalog*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionPackageStatusChangedEventArgs*, ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionPackageStatusChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.AppExtensions.AppExtensionCatalog, Windows.ApplicationModel.AppExtensions.AppExtensionPackageStatusChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionCatalog*, ABI::Windows::ApplicationModel::AppExtensions::AppExtensionPackageStatusChangedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                class AppExtensionPackageUninstallingEventArgs;
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("60e847e8-2eca-54be-8b13-9e62dbd5b95d"))
ITypedEventHandler<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionCatalog*, ABI::Windows::ApplicationModel::AppExtensions::AppExtensionPackageUninstallingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionCatalog*, ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionCatalog*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionPackageUninstallingEventArgs*, ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionPackageUninstallingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.AppExtensions.AppExtensionCatalog, Windows.ApplicationModel.AppExtensions.AppExtensionPackageUninstallingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionCatalog*, ABI::Windows::ApplicationModel::AppExtensions::AppExtensionPackageUninstallingEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                class AppExtensionPackageUpdatedEventArgs;
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a7e0dc0b-525e-52b1-b1d9-2d5b4b5294a5"))
ITypedEventHandler<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionCatalog*, ABI::Windows::ApplicationModel::AppExtensions::AppExtensionPackageUpdatedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionCatalog*, ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionCatalog*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionPackageUpdatedEventArgs*, ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionPackageUpdatedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.AppExtensions.AppExtensionCatalog, Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionCatalog*, ABI::Windows::ApplicationModel::AppExtensions::AppExtensionPackageUpdatedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                class AppExtensionPackageUpdatingEventArgs;
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("91f40910-6acf-510d-8d7b-0bd05b835883"))
ITypedEventHandler<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionCatalog*, ABI::Windows::ApplicationModel::AppExtensions::AppExtensionPackageUpdatingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionCatalog*, ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionCatalog*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionPackageUpdatingEventArgs*, ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionPackageUpdatingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.AppExtensions.AppExtensionCatalog, Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::AppExtensions::AppExtensionCatalog*, ABI::Windows::ApplicationModel::AppExtensions::AppExtensionPackageUpdatingEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class AppInfo;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IAppInfo;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIAppInfo ABI::Windows::ApplicationModel::IAppInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class Package;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackage;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackage ABI::Windows::ApplicationModel::IPackage

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage_FWD_DEFINED__

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtension
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtension[] = L"Windows.ApplicationModel.AppExtensions.IAppExtension";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                MIDL_INTERFACE("8450902c-15ed-4faf-93ea-2237bbf8cbd6")
                IAppExtension : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Package(
                        ABI::Windows::ApplicationModel::IPackage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AppInfo(
                        ABI::Windows::ApplicationModel::IAppInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetExtensionPropertiesAsync(
                        __FIAsyncOperation_1___F__CIPropertySet** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPublicFolderAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppExtension = __uuidof(IAppExtension);
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtension2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtension2[] = L"Windows.ApplicationModel.AppExtensions.IAppExtension2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                MIDL_INTERFACE("ab3b15f0-14f9-4b9f-9419-a349a242ef38")
                IAppExtension2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppUserModelId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppExtension2 = __uuidof(IAppExtension2);
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtension3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtension3[] = L"Windows.ApplicationModel.AppExtensions.IAppExtension3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                MIDL_INTERFACE("5923c101-aa38-4009-84d9-5b54a0df30ae")
                IAppExtension3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetExtensionProperties(
                        ABI::Windows::Foundation::Collections::IPropertySet** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPublicPath(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPublicFolder(
                        ABI::Windows::Storage::IStorageFolder** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppExtension3 = __uuidof(IAppExtension3);
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtensionCatalog
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtensionCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtensionCatalog[] = L"Windows.ApplicationModel.AppExtensions.IAppExtensionCatalog";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                MIDL_INTERFACE("97872032-8426-4ad1-9084-92e88c2da200")
                IAppExtensionCatalog : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FindAllAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestRemovePackageAsync(
                        HSTRING packageFullName,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PackageInstalled(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PackageInstalled(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PackageUpdating(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PackageUpdating(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PackageUpdated(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PackageUpdated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PackageUninstalling(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PackageUninstalling(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PackageStatusChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PackageStatusChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppExtensionCatalog = __uuidof(IAppExtensionCatalog);
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtensionCatalog2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtensionCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtensionCatalog2[] = L"Windows.ApplicationModel.AppExtensions.IAppExtensionCatalog2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                MIDL_INTERFACE("50056eba-58b6-4147-b5a5-8feca6dfb49d")
                IAppExtensionCatalog2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FindAll(
                        __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppExtensionCatalog2 = __uuidof(IAppExtensionCatalog2);
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtensionCatalogStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtensionCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtensionCatalogStatics[] = L"Windows.ApplicationModel.AppExtensions.IAppExtensionCatalogStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                MIDL_INTERFACE("3c36668a-5f18-4f0b-9ce5-cab61d196f11")
                IAppExtensionCatalogStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Open(
                        HSTRING appExtensionName,
                        ABI::Windows::ApplicationModel::AppExtensions::IAppExtensionCatalog** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppExtensionCatalogStatics = __uuidof(IAppExtensionCatalogStatics);
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtensionPackageInstalledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtensionPackageInstalledEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtensionPackageInstalledEventArgs[] = L"Windows.ApplicationModel.AppExtensions.IAppExtensionPackageInstalledEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                MIDL_INTERFACE("39e59234-3351-4a8d-9745-e7d3dd45bc48")
                IAppExtensionPackageInstalledEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppExtensionName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Package(
                        ABI::Windows::ApplicationModel::IPackage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Extensions(
                        __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension** values
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppExtensionPackageInstalledEventArgs = __uuidof(IAppExtensionPackageInstalledEventArgs);
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtensionPackageStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtensionPackageStatusChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtensionPackageStatusChangedEventArgs[] = L"Windows.ApplicationModel.AppExtensions.IAppExtensionPackageStatusChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                MIDL_INTERFACE("1ce17433-1153-44fd-87b1-8ae1050303df")
                IAppExtensionPackageStatusChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppExtensionName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Package(
                        ABI::Windows::ApplicationModel::IPackage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppExtensionPackageStatusChangedEventArgs = __uuidof(IAppExtensionPackageStatusChangedEventArgs);
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUninstallingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtensionPackageUninstallingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtensionPackageUninstallingEventArgs[] = L"Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUninstallingEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                MIDL_INTERFACE("60f160c5-171e-40ff-ae98-ab2c20dd4d75")
                IAppExtensionPackageUninstallingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppExtensionName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Package(
                        ABI::Windows::ApplicationModel::IPackage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppExtensionPackageUninstallingEventArgs = __uuidof(IAppExtensionPackageUninstallingEventArgs);
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtensionPackageUpdatedEventArgs[] = L"Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUpdatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                MIDL_INTERFACE("3a83c43f-797e-44b5-ba24-a4c8b5a543d7")
                IAppExtensionPackageUpdatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppExtensionName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Package(
                        ABI::Windows::ApplicationModel::IPackage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Extensions(
                        __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension** values
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppExtensionPackageUpdatedEventArgs = __uuidof(IAppExtensionPackageUpdatedEventArgs);
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtensionPackageUpdatingEventArgs[] = L"Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUpdatingEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppExtensions {
                MIDL_INTERFACE("7ed59329-1a65-4800-a700-b321009e306a")
                IAppExtensionPackageUpdatingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppExtensionName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Package(
                        ABI::Windows::ApplicationModel::IPackage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppExtensionPackageUpdatingEventArgs = __uuidof(IAppExtensionPackageUpdatingEventArgs);
            } /* AppExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.AppExtensions.AppExtension
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppExtensions.IAppExtension ** Default Interface **
 *    Windows.ApplicationModel.AppExtensions.IAppExtension2
 *    Windows.ApplicationModel.AppExtensions.IAppExtension3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtension_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtension_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppExtensions_AppExtension[] = L"Windows.ApplicationModel.AppExtensions.AppExtension";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.AppExtensions.AppExtensionCatalog
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.AppExtensions.IAppExtensionCatalogStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppExtensions.IAppExtensionCatalog ** Default Interface **
 *    Windows.ApplicationModel.AppExtensions.IAppExtensionCatalog2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionCatalog_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionCatalog_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppExtensions_AppExtensionCatalog[] = L"Windows.ApplicationModel.AppExtensions.AppExtensionCatalog";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.AppExtensions.AppExtensionPackageInstalledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppExtensions.IAppExtensionPackageInstalledEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageInstalledEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageInstalledEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppExtensions_AppExtensionPackageInstalledEventArgs[] = L"Windows.ApplicationModel.AppExtensions.AppExtensionPackageInstalledEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.AppExtensions.AppExtensionPackageStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppExtensions.IAppExtensionPackageStatusChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageStatusChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageStatusChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppExtensions_AppExtensionPackageStatusChangedEventArgs[] = L"Windows.ApplicationModel.AppExtensions.AppExtensionPackageStatusChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.AppExtensions.AppExtensionPackageUninstallingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUninstallingEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageUninstallingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageUninstallingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppExtensions_AppExtensionPackageUninstallingEventArgs[] = L"Windows.ApplicationModel.AppExtensions.AppExtensionPackageUninstallingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUpdatedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageUpdatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageUpdatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppExtensions_AppExtensionPackageUpdatedEventArgs[] = L"Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUpdatingEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageUpdatingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageUpdatingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppExtensions_AppExtensionPackageUpdatingEventArgs[] = L"Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2 __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3 __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2 __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___F__CIPropertySet __FIAsyncOperationCompletedHandler_1___F__CIPropertySet;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___F__CIPropertySet_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___F__CIPropertySet_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___F__CIPropertySet __FIAsyncOperation_1___F__CIPropertySet;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___F__CIPropertySet;

typedef struct __FIAsyncOperation_1___F__CIPropertySetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___F__CIPropertySet* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___F__CIPropertySet* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___F__CIPropertySet* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___F__CIPropertySet* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___F__CIPropertySet* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___F__CIPropertySet* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___F__CIPropertySet* This,
        __FIAsyncOperationCompletedHandler_1___F__CIPropertySet* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___F__CIPropertySet* This,
        __FIAsyncOperationCompletedHandler_1___F__CIPropertySet** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___F__CIPropertySet* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** result);

    END_INTERFACE
} __FIAsyncOperation_1___F__CIPropertySetVtbl;

interface __FIAsyncOperation_1___F__CIPropertySet
{
    CONST_VTBL struct __FIAsyncOperation_1___F__CIPropertySetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___F__CIPropertySet_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___F__CIPropertySet_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___F__CIPropertySet_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___F__CIPropertySet_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___F__CIPropertySet_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___F__CIPropertySet_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___F__CIPropertySet_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___F__CIPropertySet_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___F__CIPropertySet_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___F__CIPropertySet_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___F__CIPropertySet_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___F__CIPropertySet_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___F__CIPropertySet __FIAsyncOperationCompletedHandler_1___F__CIPropertySet;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___F__CIPropertySet;

typedef struct __FIAsyncOperationCompletedHandler_1___F__CIPropertySetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___F__CIPropertySet* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___F__CIPropertySet* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___F__CIPropertySet* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___F__CIPropertySet* This,
        __FIAsyncOperation_1___F__CIPropertySet* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___F__CIPropertySetVtbl;

interface __FIAsyncOperationCompletedHandler_1___F__CIPropertySet
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___F__CIPropertySetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___F__CIPropertySet_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___F__CIPropertySet_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___F__CIPropertySet_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___F__CIPropertySet_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___F__CIPropertySet_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension;

typedef struct __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtensionVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension __FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension;

typedef struct __FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        __FIIterator_1_Windows__CApplicationModel__CAppExtensions__CAppExtension** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtensionVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtensionVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtensionVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtensionVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFolder __x_ABI_CWindows_CStorage_CIStorageFolder;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStorageFolder __FIAsyncOperation_1_Windows__CStorage__CStorageFolder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStorageFolder;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStorageFolderVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStorageFolder
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolderVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* sender,
        __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* sender,
        __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* sender,
        __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* sender,
        __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* sender,
        __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIAppInfo __x_ABI_CWindows_CApplicationModel_CIAppInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackage __x_ABI_CWindows_CApplicationModel_CIPackage;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage_FWD_DEFINED__

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtension
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtension[] = L"Windows.ApplicationModel.AppExtensions.IAppExtension";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);
    HRESULT (STDMETHODCALLTYPE* get_AppInfo)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension* This,
        __x_ABI_CWindows_CApplicationModel_CIAppInfo** value);
    HRESULT (STDMETHODCALLTYPE* GetExtensionPropertiesAsync)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension* This,
        __FIAsyncOperation_1___F__CIPropertySet** operation);
    HRESULT (STDMETHODCALLTYPE* GetPublicFolderAsync)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_get_AppInfo(This, value) \
    ((This)->lpVtbl->get_AppInfo(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_GetExtensionPropertiesAsync(This, operation) \
    ((This)->lpVtbl->GetExtensionPropertiesAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_GetPublicFolderAsync(This, operation) \
    ((This)->lpVtbl->GetPublicFolderAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtension2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtension2[] = L"Windows.ApplicationModel.AppExtensions.IAppExtension2";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppUserModelId)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_get_AppUserModelId(This, value) \
    ((This)->lpVtbl->get_AppUserModelId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtension3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtension3[] = L"Windows.ApplicationModel.AppExtensions.IAppExtension3";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetExtensionProperties)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** result);
    HRESULT (STDMETHODCALLTYPE* GetPublicPath)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* GetPublicFolder)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_GetExtensionProperties(This, result) \
    ((This)->lpVtbl->GetExtensionProperties(This, result))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_GetPublicPath(This, result) \
    ((This)->lpVtbl->GetPublicPath(This, result))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_GetPublicFolder(This, result) \
    ((This)->lpVtbl->GetPublicFolder(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtension3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtensionCatalog
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtensionCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtensionCatalog[] = L"Windows.ApplicationModel.AppExtensions.IAppExtensionCatalog";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindAllAsync)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension** operation);
    HRESULT (STDMETHODCALLTYPE* RequestRemovePackageAsync)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* This,
        HSTRING packageFullName,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* add_PackageInstalled)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageInstalledEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PackageInstalled)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PackageUpdating)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PackageUpdating)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PackageUpdated)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUpdatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PackageUpdated)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PackageUninstalling)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageUninstallingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PackageUninstalling)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PackageStatusChanged)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CAppExtensions__CAppExtensionCatalog_Windows__CApplicationModel__CAppExtensions__CAppExtensionPackageStatusChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PackageStatusChanged)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_FindAllAsync(This, operation) \
    ((This)->lpVtbl->FindAllAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_RequestRemovePackageAsync(This, packageFullName, operation) \
    ((This)->lpVtbl->RequestRemovePackageAsync(This, packageFullName, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_add_PackageInstalled(This, handler, token) \
    ((This)->lpVtbl->add_PackageInstalled(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_remove_PackageInstalled(This, token) \
    ((This)->lpVtbl->remove_PackageInstalled(This, token))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_add_PackageUpdating(This, handler, token) \
    ((This)->lpVtbl->add_PackageUpdating(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_remove_PackageUpdating(This, token) \
    ((This)->lpVtbl->remove_PackageUpdating(This, token))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_add_PackageUpdated(This, handler, token) \
    ((This)->lpVtbl->add_PackageUpdated(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_remove_PackageUpdated(This, token) \
    ((This)->lpVtbl->remove_PackageUpdated(This, token))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_add_PackageUninstalling(This, handler, token) \
    ((This)->lpVtbl->add_PackageUninstalling(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_remove_PackageUninstalling(This, token) \
    ((This)->lpVtbl->remove_PackageUninstalling(This, token))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_add_PackageStatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_PackageStatusChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_remove_PackageStatusChanged(This, token) \
    ((This)->lpVtbl->remove_PackageStatusChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtensionCatalog2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtensionCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtensionCatalog2[] = L"Windows.ApplicationModel.AppExtensions.IAppExtensionCatalog2";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindAll)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2* This,
        __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_FindAll(This, result) \
    ((This)->lpVtbl->FindAll(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtensionCatalogStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtensionCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtensionCatalogStatics[] = L"Windows.ApplicationModel.AppExtensions.IAppExtensionCatalogStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Open)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics* This,
        HSTRING appExtensionName,
        __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalog** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_Open(This, appExtensionName, value) \
    ((This)->lpVtbl->Open(This, appExtensionName, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionCatalogStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtensionPackageInstalledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtensionPackageInstalledEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtensionPackageInstalledEventArgs[] = L"Windows.ApplicationModel.AppExtensions.IAppExtensionPackageInstalledEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppExtensionName)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);
    HRESULT (STDMETHODCALLTYPE* get_Extensions)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs* This,
        __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension** values);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_get_AppExtensionName(This, value) \
    ((This)->lpVtbl->get_AppExtensionName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_get_Extensions(This, values) \
    ((This)->lpVtbl->get_Extensions(This, values))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageInstalledEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtensionPackageStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtensionPackageStatusChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtensionPackageStatusChangedEventArgs[] = L"Windows.ApplicationModel.AppExtensions.IAppExtensionPackageStatusChangedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppExtensionName)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_get_AppExtensionName(This, value) \
    ((This)->lpVtbl->get_AppExtensionName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageStatusChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUninstallingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtensionPackageUninstallingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtensionPackageUninstallingEventArgs[] = L"Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUninstallingEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppExtensionName)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_get_AppExtensionName(This, value) \
    ((This)->lpVtbl->get_AppExtensionName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUninstallingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtensionPackageUpdatedEventArgs[] = L"Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUpdatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppExtensionName)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);
    HRESULT (STDMETHODCALLTYPE* get_Extensions)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs* This,
        __FIVectorView_1_Windows__CApplicationModel__CAppExtensions__CAppExtension** values);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_get_AppExtensionName(This, value) \
    ((This)->lpVtbl->get_AppExtensionName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_get_Extensions(This, values) \
    ((This)->lpVtbl->get_Extensions(This, values))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppExtensions_IAppExtensionPackageUpdatingEventArgs[] = L"Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUpdatingEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppExtensionName)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_get_AppExtensionName(This, value) \
    ((This)->lpVtbl->get_AppExtensionName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppExtensions_CIAppExtensionPackageUpdatingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.AppExtensions.AppExtension
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppExtensions.IAppExtension ** Default Interface **
 *    Windows.ApplicationModel.AppExtensions.IAppExtension2
 *    Windows.ApplicationModel.AppExtensions.IAppExtension3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtension_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtension_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppExtensions_AppExtension[] = L"Windows.ApplicationModel.AppExtensions.AppExtension";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.AppExtensions.AppExtensionCatalog
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.AppExtensions.IAppExtensionCatalogStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppExtensions.IAppExtensionCatalog ** Default Interface **
 *    Windows.ApplicationModel.AppExtensions.IAppExtensionCatalog2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionCatalog_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionCatalog_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppExtensions_AppExtensionCatalog[] = L"Windows.ApplicationModel.AppExtensions.AppExtensionCatalog";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.AppExtensions.AppExtensionPackageInstalledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppExtensions.IAppExtensionPackageInstalledEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageInstalledEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageInstalledEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppExtensions_AppExtensionPackageInstalledEventArgs[] = L"Windows.ApplicationModel.AppExtensions.AppExtensionPackageInstalledEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.AppExtensions.AppExtensionPackageStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppExtensions.IAppExtensionPackageStatusChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageStatusChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageStatusChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppExtensions_AppExtensionPackageStatusChangedEventArgs[] = L"Windows.ApplicationModel.AppExtensions.AppExtensionPackageStatusChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.AppExtensions.AppExtensionPackageUninstallingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUninstallingEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageUninstallingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageUninstallingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppExtensions_AppExtensionPackageUninstallingEventArgs[] = L"Windows.ApplicationModel.AppExtensions.AppExtensionPackageUninstallingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUpdatedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageUpdatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageUpdatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppExtensions_AppExtensionPackageUpdatedEventArgs[] = L"Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUpdatingEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageUpdatingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppExtensions_AppExtensionPackageUpdatingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppExtensions_AppExtensionPackageUpdatingEventArgs[] = L"Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Eappextensions_p_h__

#endif // __windows2Eapplicationmodel2Eappextensions_h__
