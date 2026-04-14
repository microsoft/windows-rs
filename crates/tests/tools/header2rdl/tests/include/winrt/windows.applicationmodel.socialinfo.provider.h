
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
#ifndef __windows2Eapplicationmodel2Esocialinfo2Eprovider_h__
#define __windows2Eapplicationmodel2Esocialinfo2Eprovider_h__
#ifndef __windows2Eapplicationmodel2Esocialinfo2Eprovider_p_h__
#define __windows2Eapplicationmodel2Esocialinfo2Eprovider_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION)

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
#include "Windows.ApplicationModel.SocialInfo.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                namespace Provider {
                    interface ISocialDashboardItemUpdater;
                } /* Provider */
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater ABI::Windows::ApplicationModel::SocialInfo::Provider::ISocialDashboardItemUpdater

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                namespace Provider {
                    interface ISocialFeedUpdater;
                } /* Provider */
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater ABI::Windows::ApplicationModel::SocialInfo::Provider::ISocialFeedUpdater

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                namespace Provider {
                    interface ISocialInfoProviderManagerStatics;
                } /* Provider */
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics ABI::Windows::ApplicationModel::SocialInfo::Provider::ISocialInfoProviderManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_FWD_DEFINED__

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
        namespace ApplicationModel {
            namespace SocialInfo {
                namespace Provider {
                    class SocialDashboardItemUpdater;
                } /* Provider */
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8bbca9a4-a4fc-5fe2-b6b1-0e5a75d05b07"))
IAsyncOperation<ABI::Windows::ApplicationModel::SocialInfo::Provider::SocialDashboardItemUpdater*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::SocialInfo::Provider::SocialDashboardItemUpdater*, ABI::Windows::ApplicationModel::SocialInfo::Provider::ISocialDashboardItemUpdater*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.SocialInfo.Provider.SocialDashboardItemUpdater>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::SocialInfo::Provider::SocialDashboardItemUpdater*> __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_USE */

#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("33cfd9aa-6c3c-50df-bbc8-34c22a0e5b6b"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::SocialInfo::Provider::SocialDashboardItemUpdater*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::SocialInfo::Provider::SocialDashboardItemUpdater*, ABI::Windows::ApplicationModel::SocialInfo::Provider::ISocialDashboardItemUpdater*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.SocialInfo.Provider.SocialDashboardItemUpdater>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::SocialInfo::Provider::SocialDashboardItemUpdater*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_USE */

#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                namespace Provider {
                    class SocialFeedUpdater;
                } /* Provider */
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4efa4da9-c556-59f4-9d99-e7801c5b0f45"))
IAsyncOperation<ABI::Windows::ApplicationModel::SocialInfo::Provider::SocialFeedUpdater*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::SocialInfo::Provider::SocialFeedUpdater*, ABI::Windows::ApplicationModel::SocialInfo::Provider::ISocialFeedUpdater*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.SocialInfo.Provider.SocialFeedUpdater>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::SocialInfo::Provider::SocialFeedUpdater*> __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_USE */

#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0b227474-80c0-5f33-9ff9-234403abd6fa"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::SocialInfo::Provider::SocialFeedUpdater*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::SocialInfo::Provider::SocialFeedUpdater*, ABI::Windows::ApplicationModel::SocialInfo::Provider::ISocialFeedUpdater*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.SocialInfo.Provider.SocialFeedUpdater>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::SocialInfo::Provider::SocialFeedUpdater*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_USE */

#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                class SocialFeedItem;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                interface ISocialFeedItem;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedItem

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_FWD_DEFINED__

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e7c8cd1f-3907-5da8-9d72-90426dc37072"))
IIterator<ABI::Windows::ApplicationModel::SocialInfo::SocialFeedItem*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::SocialInfo::SocialFeedItem*, ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.SocialInfo.SocialFeedItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::SocialInfo::SocialFeedItem*> __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_t;
#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_USE */

#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ad33d864-9569-5e2d-bd72-182a8ff50cf6"))
IIterable<ABI::Windows::ApplicationModel::SocialInfo::SocialFeedItem*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::SocialInfo::SocialFeedItem*, ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.SocialInfo.SocialFeedItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::SocialInfo::SocialFeedItem*> __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_t;
#define __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_USE */

#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e6be2bb8-fc75-585c-836c-34f3ff87680f"))
IVectorView<ABI::Windows::ApplicationModel::SocialInfo::SocialFeedItem*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::SocialInfo::SocialFeedItem*, ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.SocialInfo.SocialFeedItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::SocialInfo::SocialFeedItem*> __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_t;
#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_USE */

#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_USE
#define DEF___FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("36cd5297-36c3-56a7-9656-ec9d5bde7aba"))
IVector<ABI::Windows::ApplicationModel::SocialInfo::SocialFeedItem*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::SocialInfo::SocialFeedItem*, ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.ApplicationModel.SocialInfo.SocialFeedItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::ApplicationModel::SocialInfo::SocialFeedItem*> __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_t;
#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_USE */

#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                class SocialFeedContent;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                interface ISocialFeedContent;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedContent

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                typedef enum SocialFeedKind : int SocialFeedKind;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                typedef enum SocialFeedUpdateMode : int SocialFeedUpdateMode;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                class SocialItemThumbnail;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                interface ISocialItemThumbnail;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail ABI::Windows::ApplicationModel::SocialInfo::ISocialItemThumbnail

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_FWD_DEFINED__

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

/*
 *
 * Interface Windows.ApplicationModel.SocialInfo.Provider.ISocialDashboardItemUpdater
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.SocialInfo.Provider.SocialDashboardItemUpdater
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_SocialInfo_Provider_ISocialDashboardItemUpdater[] = L"Windows.ApplicationModel.SocialInfo.Provider.ISocialDashboardItemUpdater";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                namespace Provider {
                    MIDL_INTERFACE("3cde9dc9-4800-46cd-869b-1973ec685bde")
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    ISocialDashboardItemUpdater : public IInspectable
                    {
                    public:
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_OwnerRemoteId(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Content(
                            ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedContent** value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE put_Timestamp(
                            ABI::Windows::Foundation::DateTime value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE put_Thumbnail(
                            ABI::Windows::ApplicationModel::SocialInfo::ISocialItemThumbnail* value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Thumbnail(
                            ABI::Windows::ApplicationModel::SocialInfo::ISocialItemThumbnail** value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE CommitAsync(
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_TargetUri(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE put_TargetUri(
                            ABI::Windows::Foundation::IUriRuntimeClass* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISocialDashboardItemUpdater = __uuidof(ISocialDashboardItemUpdater);
                } /* Provider */
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.SocialInfo.Provider.ISocialFeedUpdater
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.SocialInfo.Provider.SocialFeedUpdater
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_SocialInfo_Provider_ISocialFeedUpdater[] = L"Windows.ApplicationModel.SocialInfo.Provider.ISocialFeedUpdater";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                namespace Provider {
                    MIDL_INTERFACE("7a0c0aa7-ed89-4bd5-a8d9-15f4d9861c10")
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    ISocialFeedUpdater : public IInspectable
                    {
                    public:
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialFeedUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_OwnerRemoteId(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialFeedUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Kind(
                            ABI::Windows::ApplicationModel::SocialInfo::SocialFeedKind* value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialFeedUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Items(
                            __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem** value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialFeedUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE CommitAsync(
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISocialFeedUpdater = __uuidof(ISocialFeedUpdater);
                } /* Provider */
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.SocialInfo.Provider.ISocialInfoProviderManagerStatics
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.SocialInfo.Provider.SocialInfoProviderManager
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_SocialInfo_Provider_ISocialInfoProviderManagerStatics[] = L"Windows.ApplicationModel.SocialInfo.Provider.ISocialInfoProviderManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                namespace Provider {
                    MIDL_INTERFACE("1b88e52b-7787-48d6-aa12-d8e8f47ab85a")
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    ISocialInfoProviderManagerStatics : public IInspectable
                    {
                    public:
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE CreateSocialFeedUpdaterAsync(
                            ABI::Windows::ApplicationModel::SocialInfo::SocialFeedKind kind,
                            ABI::Windows::ApplicationModel::SocialInfo::SocialFeedUpdateMode mode,
                            HSTRING ownerRemoteId,
                            __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater** operation
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE CreateDashboardItemUpdaterAsync(
                            HSTRING ownerRemoteId,
                            __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater** operation
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE UpdateBadgeCountValue(
                            HSTRING itemRemoteId,
                            INT32 newCount
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE ReportNewContentAvailable(
                            HSTRING contactRemoteId,
                            ABI::Windows::ApplicationModel::SocialInfo::SocialFeedKind kind
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE ProvisionAsync(
                            __FIAsyncOperation_1_boolean** operation
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE DeprovisionAsync(
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISocialInfoProviderManagerStatics = __uuidof(ISocialInfoProviderManagerStatics);
                } /* Provider */
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SocialInfo.Provider.SocialDashboardItemUpdater
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.SocialInfo.Provider.ISocialDashboardItemUpdater ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_Provider_SocialDashboardItemUpdater_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_Provider_SocialDashboardItemUpdater_DEFINED
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SocialInfo_Provider_SocialDashboardItemUpdater[] = L"Windows.ApplicationModel.SocialInfo.Provider.SocialDashboardItemUpdater";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SocialInfo.Provider.SocialFeedUpdater
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.SocialInfo.Provider.ISocialFeedUpdater ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_Provider_SocialFeedUpdater_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_Provider_SocialFeedUpdater_DEFINED
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialFeedUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SocialInfo_Provider_SocialFeedUpdater[] = L"Windows.ApplicationModel.SocialInfo.Provider.SocialFeedUpdater";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SocialInfo.Provider.SocialInfoProviderManager
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.SocialInfo.Provider.ISocialInfoProviderManagerStatics interface starting with version 1.0 of the Windows.ApplicationModel.SocialInfo.SocialInfoContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_Provider_SocialInfoProviderManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_Provider_SocialInfoProviderManager_DEFINED
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialInfoProviderManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SocialInfo_Provider_SocialInfoProviderManager[] = L"Windows.ApplicationModel.SocialInfo.Provider.SocialInfoProviderManager";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater;

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater;

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater;

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdaterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdaterVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdaterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdaterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdaterVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdaterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater;

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdaterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdaterVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdaterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdaterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdaterVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdaterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem;

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_FWD_DEFINED__

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem;

typedef struct __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItemVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem;

typedef struct __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItemVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItemVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem;

typedef struct __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem** items);

    END_INTERFACE
} __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItemVtbl;

interface __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem
{
    CONST_VTBL struct __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent;

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialFeedKind __x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialFeedKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialFeedUpdateMode __x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialFeedUpdateMode;

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail;

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

/*
 *
 * Interface Windows.ApplicationModel.SocialInfo.Provider.ISocialDashboardItemUpdater
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.SocialInfo.Provider.SocialDashboardItemUpdater
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_SocialInfo_Provider_ISocialDashboardItemUpdater[] = L"Windows.ApplicationModel.SocialInfo.Provider.ISocialDashboardItemUpdater";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdaterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_OwnerRemoteId)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_Timestamp)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_Thumbnail)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Thumbnail)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* CommitAsync)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_TargetUri)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_TargetUri)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdaterVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdaterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_get_OwnerRemoteId(This, value) \
    ((This)->lpVtbl->get_OwnerRemoteId(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_put_Timestamp(This, value) \
    ((This)->lpVtbl->put_Timestamp(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_put_Thumbnail(This, value) \
    ((This)->lpVtbl->put_Thumbnail(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_get_Thumbnail(This, value) \
    ((This)->lpVtbl->get_Thumbnail(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_CommitAsync(This, operation) \
    ((This)->lpVtbl->CommitAsync(This, operation))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_get_TargetUri(This, value) \
    ((This)->lpVtbl->get_TargetUri(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_put_TargetUri(This, value) \
    ((This)->lpVtbl->put_TargetUri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialDashboardItemUpdater_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.SocialInfo.Provider.ISocialFeedUpdater
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.SocialInfo.Provider.SocialFeedUpdater
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_SocialInfo_Provider_ISocialFeedUpdater[] = L"Windows.ApplicationModel.SocialInfo.Provider.ISocialFeedUpdater";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("ISocialFeedUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdaterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_OwnerRemoteId)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater* This,
        enum __x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialFeedKind* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Items)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater* This,
        __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialFeedItem** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* CommitAsync)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdaterVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdaterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_get_OwnerRemoteId(This, value) \
    ((This)->lpVtbl->get_OwnerRemoteId(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_get_Items(This, value) \
    ((This)->lpVtbl->get_Items(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_CommitAsync(This, operation) \
    ((This)->lpVtbl->CommitAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialFeedUpdater_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.SocialInfo.Provider.ISocialInfoProviderManagerStatics
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.SocialInfo.Provider.SocialInfoProviderManager
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_SocialInfo_Provider_ISocialInfoProviderManagerStatics[] = L"Windows.ApplicationModel.SocialInfo.Provider.ISocialInfoProviderManagerStatics";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* CreateSocialFeedUpdaterAsync)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics* This,
        enum __x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialFeedKind kind,
        enum __x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialFeedUpdateMode mode,
        HSTRING ownerRemoteId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialFeedUpdater** operation);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* CreateDashboardItemUpdaterAsync)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics* This,
        HSTRING ownerRemoteId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CSocialInfo__CProvider__CSocialDashboardItemUpdater** operation);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* UpdateBadgeCountValue)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics* This,
        HSTRING itemRemoteId,
        INT32 newCount);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* ReportNewContentAvailable)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics* This,
        HSTRING contactRemoteId,
        enum __x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialFeedKind kind);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* ProvisionAsync)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics* This,
        __FIAsyncOperation_1_boolean** operation);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* DeprovisionAsync)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_CreateSocialFeedUpdaterAsync(This, kind, mode, ownerRemoteId, operation) \
    ((This)->lpVtbl->CreateSocialFeedUpdaterAsync(This, kind, mode, ownerRemoteId, operation))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_CreateDashboardItemUpdaterAsync(This, ownerRemoteId, operation) \
    ((This)->lpVtbl->CreateDashboardItemUpdaterAsync(This, ownerRemoteId, operation))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_UpdateBadgeCountValue(This, itemRemoteId, newCount) \
    ((This)->lpVtbl->UpdateBadgeCountValue(This, itemRemoteId, newCount))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_ReportNewContentAvailable(This, contactRemoteId, kind) \
    ((This)->lpVtbl->ReportNewContentAvailable(This, contactRemoteId, kind))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_ProvisionAsync(This, operation) \
    ((This)->lpVtbl->ProvisionAsync(This, operation))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialInfoProviderManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_DeprovisionAsync(This, operation) \
    ((This)->lpVtbl->DeprovisionAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CProvider_CISocialInfoProviderManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SocialInfo.Provider.SocialDashboardItemUpdater
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.SocialInfo.Provider.ISocialDashboardItemUpdater ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_Provider_SocialDashboardItemUpdater_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_Provider_SocialDashboardItemUpdater_DEFINED
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialDashboardItemUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SocialInfo_Provider_SocialDashboardItemUpdater[] = L"Windows.ApplicationModel.SocialInfo.Provider.SocialDashboardItemUpdater";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SocialInfo.Provider.SocialFeedUpdater
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.SocialInfo.Provider.ISocialFeedUpdater ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_Provider_SocialFeedUpdater_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_Provider_SocialFeedUpdater_DEFINED
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialFeedUpdater is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SocialInfo_Provider_SocialFeedUpdater[] = L"Windows.ApplicationModel.SocialInfo.Provider.SocialFeedUpdater";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SocialInfo.Provider.SocialInfoProviderManager
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.SocialInfo.Provider.ISocialInfoProviderManagerStatics interface starting with version 1.0 of the Windows.ApplicationModel.SocialInfo.SocialInfoContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_Provider_SocialInfoProviderManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_Provider_SocialInfoProviderManager_DEFINED
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialInfoProviderManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SocialInfo_Provider_SocialInfoProviderManager[] = L"Windows.ApplicationModel.SocialInfo.Provider.SocialInfoProviderManager";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Esocialinfo2Eprovider_p_h__

#endif // __windows2Eapplicationmodel2Esocialinfo2Eprovider_h__
