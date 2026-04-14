
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
#ifndef __windows2Eapplicationmodel2Epackageextensions_h__
#define __windows2Eapplicationmodel2Epackageextensions_h__
#ifndef __windows2Eapplicationmodel2Epackageextensions_p_h__
#define __windows2Eapplicationmodel2Epackageextensions_p_h__


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
#ifndef ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                interface IPackageExtension;
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtension

#endif // ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                interface IPackageExtensionCatalog;
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtensionCatalog

#endif // ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                interface IPackageExtensionCatalogStatics;
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtensionCatalogStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                interface IPackageExtensionPackageInstalledEventArgs;
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtensionPackageInstalledEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                interface IPackageExtensionPackageStatusChangedEventArgs;
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtensionPackageStatusChangedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                interface IPackageExtensionPackageUninstallingEventArgs;
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtensionPackageUninstallingEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                interface IPackageExtensionPackageUpdatedEventArgs;
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtensionPackageUpdatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                interface IPackageExtensionPackageUpdatingEventArgs;
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtensionPackageUpdatingEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_FWD_DEFINED__

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
            namespace PackageExtensions {
                class PackageExtension;
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6124c984-5e57-5a3a-810c-0a04ca9fa471"))
IIterator<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtension*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtension*, ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtension*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.PackageExtensions.PackageExtension>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtension*> __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_t;
#define __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ed0a489a-437c-5629-b0ef-91bbad5bae59"))
IIterable<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtension*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtension*, ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtension*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.PackageExtensions.PackageExtension>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtension*> __FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_t;
#define __FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6b8001c9-1f6d-593f-a163-2b78b20a65a3"))
IVectorView<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtension*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtension*, ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtension*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.PackageExtensions.PackageExtension>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtension*> __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_t;
#define __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("89631036-91d7-505b-bbfd-4df6d97b6641"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.PackageExtensions.PackageExtension>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0fbafc98-e944-55e5-89bc-20a2ab19f09e"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.PackageExtensions.PackageExtension>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

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
            namespace PackageExtensions {
                class PackageExtensionCatalog;
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                class PackageExtensionPackageInstalledEventArgs;
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f0d4b2dd-52de-5f3a-93e3-3a04966b4cb3"))
ITypedEventHandler<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionCatalog*, ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionPackageInstalledEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionCatalog*, ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtensionCatalog*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionPackageInstalledEventArgs*, ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtensionPackageInstalledEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.PackageExtensions.PackageExtensionCatalog, Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageInstalledEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionCatalog*, ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionPackageInstalledEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                class PackageExtensionPackageStatusChangedEventArgs;
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6a2f2758-898a-5622-8a97-bb2fc8a2d7db"))
ITypedEventHandler<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionCatalog*, ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionPackageStatusChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionCatalog*, ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtensionCatalog*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionPackageStatusChangedEventArgs*, ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtensionPackageStatusChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.PackageExtensions.PackageExtensionCatalog, Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageStatusChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionCatalog*, ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionPackageStatusChangedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                class PackageExtensionPackageUninstallingEventArgs;
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d3c3aa82-8fea-56e6-b02c-75689b3c6cf4"))
ITypedEventHandler<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionCatalog*, ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionPackageUninstallingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionCatalog*, ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtensionCatalog*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionPackageUninstallingEventArgs*, ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtensionPackageUninstallingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.PackageExtensions.PackageExtensionCatalog, Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUninstallingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionCatalog*, ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionPackageUninstallingEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                class PackageExtensionPackageUpdatedEventArgs;
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3451288e-8d64-5204-b06b-0bb1b2537b4e"))
ITypedEventHandler<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionCatalog*, ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionPackageUpdatedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionCatalog*, ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtensionCatalog*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionPackageUpdatedEventArgs*, ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtensionPackageUpdatedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.PackageExtensions.PackageExtensionCatalog, Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUpdatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionCatalog*, ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionPackageUpdatedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                class PackageExtensionPackageUpdatingEventArgs;
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a5b1fde1-c415-5459-abb7-3271e06320c4"))
ITypedEventHandler<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionCatalog*, ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionPackageUpdatingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionCatalog*, ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtensionCatalog*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionPackageUpdatingEventArgs*, ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtensionPackageUpdatingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.PackageExtensions.PackageExtensionCatalog, Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUpdatingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionCatalog*, ABI::Windows::ApplicationModel::PackageExtensions::PackageExtensionPackageUpdatingEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

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
 * Interface Windows.ApplicationModel.PackageExtensions.IPackageExtension
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageExtensions.PackageExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_PackageExtensions_IPackageExtension[] = L"Windows.ApplicationModel.PackageExtensions.IPackageExtension";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                MIDL_INTERFACE("da70c957-7ead-5c3b-9cf0-cc43faf474b4")
                IPackageExtension : public IInspectable
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
                    virtual HRESULT STDMETHODCALLTYPE GetExtensionProperties(
                        ABI::Windows::Foundation::Collections::IPropertySet** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetExtensionPropertiesAsync(
                        __FIAsyncOperation_1___F__CIPropertySet** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPublicPath(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPublicFolder(
                        ABI::Windows::Storage::IStorageFolder** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPublicFolderAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPackageExtension = __uuidof(IPackageExtension);
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.ApplicationModel.PackageExtensions.IPackageExtensionCatalog
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageExtensions.PackageExtensionCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_PackageExtensions_IPackageExtensionCatalog[] = L"Windows.ApplicationModel.PackageExtensions.IPackageExtensionCatalog";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                MIDL_INTERFACE("0879dfe6-ac30-58b2-97f9-480b07e75bfa")
                IPackageExtensionCatalog : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FindAll(
                        __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestRemovePackageAsync(
                        HSTRING packageFullName,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PackageInstalled(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PackageInstalled(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PackageUpdating(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PackageUpdating(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PackageUpdated(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PackageUpdated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PackageUninstalling(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PackageUninstalling(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PackageStatusChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PackageStatusChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPackageExtensionCatalog = __uuidof(IPackageExtensionCatalog);
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.ApplicationModel.PackageExtensions.IPackageExtensionCatalogStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageExtensions.PackageExtensionCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_PackageExtensions_IPackageExtensionCatalogStatics[] = L"Windows.ApplicationModel.PackageExtensions.IPackageExtensionCatalogStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                MIDL_INTERFACE("9588ece4-3183-5684-9e5f-27759733ddfe")
                IPackageExtensionCatalogStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Open(
                        HSTRING packageExtensionName,
                        ABI::Windows::ApplicationModel::PackageExtensions::IPackageExtensionCatalog** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPackageExtensionCatalogStatics = __uuidof(IPackageExtensionCatalogStatics);
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageInstalledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageInstalledEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_PackageExtensions_IPackageExtensionPackageInstalledEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageInstalledEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                MIDL_INTERFACE("3c9b0067-083c-5fe3-bdfb-9feb156b4118")
                IPackageExtensionPackageInstalledEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PackageExtensionName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Package(
                        ABI::Windows::ApplicationModel::IPackage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Extensions(
                        __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPackageExtensionPackageInstalledEventArgs = __uuidof(IPackageExtensionPackageInstalledEventArgs);
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageStatusChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_PackageExtensions_IPackageExtensionPackageStatusChangedEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageStatusChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                MIDL_INTERFACE("b8fee20a-680d-5942-876c-5de12df1083c")
                IPackageExtensionPackageStatusChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PackageExtensionName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Package(
                        ABI::Windows::ApplicationModel::IPackage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPackageExtensionPackageStatusChangedEventArgs = __uuidof(IPackageExtensionPackageStatusChangedEventArgs);
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageUninstallingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUninstallingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_PackageExtensions_IPackageExtensionPackageUninstallingEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageUninstallingEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                MIDL_INTERFACE("3b8e9cb7-c539-554d-bb33-a84c0bfa3f50")
                IPackageExtensionPackageUninstallingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PackageExtensionName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Package(
                        ABI::Windows::ApplicationModel::IPackage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPackageExtensionPackageUninstallingEventArgs = __uuidof(IPackageExtensionPackageUninstallingEventArgs);
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUpdatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_PackageExtensions_IPackageExtensionPackageUpdatedEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageUpdatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                MIDL_INTERFACE("fdc31add-16a7-509d-8bc4-fde22e856d2d")
                IPackageExtensionPackageUpdatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PackageExtensionName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Package(
                        ABI::Windows::ApplicationModel::IPackage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Extensions(
                        __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPackageExtensionPackageUpdatedEventArgs = __uuidof(IPackageExtensionPackageUpdatedEventArgs);
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUpdatingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_PackageExtensions_IPackageExtensionPackageUpdatingEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageUpdatingEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace PackageExtensions {
                MIDL_INTERFACE("27ae2ce1-a1d3-532e-8e7e-8b43782fce09")
                IPackageExtensionPackageUpdatingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PackageExtensionName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Package(
                        ABI::Windows::ApplicationModel::IPackage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPackageExtensionPackageUpdatingEventArgs = __uuidof(IPackageExtensionPackageUpdatingEventArgs);
            } /* PackageExtensions */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.ApplicationModel.PackageExtensions.PackageExtension
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.PackageExtensions.IPackageExtension ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtension_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtension_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageExtensions_PackageExtension[] = L"Windows.ApplicationModel.PackageExtensions.PackageExtension";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.ApplicationModel.PackageExtensions.PackageExtensionCatalog
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.PackageExtensions.IPackageExtensionCatalogStatics interface starting with version 17.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.PackageExtensions.IPackageExtensionCatalog ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionCatalog_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionCatalog_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageExtensions_PackageExtensionCatalog[] = L"Windows.ApplicationModel.PackageExtensions.PackageExtensionCatalog";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageInstalledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageInstalledEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageInstalledEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageInstalledEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageInstalledEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageInstalledEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageStatusChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageStatusChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageStatusChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageStatusChangedEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageStatusChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUninstallingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageUninstallingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageUninstallingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageUninstallingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageUninstallingEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUninstallingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageUpdatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageUpdatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageUpdatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageUpdatedEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUpdatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageUpdatingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageUpdatingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageUpdatingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageUpdatingEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUpdatingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension;

#endif // ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog;

#endif // ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_FWD_DEFINED__

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension;

typedef struct __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension __FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension;

typedef struct __FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        __FIIterator_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* sender,
        __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* sender,
        __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* sender,
        __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* sender,
        __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* sender,
        __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackage __x_ABI_CWindows_CApplicationModel_CIPackage;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage_FWD_DEFINED__

/*
 *
 * Interface Windows.ApplicationModel.PackageExtensions.IPackageExtension
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageExtensions.PackageExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_PackageExtensions_IPackageExtension[] = L"Windows.ApplicationModel.PackageExtensions.IPackageExtension";
typedef struct __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);
    HRESULT (STDMETHODCALLTYPE* GetExtensionProperties)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** result);
    HRESULT (STDMETHODCALLTYPE* GetExtensionPropertiesAsync)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension* This,
        __FIAsyncOperation_1___F__CIPropertySet** operation);
    HRESULT (STDMETHODCALLTYPE* GetPublicPath)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* GetPublicFolder)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** result);
    HRESULT (STDMETHODCALLTYPE* GetPublicFolderAsync)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_GetExtensionProperties(This, result) \
    ((This)->lpVtbl->GetExtensionProperties(This, result))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_GetExtensionPropertiesAsync(This, operation) \
    ((This)->lpVtbl->GetExtensionPropertiesAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_GetPublicPath(This, result) \
    ((This)->lpVtbl->GetPublicPath(This, result))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_GetPublicFolder(This, result) \
    ((This)->lpVtbl->GetPublicFolder(This, result))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_GetPublicFolderAsync(This, operation) \
    ((This)->lpVtbl->GetPublicFolderAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtension_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.ApplicationModel.PackageExtensions.IPackageExtensionCatalog
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageExtensions.PackageExtensionCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_PackageExtensions_IPackageExtensionCatalog[] = L"Windows.ApplicationModel.PackageExtensions.IPackageExtensionCatalog";
typedef struct __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindAll)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This,
        __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension** result);
    HRESULT (STDMETHODCALLTYPE* FindAllAsync)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension** operation);
    HRESULT (STDMETHODCALLTYPE* RequestRemovePackageAsync)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This,
        HSTRING packageFullName,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* add_PackageInstalled)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageInstalledEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PackageInstalled)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PackageUpdating)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PackageUpdating)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PackageUpdated)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUpdatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PackageUpdated)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PackageUninstalling)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageUninstallingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PackageUninstalling)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PackageStatusChanged)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionCatalog_Windows__CApplicationModel__CPackageExtensions__CPackageExtensionPackageStatusChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PackageStatusChanged)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_FindAll(This, result) \
    ((This)->lpVtbl->FindAll(This, result))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_FindAllAsync(This, operation) \
    ((This)->lpVtbl->FindAllAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_RequestRemovePackageAsync(This, packageFullName, operation) \
    ((This)->lpVtbl->RequestRemovePackageAsync(This, packageFullName, operation))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_add_PackageInstalled(This, handler, token) \
    ((This)->lpVtbl->add_PackageInstalled(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_remove_PackageInstalled(This, token) \
    ((This)->lpVtbl->remove_PackageInstalled(This, token))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_add_PackageUpdating(This, handler, token) \
    ((This)->lpVtbl->add_PackageUpdating(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_remove_PackageUpdating(This, token) \
    ((This)->lpVtbl->remove_PackageUpdating(This, token))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_add_PackageUpdated(This, handler, token) \
    ((This)->lpVtbl->add_PackageUpdated(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_remove_PackageUpdated(This, token) \
    ((This)->lpVtbl->remove_PackageUpdated(This, token))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_add_PackageUninstalling(This, handler, token) \
    ((This)->lpVtbl->add_PackageUninstalling(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_remove_PackageUninstalling(This, token) \
    ((This)->lpVtbl->remove_PackageUninstalling(This, token))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_add_PackageStatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_PackageStatusChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_remove_PackageStatusChanged(This, token) \
    ((This)->lpVtbl->remove_PackageStatusChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.ApplicationModel.PackageExtensions.IPackageExtensionCatalogStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageExtensions.PackageExtensionCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_PackageExtensions_IPackageExtensionCatalogStatics[] = L"Windows.ApplicationModel.PackageExtensions.IPackageExtensionCatalogStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Open)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics* This,
        HSTRING packageExtensionName,
        __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalog** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_Open(This, packageExtensionName, result) \
    ((This)->lpVtbl->Open(This, packageExtensionName, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionCatalogStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageInstalledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageInstalledEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_PackageExtensions_IPackageExtensionPackageInstalledEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageInstalledEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PackageExtensionName)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);
    HRESULT (STDMETHODCALLTYPE* get_Extensions)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs* This,
        __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_get_PackageExtensionName(This, value) \
    ((This)->lpVtbl->get_PackageExtensionName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_get_Extensions(This, value) \
    ((This)->lpVtbl->get_Extensions(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageInstalledEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageStatusChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_PackageExtensions_IPackageExtensionPackageStatusChangedEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageStatusChangedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PackageExtensionName)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_get_PackageExtensionName(This, value) \
    ((This)->lpVtbl->get_PackageExtensionName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageStatusChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageUninstallingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUninstallingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_PackageExtensions_IPackageExtensionPackageUninstallingEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageUninstallingEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PackageExtensionName)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_get_PackageExtensionName(This, value) \
    ((This)->lpVtbl->get_PackageExtensionName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUninstallingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUpdatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_PackageExtensions_IPackageExtensionPackageUpdatedEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageUpdatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PackageExtensionName)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);
    HRESULT (STDMETHODCALLTYPE* get_Extensions)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs* This,
        __FIVectorView_1_Windows__CApplicationModel__CPackageExtensions__CPackageExtension** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_get_PackageExtensionName(This, value) \
    ((This)->lpVtbl->get_PackageExtensionName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_get_Extensions(This, value) \
    ((This)->lpVtbl->get_Extensions(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUpdatingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_PackageExtensions_IPackageExtensionPackageUpdatingEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageUpdatingEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PackageExtensionName)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_get_PackageExtensionName(This, value) \
    ((This)->lpVtbl->get_PackageExtensionName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPackageExtensions_CIPackageExtensionPackageUpdatingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.ApplicationModel.PackageExtensions.PackageExtension
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.PackageExtensions.IPackageExtension ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtension_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtension_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageExtensions_PackageExtension[] = L"Windows.ApplicationModel.PackageExtensions.PackageExtension";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.ApplicationModel.PackageExtensions.PackageExtensionCatalog
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.PackageExtensions.IPackageExtensionCatalogStatics interface starting with version 17.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.PackageExtensions.IPackageExtensionCatalog ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionCatalog_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionCatalog_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageExtensions_PackageExtensionCatalog[] = L"Windows.ApplicationModel.PackageExtensions.PackageExtensionCatalog";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageInstalledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageInstalledEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageInstalledEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageInstalledEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageInstalledEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageInstalledEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageStatusChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageStatusChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageStatusChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageStatusChangedEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageStatusChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUninstallingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageUninstallingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageUninstallingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageUninstallingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageUninstallingEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUninstallingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageUpdatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageUpdatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageUpdatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageUpdatedEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUpdatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.PackageExtensions.IPackageExtensionPackageUpdatingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageUpdatingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageUpdatingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageExtensions_PackageExtensionPackageUpdatingEventArgs[] = L"Windows.ApplicationModel.PackageExtensions.PackageExtensionPackageUpdatingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Epackageextensions_p_h__

#endif // __windows2Eapplicationmodel2Epackageextensions_h__
